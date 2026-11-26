//! Native XInput controls; no emulator profile or injected input.
/// Ordinary controller reference, REL+0x30db38..0x30db84. The original
/// tests projected LENGTH (DOL 0x805aea20), not squared length, against 1e-6.
/// Host sqrt does not reproduce Gekko's reciprocal-square-root rounding.
pub fn camera_movement_reference(forward: [f32; 3]) -> anyhow::Result<[f32; 3]> {
    anyhow::ensure!(
        forward.iter().all(|v| v.is_finite()),
        "Nonfinite movement camera direction"
    );
    // 30db38 projects before calling NiPoint3::Length805aea20. The paired
    // dot fuses Z*Z + X*X, then adds the separately rounded zero Y product.
    let squared = forward[2].mul_add(forward[2], forward[0] * forward[0]) + 0.;
    anyhow::ensure!(
        squared.is_finite(),
        "Movement camera projected length overflow"
    );
    if squared == 0. {
        return Ok([0.; 3]);
    }
    // frsqrte retains its double estimate. Gekko fmuls rounds its second
    // multiplicand to 25 significant bits BEFORE the single-result rounding.
    // There is no frsp between the estimate and its first square/half products.
    let estimate = super::pose_blend::estimate_double(squared);
    let bits = estimate.to_bits();
    let operand = f64::from_bits(((bits >> 28) + ((bits >> 27) & 1)) << 28);
    let square = (estimate * operand) as f32;
    let half = (estimate * 0.5) as f32;
    let correction = -square.mul_add(squared, -3.);
    let factor = correction * half;
    // The subsequent normalize805ae9b0 repeats that same refinement. Dividing
    // by the rounded length instead changes the movement basis bits.
    Ok(if squared * factor > f32::from_bits(0x358637bd) {
        [forward[0] * factor, 0., forward[2] * factor]
    } else {
        [0.; 3]
    })
}

/// Y-up movement frame. `filtered` uses the retail motion component's
/// convention (-controller X, controller Y); native upward stick is -retail Y.
/// Equivalent to cross(up, reference)*filtered.x - reference*filtered.y.
pub fn movement_direction(filtered: [f32; 2], reference: [f32; 3]) -> [f32; 3] {
    [
        reference[2].mul_add(filtered[0], -reference[0] * filtered[1]),
        0.,
        (-reference[0]).mul_add(filtered[0], -reference[2] * filtered[1]),
    ]
}

/// EM1 REL+0x306efc..0x307140: independently limit stick length and angle.
/// Input is in controller space, before the camera-relative basis conversion.
/// Runtime override/gating and the original controller deadzone remain unported.
pub fn polar_filter(
    current: [f32; 2],
    target: [f32; 2],
    length_rate: f32,
    rotation_rate: f32,
    dt: f32,
) -> [f32; 2] {
    let norm = |v: [f32; 2]| (v[0] * v[0] + v[1] * v[1]).sqrt();
    let old_length = norm(current);
    let new_length = norm(target);
    let direction = if new_length == 0.0 {
        if old_length == 0.0 {
            return [0.0; 2];
        }
        current.map(|v| v / old_length)
    } else if old_length == 0.0 {
        target.map(|v| v / new_length)
    } else {
        let angle = current[1].atan2(current[0]);
        let mut delta = target[1].atan2(target[0]) - angle;
        // Preserve the original inclusive endpoints: +pi and -pi are distinct.
        while delta > std::f32::consts::PI {
            delta -= std::f32::consts::TAU;
        }
        while delta < -std::f32::consts::PI {
            delta += std::f32::consts::TAU;
        }
        let angle = angle + delta.clamp(-rotation_rate * dt, rotation_rate * dt);
        [angle.cos(), angle.sin()]
    };
    let magnitude =
        old_length + (new_length - old_length).clamp(-length_rate * dt, length_rate * dt);
    direction.map(|v| v * magnitude)
}

