//! Read-only runtime diagnostics. Sampling never advances simulation.
use super::{assets::World, authored_preview::Preview, controller::Controller, input::Pad};
use std::{collections::VecDeque, time::Instant};

struct MemorySample {
    sampled_at: Instant,
    geometry: usize,
    images: usize,
    collision: usize,
    prefab_sources: usize,
    inactive_scenes: usize,
    compressed_scene_assets: usize,
    box3d_bytes: usize,
    gl_asset_payload_bytes: usize,
    gl_opaque_texture_savings_bytes: usize,
    rust_heap: Option<(usize, usize)>,
    heaps: Option<ProcessHeaps>,
    process: Option<(usize, usize)>,
    scripts: super::actor_scripts::MemorySummary,
}

fn add_unique_buffer(
    buffers: &mut std::collections::HashSet<usize>,
    ptr: usize,
    bytes: usize,
    total: &mut usize,
) {
    if buffers.insert(ptr) {
        *total = total.saturating_add(bytes);
    }
}

fn add_component_storage_bytes(
    component: &epic_mickey_lib_rs::scene_file::Component,
    total: &mut usize,
) {
    *total = total.saturating_add(component.class_name.capacity());
    let property_size = std::mem::size_of::<epic_mickey_lib_rs::scene_file::Property>();
    *total = total.saturating_add(component.properties.capacity().saturating_mul(property_size));
    for property in &component.properties {
        *total = total.saturating_add(property.name.capacity());
        *total = total.saturating_add(json_value_heap_bytes(&property.value));
    }
}

fn json_value_heap_bytes(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::String(value) => value.capacity(),
        serde_json::Value::Array(values) => values
            .capacity()
            .saturating_mul(std::mem::size_of::<serde_json::Value>())
            .saturating_add(values.iter().map(json_value_heap_bytes).sum::<usize>()),
        serde_json::Value::Object(values) => values
            .iter()
            .map(|(key, value)| key.capacity().saturating_add(json_value_heap_bytes(value)))
            .sum(),
        _ => 0,
    }
}

fn add_model_source_bytes(
    models: &std::collections::BTreeMap<String, Vec<u8>>,
    buffers: &mut std::collections::HashSet<usize>,
    prefab_sources: &mut usize,
) {
    for bytes in models.values() {
        if !bytes.is_empty() {
            add_unique_buffer(
                buffers,
                bytes.as_ptr() as usize,
                bytes.capacity(),
                prefab_sources,
            );
        }
    }
}

fn insert_size(buffers: &mut std::collections::HashMap<usize, usize>, ptr: usize, bytes: usize) {
    buffers.entry(ptr).or_insert(bytes);
}

/// Approximate retained CPU asset payload, counting shared Arc buffers once.
/// This deliberately excludes allocator overhead, GPU allocations and most
/// runtime state or native Box3D mesh allocations; it is an ownership
/// trend, not a process-RSS reading.
pub(super) fn tracked_asset_bytes(world: &World) -> (usize, usize, usize, usize, usize) {
    let mut buffers = std::collections::HashSet::<usize>::new();
    let mut geometry_buffers = std::collections::HashMap::<usize, usize>::new();
    let mut image_buffers = std::collections::HashMap::<usize, usize>::new();
    let mut geometry = 0usize;
    let mut images = 0usize;
    // Authored source meshes remain available for contact-feature metadata;
    // Box3D-owned query meshes are excluded because their allocator is native.
    let mut collision = 0usize;
    let mut prefab_sources = 0usize;
    let mut inactive_geometry = std::collections::HashMap::<usize, usize>::new();
    let mut active_geometry = std::collections::HashSet::<usize>::new();
    let mut inactive_images = std::collections::HashMap::<usize, usize>::new();
    let mut active_images = std::collections::HashSet::<usize>::new();
    let mut inactive_collision = std::collections::HashMap::<usize, usize>::new();
    let mut active_collision = std::collections::HashSet::<usize>::new();
    for draw in &world.draws {
        let vertices = std::sync::Arc::as_ptr(&draw.v_data) as usize;
        let vertex_bytes = draw.v_data.capacity() * std::mem::size_of::<f32>();
        insert_size(&mut geometry_buffers, vertices, vertex_bytes);
        add_unique_buffer(&mut buffers, vertices, vertex_bytes, &mut geometry);
        let indices = std::sync::Arc::as_ptr(&draw.indices) as usize;
        let index_bytes = draw.indices.capacity() * std::mem::size_of::<u32>();
        insert_size(&mut geometry_buffers, indices, index_bytes);
        add_unique_buffer(&mut buffers, indices, index_bytes, &mut geometry);
        if let Some((bytes, _, _)) = &draw.tex_bytes {
            let ptr = std::sync::Arc::as_ptr(bytes) as usize;
            insert_size(&mut image_buffers, ptr, bytes.capacity());
            add_unique_buffer(&mut buffers, ptr, bytes.capacity(), &mut images);
        }
        for image in draw.material_images.iter() {
            let ptr = std::sync::Arc::as_ptr(&image.rgba) as usize;
            insert_size(&mut image_buffers, ptr, image.rgba.capacity());
            add_unique_buffer(&mut buffers, ptr, image.rgba.capacity(), &mut images);
        }
    }
    for (entity_index, entity) in world.entities.iter().enumerate() {
        let active = !world.removed_entities.contains(&entity_index)
            && world.entity_scene_active(entity_index);
        for draw_index in entity.draws.clone() {
            let Some(draw) = world.draws.get(draw_index) else {
                continue;
            };
            let vertices = std::sync::Arc::as_ptr(&draw.v_data) as usize;
            let indices = std::sync::Arc::as_ptr(&draw.indices) as usize;
            for ptr in [vertices, indices] {
                if active {
                    active_geometry.insert(ptr);
                } else {
                    let size = geometry_buffers.get(&ptr).copied().unwrap_or(0);
                    insert_size(&mut inactive_geometry, ptr, size);
                }
            }
            let mut add_image = |ptr: usize, size: usize| {
                if active {
                    active_images.insert(ptr);
                } else {
                    insert_size(&mut inactive_images, ptr, size);
                }
            };
            if let Some((bytes, _, _)) = &draw.tex_bytes {
                add_image(std::sync::Arc::as_ptr(bytes) as usize, bytes.capacity());
            }
            for image in draw.material_images.iter() {
                add_image(
                    std::sync::Arc::as_ptr(&image.rgba) as usize,
                    image.rgba.capacity(),
                );
            }
        }
    }
    for part in &world.report.collision_parts {
        let resource = std::sync::Arc::as_ptr(&part.native_resource) as usize;
        let is_new_resource = !buffers.contains(&resource);
        let part_bytes = if is_new_resource {
            part.native_resource
                .iter()
                .map(|mesh| {
                    mesh.vertices.capacity() * std::mem::size_of::<[f32; 3]>()
                        + mesh.indices.capacity() * std::mem::size_of::<u32>()
                })
                .sum::<usize>()
        } else {
            0
        };
        let active = world.scene_resources.is_active(&part.scene);
        if active {
            active_collision.insert(resource);
        } else {
            insert_size(&mut inactive_collision, resource, part_bytes);
        }
        if buffers.insert(resource) {
            collision = collision.saturating_add(part_bytes);
        }
    }
    for graph in world.guardian_templates.values() {
        add_model_source_bytes(&graph.models, &mut buffers, &mut prefab_sources);
    }
    for template in world.sketch_templates.templates.values() {
        add_model_source_bytes(&template.graph.models, &mut buffers, &mut prefab_sources);
    }
    for bytes in world.sketch_templates.resources.values() {
        if !bytes.is_empty() {
            add_unique_buffer(
                &mut buffers,
                std::sync::Arc::as_ptr(bytes) as *const u8 as usize,
                bytes.len(),
                &mut prefab_sources,
            );
        }
    }
    let inactive_scenes = inactive_geometry
        .into_iter()
        .filter(|(ptr, _)| !active_geometry.contains(ptr))
        .map(|(_, bytes)| bytes)
        .chain(
            inactive_images
                .into_iter()
                .filter(|(ptr, _)| !active_images.contains(ptr))
                .map(|(_, bytes)| bytes),
        )
        .chain(
            inactive_collision
                .into_iter()
                .filter(|(ptr, _)| !active_collision.contains(ptr))
                .map(|(_, bytes)| bytes),
        )
        .sum();
    (geometry, images, collision, prefab_sources, inactive_scenes)
}

