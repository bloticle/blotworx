#[cfg(feature = "blotworx-memory-profile")]
#[global_allocator]
static MEMORY_PROFILE_ALLOCATOR: blotworx::memory_profile::TrackingAllocator =
    blotworx::memory_profile::TrackingAllocator;

fn main() -> anyhow::Result<()> {
    use blotworx::startup_log;
    let first_arg = std::env::args_os().nth(1);
    if first_arg.as_deref() == Some(std::ffi::OsStr::new("--support-report")) {
        print!("{}", startup_log::report(None, None, None));
        return Ok(());
    }
    #[cfg(any(windows, target_os = "macos"))]
    if std::env::var_os(startup_log::GAME_PROCESS_ENV).is_none()
        && !first_arg.as_deref().is_some_and(|arg| {
            matches!(arg.to_str(), Some("diagnose" | "--apply-update" | "-h" | "--help"))
        })
    {
        return startup_log::supervise();
    }
    #[cfg(debug_assertions)]
    let validation = {
        let mut args = std::env::args().skip(1);
        if args.next().as_deref() == Some("diagnose") {
            args.next()
        } else {
            None
        }
    };
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-dbc-hall-door") {
        return blotworx::blotworx::validate_dbc_hall_door();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-spline-events") {return blotworx::blotworx::validate_spline_events();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-inactive-motion") {return blotworx::blotworx::validate_inactive_motion();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-owned-cleanup") {return blotworx::blotworx::validate_owned_cleanup();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-palette-root") {return blotworx::blotworx::validate_palette_root();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-airlock-spawns") {return blotworx::blotworx::validate_airlock_spawns();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-audio-mutes") {return blotworx::blotworx::validate_audio_mutes();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-music") {return blotworx::blotworx::validate_music();}
    #[cfg(all(debug_assertions,feature="blotworx-ui-render"))]
    if validation.as_deref() == Some("--validate-pause-menu") {return blotworx::blotworx::validate_pause_menu();}
    #[cfg(all(debug_assertions,feature="blotworx-ui-render"))]
    if validation.as_deref() == Some("--validate-animation-audio") {return blotworx::blotworx::validate_animation_audio();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-animation-payloads") {return blotworx::blotworx::validate_animation_payloads();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-conversation-clones") {return blotworx::blotworx::validate_conversation_clones();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-dbc-arm-trigger") {
        return blotworx::blotworx::validate_dbc_arm_trigger();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-dbc-courtyard-armor") {
        return blotworx::blotworx::validate_dbc_courtyard_armor();
    }
    #[cfg(not(target_os = "horizon"))]
    if first_arg.as_deref() == Some(std::ffi::OsStr::new("diagnose")) || startup_log::install().is_none() {
        let _ = env_logger::try_init();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-story-rewards") {return blotworx::blotworx::validate_story_rewards();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-story-teardown") {return blotworx::blotworx::validate_story_teardown();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-streamed-story") {return blotworx::blotworx::validate_streamed_story();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-random-dialogue") {return blotworx::blotworx::validate_random_dialogue();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-usable-runtime") {return blotworx::blotworx::validate_usable_runtime();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-wii-morph-mesh") {return blotworx::blotworx::validate_wii_morph_mesh();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-model-billboards") {return blotworx::blotworx::validate_model_billboards();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-io-admission") {return blotworx::blotworx::validate_io_admission();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-idle-command") {
        return blotworx::blotworx::validate_idle_command();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-relay") {
        return blotworx::blotworx::validate_relay();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-completed-spawner") {
        return blotworx::blotworx::validate_completed_spawner();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-directional-combat") {
        return blotworx::blotworx::validate_directional_combat();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-streamed-triggers") {
        return blotworx::blotworx::validate_streamed_triggers();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-player-handoff") {
        return blotworx::blotworx::validate_player_handoff();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-live-inventory") {
        return blotworx::blotworx::validate_live_inventory();
    }
    #[cfg(all(debug_assertions, not(target_os = "horizon")))]
    if validation.as_deref() == Some("--validate-material-gpu") {
        struct Validation;
        impl eframe::App for Validation {
            fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        }
        return eframe::run_native(
            "Renderer validation",
            eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default().with_visible(false),
                ..Default::default()
            },
            Box::new(|cc| {
                unsafe {
                    blotworx::render_support::viewport3d::validate_material_gpu(
                        cc.gl.as_ref().expect("OpenGL context"),
                    )?;
                }
                Ok(Box::new(Validation))
            }),
        )
        .map_err(|error| anyhow::anyhow!(error.to_string()));
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-demo-progression") {
        return blotworx::blotworx::validate_demo_progression();
    }
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-brush-puzzle") {return blotworx::blotworx::validate_brush_puzzle();}
    #[cfg(debug_assertions)]
    if validation.as_deref() == Some("--validate-gv-encounter") {return blotworx::blotworx::validate_gv_encounter();}
    blotworx::blotworx::run().inspect_err(|error| log::error!("Application exited: {error:#}"))
}
