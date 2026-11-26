//! Nonblocking scene GPU intervals. These measure elapsed GPU timeline time
//! between commands, including queue/CPU gaps, rather than GPU busy occupancy.
use super::viewport3d::RenderWork;
use glow::{self, HasContext};
use std::sync::{Arc, Mutex};
struct Slot {
    query: glow::Query,
    owner: Option<Arc<Mutex<RenderWork>>>,
}
#[derive(Default)]
pub(super) struct Pool {
    slots: Vec<Slot>,
    error: Option<String>,
    checked_support: bool,
}
pub(super) struct Interval<'g> {
    gl: &'g glow::Context,
}
impl Drop for Interval<'_> {
    fn drop(&mut self) {
        unsafe {
            self.gl.end_query(glow::TIME_ELAPSED);
        }
    }
}
impl Pool {
    pub unsafe fn poll(&mut self, gl: &glow::Context) {
        for slot in &mut self.slots {
            if slot.owner.is_none() {
                continue;
            }
            // With a query buffer bound, GL interprets the output pointer as a
            // buffer offset. Preserve that binding where the API is available.
            let query_buffer = gl.version().major > 4
                || (gl.version().major == 4 && gl.version().minor >= 4)
                || gl
                    .supported_extensions()
                    .contains("GL_ARB_query_buffer_object");
            let old = if query_buffer {
                std::num::NonZeroU32::new(gl.get_parameter_i32(glow::QUERY_BUFFER_BINDING) as u32)
                    .map(glow::NativeBuffer)
            } else {
                None
            };
            if query_buffer {
                gl.bind_buffer(glow::QUERY_BUFFER, None);
            }
            // Read availability through the entry point
            let mut available = 0u64;
            gl.get_query_parameter_u64_with_offset(
                slot.query,
                glow::QUERY_RESULT_AVAILABLE,
                (&mut available as *mut u64) as usize,
            );
            if available == 0 {
                if query_buffer {
                    gl.bind_buffer(glow::QUERY_BUFFER, old);
                }
                continue;
            }
            let mut ns = 0u64;
            gl.get_query_parameter_u64_with_offset(
                slot.query,
                glow::QUERY_RESULT,
                (&mut ns as *mut u64) as usize,
            );
            if query_buffer {
                gl.bind_buffer(glow::QUERY_BUFFER, old);
            }
            let owner = slot.owner.take().unwrap();
            let mut work = owner.lock().unwrap();
            let ms = ns as f64 / 1_000_000.;
            work.gpu_samples += 1;
            work.gpu_total_ms += ms;
            work.gpu_max_ms = work.gpu_max_ms.max(ms);
        }
    }
    pub unsafe fn begin<'g>(
        &mut self,
        gl: &'g glow::Context,
        owner: Arc<Mutex<RenderWork>>,
    ) -> Option<Interval<'g>> {
        self.poll(gl);
        if !self.checked_support {
            self.checked_support = true;
            let extensions = gl.supported_extensions();
            if !(extensions.contains("GL_ARB_timer_query")
                || extensions.contains("GL_EXT_timer_query")
                || extensions.contains("GL_EXT_disjoint_timer_query"))
            {
                self.error = Some("GPU timer queries are unsupported by this driver".to_string());
            }
        }
        if self.slots.is_empty() && self.error.is_none() {
            // Four diagnostic samples in flight, independent of game work.
            for _ in 0..4 {
                match gl.create_query() {
                    Ok(query) => self.slots.push(Slot { query, owner: None }),
                    Err(error) => {
                        self.error = Some(error);
                        break;
                    }
                }
            }
        }
        if let Some(error) = &self.error {
            owner.lock().unwrap().gpu_timing_error = Some(error.clone());
            return None;
        }
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.owner.is_none()) else {
            owner.lock().unwrap().gpu_skipped_samples += 1;
            return None;
        };
        gl.begin_query(glow::TIME_ELAPSED, slot.query);
        slot.owner = Some(owner);
        Some(Interval { gl })
    }
    pub unsafe fn destroy(&mut self, gl: &glow::Context) {
        for slot in self.slots.drain(..) {
            gl.delete_query(slot.query);
        }
        self.error = None;
        self.checked_support = false;
    }
}
pub(super) unsafe fn validate(gl: &glow::Context) -> anyhow::Result<serde_json::Value> {
    use anyhow::{ensure, Context};
    let owner = Arc::new(Mutex::new(RenderWork::default()));
    let mut pool = Pool::default();
    {
        let _interval = pool
            .begin(gl, owner.clone())
            .context("GPU interval query unavailable")?;
        gl.clear(glow::COLOR_BUFFER_BIT);
    }
    // Only this explicit hardware validation waits. Production polls AVAILABLE
    // once per pending slot and never calls Finish or waits for a query.
    gl.finish();
    pool.poll(gl);
    pool.destroy(gl);
    let work = owner.lock().unwrap();
    ensure!(
        work.gpu_samples == 1 && work.gpu_total_ms.is_finite() && work.gpu_total_ms >= 0.,
        "GPU interval did not resolve"
    );
    ensure!(gl.get_error() == glow::NO_ERROR, "GPU interval GL error");
    Ok(
        serde_json::json!({"samples":work.gpu_samples,"elapsed_ms":work.gpu_total_ms,"production_waits":false}),
    )
}