/// Estimate runtime-owned entity identity/component/property allocations.
/// This reports container capacities and nested JSON heap payload; it excludes
/// HashMap/Vec allocator metadata and unrelated subsystem state.
pub(super) fn runtime_entity_heap_bytes(world: &World) -> (usize, usize, usize) {
    let mut identity = world
        .entities
        .capacity()
        .saturating_mul(std::mem::size_of::<super::assets::RuntimeEntity>());
    let mut components = 0usize;
    let mut properties = 0usize;
    for entity in &world.entities {
        identity = identity
            .saturating_add(entity.scene.capacity())
            .saturating_add(entity.name.capacity())
            .saturating_add(
                entity
                    .draws
                    .len()
                    .saturating_mul(std::mem::size_of::<usize>()),
            )
            .saturating_add(
                entity
                    .collision_parts
                    .len()
                    .saturating_mul(std::mem::size_of::<usize>()),
            );
        components = components.saturating_add(
            entity
                .components
                .capacity()
                .saturating_mul(std::mem::size_of::<epic_mickey_lib_rs::scene_file::Component>()),
        );
        for component in &entity.components {
            components = components.saturating_add(component.class_name.capacity());
            properties = properties.saturating_add(
                component.properties.capacity().saturating_mul(
                    std::mem::size_of::<epic_mickey_lib_rs::scene_file::Property>(),
                ),
            );
            for property in &component.properties {
                properties = properties
                    .saturating_add(property.class_name.capacity())
                    .saturating_add(property.name.capacity())
                    .saturating_add(json_value_heap_bytes(&property.value));
            }
        }
    }
    (identity, components, properties)
}

/// Allocator-container totals for this process
pub(super) struct ProcessHeaps {
    /// Containers whose counters were readable (Windows heaps or macOS malloc zones)
    pub(super) containers: usize,
    /// Bytes allocated from those containers
    pub(super) allocated: usize,
    /// Bytes of backing virtual memory the containers hold
    pub(super) committed: usize,
    /// Bytes reserved. I believe this is Windows only
    pub(super) reserved: Option<usize>,
    /// Platform name for the containers
    pub(super) label: &'static str,
}

/// Names for the overlay counters
#[cfg(windows)]
pub(super) const PROCESS_MEMORY_LABELS: (&str, &str) = ("working set", "private");
#[cfg(target_os = "macos")]
pub(super) const PROCESS_MEMORY_LABELS: (&str, &str) = ("resident", "footprint");
#[cfg(not(any(windows, target_os = "macos")))]
pub(super) const PROCESS_MEMORY_LABELS: (&str, &str) = ("working set", "private");

/// Return (working set, private bytes) for this process when the host exposes
/// process counters. The asset-payload estimate below remains useful on other
/// platforms and is intentionally reported separately.
pub(super) fn process_memory_bytes() -> Option<(usize, usize)> {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::{
            ProcessStatus::{
                K32GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS, PROCESS_MEMORY_COUNTERS_EX,
            },
            Threading::GetCurrentProcess,
        };
        let mut counters = PROCESS_MEMORY_COUNTERS_EX::default();
        counters.cb = std::mem::size_of::<PROCESS_MEMORY_COUNTERS_EX>() as u32;
        let success = unsafe {
            K32GetProcessMemoryInfo(
                GetCurrentProcess(),
                (&mut counters as *mut PROCESS_MEMORY_COUNTERS_EX)
                    .cast::<PROCESS_MEMORY_COUNTERS>(),
                counters.cb,
            )
        };
        return (success != 0).then_some((counters.WorkingSetSize, counters.PrivateUsage));
    }
    #[cfg(target_os = "linux")]
    {
        let status = std::fs::read_to_string("/proc/self/status").ok()?;
        let value_kib = |key: &str| {
            status
                .lines()
                .find_map(|line| line.strip_prefix(key))?
                .split_whitespace()
                .next()?
                .parse::<usize>()
                .ok()
                .map(|kib| kib * 1024)
        };
        return Some((value_kib("VmRSS:")?, value_kib("VmData:")?));
    }
    #[cfg(target_os = "macos")]
    {
        // Mach shows the phys_footprint ledger
        let (resident, footprint) = macos::task_memory()?;
        return Some((resident, footprint));
    }
    #[cfg(not(any(windows, target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

/// Summarize process heaps when HeapSummary exists on this OS. Resolve
/// dynamically because HeapSummary is only available on recent Windows
/// releases; older systems simply omit this diagnostic. The figures aren't
/// really compatible with each other
pub(super) fn process_heap_bytes() -> Option<ProcessHeaps> {
    #[cfg(windows)]
    {
        use std::mem::transmute;
        use windows_sys::core::BOOL;
        use windows_sys::Win32::{
            Foundation::HANDLE,
            System::{
                LibraryLoader::{GetModuleHandleA, GetProcAddress},
                Memory::{GetProcessHeaps, HEAP_SUMMARY},
            },
        };
        let heap_count = unsafe { GetProcessHeaps(0, std::ptr::null_mut()) };
        if heap_count == 0 {
            return None;
        }
        let mut heaps = vec![std::ptr::null_mut::<std::ffi::c_void>(); heap_count as usize];
        let heap_count = unsafe { GetProcessHeaps(heap_count, heaps.as_mut_ptr()) };
        if heap_count == 0 || heap_count as usize > heaps.len() {
            return None;
        }
        let module = unsafe { GetModuleHandleA(b"kernel32.dll\0".as_ptr()) };
        if module.is_null() {
            return None;
        }
        let address = unsafe { GetProcAddress(module, b"HeapSummary\0".as_ptr()) }?;
        let summarize: unsafe extern "system" fn(HANDLE, u32, *mut HEAP_SUMMARY) -> BOOL =
            unsafe { transmute(address) };
        let mut totals = (0usize, 0usize, 0usize, 0usize);
        for &heap in heaps.iter().take(heap_count as usize) {
            if heap.is_null() {
                continue;
            }
            let mut summary = HEAP_SUMMARY {
                cb: std::mem::size_of::<HEAP_SUMMARY>() as u32,
                ..Default::default()
            };
            if unsafe { summarize(heap, 0, &mut summary) } != 0 {
                totals.0 += 1;
                totals.1 = totals.1.saturating_add(summary.cbAllocated);
                totals.2 = totals.2.saturating_add(summary.cbCommitted);
                totals.3 = totals.3.saturating_add(summary.cbReserved);
            }
        }
        (totals.0 > 0).then_some(ProcessHeaps {
            containers: totals.0,
            allocated: totals.1,
            committed: totals.2,
            reserved: Some(totals.3),
            label: "Windows heaps",
        })
    }
    #[cfg(target_os = "macos")]
    {
        let (containers, allocated, committed) = macos::malloc_zone_totals()?;
        return Some(ProcessHeaps {
            containers,
            allocated,
            committed,
            reserved: None,
            label: "Malloc zones",
        });
    }
    #[cfg(not(any(windows, target_os = "macos")))]
    {
        None
    }
}