#[cfg(test)]
mod filter_tests {
    use super::{camera_movement_reference, movement_direction, polar_filter};
    #[test]
    fn camera_reference_ignores_pitch_but_rejects_degenerate_projection() -> anyhow::Result<()> {
        let reference = camera_movement_reference([0.3, -0.8660254, 0.4])?;
        assert!((reference[0] - 0.6).abs() < 1e-6 && (reference[2] - 0.8).abs() < 1e-6);
        assert_eq!(camera_movement_reference([0., -1., 0.])?, [0.; 3]);
        assert_eq!(camera_movement_reference([1e-6, 1., 0.])?, [0.; 3]);
        assert!((camera_movement_reference([2e-6, 1., 0.])?[0] - 1.).abs() < 1e-6);
        Ok(())
    }
    #[test]
    fn movement_camera_basis_matches_original_instruction_corpus() -> anyhow::Result<()> {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../tools/blotworx/fixtures/movement_camera_basis.json"
        ))?;
        for case in fixture["cases"].as_array().unwrap() {
            let forward = std::array::from_fn(|i| {
                f32::from_bits(case["forward_bits"][i].as_u64().unwrap() as u32)
            });
            let result = camera_movement_reference(forward)?;
            assert_eq!(
                serde_json::to_value(result.map(f32::to_bits))?,
                case["output_bits"],
                "{case}"
            );
        }
        for forward in [
            [f32::NAN, 0., 1.],
            [1., f32::INFINITY, 0.],
            [f32::MAX, 0., f32::MAX],
        ] {
            assert!(camera_movement_reference(forward).is_err());
        }
        Ok(())
    }
    #[test]
    fn retail_axes_follow_reference_and_preserve_analog_magnitude() {
        assert_eq!(movement_direction([0., -1.], [0., 0., 1.]), [0., 0., 1.]);
        assert_eq!(movement_direction([-1., 0.], [0., 0., 1.]), [-1., 0., 0.]);
        assert_eq!(movement_direction([0., -1.], [-1., 0., 0.]), [-1., 0., 0.]);
        let direction = movement_direction([-0.3, -0.4], [0.6, 0., 0.8]);
        assert!((direction[0] * direction[0] + direction[2] * direction[2] - 0.25).abs() < 1e-6);
        assert_eq!(movement_direction([-1., -1.], [0.; 3]), [0.; 3]);
    }
    #[test]
    fn starts_in_requested_direction_and_releases_without_rotating() {
        assert_eq!(polar_filter([0.; 2], [0., 1.], 3., 0., 0.1), [0., 0.3]);
        assert_eq!(polar_filter([0., 1.], [0.; 2], 3., 4.5, 0.1), [0., 0.7]);
        assert_eq!(polar_filter([0.; 2], [0.; 2], 30., 4.5, 0.1), [0.; 2]);
    }
    #[test]
    fn turning_is_limited_independently_of_magnitude() {
        let v = polar_filter([1., 0.], [0., 0.5], 30., 4.5, 0.1);
        assert!((v[0] - 0.5 * 0.45f32.cos()).abs() < 1e-6);
        assert!((v[1] - 0.5 * 0.45f32.sin()).abs() < 1e-6);
        let a = 179f32.to_radians();
        let b = -179f32.to_radians();
        let v = polar_filter([a.cos(), a.sin()], [b.cos(), b.sin()], 30., 4.5, 0.1);
        assert!((v[0] - b.cos()).abs() < 1e-6 && (v[1] - b.sin()).abs() < 1e-6);
    }
}
#[derive(Default, Clone, Copy)]
pub struct Pad {
    pub left: [f32; 2],
    pub right: [f32; 2],
    pub buttons: u16,
    /// Host XInput triggers, left/right. These are adapter controls, not
    /// reconstructed Wii input-map semantics.
    pub triggers: [u8; 2],
}
/// Shared desktop input adapter for both game frontends. This maps host
/// controls only; authored ability gates and simulation remain in the runtime.
pub(super) fn movement_controls(
    state: &egui::InputState,
    pad: Pad,
    jump_pending: bool,
    sensitivity: f32,
    invert_y: bool,
    mouse_look: bool,
) -> super::controller::Input {
    let vertical = if invert_y { -1. } else { 1. };
    let delta = state.pointer.delta();
    super::controller::Input {
        movement: [
            (state.key_down(egui::Key::D) as i32 - state.key_down(egui::Key::A) as i32) as f32 + pad.left[0],
            (state.key_down(egui::Key::W) as i32 - state.key_down(egui::Key::S) as i32) as f32 + pad.left[1],
        ],
        camera_stick: [pad.right[0] * sensitivity, pad.right[1] * sensitivity * vertical],
        camera_delta: if mouse_look && state.pointer.secondary_down() {
            [-delta.x * 0.006 * sensitivity, delta.y * 0.006 * sensitivity * vertical]
        } else { [0.; 2] },
        jump_pressed: jump_pending || state.key_pressed(egui::Key::Space),
        jump_held: state.key_down(egui::Key::Space) || pad.buttons & 0x1000 != 0,
    }
}
fn stick(x: i16, y: i16) -> [f32; 2] {
    let x = x as f32 / 32768.;
    let y = y as f32 / 32768.;
    let r = (x * x + y * y).sqrt();
    if r < 0.18 {
        return [0.; 2];
    }
    let scale = ((r.min(1.) - 0.18) / 0.82) / r;
    [x * scale, y * scale]
}
#[cfg(windows)]
pub fn poll() -> Pad {
    #[repr(C)]
    #[derive(Default)]
    struct Gamepad {
        buttons: u16,
        lt: u8,
        rt: u8,
        lx: i16,
        ly: i16,
        rx: i16,
        ry: i16,
    }
    #[repr(C)]
    #[derive(Default)]
    struct State {
        packet: u32,
        pad: Gamepad,
    }
    #[link(name = "xinput9_1_0")]
    extern "system" {
        fn XInputGetState(index: u32, state: *mut State) -> u32;
    }
    for index in 0..4 {
        let mut s = State::default();
        if unsafe { XInputGetState(index, &mut s) } == 0 {
            return Pad {
                left: stick(s.pad.lx, s.pad.ly),
                right: stick(s.pad.rx, s.pad.ry),
                buttons: s.pad.buttons,
                triggers: [s.pad.lt, s.pad.rt],
            };
        }
    }
    Pad::default()
}
#[cfg(target_os = "android")]
pub fn poll() -> Pad {
    let (buttons, [lx, ly, rx, ry, lt, rt]) = crate::android_app::gamepad();
    let axis = |value: f32| (value.clamp(-1., 1.) * i16::MAX as f32) as i16;
    let trigger = |value: f32| (value.clamp(0., 1.) * u8::MAX as f32) as u8;
    Pad { left: stick(axis(lx), axis(ly)), right: stick(axis(rx), axis(ry)),
        buttons, triggers: [trigger(lt), trigger(rt)] }
}