/// Mach and malloc-zone counters
#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::{c_int, c_void};

    type KernReturn = c_int;
    type MachPort = u32;

    const MACH_TASK_BASIC_INFO: u32 = 20;
    const TASK_VM_INFO: u32 = 22;

    #[repr(C)]
    #[derive(Default)]
    #[allow(dead_code)]
    struct TimeValue {
        seconds: i32,
        microseconds: i32,
    }

    /// `mach_task_basic_info_data_t`: 48 bytes
    #[repr(C)]
    #[derive(Default)]
    #[allow(dead_code)]
    struct MachTaskBasicInfo {
        virtual_size: u64,
        resident_size: u64,
        resident_size_max: u64,
        user_time: TimeValue,
        system_time: TimeValue,
        policy: i32,
        suspend_count: i32,
    }

    const _: () = assert!(std::mem::size_of::<MachTaskBasicInfo>() == 48);

    /// The kernel rejects a buffer shorter than what its revised by, so pad it
    #[repr(C)]
    #[derive(Default)]
    #[allow(dead_code)]
    struct TaskVmInfo {
        virtual_size: u64,
        region_count: i32,
        page_size: i32,
        resident_size: u64,
        resident_size_peak: u64,
        device: u64,
        device_peak: u64,
        internal: u64,
        internal_peak: u64,
        external: u64,
        external_peak: u64,
        reusable: u64,
        reusable_peak: u64,
        purgeable_volatile_pmap: u64,
        purgeable_volatile_resident: u64,
        purgeable_volatile_virtual: u64,
        compressed: u64,
        compressed_peak: u64,
        compressed_lifetime: u64,
        phys_footprint: u64,
        // Nested so the padding can derive Default
        trailing: [[u64; 8]; 14],
    }

    const _: () = assert!(std::mem::size_of::<TaskVmInfo>() >= 1024);

    /// `malloc_statistics_t`.
    #[repr(C)]
    #[derive(Default)]
    #[allow(dead_code)]
    struct MallocStatistics {
        blocks_in_use: u32,
        size_in_use: usize,
        max_size_in_use: usize,
        size_allocated: usize,
    }

    const _: () = assert!(std::mem::size_of::<MallocStatistics>() == 32);

    unsafe extern "C" {
        /// `mach_task_self()` is a macro over libSystem global
        static mach_task_self_: MachPort;
        fn task_info(
            target_task: MachPort,
            flavor: u32,
            task_info_out: *mut c_int,
            task_info_out_count: *mut u32,
        ) -> KernReturn;
        fn malloc_get_all_zones(
            task: MachPort,
            reader: Option<unsafe extern "C" fn(MachPort, usize, usize, *mut *mut c_void) -> KernReturn>,
            addresses: *mut *mut usize,
            count: *mut u32,
        ) -> KernReturn;
        fn malloc_zone_statistics(zone: *const c_void, stats: *mut MallocStatistics);
    }

    /// `(resident set, phys_footprint)`
    pub(super) fn task_memory() -> Option<(usize, usize)> {
        let task = unsafe { mach_task_self_ };
        let mut basic = MachTaskBasicInfo::default();
        let mut count =
            (std::mem::size_of::<MachTaskBasicInfo>() / std::mem::size_of::<u32>()) as u32;
        let status = unsafe {
            task_info(
                task,
                MACH_TASK_BASIC_INFO,
                std::ptr::from_mut(&mut basic).cast::<c_int>(),
                &mut count,
            )
        };
        if status != 0 {
            return None;
        }
        let mut vm = TaskVmInfo::default();
        let mut count = (std::mem::size_of::<TaskVmInfo>() / std::mem::size_of::<u32>()) as u32;
        let status = unsafe {
            task_info(
                task,
                TASK_VM_INFO,
                std::ptr::from_mut(&mut vm).cast::<c_int>(),
                &mut count,
            )
        };
        if status != 0 {
            return None;
        }
        Some((basic.resident_size as usize, vm.phys_footprint as usize))
    }

    pub(super) fn malloc_zone_totals() -> Option<(usize, usize, usize)> {
        let task = unsafe { mach_task_self_ };
        let mut addresses: *mut usize = std::ptr::null_mut();
        let mut count = 0u32;
        let status = unsafe { malloc_get_all_zones(task, None, &mut addresses, &mut count) };
        if status != 0 || addresses.is_null() || count == 0 {
            return None;
        }
        let zones = unsafe { std::slice::from_raw_parts(addresses, count as usize) };
        let mut containers = 0usize;
        let mut allocated = 0usize;
        let mut committed = 0usize;
        for &zone in zones {
            if zone == 0 {
                continue;
            }
            let mut stats = MallocStatistics::default();
            unsafe { malloc_zone_statistics(zone as *const c_void, &mut stats) };
            containers += 1;
            allocated = allocated.saturating_add(stats.size_in_use);
            committed = committed.saturating_add(stats.size_allocated);
        }
        (containers > 0).then_some((containers, allocated, committed))
    }
}

/// Re-evaluate the current mover's candidates without publishing response or
/// changing simulation state. Missing native shape types remain visible errors.
pub(super) fn native_contact_probe(
    world: &World,
    position: [f32; 3],
) -> anyhow::Result<Vec<String>> {
    use super::{
        native_gsk::{Cache, ConvexSupport},
        native_hk_transform, shape_welding,
    };
    use anyhow::{ensure, Context};
    let owners = world.collision.contact_owners();
    let sources = world.collision.contact_sources();
    ensure!(
        owners.len() == sources.len(),
        "Contact snapshot lengths changed"
    );
    let imported = world
        .report
        .player_native_support
        .as_ref()
        .context("Missing Mickey native support shape")?;
    let mut support = ConvexSupport {
        packed_vertices: imported.packed_vertices.clone(),
        vertex_count: imported.vertex_count,
        stored_center: imported.stored_center,
    };
    let radius = world
        .report
        .player_collision
        .as_ref()
        .context("Missing Mickey collision radius")?
        .radius;
    let pose = native_hk_transform::from_entity([1., 0., 0., 0., 1., 0., 0., 0., 1.], position);
    let mut lines = vec![format!(
        "Native probe: {} candidates / live static poses / no penetration solve",
        owners.len()
    )];
    for (index, (owner, (_, triangle))) in owners.into_iter().zip(sources).enumerate() {
        let probe = (|| -> anyhow::Result<String> {
            let owner = owner.context("Missing contact owner")?;
            let triangle = triangle.context("Contact has no supported triangle identity")?;
            let (mut shape, transform) = world.initial_static_contact(owner, triangle)?;
            let relative = native_hk_transform::relative(&pose, &transform);
            let mut cache = Cache([0; 12]);
            cache.initialize_triangle(&mut support, shape.support.vertices, &relative)?;
            let result = shape_welding::query_cached_triangle_contact(
                &mut cache,
                &mut support,
                &mut shape.support,
                &pose,
                &transform,
                radius,
                shape.radius,
                f32::MAX,
                false,
                &mut [[0.; 4]; 4],
                None,
                true,
                shape.mode,
                shape.word,
                &world.collision.contact_welding,
            )?;
            let identity = format!("Native #{index}: owner {owner} triangle {triangle}");
            Ok(match result {
                shape_welding::ClosestQuery::Contact(contact) => format!(
                    "{identity} d={:.7} n={:.3?}",
                    contact.separating[3],
                    &contact.separating[..3]
                ),
                shape_welding::ClosestQuery::NoContact => format!("{identity} welding rejected"),
                shape_welding::ClosestQuery::Unresolved => {
                    format!("{identity} requires penetration solve (scratch unavailable)")
                }
            })
        })();
        lines.push(probe.unwrap_or_else(|e| format!("Native #{index}: {e:#}")));
    }
    Ok(lines)
}

pub struct DebugView {
    pub open: bool,
    pub performance_lines: Vec<String>,
    pub export_performance: bool,
    developer: bool,
    selected: Option<usize>,
    pointer: Option<(super::cursor::Ray, Option<super::physics::StaticRayHit>)>,
    pointer_error: Option<String>,
    pointer_surfaces: usize,
    last: Instant,
    frames: VecDeque<f32>,
    native_contacts: bool,
    native_contact_lines: Vec<String>,
    phantom_bounds_errors: std::cell::RefCell<Vec<String>>,
    memory_sample: Option<MemorySample>,
}
impl DebugView {
    pub fn new(open: bool) -> Self {
        Self {
            open,
            performance_lines: Vec::new(),
            export_performance: false,
            developer: false,
            selected: None,
            pointer: None,
            pointer_error: None,
            pointer_surfaces: 0,
            last: Instant::now(),
            frames: VecDeque::with_capacity(240),
            native_contacts: false,
            native_contact_lines: Vec::new(),
            phantom_bounds_errors: Default::default(),
            memory_sample: None,
        }
    }
    pub fn native_contact_probe_enabled(&self) -> bool {
        self.native_contacts
    }
    pub fn clear_pointer(&mut self) {
        self.pointer = None;
        self.pointer_error = None;
        self.pointer_surfaces = 0;
    }
    pub fn probe_pointer(
        &mut self,
        camera: &super::cursor::Camera,
        pixel: [i32; 2],
        size: [u32; 2],
        collision: &super::physics::Collision,
        max: f32,
    ) {
        self.clear_pointer();
        match camera.window_ray(pixel, size) {
            Ok(Some(ray)) => match collision.ray_hits(ray.origin, ray.direction.map(|v| v * max)) {
                Ok(hits) => {
                    self.pointer_surfaces = hits.len();
                    let nearest = hits
                        .into_iter()
                        .min_by(|a, b| a.fraction.total_cmp(&b.fraction));
                    self.pointer = Some((ray, nearest));
                }
                Err(error) => self.pointer_error = Some(error.to_string()),
            },
            Ok(None) => {}
            Err(error) => self.pointer_error = Some(error.to_string()),
        }
    }
    pub fn reset_selection(&mut self) {
        self.selected = None;
        self.phantom_bounds_errors.borrow_mut().clear();
    }
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let ms = now.duration_since(self.last).as_secs_f32() * 1000.;
        self.last = now;
        if self.frames.len() == 240 {
            self.frames.pop_front();
        }
        self.frames.push_back(ms);
    }
    pub fn frame_timing_summary(&self) -> serde_json::Value {
        let mut values = self.frames.iter().copied().collect::<Vec<_>>();
        values.sort_by(f32::total_cmp);
        let mean = values.iter().sum::<f32>() / values.len().max(1) as f32;
        let percentile = |p: f32| {
            values
                .get(((values.len().saturating_sub(1) as f32 * p).ceil()) as usize)
                .copied()
        };
        serde_json::json!({"scope":"Recent application callback intervals, not display presents or GPU occupancy",
            "samples":values.len(),"window_limit":240,"mean_ms":mean,
            "callbacks_per_second":if mean>0. {Some(1000./mean)}else{None},
            "p50_ms":percentile(0.5),"p95_ms":percentile(0.95),"p99_ms":percentile(0.99),"max_ms":values.last()})
    }
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        world: Option<&World>,
        controller: Option<&Controller>,
        animation: Option<&Preview>,
        pad: &Pad,
        paused: bool,
        error: Option<&str>,
    ) {
        if !self.open {
            return;
        }
        self.native_contact_lines.clear();
        if self.native_contacts {
            if let (Some(world), Some(controller)) = (world, controller) {
                match native_contact_probe(world, controller.player.position) {
                    Ok(lines) => self.native_contact_lines = lines,
                    Err(error) => self
                        .native_contact_lines
                        .push(format!("Native contact probe: {error:#}")),
                }
            }
        }
        self.game_overlay(ctx, world, controller, animation, pad, paused);
        if !self.developer {
            return;
        }
        egui::Window::new("Blotworx • Runtime diagnostics")
            .open(&mut self.developer).default_pos([18., 105.]).default_width(440.)
            .default_height(610.).vscroll(true).show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.colored_label(if paused { egui::Color32::YELLOW } else { egui::Color32::LIGHT_GREEN },
                    if paused { "PAUSED" } else { "RUNNING" });
                ui.label("F3 to hide · Read-only telemetry");
            });
            if let Some(e) = error { ui.colored_label(egui::Color32::LIGHT_RED, e); }
            ui.checkbox(&mut self.native_contacts,"Probe native static contacts");
            ui.small("Diagnostic only; response still uses Box3D. Probe time affects frame timing.");
            ui.separator();
            let mean = self.frames.iter().sum::<f32>() / self.frames.len().max(1) as f32;
            let peak = self.frames.iter().copied().fold(0., f32::max);
            ui.label(format!("Application callbacks  {:.1}/s   {:.2} ms avg   {:.2} ms peak", 1000. / mean.max(0.001), mean, peak));
            ui.small("Last 240 UI frames · wall time, not GPU execution time");
            let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 65.), egui::Sense::hover());
            ui.painter().rect_filled(rect, 3., egui::Color32::from_rgb(15, 21, 29));
            let scale = peak.max(33.334);
            let y = rect.bottom() - 33.334 / scale * rect.height();
            ui.painter().line_segment([egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)], egui::Stroke::new(1., egui::Color32::DARK_GRAY));
            let points: Vec<_> = self.frames.iter().enumerate().map(|(i, ms)| egui::pos2(
                rect.left() + i as f32 / 239. * rect.width(), rect.bottom() - ms / scale * rect.height())).collect();
            if points.len() > 1 { ui.painter().add(egui::Shape::line(points, egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE))); }
            ui.collapsing("Controller / collision", |ui| {
                if let Some(c) = controller {
                    let p = &c.player;
                    ui.monospace(format!("Tick {} · 30 Hz simulation\nPosition   {:.4?}\nVelocity   {:.4?}\nSpeed      {:.4}\nGrounded   {}\nFacing     {:.4} rad\nCamera     {:.4} / {:.4} rad\nRespawns   {}\nSpawn      {:.4?}", c.ticks, p.position, p.velocity,
                        super::physics::length(p.velocity), p.grounded, p.yaw, c.camera_yaw, c.camera_pitch, p.respawns, p.spawn));
                    ui.monospace(format!("Jump ground/air/second: {} / {} / {}", p.ground_jump_started, p.air_first_jump_started, p.second_jump_started));
                } else { ui.label("Controller not loaded"); }
            });
            ui.collapsing("Input devices", |ui| {
                ui.monospace(format!("Left stick  {:.3?}\nButtons     0x{:04X}\nWindow focus {}", pad.left, pad.buttons, ctx.input(|i| i.focused)));
            });
            ui.collapsing("Animation / abilities", |ui| {
                if let Some(a) = animation {
                    ui.label(format!("Pose bones: {} · Observed events: {}", a.pose.len(), a.observed_events));
                    ui.label(format!("Events this tick: {} · Unapplied combat queries: {}", a.delivered_events.len(), a.pending_combat_queries));
                }
                ui.small("Combat queries are diagnostic; damage is not executed.");
            });
            egui::CollapsingHeader::new("Paint / thinner integration").default_open(true).show(ui, |ui| {
                if let Some(w) = world {
                    ui.monospace(format!("Ammo paint / thinner: {} / {}\nDecal resources: {}\nStimulus definitions: {}", w.ammo.available(0), w.ammo.available(1), w.brush_decals.len(), w.brush_stimuli.len()));
                }
                ui.colored_label(egui::Color32::YELLOW, "Firing and impact gameplay not connected yet");
                ui.small("Imported resources and tested decal geometry do not indicate a playable brush.");
            });
            if let Some(w) = world {
                let refresh_memory = self
                    .memory_sample
                    .as_ref()
                    .is_none_or(|sample| sample.sampled_at.elapsed().as_secs_f32() >= 1.);
                if refresh_memory {
                    let (geometry, images, collision, prefab_sources, inactive_scenes) =
                        tracked_asset_bytes(w);
                    self.memory_sample = Some(MemorySample {
                        sampled_at: Instant::now(),
                        geometry,
                        images,
                        collision,
                        prefab_sources,
                        inactive_scenes,
                        compressed_scene_assets: w
                            .cold_scene_assets
                            .values()
                            .map(super::assets::ColdSceneAssets::compressed_bytes)
                            .sum(),
                        box3d_bytes: super::physics::box3d_allocated_bytes(),
                        gl_asset_payload_bytes:
                            crate::render_support::viewport3d::cached_scene_asset_payload_bytes(),
                        gl_opaque_texture_savings_bytes:
                            crate::render_support::viewport3d::cached_opaque_texture_savings_bytes(),
                        rust_heap: crate::memory_profile::snapshot(),
                        heaps: process_heap_bytes(),
                        process: process_memory_bytes(),
                        scripts: w.scripts.memory_summary(),
                    });
                }
                let memory = self.memory_sample.as_ref().unwrap();
                ui.collapsing("Scene / renderer", |ui| {
                    ui.label(&w.report.level);
                    ui.monospace(format!("Entities {} · Draw commands {}\nRender triangles {} · Collision triangles {}\nTextures {} · Lights {}\nScenes {} · Packs {}", w.entities.len(), w.draws.len(), w.report.render_triangles, w.collision.triangle_count(), w.report.textures, w.lights.len(), w.report.scenes.len(), w.report.packs.len()));
                    ui.monospace(format!("Tracked CPU asset payload ~{:.1} MiB (geometry {:.1} · images {:.1} · collision {:.1} · prefab sources {:.1})\nShared buffers counted once; excludes GPU, Box3D, allocator overhead and runtime state", (memory.geometry + memory.images + memory.collision + memory.prefab_sources) as f64 / 1_048_576., memory.geometry as f64 / 1_048_576., memory.images as f64 / 1_048_576., memory.collision as f64 / 1_048_576., memory.prefab_sources as f64 / 1_048_576.));
                    ui.monospace(format!("Box3D live allocations: {:.2} MiB (Box3D native allocator counter)", memory.box3d_bytes as f64 / 1_048_576.));
                    ui.monospace(format!("Estimated GL scene-asset storage: {:.2} MiB (includes texture mip chains; excludes driver overhead and render targets)", memory.gl_asset_payload_bytes as f64 / 1_048_576.));
                    ui.monospace(format!("Opaque texture RGB storage saves ~{:.2} MiB versus RGBA8", memory.gl_opaque_texture_savings_bytes as f64 / 1_048_576.));
                    ui.monospace(format!("Legacy 2048² shadow target: {} (about 16 MiB depth storage when allocated)", if crate::render_support::viewport3d::legacy_shadow_target_allocated() { "allocated" } else { "not allocated" }));
                    if let Some((live, peak)) = memory.rust_heap {
                        ui.monospace(format!("Rust heap requested bytes: {:.1} MiB live · {:.1} MiB peak (profiling build)", live as f64 / 1_048_576., peak as f64 / 1_048_576.));
                    }
                    ui.monospace(format!("Lua VM {:.1} MiB · imported scripts {:.1} MiB across {} files · {} loaded scripts · {} pending tasks (task slots {:.2} MiB)", memory.scripts.lua_heap_bytes as f64 / 1_048_576., memory.scripts.imported_source_payload_bytes as f64 / 1_048_576., memory.scripts.imported_source_count, memory.scripts.loaded_script_count, memory.scripts.pending_task_count, memory.scripts.pending_task_record_capacity_bytes as f64 / 1_048_576.));
                    if let Some(heaps) = &memory.heaps {
                        let mut line = format!("{} ({}): {:.1} MiB allocated · {:.1} MiB committed", heaps.label, heaps.containers, heaps.allocated as f64 / 1_048_576., heaps.committed as f64 / 1_048_576.);
                        if let Some(reserved) = heaps.reserved {
                            line.push_str(&format!(" · {:.1} MiB reserved", reserved as f64 / 1_048_576.));
                        }
                        ui.monospace(line);
                    }
                    ui.monospace(format!("Inactive scenes retain ~{:.1} MiB of unique render + authored collision source data\nExcludes shared assets still referenced by active scenes and GPU allocations", memory.inactive_scenes as f64 / 1_048_576.));
                    ui.monospace(format!("Lossless cold-stored inactive-scene mesh + texture data: {:.1} MiB", memory.compressed_scene_assets as f64 / 1_048_576.));
                    let (entity_heap, component_heap, property_heap) = runtime_entity_heap_bytes(w);
                    ui.monospace(format!("Runtime heap estimate: {:.1} MiB entity records/identity · {:.1} MiB component storage · {:.1} MiB property trees\nCapacity-based payload estimate; excludes allocator/hash-table overhead and subsystem state", entity_heap as f64 / 1_048_576., component_heap as f64 / 1_048_576., property_heap as f64 / 1_048_576.));
                    if let Some((working, private)) = memory.process {
                        ui.monospace(format!("Process memory: {:.1} MiB {} · {:.1} MiB {}", working as f64 / 1_048_576., PROCESS_MEMORY_LABELS.0, private as f64 / 1_048_576., PROCESS_MEMORY_LABELS.1));
                    }
                });
                ui.collapsing(format!("Coverage / warnings ({})", w.report.warnings.len() + w.report.unsupported.len()), |ui| {
                    for message in w.report.unsupported.iter().chain(&w.report.warnings) { ui.label(message); }
                });
            }
        });
    }
}