#[cfg(target_os = "macos")]
// Convert to XInput
pub fn poll() -> Pad {
    use gilrs::{Axis, Button};
    use std::cell::RefCell;
    thread_local! {
        static GILRS: RefCell<Option<gilrs::Gilrs>> = const { RefCell::new(None) };
        static FAILED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    }
    GILRS.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.is_none() {
            if FAILED.with(|failed| failed.get()) {
                return Pad::default();
            }
            match gilrs::Gilrs::new() {
                Ok(gilrs) => *slot = Some(gilrs),
                Err(error) => {
                    log::warn!("Gamepad support unavailable: {error}");
                    FAILED.with(|failed| failed.set(true));
                    return Pad::default();
                }
            }
        }
        let gilrs = slot.as_mut().unwrap();
        while gilrs.next_event().is_some() {}
        let Some((_, gamepad)) = gilrs.gamepads().find(|(_, gamepad)| gamepad.is_connected()) else {
            return Pad::default();
        };
        let axis = |axis: Axis| (gamepad.value(axis).clamp(-1., 1.) * i16::MAX as f32) as i16;
        let trigger = |axis: Axis, button: Button| {
            let value = gamepad.value(axis).clamp(0., 1.);
            if value > 0. {
                (value * f32::from(u8::MAX)) as u8
            } else if gamepad.is_pressed(button) {
                u8::MAX
            } else {
                0
            }
        };
        let mut buttons = 0u16;
        for (button, bit) in [
            (Button::DPadUp, 0x0001),
            (Button::DPadDown, 0x0002),
            (Button::DPadLeft, 0x0004),
            (Button::DPadRight, 0x0008),
            (Button::Start, 0x0010),
            (Button::Select, 0x0020),
            (Button::LeftThumb, 0x0040),
            (Button::RightThumb, 0x0080),
            (Button::LeftTrigger, 0x0100),
            (Button::LeftTrigger2, 0x0100),
            (Button::RightTrigger, 0x0200),
            (Button::RightTrigger2, 0x0200),
            (Button::Mode, 0x0400),
            (Button::South, 0x1000),
            (Button::East, 0x2000),
            (Button::West, 0x4000),
            (Button::North, 0x8000),
        ] {
            if gamepad.is_pressed(button) {
                buttons |= bit;
            }
        }
        Pad {
            left: stick(axis(Axis::LeftStickX), axis(Axis::LeftStickY)),
            right: stick(axis(Axis::RightStickX), axis(Axis::RightStickY)),
            buttons,
            triggers: [
                trigger(Axis::LeftZ, Button::LeftTrigger),
                trigger(Axis::RightZ, Button::RightTrigger),
            ],
        }
    })
}

#[cfg(not(any(windows, target_os = "android", target_os = "macos")))]
pub fn poll() -> Pad {
    Pad::default()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn radial_deadzone_and_diagonal_limit() {
        assert_eq!(stick(100, 100), [0.; 2]);
        let p = stick(32767, 32767);
        assert!((p[0] * p[0] + p[1] * p[1] - 1.).abs() < 1e-5);
    }
}