impl DebugView {
    fn game_overlay(
        &mut self,
        ctx: &egui::Context,
        world: Option<&World>,
        controller: Option<&Controller>,
        animation: Option<&Preview>,
        pad: &Pad,
        paused: bool,
    ) {
        let Some(w) = world else {
            return;
        };
        let font = match &w.debug_font {
            Ok(font) => font,
            Err(error) => {
                egui::Area::new(egui::Id::new("Debug font error"))
                    .fixed_pos([18., 105.])
                    .show(ctx, |ui| {
                        ui.colored_label(egui::Color32::LIGHT_RED, error);
                    });
                return;
            }
        };
        egui::Window::new("Game data / F3").title_bar(false).default_pos([18.,150.])
            .frame(egui::Frame::NONE).default_width(550.).default_height(650.)
            .resizable(true).vscroll(true).show(ctx, |ui| {
            ui.spacing_mut().item_spacing.y=0.;
            let white=egui::Color32::WHITE;
            let cyan=egui::Color32::from_rgb(100,230,255);
            let text=|ui:&mut egui::Ui,value:String| {font.text(ui,value,white);};
            text(ui,format!("MICKEY - {} - F3",if paused {"PAUSED"} else {"LIVE"}));
            if !self.frames.is_empty() {
                let mean=self.frames.iter().sum::<f32>()/self.frames.len() as f32;
                text(ui,format!("Render callbacks {:.1}/s; {:.2} ms; {} frames",1000./mean.max(f32::MIN_POSITIVE),mean,self.frames.len()));
            }
            text(ui,"CPU mean/p95 ms (recent; nested costs overlap)".into());
            for line in &self.performance_lines {text(ui,line.clone());}
            if font.text(ui,"[Save performance report]",cyan).clicked() {self.export_performance=true;}
            if let Some(controller)=controller {
                text(ui,match controller.player.motion_surface_normal {
                    Some(normal)=>format!("Motion surface (retained) {:.4} {:.4} {:.4} / {:.4}",normal[0],normal[1],normal[2],normal[3]),
                    None=>"Motion surface: not published".into(),
                });
            }
            if let Some(target)=w.gameplay_camera_targets.frame {
                text(ui,format!("Camera target {:.3}, {:.3}, {:.3} / path {} / updates {}",
                    target.adjusted[0],target.adjusted[1],target.adjusted[2],w.gameplay_camera_targets.breadcrumbs.len(),w.gameplay_camera_targets.updates));
                text(ui,format!("Target Y: published {:.3} / unfiltered {:.3} (camera view pending)",target.published[1],target.unfiltered[1]));
            }
            if let Some(error)=&w.gameplay_camera_targets.error {text(ui,error.clone());}
            for (&entity,state) in &w.npc_relocations {
                if state.moves==0 {continue;}
                let owner=&w.entities[entity];let p=&owner.transform[12..15];
                text(ui,format!("NPC warp {} #{}: {:.2}, {:.2}, {:.2}",owner.name.rsplit('.').next().unwrap_or(&owner.name),state.moves,p[0],p[1],p[2]));
            }
            text(ui,format!("Named FX {} created / {} live / {} completed",w.named_fx.created,w.named_fx.live.len(),w.named_fx.completed));
            text(ui,format!("Cutscene {} actions / {} known AI / {} queries",
                w.cutscene_actions.requests,w.cutscene_actions.receiver_calls,w.cutscene_actions.queries));
            if let Some(knockback)=&w.player_knockback {
                text(ui,format!("Knockback {} phase {} admitted {} recovered {}",knockback.state.active,
                    knockback.state.phase,knockback.state.admissions,knockback.state.recoveries));
            }
            text(ui,format!("Named components {} requests / {} matched",
                w.component_control.named_requests,w.component_control.named_matches));
            let (attachments,attempts)=w.attachment_modifier_counts();
            text(ui,format!("Attachment modifiers {} active / {} attempts",attachments,attempts));
            text(ui,format!("Layer writes {} / body {} / refresh {}",w.rigid_layer_updates.requests,
                w.rigid_layer_updates.body_writes,w.rigid_layer_updates.refreshes));
            text(ui,format!("NPC bodies {} / {} suspended; AI motion pending",
                w.character_physics.values().filter(|body|body.is_ok()).count(),
                w.character_physics.values().filter(|body|body.as_ref().is_ok_and(|body|body.motion.current.motion_type()==4)).count()));
            text(ui,format!("Audio {}/{} active / {} positions; mixer pending",
                w.multi_position_audio.values().filter(|state|state.active).count(),w.multi_position_audio.len(),
                w.multi_position_audio.values().map(|state|state.positions.len()).sum::<usize>()));
            let resource=w.level_resource.summary();
            text(ui,format!("Level resource: {}",resource["level"].as_str().unwrap_or("<missing>")));
            text(ui,format!("Parts: {} / {}",resource["parts"][0].as_str().unwrap_or("<none>"),
                resource["parts"][1].as_str().unwrap_or("<none>")));
            match w.checkpoint_summary() {
                Ok(state)=>text(ui,format!("Save summary: {} tickets / {} pins / {} film reels",state.tickets,state.pins,state.film_reels)),
                Err(error)=>text(ui,format!("Save summary error: {error:#}")),
            }
            match w.checkpoint_inventory_limits() {
                Ok(state)=>text(ui,format!("Wallet max {} / SketchMax[2,0,1] {:?}",state.maximum_tickets,state.sketch_maxima_bits.map(f32::from_bits))),
                Err(error)=>text(ui,format!("Inventory header error: {error:#}")),
            }
            text(ui,format!("Shared clock {:.3}s / playtime {} (frame order unverified)",
                w.frame_clock.clock.unscaled_seconds,w.playtime.summary()["accumulated_seconds"]));
            if let Some(change)=&w.animation_variable_changes.last {
                text(ui,format!("Animation variables: {} mapped / {} requests; {} = 0x{:08x}",
                    w.animation_variable_changes.mapped_writes,w.animation_variable_changes.requests,
                    change.name.as_deref().unwrap_or("<unmapped>"),change.event.bits));
            }
            if let Some(snapshot)=&w.checkpoint.latest {
                text(ui,format!("Memory checkpoint: {} bytes / 9 owners; reload unavailable",snapshot.logical_size));
                if let Some(service)=&w.checkpoint.service {
                    text(ui,format!("Disk saves: {} committed / deferred {} / active {}",service.writes(),service.pending(),service.blocks_memory_capture()));
                }else {text(ui,"Disk save backend unavailable".into());}
            }else {text(ui,"Memory checkpoint: none".into());}
            text(ui,format!("Extras: {} / {} unlocked",w.extras.entries.iter().filter(|extra|extra.unlocked).count(),w.extras.entries.len()));
            text(ui,format!("Collectibles: {} / {} owned; {} UI requests pending delivery",w.collectibles.entries.iter().filter(|e|e.count>0).count(),w.collectibles.entries.len(),w.collectibles.notifications.len()+w.ui_notifications.len()));
            text(ui,format!("Phantoms: {} box / {} shape; {} shared assets",
                w.size_phantom_handles.len(),w.resource_phantom_handles.len(),w.phantom_resources.len()));
            text(ui,format!("Pickup listeners: {}; phantoms in world: {}",
                w.pickup_lifecycles.values().filter(|state|state.listener.is_some()).count(),
                w.physics_objects.phantoms.values().filter(|phantom|phantom.world.is_some()).count()));
            text(ui,format!("World input: gravity Y {:.1}; tolerance {:.3}",w.report.physics_world.gravity[1],w.report.physics_world.collision_tolerance));
            for error in self.phantom_bounds_errors.borrow().iter() {text(ui,format!("Phantom bounds: {error}"));}
            text(ui,format!("Cutscene input {} / references {}",if w.cutscene_input.restricted {"BLOCKED"}else{"ENABLED"},w.cutscene_input.count));
            if let Some(c)=controller {if let Some(ground)=&c.player.motion_ground {
                text(ui,format!("Ground clearance {:?} / radius {:.3} / support {} / ray hit {}",
                    ground.distance,c.player.motion.ground_probe_start_offset,ground.support,ground.hit_this_update));
                text(ui,format!("Ground point {:?} / surface velocity {:?}",ground.last_hit.map(|p|[p[0],p[1],p[2]]),
                    ground.surface_velocity.map(|v|[v[0],v[1],v[2]])));
            }}
            text(ui,format!("Camera events {} / inputs {} / last {:?}:{:?}",
                w.camera_feedback.received,w.camera_feedback.injected,w.camera_feedback.last_id,w.camera_feedback.last_flag));
            text(ui,format!("Camera airborne {} / spin follow {}",w.camera_feedback.airborne,w.camera_feedback.spin_follow_active));
            text(ui,format!("Model shadow {} caster / {} receiver draws; {} errors",w.model_shadow.casters,w.model_shadow.receivers,w.model_shadow.errors.len()));
            if self.developer {for (owner,error) in &w.model_shadow.errors {text(ui,format!("Shadow {owner}: {error}"));}}
            if let Some(tracker)=&w.mickey_tracker {
                text(ui,format!("MickeyTracker {:.3?} / updates {} / {}",tracker.state.position,tracker.state.updates,
                    tracker.error.as_deref().unwrap_or("live source history / ground plane")));
            }
            text(ui,format!("Silhouette gate {} / candidates {} (enrollment / shader pass pending)",w.render_options.draw_silhouettes,
                w.render_objects.values().filter(|object|w.render_options.admits_silhouette(object.flags)).count()));
            if let Some(state)=w.ledge_controls.get(&w.player_entity) {
                text(ui,format!("Ledge flags {:08x} / script {} / flags admit {} (query/hang pending)",state.flags,state.script_enabled(),state.flags_admit_query()));
            }
            if let Some(camera)=w.nif_camera {
                text(ui,if camera.entity_projection.is_some() {
                    format!("Entity camera {} / focus {:?} (application lens; gameplay controller pending)",camera.entity,camera.focus)
                }else{format!("NIF camera entity {} / node {} / focus {:?} (application lens; gameplay controller pending)",camera.entity,camera.node,camera.focus)});
                text(ui,format!("Lens FOV {:.3} / aspect {:.4} / clips {:.3}..{:.1}",w.camera_lens.field_of_view,
                    w.camera_lens.properties.aspect,w.camera_lens.properties.near,w.camera_lens.properties.far));
            }
            text(ui,format!("Counters {} / hit delivery pending",w.paint_counters.len()));
            text(ui,format!("Counter material initialized {} / failed {}",w.counter_materials.values().filter(|state|state.initialized).count(),w.counter_materials.values().filter(|state|state.error.is_some()).count()));
            text(ui,format!("Counter paint rim RGB {:?}",w.report.counter_rim_colors.paint));
            text(ui,format!("Counter thinner rim RGB {:?}",w.report.counter_rim_colors.thinner));
            text(ui,format!("Global states {} / zone first visit {:?}",w.globals.names_len(),w.globals.get("PlayerFirstTimeInZone")));
            text(ui,format!("AISS instance changes {} (mood updates / save restore pending)",w.instance_changes.entries().len()));
            text(ui,format!("Playthrough {}",w.playthrough.value));
            text(ui,format!("World save scene requests {} (streaming lifecycle pending)",w.scene_resources.save_names(&w.scene_requests).len()));
            text(ui,format!("Playstyle points {} (scoring/reconciliation pending)",w.playstyle_points.points(1)));
            text(ui,format!("Options left-handed {} / rumble {} / SFX {} / music {} (runtime application pending)",
                w.game_options.left_handed!=0,w.game_options.rumble!=0,w.game_options.sfx_volume,w.game_options.music_volume));
            text(ui,format!("Quest slots {} / authored {} / current {} (evaluation pending)",w.quests.slots.len(),
                w.quests.slots.iter().filter(|slot|slot.definition.is_some()).count(),
                w.quests.slots.iter().filter(|slot|slot.current!=u16::MAX).count()));
            text(ui,format!("Pause map {} / {}",w.map_info.map,w.map_info.map_type));
            if let Some(spawn)=w.player_spawn.override_transform {
                text(ui,format!("Stored player spawn {:.2}, {:.2}, {:.2} / scale {} (checkpoint reload pending)",
                    spawn.position[0],spawn.position[1],spawn.position[2],spawn.scale));
            }
            text(ui,format!("Native camera profiles {} / shared scalar properties {} (controller output pending)",
                w.camera_profiles.controller.library.profiles.len(),w.camera_profiles.controller.library.properties.len()));
            let camera_controller=&w.camera_profiles.controller;
            if let Ok(current)=usize::try_from(camera_controller.selection.current) {
                if let Some(profile)=camera_controller.library.profiles.get(current) {
                    let value=|lane:usize|camera_controller.library.properties[profile.properties[lane]].value;
                    text(ui,format!("Camera profile {} / max {} / placement {} / look {}",profile.name,value(1),value(7),value(6)));
                }
            }
            text(ui,format!("Actor managers {} / idle flags {}",w.actors.len(),w.idle_actors.len()));
            text(ui,format!("Authored Havok actors {} (AI/controller phase pending)",w.authored_havok.len()));
            if let Ok(events)=&w.animation_events {text(ui,format!("Native controller animation events {}",events.len()));}
            text(ui,format!("Animation noise requests {} (perception pending)",w.controller_feedback.noise_requests));
            text(ui,format!("Automatic actor event exports {}",w.controller_feedback.exported_events));
            text(ui,format!("AI uninterruptible fidgets {}",w.ai_brains.values().filter(|brain|brain.uninterruptible_fidget).count()));
            text(ui,format!("Native navigation floors {} (scripted task/controller handoff pending)",w.ai_navigation.native_floor_count()));
            text(ui,format!("Deferred entity events {} / AI manager gate {:?} (receiver delivery pending)",
                w.entity_event_queue.pending(),w.ai_manager_enabled));
            text(ui,format!("AI registry {} / brains {} / active Paused states {} / reevaluation pending {} (behavior selection pending)",
                w.ai_registry.len(),w.ai_brains.len(),w.ai_brains.values().flat_map(|brain|brain.paused.values()).filter(|state|state.active).count(),
                w.ai_brains.values().filter(|brain|brain.reevaluation_requested).count()));
            for entity in w.idle_actors.iter().copied().collect::<std::collections::BTreeSet<_>>() {
                if let Some(actor)=w.actors.get(&entity) {
                    if actor.clock.use_delta_time {
                        text(ui,format!("Idle {} / local clock {:.3}",w.entities[entity].name,actor.clock.initial_update_time()));
                    }else {text(ui,format!("Idle {} / global-time sampling gated",w.entities[entity].name));}
                }
            }
            if w.diagnostic_pickup_commands {text(ui,"DIAGNOSTIC command replay; pickup remains".into());}
            text(ui,format!("Phantom bounds: {} box (orange), {} shape (green); admission pending",w.size_phantom_handles.len(),w.resource_phantom_handles.len()));
            let bodies=&w.dynamic_body_sources;
            text(ui,format!("Dynamic source {}/{} components / {} bodies",bodies.decoded,bodies.components,bodies.bodies));
            text(ui,format!("Source motion modes {:?}; native parity pending",bodies.modes));
            text(ui,format!("Systems {} / constraints {} / actions {} / phantoms {}",bodies.systems,bodies.constraints,bodies.actions,bodies.phantoms));
            let constructed=w.dynamic_constructions.values().filter(|state|state.completed).map(|state|state.bodies.len()).sum::<usize>();
            let admitted=w.dynamic_runtime_sources.keys().map(|source|w.dynamic_constructions[source].bodies.len()).sum::<usize>();
            text(ui,format!("Constructed {} bodies; Box3D admitted {}",constructed,admitted));
            if let Some([accepted,rejected,unresolved])=w.collision.native_character_contact_counts() {
                text(ui,format!("Native static contacts {accepted} / rejected {rejected} / unresolved {unresolved}"));
            }
            text(ui,format!("Remaining diagnostic bodies {}; Havok parity pending",w.dynamic_query_owners.len()-admitted));
            let collectors=w.pickup_receivers.values().filter(|receiver|receiver.collector(|index|index<w.entities.len()).is_some()).count();
            text(ui,format!("Pickups {} / collectors {}; grants pending",w.pickup_receivers.len(),collectors));
            text(ui,format!("Initialized pickup listeners {} / {}",
                w.pickup_lifecycles.values().filter(|state|state.listener.is_some()).count(),w.pickup_receivers.len()));
            text(ui,format!("Pickup src {}; bob {:.2}/{:.2}; update pending",
                w.pickup_settings.len(),w.pickup_bob_globals.amplitude,w.pickup_bob_globals.frequency));
            if let Some(inventory)=w.authored_inventory.get(&w.player_entity) {
                text(ui,format!("Inv src {} floats/{} sketches; runtime pending",
                    inventory.floats.len(),inventory.sketches.len()));
            }
            match w.player_health_save_state() {
                Ok(health)=>text(ui,format!("Health {} / MaxHealth property {}",health.current,health.maximum)),
                Err(error)=>text(ui,format!("Health unavailable: {error:#}")),
            }
            for (name,state) in &w.player_abilities {
                text(ui,format!("{name} P{} T{} levels {:?} disable {}",
                    u8::from(state.owned[0]),u8::from(state.owned[1]),state.upgrades,state.disabled));
            }
            text(ui,format!("Ability setup/gates pending; HUD requests {}",w.ability_hud_requests.len()));
            text(ui,format!("Audio preview: {} rejected posts / {} cases; full mixer pending",
                w.audio_playback.rejected_post_count,w.audio_playback.rejected_posts.len()));
            if font.text(ui,"[Developer diagnostics]",cyan).clicked() {self.developer=!self.developer;}
            let Some(c)=controller else {text(ui,"Waiting for player".into());return;};
            let p=&c.player;
            text(ui,format!("XYZ {:8.3} {:8.3} {:8.3}\nVEL {:8.3} {:8.3} {:8.3}",
                p.position[0],p.position[1],p.position[2],p.velocity[0],p.velocity[1],p.velocity[2]));
            text(ui,format!("{} Speed {:.3} Facing {:.1} deg",if p.grounded {"GROUND"} else {"AIR"},
                super::physics::length(p.velocity),p.yaw.to_degrees()));
            text(ui,format!("Max speed {:.3} Authored {:.3}",p.max_move_speed(),p.motion.speed));
            text(ui,format!("Preserve movement {}",p.maintains_ground_velocity()));
            for line in &self.native_contact_lines {text(ui,line.clone());}
            text(ui,format!("Jump ground/air/second {} / {} / {}",p.ground_jump_started,p.air_first_jump_started,p.second_jump_started));
            text(ui,format!("Camera yaw {:.1} pitch {:.1}",c.camera_yaw.to_degrees(),c.camera_pitch.to_degrees()));
            text(ui,format!("Jigsaw transitions {}  ThinEffectTimeScale {:.4}",w.jigsaw_effect_queue.len(),w.report.jigsaw_time_scale));
            let shots=&w.projectile_preview;
            text(ui,format!("Brush TRACE shots {} active {} hit {} expired {}",shots.emitted,
                shots.pool.entries.iter().filter(|p|p.active).count(),shots.contacts,shots.expired));
            if let Some(contact)=&shots.last_contact {
                text(ui,format!("Last {:?} #{} impact {:.3?}",contact.ink,contact.projectile,contact.hit.position));
                if let Some(entity)=w.collision_entity(contact.hit.owner) {
                    text(ui,format!("Projectile hit {} #{}",entity.name,entity.id));
                }
                if let Some(source)=w.jigsaw_owners.get(&contact.hit.owner) {
                    let instance=&w.jigsaw_instances[source.instance];
                    let entity=&w.entities[instance.entity];
                    let painted=entity.jigsaw_states[instance.node].painted[source.piece];
                    text(ui,format!("Jigsaw node {} piece {} painted {} (transition preview)",instance.node,source.piece,painted));
                }
            }
            text(ui,format!("Ammo stored {:?} / waits {:.2}, {:.2}",w.ammo.inventory.as_ref().map(|inventory|inventory.amounts),w.ammo.waits[0],w.ammo.waits[1]));
            text(ui,format!("Ammo available {:.2}, {:.2} / infinite {} / shared {}",w.ammo.available(0),w.ammo.available(1),w.ammo.policy.infinite,w.ammo.policy.shared));
            text(ui,format!("Inventory writes awaiting receivers: {}",w.brush_runtime.inventory_notifications));
            text(ui,format!("Audio unimplemented: {} x{}",super::brush_runtime::DRY_SOUND_EVENT,w.brush_runtime.dry_sound_requests));
            if let Some((_,hit))=&self.pointer {
                text(ui,format!("Static ray: {} intersected surfaces",self.pointer_surfaces));
                if let Some(hit)=hit {
                    text(ui,format!("Pointer probe XYZ {:.2?}",hit.position));
                    if let Some(entity)=w.collision_entity(hit.owner) {text(ui,format!("Pointer hit {} #{}",entity.name,entity.id));}
                    if let Some(source)=w.jigsaw_owners.get(&hit.owner) {
                        let instance=&w.jigsaw_instances[source.instance];
                        let entity=&w.entities[instance.entity];
                        let asset=&w.jigsaws[entity.jigsaw_asset.as_ref().unwrap()][instance.node];
                        let painted=entity.jigsaw_states[instance.node].painted[source.piece];
                        text(ui,format!("Jigsaw piece {}: {}",source.piece,if painted {"painted"}else{"thinned"}));
                        let key=if let Some(triangle)=hit.source_triangle {
                            instance.collision.triangle_key(asset,triangle)
                        } else {instance.collision.convex_key(asset,hit.shape)};
                        match key {
                            Ok(Some(key))=>text(ui,format!("Jigsaw shape key {key:08x}")),
                            Ok(None)=>text(ui,"Jigsaw shape key unavailable".into()),
                            Err(error)=>text(ui,format!("Jigsaw key error: {error}")),
                        }
                        match w.collision.shape_penetration(hit.shape,p.position) {
                            Ok(depth)=>text(ui,format!("Box3D overlap depth {depth:.4}")),
                            Err(error)=>text(ui,format!("Overlap query: {error}")),
                        }
                        text(ui,format!("Restoration contact guard: {} (native ray phase pending)",
                            if w.report.jigsaw_overlap_guard_enabled {"on"}else{"off"}));
                    }
                    if let Some(part)=w.imported_static_part(hit.owner) {
                        let triangle=hit.source_triangle.and_then(|id|id.checked_sub(part.triangle_start))
                            .filter(|id|*id<part.triangle_count);
                        text(ui,format!("HKX body {:x} part {} tri {}",part.body_offset,part.subpart_index,
                            triangle.map(|id|id.to_string()).unwrap_or_else(||"none".into())));
                        text(ui,format!("Serialized body filter {:08x}",part.body_filter_info));
                        match part.shape_keys.key(triangle).and_then(|key|part.child_filters.at(triangle).map(|filter|(key,filter))) {
                            Ok((key,filter))=> {
                                text(ui,format!("Shape key {key:08x} child filter {filter:08x}"));
                                let entity=w.collision_entity(hit.owner);
                                let has_component=|name:&str|entity.is_some_and(|e|e.components.iter().any(|c|c.class_name==name));
                                // Imported static decoder currently requires a MOPP root.
                                let body=super::collision::FilterBody {shape_type:0xa,filter:part.body_filter_info,
                                    has_jigsaw_component:has_component("PhysicsJigsawComponent")};
                                match super::collision::resolve_filter(Some(body),key,|_|Ok(Some(filter))) {
                                    Ok(resolved)=> {text(ui,format!("Imported filter {resolved:08x} targetable {}",has_component("TargetableComponent")));}
                                    Err(error)=> {text(ui,format!("Filter resolution error: {error}"));}
                                }
                            }
                            Err(error)=> {text(ui,format!("Shape metadata error: {error}"));}
                        }
                    }
                } else {text(ui,"Pointer probe: no collision".into());}
                text(ui,"Diagnostic ray, not brush target".into());
            }
            if let Some(error)=&self.pointer_error {text(ui,format!("Pointer probe error: {error}"));}
            text(ui,format!("Stick L {:.2?} R {:.2?}\nButtons {:04X} Tick {} Respawns {}",pad.left,pad.right,pad.buttons,c.ticks,p.respawns));
            if let Some(a)=animation {
                text(ui,format!("Animation {} bones {} events",a.pose.len(),a.delivered_events.len()));
                text(ui,format!("Idle state {} fidgets {} timers {:.2} / {:.2}",a.idle.controller_state,
                    a.idle.fidgets,a.idle.combat_remaining,a.idle.fidget_remaining));
                if a.idle.combat_query_missing {text(ui,"Combat-idle threat query unavailable".into());}
                let (elevation,yaw)=a.brush_animation_inputs(&w.player_graph.state);
                text(ui,format!("Brush blend elevation {:.3} yaw {:.1}",elevation,yaw));
            }
            for node in &w.player_graph.plan.nodes {
                if let Some(runtime)=w.player_graph.machines.get(&node.object) {
                    if let Some(machine)=w.player_graph.state.machines.get(&node.object) {
                        if let Some(index)=machine.state_indices.get(&runtime.current) {
                            text(ui,format!("{}: {}",machine.settings.name,machine.states[*index].info.name));
                        }
                    }
                }
                if let Some(clip)=w.player_graph.state.clips.get(&node.object) {
                    if let Ok(instance)=w.player_graph.clips.get(&node.object) {
                        text(ui,format!("{} {:.3}s{}",clip.settings.animation_name,instance.clock.local_time,
                            if instance.clock.at_end {" [end]"} else {""}));
                    }
                }
            }
            text(ui,format!("Paint {:.2} Thinner {:.2}",w.ammo.available(0),w.ammo.available(1)));
            text(ui,w.report.level.clone());
            let origin=super::physics::add(p.position,[0.,0.1,0.]);
            if let Some(hit)=w.collision.ray_hit(origin,[0.,-1.,0.],10.) {
                text(ui,format!("Probe {:.3} Normal {:.3?}",hit.distance,hit.normal));
                if let Some(entity)=w.collision_entity(hit.owner) {text(ui,format!("Hit {} #{}",entity.name,entity.id));}
            }else {text(ui,"Probe no hit within 10 units".into());}
            text(ui,"Probe is independent of grounded state".into());
            text(ui,"Nearby entities - click to inspect".into());
            let mut nearest:Vec<_>=w.entities.iter().enumerate().filter(|(i,_)|!w.removed_entities.contains(i)).map(|(i,e)| {
                let d=(0..3).map(|axis|(e.transform[12+axis]-p.position[axis]).powi(2)).sum::<f32>();(i,d)
            }).collect();
            nearest.sort_unstable_by(|a,b|a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
            for (i,d) in nearest.into_iter().take(8) {
                let e=&w.entities[i];
                if font.text(ui,format!("{} {:.2} {} #{}",if self.selected==Some(i) {">"} else {" "},d.sqrt(),e.name,e.id),cyan).clicked() {
                    self.selected=Some(i);
                }
            }
            if self.selected.is_some_and(|entity|w.removed_entities.contains(&entity)) {self.selected=None;}
            if let Some(e)=self.selected.and_then(|i|w.entities.get(i)) {
                text(ui,format!("{} #{}\n{}\nXYZ {:.3?}\nAuthored XYZ {:.3?}",e.name,e.id,e.scene,&e.transform[12..15],&e.authored_transform[12..15]));
                if let Some(state)=self.selected.and_then(|i|w.runtime_lifetimes.get(&i)) {
                    text(ui,format!("Live lifetime {:.3}s | paused {} | IGC paused {}",
                        state.timer.remaining,state.timer.paused,state.timer.paused_during_igc));
                }
                if let Some(state)=self.selected.and_then(|i|w.pickup_lifecycles.get(&i)) {
                    text(ui,format!("Owned pickup: component {:?}, phantom {:?}, listener {:?}",state.physics_component,state.phantom,state.listener));
                    if let Some(record)=state.phantom.and_then(|id|w.physics_objects.phantoms.get(&id)) {
                        text(ui,format!("Phantom refs {}, world {:?}, listeners {}, overlaps {}",
                            record.header.references,record.world,record.overlaps.listeners.len(),record.overlaps.collidables.len()));
                    }
                }
                if let Some(index)=self.selected.filter(|index|w.size_phantom_handles.contains_key(index)
                    || w.resource_phantom_handles.contains_key(index)) {
                    match w.phantom_admission_sweep_bounds(index) {
                        Ok(bounds)=>text(ui,format!("Prepared sweep endpoints {:?} .. {:?}",bounds.minimum,bounds.maximum)),
                        Err(error)=>text(ui,format!("Sweep bounds unavailable: {error:#}")),
                    }
                }
                text(ui,"Component properties (live where runtime setters are connected)".into());
                for component in &e.components {
                    text(ui,component.class_name.clone());
                    for property in &component.properties {text(ui,format!("  {}: {}",property.name,property.value));}
                }
            }
        });
    }
}

impl DebugView {
    pub fn world_lines(
        &self,
        world: &World,
        controller: &Controller,
    ) -> Vec<crate::render_support::viewport3d::GpuLineSegment> {
        use super::physics::{add, mul};
        use crate::render_support::viewport3d::GpuLineSegment;
        self.phantom_bounds_errors.borrow_mut().clear();
        if !self.open {
            return vec![];
        }
        let mut phantom_lines = Vec::new();
        for &index in world
            .size_phantom_handles
            .keys()
            .chain(world.resource_phantom_handles.keys())
        {
            match world.phantom_admission_bounds(index) {
                Ok(bounds) => {
                    let color = if world.resource_phantom_handles.contains_key(&index) {
                        [0.2, 1., 0.4, 1.]
                    } else {
                        [1., 0.55, 0.1, 1.]
                    };
                    let corners: [[f32; 3]; 8] = std::array::from_fn(|corner| {
                        std::array::from_fn(|axis| {
                            if corner & (1 << axis) == 0 {
                                bounds.min[axis]
                            } else {
                                bounds.max[axis]
                            }
                        })
                    });
                    for corner in 0..8 {
                        for axis in 0..3 {
                            if corner & (1 << axis) == 0 {
                                phantom_lines.push(GpuLineSegment {
                                    start: corners[corner],
                                    end: corners[corner | (1 << axis)],
                                    color,
                                });
                            }
                        }
                    }
                }
                Err(error) => self
                    .phantom_bounds_errors
                    .borrow_mut()
                    .push(format!("{}: {error:#}", world.entities[index].name)),
            }
        }
        let p = &controller.player;
        let origin = add(p.position, [0., 0.1, 0.]);
        let mut lines = vec![GpuLineSegment {
            start: origin,
            end: add(origin, p.velocity),
            color: [0., 1., 1., 1.],
        }];
        // Show the movement owner's actual query, not an unrelated debug
        // ray whose offset/range can disagree with jump/tracker state.
        if let Some(ground) = &p.motion_ground {
            if let Some((start, end)) = ground.ray {
                let start = [start[0], start[1], start[2]];
                let end = [end[0], end[1], end[2]];
                lines.push(GpuLineSegment {
                    start,
                    end,
                    color: [1., 0.8, 0., 1.],
                });
                if ground.hit_this_update {
                    if let Some(point) = ground.last_hit {
                        let point = [point[0], point[1], point[2]];
                        lines.push(GpuLineSegment {
                            start,
                            end: point,
                            color: [0., 1., 0., 1.],
                        });
                        if let Some(normal) = ground.normal {
                            lines.push(GpuLineSegment {
                                start: point,
                                end: add(point, mul([normal[0], normal[1], normal[2]], 0.5)),
                                color: [0., 1., 0., 1.],
                            });
                        }
                    }
                }
            }
        }
        if let Some(entity) = self.selected.and_then(|i| world.entities.get(i)) {
            let o = [
                entity.transform[12],
                entity.transform[13],
                entity.transform[14],
            ];
            for axis in 0..3 {
                let mut d = [0.; 3];
                d[axis] = 0.5;
                let mut color = [0., 0., 0., 1.];
                color[axis] = 1.;
                lines.push(GpuLineSegment {
                    start: o,
                    end: add(o, d),
                    color,
                });
            }
        }
        lines.extend(phantom_lines);
        lines
    }
}
