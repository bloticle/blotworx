//! Blotworx Engine: native, read-only exploration of the original cooked level.
#[cfg(not(target_os = "horizon"))]
mod em2;
#[cfg(target_os = "horizon")]
pub(crate) mod horizon_3ds;
mod em2_global_state;
mod em2_quest_path;
mod em2_music;
mod em2_tex_anim;
mod compiled_level;
mod em2_level_manager;
mod sequence44;
mod em2_sequences;
mod em2_hud;
mod em2_abilities;
mod em2_ability_disable;
mod em2_usable;
mod em2_cutscene_input;
mod em2_game_state_events;
mod em2_igc_control;
mod em2_animation_events;
mod em2_igc_animation;
mod split_animation;
mod streamed_animation_resources;
mod igc;
mod igc_package;
mod camera_ar_filter;
mod em2_camera_reset;
mod em2_camera_hints;
mod em2_camera_focus;
mod em2_camera_yaw;
mod em2_camera_pass;
mod d3d9_shader;
mod d3d9_constants;
mod d3d9_effect;
mod d3d9_preshader;
mod d3d9_sampler;
mod d3d9_bindings;
mod d3d9_state;
mod d3d9_raster;
mod d3d9_sampling;
mod d3d9_images;
mod d3d9_material;
mod d3d9_draw;
mod d3d9_predefined;
mod d3d9_camera;
mod d3d9_attributes;
mod d3d9_globals;
mod d3d9_light;
mod d3d9_sh;
mod d3d9_sh_cache;
mod d3d9_scene_lights;
mod d3d9_sh_probe;
mod d3d9_sh_query;
mod d3d9_sh_pool;
mod d3d9_sh_worker;
mod d3d9_sh_repository;
mod d3d9_texture_effects;
mod render_profiles;
mod render_profile_settings;
mod render_profile_interpolation;
mod d3d9_texture_transform;
mod d3d9_gpu;
mod d3d9_vertices;
mod d3d9_translate;
mod player_constraints;
mod em2_camera_manager;
mod em2_follow_camera;
mod authored_motion;
mod em2_cursor_camera;
#[cfg(not(target_os = "horizon"))]
mod em2_audio_player;
mod animation_chunks;
mod igc_timeline;
mod gamebryo_camera;
mod usable_selection;
mod usable_prompt;
#[cfg(not(target_os = "horizon"))]
mod disc_setup;
#[cfg(not(target_os = "horizon"))]
mod multiplayer;
pub mod export_3ds;
#[cfg(not(target_os = "horizon"))]
mod updater;
mod em2_player_tools;
mod em2_children;
mod em2_child_pose;
mod em2_attach_points;
mod em2_float_property;
mod em2_audio;
mod audio_codec;
mod audio_source72;
mod audio_node72;
mod audio_post72;
mod em2_entity_startup;
mod model_body_binding;
mod havok_2012_geometry;
mod havok_2012_motion;
mod havok_2012_body_properties;
mod scene_static_collision;
mod scene_event_runtime;
mod player_source;
mod character_runtime;
mod scene_light_registry;
mod options_menu;
mod ability_ownership;
mod ability_persistence;
mod ability_registry;
mod ability_value;
mod action_event_dispatch;
mod action_event_queue;
mod trigger_plane_segment;
mod native_float_conversion;
mod actor_bone_query;
mod actor_io;
mod actor_pose_compose;
mod actor_pose_storage;
mod actor_runtime;
mod actor_scripts;
mod entity_liveness;
mod ai_behavior_events;
mod ai_actor_pose;
mod ai_controller_input;
mod ai_controller_speed;
mod ai_navigation;
mod ai_navigation_brain;
mod ai_navigation_brain_binding;
mod ai_navigation_brain_resources;
mod ai_navigation_region_cache;
mod ai_navigation_region_world;
mod ai_navigation_refresh;
mod ai_navigation_refresh_sources;
mod ai_navigation_brain_base;
mod ai_navigation_factory;
mod ai_navigation_factory_pool;
mod ai_navigation_action;
mod ai_navigation_action_world;
mod ai_navigation_construction;
mod ai_navigation_entity_base;
mod ai_navigation_entity;
mod ai_navigation_outline;
mod ai_navigation_entity_world;
mod ai_navigation_entity_init;
mod ai_navigation_definitions;
mod ai_navigation_resolver;
mod ai_biped_catalog;
mod ai_biped_catalog_world;
mod ai_pathfinder_constructor;
mod ai_pathfinder_reset;
mod ai_pathfinder_reset_binding;
mod ai_pathfinder_initialization;
mod ai_navigation_service_factory;
mod ai_navigation_agent_factory;
mod ai_navigation_service_init;
mod ai_pathfinder_attributes;
mod ai_pathfinder_modifiers;
mod ai_goto_gap;
mod ai_goto_cost;
mod ai_goto_cones;
mod ai_goto_refresh;
mod ai_goto_refresh_profiles;
mod ai_entity_profile_query;
mod ai_source_point;
mod ai_graph_connection;
mod ai_graph_payload;
mod ai_graph_loaded_buffer;
mod ai_graph_loaded_group;
mod ai_graph_connection_binding;
mod ai_goto_cost_native_graph;
mod ai_goto_cost_binding;
mod ai_refine_goal;
mod ai_refine_projection;
mod ai_refine_entry;
mod ai_goto_gap_attributes;
mod ai_goto_gap_setup;
mod ai_gap_manager;
mod ai_gap_compute;
mod ai_navigation_modifier_init;
mod ai_navigation_attribute_query;
mod ai_runtime_mesh;
mod ai_mesh_provider;
mod ai_graph_finalize;
mod ai_graph_finalize_lifecycle;
mod ai_traversal_registration;
mod ai_traversal_policy_release;
mod ai_navigation_world_registration;
mod ai_navigation_world_owned;
mod ai_navigation_profile_lifecycle;
mod ai_navigation_brain_lifecycle;
mod ai_pathfinder_world_lifecycle;
mod ai_traversal_world_callbacks;
mod ai_registry;
mod ai_selection;
mod ai_brain_update;
mod ai_stand_idle;
mod ai_stand_idle_world;
mod ai_patrol;
mod ai_patrol_world;
mod ai_patrol_route;
mod ai_patrol_route_world;
mod motion_turn_rates;
mod motion_turn_rates_world;
mod ai_conversation;
mod ai_conversation_world;
mod ai_attributes;
mod ai_animation_wait;
mod ai_animation_wait_world;
mod ai_scripted_move;
mod ai_task_runtime;
mod ai_task_update_world;
mod ai_task_world;
mod ai_turn_task;
mod ai_move_task;
mod ai_agent_profiles;
mod ai_agent_base;
mod ai_agent_action_init;
mod ai_agent_actions;
mod ai_agent_definition_binding;
mod ai_path_agent;
mod ai_path_agent_binding;
mod ai_agent_path;
mod ai_agent_path_runtime;
mod ai_agent_path_request;
mod ai_agent_path_initialize;
mod ai_pathfinder_binding;
mod ai_traversal_registry;
mod ai_traversal_instances;
mod ai_traversal_manager;
mod ai_task_manager;
mod ai_task_frame;
mod ai_task_startup;
mod ai_task_clock;
mod ai_task_context_world;
mod ai_traversal_acquisition;
mod ai_traversal_release;
mod ai_goto_agent;
mod ai_path_agent_attributes;
mod ai_path_agent_attribute_binding;
mod ai_flee_agent;
mod ai_flee_binding;
mod ai_flee_attributes;
mod ai_flee_definition_binding;
mod ai_goto_configuration;
mod ai_goto_traversal;
mod ai_wander_traversal;
mod ai_goto_initialization;
mod ai_flat_handle_strategy;
mod ai_graph_handles;
mod ai_astar_memory;
mod ai_astar_reset;
mod ai_graph_points;
mod ai_graph_queries;
mod ai_astar_graph_services;
mod ai_graph_normalization;
mod ai_graph_resource_group;
mod ai_graph_buffers;
mod ai_graph_loading;
mod ammo;
mod animation_attributes;
mod animation_event_queue;
mod animation_events;
mod animation_float;
mod animation_metadata;
mod animation_resources;
mod animation_sync;
mod animation_variables;
mod assets;
mod attachment_direction;
mod attachment_modifier;
mod attachment_node_lookup;
mod attachment_pose;
mod attachment_publish;
mod audio_attenuation_resources;
mod audio_bank;
mod audio_bank_framing;
mod audio_consumer;
mod audio_control;
mod audio_conversion;
mod audio_distance;
mod audio_dsp;
mod audio_filter;
mod audio_graph;
mod audio_spatial;
mod audio_limits;
mod audio_loop;
mod audio_sis;
mod audio_file;
mod audio_signal;
mod audio_events;
mod audio_execution;
mod audio_ingress;
mod audio_listener;
mod audio_nodes;
mod audio_output;
#[cfg(target_os = "android")]
mod android_pcm;
mod audio_panner;
mod audio_play_action;
mod audio_playback;
mod audio_parameters;
mod audio_mute_mix;
mod audio_music;
mod audio_music_state;
#[cfg(debug_assertions)]
pub fn validate_music() -> anyhow::Result<()> { audio_music::validate() }
#[cfg(all(debug_assertions, feature="blotworx-ui-render"))]
pub fn validate_pause_menu() -> anyhow::Result<()> { ui_movie::validate_pause_menu() }
#[cfg(debug_assertions)]
pub fn validate_audio_mutes()->anyhow::Result<()> {audio_playback::validate_mutes()}
mod audio_playing;
mod audio_position_backend;
mod audio_position_paths;
mod audio_position_state;
mod audio_radius;
mod audio_schedule;
mod audio_sound;
mod audio_switch;
mod audio_transition;
mod audio_voice_bindings;
mod audio_voice_create;
mod audio_voice_init;
mod audio_voice_position;
mod audio_voice_rtpc;
mod audio_voice_term;
mod authored_havok;
mod authored_preview;
mod behavior;
mod behavior_expression;
mod behavior_state;
mod blender;
mod body_activation;
mod body_agents;
mod body_filter;
mod body_group_assignment;
mod body_group_pair;
mod body_info;
mod body_initialization;
mod body_listeners;
mod body_material;
mod body_mass_properties;
mod body_motion;
mod body_properties;
mod body_quality;
mod body_scene_context;
mod brush;
mod brush_assets;
mod brush_particles;
mod brush_preview;
mod brush_render;
mod brush_runtime;
mod camera_attributes;
mod camera_overrides;
mod em2_camera_data;
mod em2_camera_profiles;
mod pc_float;
mod em2_camera_tracking;
mod em2_camera_visibility;
mod camera_auto_level;
mod camera_automatic_mode;
mod camera_base;
mod camera_catalog;
mod camera_clear;
mod camera_collision;
mod camera_collision_box3d;
mod camera_constructor;
mod camera_defaults;
mod camera_desired_polar;
mod camera_desired_yaw;
mod camera_distance;
mod camera_feedback;
mod camera_feedback_world;
mod camera_focus;
mod camera_generator_input;
mod camera_generator_target;
mod camera_generator_timing;
mod camera_jump_pitch;
mod camera_lens;
mod camera_lookthrough;
mod camera_orientation;
mod camera_output;
mod camera_pipeline;
mod side_scroller;
mod contact_stimuli;
mod camera_pitch;
mod camera_placement;
mod camera_placement_pass;
mod camera_polar_prepare;
mod camera_profile_selection;
mod camera_profiles;
mod camera_projection;
mod camera_reset;
mod camera_screen_steer;
mod camera_seed;
mod camera_target;
mod camera_transition;
mod camera_wrapper;
mod camera_yaw_control;
mod camera_yaw_pass;
mod cast_contacts;
mod ai_agent_motion;
mod ai_agent_motion_world;
mod ai_controller_navigation;
mod ai_controller_navigation_world;
mod ai_controller_navigation_factory;
mod ai_navigation_owned_factory;
mod ai_navigation_children_init;
mod ai_pathfinder_definition_binding;
mod ai_definition_strings;
mod ai_goto_gap_definition_binding;
mod ai_pathfinder_modifier_factory;
mod ai_pathfinder_concrete_modifiers;
mod ai_predict_move;
mod ai_node_candidate;
mod ai_node_reached;
mod native_direction;
mod ai_steering;
mod ai_compute_target;
mod ai_nearest_search;
mod ai_node_collection;
mod ai_node_collection_binding;
mod ai_find_nodes;
mod ai_find_nodes_definition;
#[cfg(test)]
mod ai_find_nodes_composed_tests;
mod ai_compute_target_definition;
mod native_segment_intersection;
mod ai_steering_binding;
mod ai_steering_definition;
mod ai_edge_awareness;
mod ai_edge_awareness_binding;
mod ai_node_candidate_definition;
mod ai_predict_move_definition;
mod ai_predict_move_binding;
mod ai_navigation_owner_queries;
mod ai_goal_detection;
mod ai_pathfinder_can_go;
mod ai_goal_pathfinder_binding;
mod ai_lpf_module;
mod ai_lpf_context;
mod native_text_number;
mod ai_lpf_definition_binding;
mod entity_visibility;
mod entity_visibility_world;
mod character_body_info;
mod character_contact_response;
mod character_contacts;
mod character_hull;
mod character_physics;
mod character_runtime_storage;
mod checkpoint;
mod checkpoint_metadata;
mod checkpoint_time;
mod child_entities;
mod child_templates;
mod clip_instance;
mod clip_preview;
mod clip_runtime;
mod collectibles;
mod collidable_owner;
mod collision;
mod component_control;
mod component_events;
mod component_names;
mod context_stack;
mod controller;
mod controller_animation;
mod controller_feedback;
#[cfg(debug_assertions)]
pub fn validate_animation_payloads()->anyhow::Result<()> {controller_feedback::validate_payloads()}
#[cfg(all(debug_assertions,feature="blotworx-ui-render"))]
pub fn validate_animation_audio()->anyhow::Result<()> {audio_playback::validate_animation_audio()}
mod script_feedback;
mod controller_ground;
mod controller_lifecycle;
mod cursor;
mod cursor_runtime;
mod cutscene_actions;
mod cutscene_input;
mod debug_font;
mod debug_view;
mod decal;
mod decal_geometry;
mod decal_material;
mod decal_preview;
mod decal_queue;
mod decal_targets;
mod demo_ai;
mod enemy_armor;
mod boss_ai;
mod ai_projectiles;
mod demo_npc_behaviors;
mod demo_counter;
mod demo_gameplay;
mod demo_scene;
mod demo_story;
mod dialogue_selection;
mod demo_triggers;
mod dynamic_reward;
mod dynamic_body;
#[cfg(debug_assertions)]
pub fn validate_dbc_courtyard_armor() -> anyhow::Result<()> {
    dynamic_runtime::validate_dbc_courtyard_armor()
}
mod dynamic_runtime;
#[cfg(debug_assertions)]
pub fn validate_dbc_hall_door() -> anyhow::Result<()> { dynamic_runtime::validate_dbc_hall_door() }
#[cfg(debug_assertions)]
pub fn validate_dbc_arm_trigger() -> anyhow::Result<()> {
    demo_triggers::validate_dbc_arm_trigger()
}
mod ears;
mod effect_runtime;
mod effect_visibility;
mod emitter_control;
mod entity_cleanup;
mod entity_component_lookup;
mod entity_events;
mod entity_handle_lookup;
mod entity_handle_registration;
mod entity_handle_runtime;
mod entity_resource_lifecycle;
mod scene_group_registry;
mod scene_key;
mod entity_pose_copy;
mod entity_property;
mod parenting;
mod entity_registry;
mod entity_relocation;
mod entity_status;
mod entity_tracker;
mod expression;
mod expression_instances;
mod extras;
mod frame_clock;
mod game_clock;
mod game_events;
mod game_random;
mod generated_playback;
mod generator_output;
mod generator_plan;
mod global_state;
mod graph_events;
mod graph_update;
mod ground_history;
mod guardian_parameters;
mod guardians;
mod havok_playback;
mod health;
mod hk_reference;
mod input;
mod input_playback;
mod input_presentation;
mod inspection_vertex_cache;
mod inventory_commands;
mod inventory_properties;
mod inventory_feedback;
mod inventory_state;
#[doc(hidden)]
pub mod jigsaw;
mod jigsaw_collision;
mod jigsaw_effects;
mod jigsaw_material;
mod jigsaw_overlap;
mod jigsaw_state;
mod jigsaw_topology;
mod jigsaw_transition;
mod knockback;
mod ledge_control;
mod ledge_grab;
mod lifetime;
mod level_session;
mod loading_screen;
mod level_transition;
mod endgame;
mod locomotion_actions;
mod locomotion_animation;
mod lod;
mod lua_random;
mod lua_vector;
mod machine_arbitration;
mod machine_echo;
mod machine_runtime;
mod machine_transitions;
mod map_info;
mod pressure_plate;
mod material_animation;
mod material_curve;
mod material_projection;
mod mickey_idle;
mod model_paint;
mod model_billboard;
mod wii_morph_validation;
mod usable_runtime_validation;
mod model_shadow_scene;
mod mopp_query;
mod motion_ground;
mod motion_overrides;
mod movie;
mod movie_skip;
mod movie_tracks;
mod movie_audio;
#[cfg(target_os = "macos")]
mod macos_audio;
mod movie_codec;
mod multi_position_audio;
mod named_effects;
mod native_angle;
mod native_gsk;
mod native_hk_transform;
mod native_matrix;
mod native_motion_limits;
mod native_motion_record;
mod native_motion_step;
mod native_names;
mod native_power;
mod native_rig_pose;
mod native_transform_record;
mod nif_camera;
mod nif_properties;
mod nif_quaternion;
mod nif_runtime;
#[doc(hidden)]
pub mod nif_sequence;
mod nif_transform;
mod nif_types;
mod npc_relocation;
mod paint_counter;
mod particle_mesh;
mod particle_preview;
mod particle_random;
mod particle_space;
mod particle_torus;
mod performance;
mod phantom_assets;
mod phantom_bounds;
mod phantom_objects;
mod phantom_overlaps;
mod phantom_pairs;
mod phantom_update;
mod phantom_world;
mod physics;
mod physics_filter;
mod physics_operations;
mod physics_system;
mod physics_world;
mod pickup;
mod pickup_lifecycle;
mod pickup_listener;
mod pickup_world_init;
mod player_knockback;
mod player_physics;
mod player_relocation;
mod player_spawn;
mod playstyle_points;
mod pose_blend;
mod projected_shadow;
mod projectile_shape;
mod projectile_stimulus;
mod quest_state;
pub(crate) mod raster_lighting;
mod render_color_effect;
mod render_object;
mod render_options;
mod render_probe;
mod render_transform;
mod resource_path;
mod resource_store;
mod ribbon;
mod rigid_layer;
mod robot_arm;
mod robot_arm_spotlight;
mod robot_arm_turn;
mod robot_arm_behavior;
mod robot_arm_behavior_world;
mod robot_arm_target;
mod rotator;
mod rotator_runtime;
mod rotator_tracking;
mod save_game;
mod save_instances;
mod save_options;
mod save_player;
mod save_service;
mod save_world;
mod scene_bounds;
mod scene_loading;
mod scene_particles;
mod scene_resources;
mod score_state;
mod screen_fade;
mod script_stimulus;
mod shadow_scene;
mod shadow_volume;
mod shape_bounds;
mod shape_bounds_import;
mod shape_welding;
mod skeletal_children;
mod sketch_assets;
mod sketch_render;
mod skin;
mod smoke_validation;
mod spawner_schedule;
mod spawn_selection;
mod script_handles;
mod countdown;
mod logic_timer;
mod logic_timer_runtime;
mod logic_relay;
mod io_admission;
mod config_value;
mod countdown_runtime;
mod spherical_lighting;
mod spin;
mod stairs;
mod state_machine;
mod state_start;
mod story_event;
mod story_instance;
mod story_sequences;
mod story_rewards;
mod spline_branch;
mod spline_events;
mod string_property;
mod integer_property;
mod script_damage;
mod facing_rotation;
mod script_transformation;
mod movement_input;
mod counter_queries;
mod entity_event_runtime;
mod subgraph_color;
mod support;
pub(crate) mod texture_effect;
mod tracker_world;
mod transform_attachment;
mod transform_cache;
mod transform_cache_provider;
mod transform_child_list;
mod transform_children;
mod transform_depth;
mod transform_descendants;
mod transform_detach;
mod transform_dirty;
mod transform_hierarchy;
mod transform_local;
mod transform_parent;
mod transform_parent_cache;
mod transform_record_hooks;
mod transform_rotation_scale;
mod transform_runtime;
mod transform_source;
mod transform_storage;
mod transform_translation;
mod transform_update;
mod transform_world;
mod transition_effect;
mod transition_instances;
mod transition_select;
mod transition_timing;
mod trigger_damage;
mod twist_modifier;
mod ui_calls;
#[cfg(feature = "blotworx-ui")]
mod ui_effects;
#[cfg(feature = "blotworx-ui")]
mod ui_images;
mod ui_initialization;
#[cfg(feature = "blotworx-ui")]
pub mod ui_movie;
mod variable_bindings;
mod visibility_children;
mod vulnerability;
mod vulnerability_api;
mod vulnerability_direction;
mod weapon_direction;
use crate::render_support::viewport3d::{self, LightingMode, RenderMode, Viewport3DState};
use anyhow::{Context, Result};
use assets::World;
use physics::{add, Player};
use std::{path::PathBuf, sync::mpsc, time::Instant};
// Stock EM1 controller timestep, measured at motion+0x118 in the 30 FPS
// VillageTest reference capture. Render interpolation remains independent.
const STEP: f32 = 1.0 / 30.0;
struct Options {
    updates: bool,
    continue_checkpoint: bool,
    shared_playtest: bool,
    save_directory: Option<PathBuf>,
    save_slot: usize,
    rtc_seconds: Option<u32>,
    capture_realtime: bool,
    diagnostic_serial_hud: bool,
    original_movie_hud: bool,
    diagnostic_eager_hud: bool,
    diagnostic_no_view_culling: bool,
    diagnostic_no_hud_reuse: bool,
    diagnostic_hud_driver_validation: bool,
    diagnostic_checkpoint_timing: bool,
    diagnostic_counter_impact: Option<(String, f32)>,
    particle_asset_preview: Option<String>,
    particle_materials: bool,
    preview_entry: Option<String>,
    render_probe: Option<[f32; 2]>,
    preview_runtime: bool,
    strict_npc_ai: bool,
    show_options: bool,
    graphics_profile: Option<options_menu::Profile>,
    capture_hide_hose: bool,
    diagnostic_no_model_shadow: bool,
    capture_hide_decals: bool,
    input_playback: Option<input_playback::Playback>,
    diagnostic_pickup_commands: Option<String>,
    diagnostic_relocation: Option<(String, [f32; 3])>,
    live_actor: Option<(String, String)>,
    runtime_script: Option<PathBuf>,
    actor_focus: Option<String>,
    diagnostic_morph: Option<assets::MorphPreview>,
    game_rendering: bool,
    diagnostic_hose: Option<usize>,
    diagnostic_hose_ticks: Option<u64>,
    diagnostic_aim: [f32; 2],
    capture_camera_target: Option<[f32; 3]>,
    capture_jigsaw_endpoint: Option<bool>,
    capture_jigsaw_animate: bool,
    diagnostic_strafe: bool,
    diagnostic_shuffle: Option<bool>,
    diagnostic_rig: Option<PathBuf>,
    diagnostic_clip: Option<PathBuf>,
    root: Option<PathBuf>,
    level: String,
    inspect: Option<PathBuf>,
    inspect_save: Option<PathBuf>,
    smoke: bool,
    diagnostic_stream_memory_cycle: Option<String>,
    debug: bool,
    capture: Option<PathBuf>,
    capture_tick: Option<u64>,
    capture_movie_frame: Option<u32>,
    capture_switch_level: Option<String>,
    join: Option<String>,
}
fn options() -> Result<Options> {
    let mut o = Options {
        updates: false,
        continue_checkpoint: false,
        shared_playtest: false,
        save_directory: None,
        save_slot: 0,
        rtc_seconds: None,
        capture_realtime: false,
        original_movie_hud: cfg!(feature = "blotworx-ui-render"),
        diagnostic_serial_hud: false,
        diagnostic_eager_hud: false,
        diagnostic_no_view_culling: false,
        diagnostic_no_hud_reuse: false,
        diagnostic_hud_driver_validation: false,
        diagnostic_checkpoint_timing: false,
        particle_asset_preview: None,
        particle_materials: false,
        preview_entry: None,
        render_probe: None,
        preview_runtime: true,
        strict_npc_ai: false,
        show_options: false,
        graphics_profile: None,
        capture_hide_hose: false,
        diagnostic_no_model_shadow: false,
        capture_hide_decals: false,
        input_playback: None,
        diagnostic_pickup_commands: None,
        diagnostic_counter_impact: None,
        diagnostic_relocation: None,
        live_actor: None,
        runtime_script: None,
        actor_focus: None,
        diagnostic_morph: None,
        game_rendering: true,
        diagnostic_hose: None,
        diagnostic_hose_ticks: None,
        diagnostic_aim: [0.5, 0.5],
        capture_camera_target: None,
        capture_jigsaw_endpoint: None,
        capture_jigsaw_animate: false,
        diagnostic_strafe: false,
        diagnostic_shuffle: None,
        diagnostic_rig: None,
        diagnostic_clip: None,
        root: None,
        level: "Levels/DBC_Start_1st.Level".into(),
        inspect: None,
        inspect_save: None,
        smoke: false,
        diagnostic_stream_memory_cycle: None,
        debug: false,
        capture: None,
        capture_tick: None,
        capture_movie_frame: None,
        capture_switch_level: None,
        join: None,
    };
    let mut args = std::env::args().skip(1).peekable();
    let diagnostic = args.next_if(|arg| arg == "diagnose").is_some();
    if !diagnostic {
        #[cfg(not(target_os = "horizon"))]
        { o.level = disc_setup::START_LEVEL.into(); }
        #[cfg(target_os = "horizon")]
        { o.level = "Levels/GV_ZoneA_Start.level".into(); }
        o.updates = true;
    }
    while let Some(a) = args.next() {
        if a.starts_with("blotworx://") || a.starts_with("BLOTWORX://") {
            o.join = Some(a);
            continue;
        }
        if !diagnostic && !matches!(a.as_str(), "--help" | "-h" | "--data" | "--level" | "--continue" | "--shared-playtest" | "--join") {
            anyhow::bail!("Unknown launch option {a}. Use the in-game menus or run blotworx diagnose --help for developer tools.");
        }
        match a.as_str() {
            "--join" => o.join = Some(args.next().context("--join needs an IP:port or blotworx:// link")?),
            "--continue" => o.continue_checkpoint = true,
            "--save-dir" => {
                o.save_directory = Some(PathBuf::from(
                    args.next().context("--save-dir needs a directory")?,
                ))
            }
            "--save-slot" => {
                o.save_slot = args.next().context("--save-slot needs0,1 or2")?.parse()?;
                anyhow::ensure!(o.save_slot < 3, "Save slot must be0,1 or2");
            }
            "--rtc-seconds" => {
                o.rtc_seconds = Some(
                    args.next()
                        .context("--rtc-seconds needs whole Wii RTC seconds since2000")?
                        .parse()?,
                )
            }
            "--inspect-save" => {
                o.inspect_save = Some(PathBuf::from(
                    args.next().context("--inspect-save needs a save path")?,
                ))
            }
            "--diagnostic-checkpoint-timing" => o.diagnostic_checkpoint_timing = true,
            "--capture-realtime" => o.capture_realtime = true,
            "--original-movie-hud" => {
                anyhow::ensure!(cfg!(feature = "blotworx-ui-render"), "Original HUD renderer is unavailable in this build");
                o.original_movie_hud = true;
            }
            "--basic-hud" => o.original_movie_hud = false,
            "--diagnostic-serial-hud" => o.diagnostic_serial_hud = true,
            "--diagnostic-eager-hud" => o.diagnostic_eager_hud = true,
            "--diagnostic-hud-driver-validation" => o.diagnostic_hud_driver_validation = true,
            "--diagnostic-no-hud-reuse" => o.diagnostic_no_hud_reuse = true,
            "--diagnostic-no-view-culling" => o.diagnostic_no_view_culling = true,
            "--diagnostic-guardian-particles" => {
                o.particle_materials = false;
                let kind = args
                    .next()
                    .context("--diagnostic-guardian-particles needs paint or thinner")?;
                o.particle_asset_preview = Some(
                    match kind.as_str() {
                        "paint" => "paintguardian",
                        "thinner" => "thinnerguardian",
                        _ => anyhow::bail!("Guardian particle inspection needs paint or thinner"),
                    }
                    .into(),
                );
            }
            "--diagnostic-particle-template" => {
                o.particle_materials = true;
                o.particle_asset_preview = Some(
                    args.next()
                        .context("--diagnostic-particle-template needs an entity template name")?,
                );
            }
            "--preview-entry" => {
                o.preview_entry = Some(
                    args.next()
                        .context("--preview-entry requires an authored entity name")?,
                )
            }
            "--diagnostic-render-probe" => {
                let x: f32 = args
                    .next()
                    .context("Render probe requires normalized x and y")?
                    .parse()?;
                let y: f32 = args
                    .next()
                    .context("Render probe requires normalized x and y")?
                    .parse()?;
                anyhow::ensure!(
                    [x, y]
                        .iter()
                        .all(|v| v.is_finite() && (0.0..=1.).contains(v)),
                    "Invalid render probe coordinates"
                );
                o.render_probe = Some([x, y]);
            }
            "--shared-playtest" => o.shared_playtest = true,
            "--strict-runtime" => o.preview_runtime = false,
            "--strict-npc-ai" => o.strict_npc_ai = true,
            "--show-options" => o.show_options = true,
            "--graphics-profile" => o.graphics_profile=Some(match args.next().context("Expected classic, classic+ or remix")?.as_str(){"classic"=>options_menu::Profile::Classic,"classic+"=>options_menu::Profile::ClassicPlus,"remix"=>options_menu::Profile::Remix,_=>anyhow::bail!("Expected classic, classic+ or remix")}),
            "--diagnostic-hide-hose" => o.capture_hide_hose = true,
            "--diagnostic-no-model-shadow" => o.diagnostic_no_model_shadow = true,
            "--diagnostic-hide-decals" => o.capture_hide_decals = true,
            "--input-playback" => {
                o.input_playback = Some(input_playback::Playback::load(std::path::Path::new(
                    &args.next().context("--input-playback needs a JSON path")?,
                ))?)
            }
            "--diagnostic-pickup-commands" => {
                o.diagnostic_pickup_commands = Some(
                    args.next()
                        .context("--diagnostic-pickup-commands needs an entity name")?,
                );
            }
            "--diagnostic-relocation" => {
                let name = args
                    .next()
                    .context("--diagnostic-relocation needs an entity name")?;
                let xyz = (0..3)
                    .map(|_| {
                        args.next()
                            .context("--diagnostic-relocation needs absolute x y z")?
                            .parse::<f32>()
                            .context("Invalid relocation position")
                    })
                    .collect::<Result<Vec<_>>>()?;
                anyhow::ensure!(
                    xyz.iter().all(|v| v.is_finite()),
                    "Invalid relocation position"
                );
                o.diagnostic_relocation = Some((name, xyz.try_into().unwrap()));
            }
            "--play-actor" => {
                o.live_actor = Some((
                    args.next().context("--play-actor needs an entity name")?,
                    args.next().context("--play-actor needs a sequence name")?,
                ));
            }
            "--run-script" => {
                o.runtime_script = Some(PathBuf::from(
                    args.next().context("--run-script needs a Lua path")?,
                ));
            }
            "--focus-actor" => {
                o.actor_focus = Some(args.next().context("--focus-actor needs an entity name")?);
            }
            "--diagnostic-counter-impact" => {
                let entity = args
                    .next()
                    .context("--diagnostic-counter-impact needs an entity name")?;
                let amount = match args.next().as_deref() {
                    Some("paint") => 1.,
                    Some("thinner") => -1.,
                    _ => anyhow::bail!("--diagnostic-counter-impact needs paint or thinner"),
                };
                o.diagnostic_counter_impact = Some((entity, amount));
            }
            "--diagnostic-morph" => {
                let model = args
                    .next()
                    .context("--diagnostic-morph needs model, keyframes, sequence and time")?;
                let keyframes = args.next().context("Missing morph keyframe asset")?;
                let sequence = args.next().context("Missing morph sequence name")?;
                let time: f32 = args
                    .next()
                    .context("Missing morph sequence time")?
                    .parse()?;
                anyhow::ensure!(time.is_finite() && time >= 0., "Invalid morph time");
                o.diagnostic_morph = Some(assets::MorphPreview {
                    model,
                    keyframes,
                    sequence,
                    time,
                });
            }
            "--game-rendering" => o.game_rendering = true,
            "--inspection-rendering" => o.game_rendering = false,
            "--diagnostic-brush-strafe" => o.diagnostic_strafe = true,
            "--diagnostic-brush-shuffle" => {
                o.diagnostic_shuffle = Some(match args.next().as_deref() {
                    Some("left") => true,
                    Some("right") => false,
                    _ => anyhow::bail!("--diagnostic-brush-shuffle requires left or right"),
                });
            }
            "--capture-movie-frame" => {
                o.capture_movie_frame = Some(
                    args.next()
                        .context("--capture-movie-frame needs a frame index")?
                        .parse()?,
                );
            }
            "--capture-tick" => {
                let tick: u64 = args
                    .next()
                    .context("--capture-tick needs a simulation tick")?
                    .parse()?;
                anyhow::ensure!(tick > 0, "--capture-tick must be positive");
                o.capture_tick = Some(tick);
            }
            "--capture-jigsaw-transition" => {
                o.capture_jigsaw_animate = true;
                o.capture_jigsaw_endpoint = Some(match args.next().as_deref() {
                    Some("paint") => true,
                    Some("thinner") => false,
                    _ => anyhow::bail!("Expected paint or thinner"),
                });
            }
            "--capture-jigsaw-endpoint" => {
                o.capture_jigsaw_endpoint = Some(match args.next().as_deref() {
                    Some("paint") => true,
                    Some("thinner") => false,
                    _ => anyhow::bail!("Expected paint or thinner"),
                })
            }
            "--capture-camera-target" => {
                let v = args.next().context("--capture-camera-target needs x,y,z")?;
                let xyz = v
                    .split(',')
                    .map(str::parse::<f32>)
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                anyhow::ensure!(
                    xyz.len() == 3 && xyz.iter().all(|v| v.is_finite()),
                    "Invalid capture camera target"
                );
                o.capture_camera_target = Some(xyz.try_into().unwrap());
            }
            "--diagnostic-aim" => {
                let value = args
                    .next()
                    .context("--diagnostic-aim needs normalized x,y")?;
                let (x, y) = value
                    .split_once(',')
                    .context("--diagnostic-aim needs normalized x,y")?;
                o.diagnostic_aim = [x.parse()?, y.parse()?];
                anyhow::ensure!(
                    o.diagnostic_aim
                        .iter()
                        .all(|v| v.is_finite() && (0. ..=1.).contains(v)),
                    "Diagnostic aim must be within 0..1"
                );
            }
            "--diagnostic-hose-ticks" => {
                let ticks: u64 = args
                    .next()
                    .context("--diagnostic-hose-ticks needs a count")?
                    .parse()?;
                anyhow::ensure!(ticks > 0, "--diagnostic-hose-ticks must be positive");
                o.diagnostic_hose_ticks = Some(ticks);
            }
            "--diagnostic-hose" => {
                o.diagnostic_hose = Some(match args.next().as_deref() {
                    Some("paint") => 0,
                    Some("thinner") => 1,
                    _ => anyhow::bail!("--diagnostic-hose expects paint or thinner"),
                })
            }
            "--diagnostic-rig" => {
                o.diagnostic_rig =
                    Some(args.next().context("--diagnostic-rig needs a path")?.into())
            }
            "--diagnostic-clip" => {
                o.diagnostic_clip = Some(
                    args.next()
                        .context("--diagnostic-clip needs a path")?
                        .into(),
                )
            }
            "--data" => o.root = Some(args.next().context("--data needs a path")?.into()),
            "--level" => o.level = args.next().context("--level needs an asset path")?,
            "--capture-switch-level" => {
                o.capture_switch_level = Some(
                    args.next()
                        .context("--capture-switch-level needs an asset path")?,
                )
            }
            "--inspect" => {
                o.inspect = Some(
                    args.next()
                        .context("--inspect needs an output JSON path")?
                        .into(),
                )
            }
            "--diagnostic-stream-memory-cycle" => {
                o.diagnostic_stream_memory_cycle = Some(
                    args.next()
                        .context("--diagnostic-stream-memory-cycle needs a .gsa scene path")?,
                );
            }
            "--smoke" => o.smoke = true,
            "--debug" => o.debug = true,
            "--capture" => {
                o.capture = Some(args.next().context("--capture needs a PNG path")?.into())
            }
            "--help" | "-h" => {
                if !diagnostic {
                    println!(concat!(
                        "Blotworx\n\n",
                        "Run blotworx with no arguments to play. Choose a game folder in the app.\n",
                        "F10 opens settings and the level selector. F3 opens the debug overlay.\n\n",
                        "Optional shortcuts: --data <folder> --level <level> --continue\n",
                        "Developer tools: blotworx diagnose --help",
                    ));
                    std::process::exit(0);
                }
                println!("  --save-dir <directory> (default: Blotworx Engine application storage; separate from native saves)\n  --save-slot <0|1|2>\n  --rtc-seconds <whole seconds since2000> (repeatable hardware counter seed)");
                println!("  --diagnostic-checkpoint-timing (explicit mutating timer capture; requires timed --capture; no full checkpoint)");
                println!("  --strict-npc-ai (disable the explicitly temporary Gremlin NPC fallback)");
                println!("  --capture-realtime (use interactive monotonic timing in --capture; incompatible with --input-playback)");
                println!("  --diagnostic-hud-driver-validation (enable native API graphics debug layers)");
                println!("  --diagnostic-no-hud-reuse (register every HUD vector for controlled comparison)");
                println!("  --diagnostic-no-view-culling (controlled renderer comparison)");
                println!("  --diagnostic-eager-hud (wait for pixel readback before scene submission; controlled comparison)");
                println!("  --diagnostic-serial-hud (wait for HUD only after scene preparation; controlled worker benchmark)");
                println!("  --diagnostic-guardian-particles <paint|thinner> (root particle asset inspection; no guardian gameplay or ribbons)");
                println!("  --diagnostic-particle-template <name> (root particles from a loaded scene/palette; no effect entity, scripts or audio)");
                println!("  --shared-playtest (EM2 provisional shared movement/camera; also toggled with F6)");
                println!("  --strict-runtime (disable temporary gameplay adapters for diagnostics)");
                println!("  --input-playback <json> (live tick inputs; requires --capture and --capture-tick, matching --level)");
                println!("  --diagnostic-pickup-commands <entity> (replay authored Give commands; no overlap or pickup removal)");
                println!("  --run-script <Lua source or original Lua 5.1 chunk>\n  --focus-actor <exact entity name> (camera only)\n  --play-actor <entity> <sequence> (manual repeating inspection override)");
                println!("Blotworx Engine\n  --data <extracted DATA or files folder>\n  --level <Levels/EB_VillageTest.level>\n  --inspect <report.json> [--smoke]\n  --capture <screenshot.png> [--capture-tick <simulation tick>]\n  --game-rendering (default: game materials and scene lighting)\n  --inspection-rendering (diagnostic headlight; replaces game materials and baked lighting)\n  --diagnostic-hose <paint|thinner> (visual capture only)\n  --diagnostic-hose-ticks <count> (release after count simulation ticks)\n  --diagnostic-aim <x,y> (normalized capture aim, default 0.5,0.5)\n  --diagnostic-rig <rig.hkx_wii> --diagnostic-clip <clip.hkx_wii> (skeletal inspection only)\n--diagnostic-morph <model> <keyframes> <sequence> <seconds> (fixed morph pose, not gameplay activation)\nWithout --data, uses the most recent available BlueThinner build.\nWASD: move | Space: jump | Right drag: camera | Wheel: zoom | R: respawn | Escape: pause");
                println!("  --diagnostic-stream-memory-cycle <scene.gsa> (inspect-mode deactivate/reactivate round trip)");
                std::process::exit(0)
            }
            _ => anyhow::bail!("Unknown option {a}"),
        }
    }
    anyhow::ensure!(
        !(o.capture_hide_hose || o.capture_hide_decals) || o.capture.is_some(),
        "Effect visibility diagnostics require --capture; they suppress drawing only"
    );
    anyhow::ensure!(
        !o.diagnostic_checkpoint_timing || (o.capture.is_some() && o.capture_tick.is_some()),
        "Checkpoint timing diagnostic requires --capture and --capture-tick"
    );
    anyhow::ensure!(
        !o.capture_realtime || (o.capture.is_some() && o.input_playback.is_none()),
        "Realtime capture requires --capture and cannot use --input-playback"
    );
    anyhow::ensure!(
        o.render_probe.is_none() || (o.capture.is_some() && o.capture_tick.is_some()),
        "Render probe requires a timed capture"
    );
    anyhow::ensure!(
        o.capture_jigsaw_endpoint.is_none() || o.capture.is_some(),
        "--capture-jigsaw-endpoint requires --capture (diagnostic, not gameplay)"
    );
    anyhow::ensure!(
        o.capture_camera_target.is_none() || o.capture.is_some(),
        "--capture-camera-target requires --capture"
    );
    anyhow::ensure!(
        o.capture_switch_level.is_none() || (o.capture.is_some() && o.capture_tick.is_some()),
        "--capture-switch-level requires --capture and --capture-tick"
    );
    anyhow::ensure!(
        o.diagnostic_stream_memory_cycle.is_none() || o.inspect.is_some(),
        "--diagnostic-stream-memory-cycle requires --inspect"
    );
    anyhow::ensure!(
        !o.diagnostic_strafe || o.diagnostic_shuffle.is_some(),
        "--diagnostic-brush-strafe requires --diagnostic-brush-shuffle left|right"
    );
    anyhow::ensure!(
        o.diagnostic_rig.is_some() == o.diagnostic_clip.is_some(),
        "Supply both --diagnostic-rig and --diagnostic-clip"
    );
    if let Some(playback) = &o.input_playback {
        anyhow::ensure!(o.capture.is_some(), "--input-playback requires --capture");
        let tick = o
            .capture_tick
            .context("--input-playback requires --capture-tick")?;
        anyhow::ensure!(
            tick <= playback.frames.len() as u64,
            "Capture tick exceeds playback length"
        );
        anyhow::ensure!(
            o.level
                .replace('\\', "/")
                .eq_ignore_ascii_case(&playback.level.replace('\\', "/")),
            "Playback level differs from --level"
        );
        anyhow::ensure!(
            o.diagnostic_hose.is_none()
                && !o.diagnostic_strafe
                && o.diagnostic_shuffle.is_none()
                && o.diagnostic_morph.is_none()
                && o.diagnostic_rig.is_none(),
            "Input playback cannot be mixed with diagnostic pose/action overrides"
        );
    }
    Ok(o)
}
fn player_scalar_capture(world: &assets::World) -> serde_json::Value {
    let capture = (|| -> Result<serde_json::Value> {
        let health = world.player_health_save_state()?;
        let ammo = world.player_ammo_save_state()?;
        let inventory = world.player_inventory_save_state()?;
        let mut records = Vec::new();
        health.append(&mut records)?;
        world.playstyle_points.append_record(&mut records)?;
        inventory.append(&mut records)?;
        ammo.append(&mut records)?;
        Ok(
            serde_json::json!({"health":health,"inventory":inventory,"ammo":ammo.summary(),"playstyle_points":world.playstyle_points.summary(),
            "record_count":save_game::records(&records)?.len(),"complete_mick":false,
            "player_initialization_implemented":false,"checkpoint_integrated":false,
            "missing_live_groups":["playstyle scoring/reconciliation events","ability initialization and derived Give/action side effects",
                "ticket/sketch inventory mutations","remaining checkpoint header/validity and spawn coordination"]}),
        )
    })();
    capture.unwrap_or_else(|error|serde_json::json!({"error":format!("{error:#}"),"complete_mick":false,"checkpoint_integrated":false}))
}
fn player_record_capture(world: &assets::World) -> serde_json::Value {
    world
        .capture_player_records()
        .and_then(|capture| capture.summary())
        .unwrap_or_else(
            |error| serde_json::json!({"error":format!("{error:#}"),"checkpoint_integrated":false}),
        )
}
pub fn run() -> Result<()> {
    run_platform(#[cfg(target_os = "android")] None)
}

#[cfg(target_os = "horizon")]
pub(crate) fn run_platform() -> Result<()> {
    horizon_3ds::run()
}

#[cfg(not(target_os = "horizon"))]
pub(crate) fn run_platform(
    #[cfg(target_os = "android")] android_app: Option<android_activity::AndroidApp>,
) -> Result<()> {
    if updater::finish_if_requested()? { return Ok(()); }
    {
        let mut args = std::env::args().skip(1);
        if args.next().as_deref() == Some("export-3ds") {
            let mut data_root: Option<PathBuf> = None;
            let mut output_dir: Option<PathBuf> = None;
            let mut levels: Vec<String> = Vec::new();
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--data" => data_root = Some(PathBuf::from(args.next().context("--data requires a directory")?)),
                    "--output" => output_dir = Some(PathBuf::from(args.next().context("--output requires a directory")?)),
                    "--level" => levels.push(args.next().context("--level requires a level path")?),
                    "--help" | "-h" => {
                        println!("Usage: blotworx export-3ds --data <DATA_DIR> --output <SD_BLOTWORX_DIR> [--level <Levels/Name.level>]");
                        return Ok(());
                    }
                    other => anyhow::bail!("Unknown export-3ds option {other}"),
                }
            }
            let data = data_root
                .or_else(assets::default_root)
                .context("Specify --data <extracted DATA folder> for 3DS bundle export")?;
            let out = output_dir.context("Specify --output <output folder> for 3DS bundle export")?;
            export_3ds::export_bundle_cli(&data, &out, &levels)?;
            return Ok(());
        }
    }
    #[cfg(debug_assertions)]
    {
        let mut args = std::env::args_os().skip(1);
        if args.next().as_deref() == Some(std::ffi::OsStr::new("diagnose"))
            && args.next().as_deref() == Some(std::ffi::OsStr::new("--verify-disc-import"))
        {
            let image = PathBuf::from(args.next().context("Choose a disc image")?);
            let destination = PathBuf::from(args.next().context("Choose a test library directory")?);
            return disc_setup::verify_import(&image, &destination);
        }
    }
    let o = options()?;
    if let Some(root) = o.root.as_deref().map(assets::files_root) {
        if em2::is_install(&root) { return em2::run(o, &root); }
    }
    if let Some(save_path) = &o.inspect_save {
        let output = o
            .inspect
            .as_ref()
            .context("--inspect-save requires --inspect REPORT.json")?;
        let bytes = std::fs::read(save_path)
            .with_context(|| format!("Read save {}", save_path.display()))?;
        let save = save_game::Archive::parse(&bytes)?;
        let sections=save.sections.iter().map(|section|serde_json::json!({
            "tag":String::from_utf8_lossy(&section.tag),"version":section.version,"bytes":section.payload.len()
        })).collect::<Vec<_>>();
        let mut report = serde_json::json!({"format":"EM1 GAME","version":16,
            "storage_bytes":bytes.len(),"logical_bytes":save.logical_size,
            "level":save.header_string(0x10,0x3c)?,"sections":sections,
            "gameplay_restore_implemented":false,
            "player_spawn":save.optional_section(*b"MICK")?
                .map(|section|player_spawn::saved_transform(&section.payload)).transpose()?.flatten()});
        report["options"] = save_options::State::from_header(&bytes)?.summary();
        // LAST-only empty/carryover slots do not establish that the map and
        // player summary fields were populated by the checkpoint coordinator.
        report["checkpoint_summary"] = save
            .optional_section(*b"MICK")?
            .map(|_| checkpoint_metadata::Summary::from_header(&bytes).map(|state| state.summary()))
            .transpose()?
            .unwrap_or(serde_json::Value::Null);
        report["checkpoint_global_metadata"] =
            checkpoint_metadata::Globals::from_header(&bytes)?.summary();
        report["checkpoint_inventory_metadata"] = save
            .optional_section(*b"MICK")?
            .map(|_| {
                checkpoint_metadata::InventoryLimits::from_header(&bytes)
                    .map(|state| state.summary())
            })
            .transpose()?
            .unwrap_or(serde_json::Value::Null);
        report["checkpoint_timing"] = save
            .optional_section(*b"MICK")?
            .map(|_| checkpoint_time::Timing::from_header(&bytes).map(|state| state.summary()))
            .transpose()?
            .unwrap_or(serde_json::Value::Null);
        report["playthrough"] = serde_json::json!({"value":score_state::Playthrough::from_header(&bytes)?.value,"gameplay_restore_implemented":false});
        report["player_scalar_records"]=save.optional_section(*b"MICK")?.map(|section|->Result<serde_json::Value> {
            anyhow::ensure!(section.version==3,"Unsupported MICK save version");
            let mut playstyle=playstyle_points::State::default();playstyle.restore_record(&section.payload)?;
            Ok(serde_json::json!({"health":save_player::Health::decode(&section.payload)?,
                "inventory":save_player::Inventory::decode(&section.payload)?,
                "ammo":save_player::Ammo::decode(&section.payload)?.summary(),"playstyle_points":playstyle.summary(),"gameplay_restore_implemented":false}))
        }).transpose()?.unwrap_or(serde_json::Value::Null);
        report["instance_changes"]=save.optional_section(*b"AISS")?.map(|section|->Result<serde_json::Value> {
            anyhow::ensure!(section.version==2,"Unsupported AISS save version");
            let mut state=save_instances::State::default();state.restore_payload(&section.payload)?;
            Ok(serde_json::json!({"state":state.summary(),"section_round_trip_matches":state.save_payload()?==*section.payload}))
        }).transpose()?.unwrap_or(serde_json::Value::Null);
        let options = save.optional_section(*b"OPTN")?;
        if let Some(section) = options {
            anyhow::ensure!(section.version == 1, "Unsupported OPTN save version");
        }
        report["options"]["present"] = serde_json::json!(options.is_some());
        report["script_save_owner"] = serde_json::json!({"version":save_options::SCRIPT_SECTION.1,
            "serialized_lua_state":false,"expected_payload_bytes":save_options::SCRIPT_SECTION.2.len(),
            "present":save.optional_section(save_options::SCRIPT_SECTION.0)?.is_some(),"checkpoint_integrated":false});
        report["world_scene_requests"] = save
            .optional_section(*b"WRLD")?
            .map(|section| {
                anyhow::ensure!(section.version == 1, "Unsupported WRLD save version");
                save_world::decode(&section.payload)
            })
            .transpose()?
            .map(|scenes| serde_json::json!({"scenes":scenes,"restoration_implemented":false}))
            .unwrap_or(serde_json::Value::Null);
        let rebuilt = save.rebuild_memory()?;
        let rebuilt_archive = save_game::Archive::parse(&rebuilt.bytes)?;
        let owners_match = save.sections.len() == rebuilt_archive.sections.len()
            && save
                .sections
                .iter()
                .zip(&rebuilt_archive.sections)
                .all(|(a, b)| (a.tag, a.version) == (b.tag, b.version) && a.payload == b.payload);
        report["native_memory_writer"] = serde_json::json!({"logical_bytes":rebuilt.logical_size,
            "storage_bytes":rebuilt.bytes.len(),"header_preserved":bytes[..save_game::HEADER_SIZE]==rebuilt.bytes[..save_game::HEADER_SIZE],
            "section_payloads_versions_order_preserved":owners_match,"byte_identical":bytes==rebuilt.bytes,
            "live_checkpoint_capture_implemented":false});
        if let Some(root) = &o.root {
            let (mut collectibles, mut extras) = assets::collectible_owners_from_game(root)?;
            extras.restore_save_header(&bytes)?;
            let mut rewritten = bytes.clone();
            extras.write_save_header(&mut rewritten)?;
            report["extras"] = extras.summary();
            report["extras"]["header_round_trip_matches"] = serde_json::json!(rewritten == bytes);
            collectibles.restore_save_header(&bytes, true)?;
            let mut rewritten = bytes.clone();
            collectibles.write_save_header(&mut rewritten)?;
            report["collectibles"] = collectibles.summary();
            report["collectibles"]["header_round_trip_matches"] =
                serde_json::json!(rewritten == bytes);
            let mut quests = assets::quest_owner_from_game(root)?;
            if let Some(section) = save.optional_section(*b"QUES")? {
                anyhow::ensure!(section.version == 4, "Unsupported QUES version");
                let restored = quests.restore_payload(&section.payload)?;
                let mut writer = save_game::MemoryWriter::from_storage(bytes.clone())?;
                writer.append_section(*b"QUES", 4, &quests.save_payload()?)?;
                let copy = writer.finish()?;
                let rebuilt = save_game::Archive::parse(&copy.bytes)?;
                report["quests"] = quests.summary();
                report["quests"]["restored"] = serde_json::to_value(restored)?;
                report["quests"]["section_round_trip_matches"] =
                    serde_json::json!(section.payload == rebuilt.section(*b"QUES")?.payload);
            } else {
                report["quests"] = quests.summary();
                report["quests"]["restored"] = serde_json::Value::Null;
            }
        }
        std::fs::write(output, serde_json::to_vec_pretty(&report)?)?;
        return Ok(());
    }
    if let Some(path) = &o.inspect {
        let root = o
            .root
            .clone()
            .or_else(assets::default_root)
            .context("Import a game in BlueThinner or use --data")?;
        let mut world = assets::load_with_morph(
            &root,
            &o.level,
            &egui::Context::default(),
            o.diagnostic_morph.as_ref(),
        )?;
        let mut stream_memory_cycle = serde_json::Value::Null;
        if let Some(requested) = &o.diagnostic_stream_memory_cycle {
            let scene = scene_resources::scene_name(requested);
            anyhow::ensure!(
                world.scene_resources.is_active(&scene),
                "Scene {scene} is not currently active in {}",
                o.level
            );
            let player_scene = scene_resources::scene_name(
                &world.entities[world.player_entity].scene,
            );
            anyhow::ensure!(
                scene != player_scene,
                "Cannot deactivate the active player scene {scene}"
            );
            let mut cycles = Vec::with_capacity(8);
            for cycle in 0..8 {
                anyhow::ensure!(
                    world.scene_resources.is_active(&scene),
                    "Scene {scene} was not active before stream-memory cycle {cycle}"
                );
                let before = debug_view::tracked_asset_bytes(&world);
                let process_before = debug_view::process_memory_bytes();
                let box3d_before = physics::box3d_allocated_bytes();
                let hidden = world.deactivate_streamed_scene(&scene)?;
                let compressed_bytes = world
                    .cold_scene_assets
                    .get(&scene)
                    .map(assets::ColdSceneAssets::compressed_bytes)
                    .unwrap_or_default();
                let cold_draw_slots = world
                    .cold_scene_assets
                    .get(&scene)
                    .map(|cold| cold.draw_vertices.len() + cold.draw_indices.len())
                    .unwrap_or_default();
                let cold_texture_draws = world
                    .cold_scene_assets
                    .get(&scene)
                    .map(|cold| cold.draw_images.len() + cold.spherical_images.len())
                    .unwrap_or_default();
                let cold_texture_images = world
                    .cold_scene_assets
                    .get(&scene)
                    .map(|cold| cold.images.len())
                    .unwrap_or_default();
                let inactive = debug_view::tracked_asset_bytes(&world);
                anyhow::ensure!(
                    compressed_bytes > 0,
                    "Scene {scene} had no exclusive compressible static mesh buffers"
                );
                let restored_slots = world.activate_streamed_scene(&scene)?;
                anyhow::ensure!(
                    !world.cold_scene_assets.contains_key(&scene),
                    "Scene {scene} cold geometry was not fully restored"
                );
                let after = debug_view::tracked_asset_bytes(&world);
                let process_after = debug_view::process_memory_bytes();
                let box3d_after = physics::box3d_allocated_bytes();
                anyhow::ensure!(
                    after.0 <= before.0 && after.1 <= before.1 && after.2 <= before.2,
                    "Scene {scene} stream cycle {cycle} grew resident asset payload: geometry {} -> {}, images {} -> {}, collision {} -> {}",
                    before.0, after.0, before.1, after.1, before.2, after.2
                );
                cycles.push(serde_json::json!({
                    "cycle":cycle + 1,
                    "entities_hidden":hidden,
                    "draw_slots_restored":restored_slots,
                    "cold_draw_buffers":cold_draw_slots,
                    "cold_texture_draws":cold_texture_draws,
                    "cold_texture_images":cold_texture_images,
                    "compressed_asset_bytes":compressed_bytes,
                    "resident_geometry_bytes_before":before.0,
                    "resident_geometry_bytes_inactive":inactive.0,
                    "resident_geometry_bytes_restored":after.0,
                    "resident_image_bytes_before":before.1,
                    "resident_image_bytes_inactive":inactive.1,
                    "resident_image_bytes_restored":after.1,
                    "resident_collision_bytes_before":before.2,
                    "resident_collision_bytes_inactive":inactive.2,
                    "resident_collision_bytes_restored":after.2,
                    "resident_asset_bytes_released_after_restore":before.0.saturating_sub(after.0)
                        .saturating_add(before.1.saturating_sub(after.1))
                        .saturating_add(before.2.saturating_sub(after.2)),
                    "inactive_scene_bytes_inactive":inactive.4,
                    "box3d_bytes_before":box3d_before,
                    "box3d_bytes_after":box3d_after,
                    "process_private_bytes_before":process_before.map(|(_, private)| private),
                    "process_private_bytes_after":process_after.map(|(_, private)| private),
                    "resident_payload_non_growing":true,
                    "round_trip_completed":true
                }));
            }
            let first = cycles.first().context("No stream-memory cycles executed")?;
            stream_memory_cycle = serde_json::json!({
                "scene":scene,
                "cycles_completed":cycles.len(),
                "entities_hidden":first["entities_hidden"],
                "draw_slots_restored":first["draw_slots_restored"],
                "cold_draw_buffers":first["cold_draw_buffers"],
                "cold_texture_draws":first["cold_texture_draws"],
                "cold_texture_images":first["cold_texture_images"],
                "compressed_asset_bytes":first["compressed_asset_bytes"],
                "resident_geometry_bytes_before":first["resident_geometry_bytes_before"],
                "resident_geometry_bytes_inactive":first["resident_geometry_bytes_inactive"],
                "inactive_scene_bytes_inactive":first["inactive_scene_bytes_inactive"],
                "round_trip_completed":cycles.iter().all(|entry| entry["round_trip_completed"] == true),
                "cycles":cycles,
                "exact_array_round_trip_unit_test":"cold_scene_arrays_roundtrip_bit_exactly"
            });
        }
        if let Some(name) = &o.diagnostic_pickup_commands {
            world.replay_pickup_commands(name)?;
        }
        let mut report = serde_json::to_value(&world.report)?;
        report["diagnostic_stream_memory_cycle"] = stream_memory_cycle;
        report["material_animation"] = world.material_animation.summary();
        report["quests"] = world.quests.summary();
        report["game_options"] = world.game_options.summary();
        report["playthrough"] = serde_json::to_value(world.playthrough)?;
        report["instance_changes"] = world.instance_changes.summary();
        report["player_scalar_capture"] = player_scalar_capture(&world);
        report["player_record_capture"] = player_record_capture(&world);
        report["world_scene_capture"] = world.scene_resources.summary(&world.scene_requests);
        let (geometry_bytes, image_bytes, collision_bytes, prefab_bytes, inactive_scene_bytes) =
            debug_view::tracked_asset_bytes(&world);
        let (entity_heap_bytes, component_heap_bytes, property_heap_bytes) =
            debug_view::runtime_entity_heap_bytes(&world);
        let process_memory = debug_view::process_memory_bytes();
        let process_heaps = debug_view::process_heap_bytes();
        let rust_heap = crate::memory_profile::snapshot();
        report["memory"] = serde_json::json!({
            "tracked_cpu_asset_bytes": {
                "geometry": geometry_bytes,
                "images": image_bytes,
                "collision": collision_bytes,
                "prefab_sources": prefab_bytes,
                "total": geometry_bytes.saturating_add(image_bytes).saturating_add(collision_bytes).saturating_add(prefab_bytes)
            },
            "inactive_scene_render_and_collision_bytes": inactive_scene_bytes,
            "compressed_inactive_scene_asset_bytes": world
                .cold_scene_assets
                .values()
                .map(assets::ColdSceneAssets::compressed_bytes)
                .sum::<usize>(),
            "runtime_entity_identity_bytes": entity_heap_bytes,
            "runtime_component_storage_bytes": component_heap_bytes,
            "runtime_component_property_bytes": property_heap_bytes,
            "startup_entity_payload_compacted_bytes": world.startup_entity_payload_compacted_bytes,
            "rust_allocator_live_bytes": rust_heap.map(|(live, _)| live),
            "rust_allocator_peak_bytes": rust_heap.map(|(_, peak)| peak),
            "box3d_live_allocation_bytes": physics::box3d_allocated_bytes(),
            "process_working_set_bytes": process_memory.map(|(working, _)| working),
            "process_private_bytes": process_memory.map(|(_, private)| private),
            "process_heap_container_count": process_heaps.as_ref().map(|heaps| heaps.containers),
            "process_heap_allocated_bytes": process_heaps.as_ref().map(|heaps| heaps.allocated),
            "process_heap_committed_bytes": process_heaps.as_ref().map(|heaps| heaps.committed),
            "process_heap_reserved_bytes": process_heaps.as_ref().and_then(|heaps| heaps.reserved),
            "scope": "Loaded scene payload estimates and process snapshot; excludes renderer allocations in inspect mode"
        });
        report["player_ability_checkpoint_records"] = serde_json::json!({"records":world.player_ability_save_summary()?,
            "registry":world.player_ability_registry,"scope":"Native registration order/common ownership; derived action initialization and full checkpoint pending"});
        report["player_ability_ownership"] = serde_json::json!({"state":world.player_abilities,
            "diagnostic_command_replay":world.diagnostic_pickup_commands,
            "hud_requests":world.ability_hud_requests,"initialization_complete":false,
            "playable_gates_connected":false,"reference_validated":false});
        // Geometry import records precede generated lighting-map attachment.
        // Audit the prepared frame separately instead of counting those
        // transient missing-map messages as final material failures.
        let mut material_draws = world.draws.clone();
        let view = viewport3d::look_at_matrix(
            add(world.report.spawn, [0., 2., 5.]),
            world.report.spawn,
            [0., 1., 0.],
        );
        world.apply_spherical_lighting(&mut material_draws, &view);
        let mut prepared = 0usize;
        let mut missing = 0usize;
        let mut failures = std::collections::BTreeMap::<String, usize>::new();
        for draw in &material_draws {
            match &draw.preview {
                Some(Ok(_)) => prepared += 1,
                Some(Err(error)) => *failures.entry(error.clone()).or_default() += 1,
                None => missing += 1,
            }
        }
        report["initial_material_preparation"] = serde_json::json!({
            "scope":"World draws after generated lighting attachment; excludes runtime attachments and GPU compilation",
            "view_matrix":view,
            "draws":material_draws.len(), "prepared":prepared,
            "without_program":missing, "errors":failures,
            "gpu_execution_validated":false,
            "original_execution_validated":false,
        });
        report["ear_rig_diagnostic"] = {
            let bones = &world.player_animations.rig.bones;
            let model = skin::model_pose(bones, &[])?;
            serde_json::json!({"reference_validated":false,"bones":bones.iter().enumerate()
                .filter(|(_,bone)| matches!(bone.name.to_ascii_lowercase().as_str(),"skel_head"|"skel_ear_l"|"skel_ear_r"))
                .map(|(index,bone)|serde_json::json!({"index":index,"name":bone.name,
                    "parent":bone.parent_index,"model_transform":model[index]})).collect::<Vec<_>>()})
        };
        report["prepared_behavior"] = serde_json::json!({
            "variable_count":world.player_graph.state.words().len(),
            "state_machine_count":world.player_graph.state.machines.len(),
            "expression_modifier_count":world.player_graph.state.expression_modifiers.len(),
            "expression_count":world.player_graph.state.expression_modifiers.values().map(|m| m.statements.len()).sum::<usize>(),
            "unresolved_expression_targets":world.player_graph.state.expression_modifiers.iter().flat_map(|(id, m)|
                m.statements.iter().enumerate().filter_map(move |(index, s)| s.unresolved_target.as_ref()
                    .map(|name| serde_json::json!({"modifier":id,"statement":index,"target":name})))
            ).collect::<Vec<_>>(),
            "retained_active_machines":world.player_graph.machines.len(),
            "retained_blenders":world.player_graph.blenders.len(),
            "transition_execution_implemented":false
        });
        report["initial_generator_plan"] = {
            let plan = &world.player_graph.plan;
            let mut resources = vec![];
            let rig = world.player_animations.rig.clone();
            for node in &plan.nodes {
                if let Some(definition) = world.player_graph.state.clips.get(&node.object) {
                    let name = &definition.settings.animation_name;
                    let clip = world.player_animations.clip(name)?;
                    let pose = clip.sample_bones(&rig, 0.)?;
                    resources.push(serde_json::json!({"generator":node.object,"animation":name,
                            "duration":clip.info.duration,"tracks":clip.info.num_tracks,
                            "has_extracted_motion":clip.has_extracted_motion,
                            "diagnostic_sample_time":0.,"sampled_bones":pose.len()}));
                }
            }
            serde_json::json!({"plan":plan,"resolved_clip_resources":resources,"pose_execution_implemented":false})
        };
        if let (Some(rig), Some(clip)) = (&o.diagnostic_rig, &o.diagnostic_clip) {
            let mut preview = clip_preview::ClipPreview::load(rig, clip)?;
            let mut samples = vec![];
            for _ in 0..4 {
                let mut draws = world.draws.clone();
                preview.apply(&world, &mut draws)?;
                samples.push(
                    serde_json::json!({"time":preview.time,"bounds":world.player_draws.iter()
                    .map(|&i| (draws[i].aabb_min,draws[i].aabb_max)).collect::<Vec<_>>() }),
                );
                preview.advance(0.1);
            }
            report["diagnostic_clip"] =
                serde_json::json!({"samples":samples,"retail_parity_verified":false});
        }
        if o.smoke {
            report["ear_pose_probe"] = {
                let bones = &world.player_animations.rig.bones;
                let mut ears = ears::Pose::new(bones)?;
                let actor = world.draws
                    [*world.player_draws.first().context("Missing Mickey draw")?]
                .trans_mat;
                let camera = ears::EarTransform {
                    rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
                    position: [actor[12], actor[13] + 2., actor[14] + 5.],
                };
                ears.update(
                    world
                        .report
                        .ear_settings
                        .as_ref()
                        .context("Missing ear settings")?,
                    bones,
                    actor,
                    camera,
                )?;
                let pose = skin::model_pose(bones, &ears.overrides)?;
                let mut draws = world.draws.clone();
                clip_preview::apply_model_pose(&world, &pose, &mut draws)?;
                serde_json::json!({"error":null,"reference_validated":false,
                    "overrides":ears.overrides,"bounds":world.player_draws.iter()
                        .map(|&i|(draws[i].aabb_min,draws[i].aabb_max)).collect::<Vec<_>>()})
            };
            let mut animation = authored_preview::Preview::new(&mut world)?;
            let before_pose = animation.pose.clone();
            let before_variables = world.player_graph.state.words().to_vec();
            let mut before = world.draws.clone();
            clip_preview::apply_pose(&world, &animation.pose, &mut before)?;
            animation.advance(&mut world, 15)?;
            let mut after = world.draws.clone();
            clip_preview::apply_pose(&world, &animation.pose, &mut after)?;
            let changed = world
                .player_draws
                .iter()
                .filter(|&&i| before[i].v_data != after[i].v_data)
                .count();
            anyhow::ensure!(
                before_pose != animation.pose && changed > 0,
                "Authored animation did not deform player meshes"
            );
            report["visible_startup_animation"] = serde_json::json!({
                "simulation_ticks":15,"changed_meshes":changed,"sampled_bones":animation.pose.len(),
                "observed_events":animation.observed_events,"full_movement_parity_verified":false,
                "changed_variables":before_variables.iter().zip(world.player_graph.state.words())
                    .enumerate().filter_map(|(index,(before,after))|(before!=after).then_some(
                        serde_json::json!({"index":index,"before_bits":before,"after_bits":after}))).collect::<Vec<_>>(),
                "active_expression_modifiers":world.player_graph.plan.nodes.iter()
                    .filter(|n|world.player_graph.expressions.is_active(&n.object)).count()});
            report["smoke"] = smoke(&world)?;
            let mut support_routes = Vec::new();
            for route in 0..8 {
                let angle = route as f32 * std::f32::consts::FRAC_PI_4;
                let direction = [angle.sin(), 0., angle.cos()];
                let mut runner = Player::new(world.report.spawn, world.report.motion.clone());
                for _ in 0..240 {
                    runner.step(&world.collision, [0.; 3], false, STEP);
                }
                let mut losses = Vec::new();
                for tick in 0..180 {
                    let supported = runner.grounded;
                    runner.step(&world.collision, direction, false, STEP);
                    if supported && !runner.grounded {
                        losses.push(serde_json::json!({"tick":tick,"position":runner.position,"velocity":runner.velocity,
                            "contacts":world.collision.contact_planes()}));
                    }
                }
                support_routes.push(serde_json::json!({"direction":direction,"losses":losses}));
            }
            report["running_support_diagnostic"] =
                serde_json::json!({"routes":support_routes,"reference_validated":false});
            // Exercise the controller/animation bridge separately from startup.
            // Record the first unsupported operation rather than mistaking a
            // successful idle sample for a working movement controller.
            let mut player = Player::new(world.report.spawn, world.report.motion.clone());
            for _ in 0..240 {
                player.step(&world.collision, [0.; 3], false, STEP);
            }
            let mut completed = 0;
            let mut animation_phases = Vec::new();
            let movement: Result<()> = (|| {
                for tick in 0..90 {
                    let direction = if (10..60).contains(&tick) {
                        [0., 0., 1.]
                    } else {
                        [0.; 3]
                    };
                    player.step(&world.collision, direction, false, STEP);
                    world.player_controller_tick(&mut animation, &mut player)?;
                    if [9, 59, 89].contains(&tick) {
                        animation_phases.push(serde_json::json!({"tick":tick,
                            "machines":world.player_graph.plan.nodes.iter().filter_map(|node|
                                world.player_graph.machines.get(&node.object).map(|m|
                                    serde_json::json!({"object":node.object,"state":m.current}))).collect::<Vec<_>>(),
                            "clips":world.player_graph.plan.nodes.iter().filter_map(|node|
                                world.player_graph.state.clips.get(&node.object).map(|c|
                                    c.settings.animation_name.clone())).collect::<Vec<_>>() }));
                    }
                    completed += 1;
                }
                Ok(())
            })();
            report["movement_animation_probe"] = serde_json::json!({
                "completed_ticks":completed,"requested_ticks":90,
                "error":movement.err().map(|e|format!("{e:#}")),
                "phases":animation_phases,
                "reference_validated":false});
            if completed == 90 {
                for second_jump in [false, true] {
                    let mut second_jump_tick = None;
                    let mut jump_ticks = 0;
                    let mut landing_tick = None;
                    let mut jump_clips = std::collections::BTreeSet::new();
                    let jump: Result<()> = (|| {
                        anyhow::ensure!(player.grounded, "Jump probe did not start grounded");
                        player.step(&world.collision, [0.; 3], (true, true), STEP);
                        anyhow::ensure!(!player.grounded, "Jump probe failed to leave ground");
                        // Exercise the same provisional routing as interactive play.
                        for tick in 0..120 {
                            if tick > 0 {
                                player.step(
                                    &world.collision,
                                    [0.; 3],
                                    (
                                        second_jump && tick == 12,
                                        tick < 6 || (second_jump && (12..18).contains(&tick)),
                                    ),
                                    STEP,
                                );
                            }
                            if player.second_jump_started {
                                second_jump_tick = Some(tick);
                            }
                            if player.grounded && landing_tick.is_none() {
                                landing_tick = Some(tick);
                            }
                            world.player_controller_tick(&mut animation, &mut player)?;
                            for node in &world.player_graph.plan.nodes {
                                if let Some(clip) = world.player_graph.state.clips.get(&node.object)
                                {
                                    jump_clips.insert(clip.settings.animation_name.clone());
                                }
                            }
                            jump_ticks += 1;
                        }
                        Ok(())
                    })();
                    report[if second_jump {
                        "second_jump_animation_probe"
                    } else {
                        "jump_animation_probe"
                    }] = serde_json::json!({"completed_ticks":jump_ticks,
                    "observed_clips":jump_clips,
                    "second_jump_tick":second_jump_tick,"landing_tick":landing_tick,"requested_ticks":120,"error":jump.err().map(|e|format!("{e:#}")),
                    "reference_validated":false});
                }
                // Earlier movement probes can finish against level geometry.
                // A movement-after-landing assertion needs an unobstructed start, not
                // the position accumulated by unrelated scenarios.
                player = Player::new(world.report.spawn, world.report.motion.clone());
                for _ in 0..240 {
                    player.step(&world.collision, [0.; 3], false, STEP);
                }
                let moving_land_start = player.position;
                let mut moving_land_tick = None;
                let mut resumed_locomotion_tick = None;
                let mut moving_land_trace = Vec::new();
                let moving_land: Result<()> = (|| {
                    anyhow::ensure!(player.grounded, "Moving landing probe must start grounded");
                    for tick in 0..100 {
                        // Keep movement held through the landing itself. The
                        // old 10..60 interval ended before the authored jump
                        // could land, so this probe reported the stationary
                        // post-landing idle pose as a movement transition bug.
                        let moving = (10..100).contains(&tick);
                        player.step(
                            &world.collision,
                            if moving { [0., 0., 1.] } else { [0.; 3] },
                            (tick == 0, tick < 6),
                            STEP,
                        );
                        if tick > 0 && player.grounded && moving_land_tick.is_none() {
                            moving_land_tick = Some(tick);
                        }
                        world.player_controller_tick(&mut animation, &mut player)?;
                        let event_names = world
                            .player_behavior
                            .single("hkbBehaviorGraphStringData")?["eventNames"]
                            .as_array()
                            .context("Missing event names")?;
                        moving_land_trace.push(serde_json::json!({"tick":tick,"grounded":player.grounded,
                          "position":player.position,"velocity":player.velocity,"maximum_speed":player.max_move_speed(),
                          "ground_motion":player.ground_vectors.as_ref().map(|v|serde_json::json!({"movement":v.movement,"desired":v.desired})),
                          "ground_speed":animation.ground_history.latest().map(|v|v.speed),
                          "clips":world.player_graph.plan.nodes.iter().filter_map(|n|world.player_graph.state.clips.get(&n.object).map(|c|&c.settings.animation_name)).collect::<Vec<_>>(),
                          "events":animation.delivered_events.iter().map(|e|event_names.get(e.id as usize).cloned().unwrap_or(serde_json::json!(e.id))).collect::<Vec<_>>(),
                          "states":world.player_graph.plan.nodes.iter().filter_map(|node| {
                              let runtime=world.player_graph.machines.get(&node.object)?;
                              let definition=&world.player_graph.state.machines[&node.object];
                              Some((&definition.settings.name,&definition.states[definition.state_indices[&runtime.current]].info.name))
                          }).collect::<std::collections::BTreeMap<_,_>>() }));
                        if moving
                            && moving_land_tick.is_some()
                            && world.player_graph.plan.nodes.iter().any(|n| {
                                world
                                    .player_graph
                                    .state
                                    .clips
                                    .get(&n.object)
                                    .is_some_and(|c| {
                                        matches!(
                                            c.settings.animation_name.as_str(),
                                            "Mickey_mov_run_brush.hkx"
                                                | "mickey_mov_sneak_pos.hkx"
                                                | "mickey_mov_sneak_neg.hkx"
                                        )
                                    })
                            })
                        {
                            resumed_locomotion_tick.get_or_insert(tick);
                        }
                    }
                    let landed = moving_land_tick.context("Moving jump did not land")?;
                    let moving_again = resumed_locomotion_tick
                        .context("Movement through landing remained in idle")?;
                    anyhow::ensure!(
                        moving_again <= landed + 2,
                        "Locomotion animation did not resume promptly after landing"
                    );
                    Ok(())
                })();
                report["moving_landing_animation_probe"] = serde_json::json!({
                  "landing_tick":moving_land_tick,"resumed_locomotion_tick":resumed_locomotion_tick,"trace":moving_land_trace,
                  "initial_position":moving_land_start,"initialization":"Fresh controller settled at authored spawn; graph retained from preceding idle",
                  "error":moving_land.err().map(|e|format!("{e:#}")),"reference_validated":false});
                for air_jump in [false, true] {
                    let mut fall_clips = std::collections::BTreeSet::new();
                    let fall: Result<()> = (|| {
                        anyhow::ensure!(player.grounded, "Contact-loss probe must start grounded");
                        // Artificial support loss tests the real controller/graph
                        // bridge, not retail ledge detection or reference timing.
                        player.position[1] += 2.;
                        let root = world.player_behavior.single("hkbBehaviorGraph")?
                            ["rootGenerator"]["ref"]
                            .as_str()
                            .context("Missing root generator")?
                            .to_owned();
                        let idle = world.player_graph.state.machines[&root]
                            .settings
                            .start_state_id;
                        for tick in 0..120 {
                            player.step(
                                &world.collision,
                                [0.; 3],
                                (air_jump && tick == 4, air_jump && (4..10).contains(&tick)),
                                STEP,
                            );
                            if air_jump && tick == 4 {
                                anyhow::ensure!(
                                    player.air_first_jump_started && !player.second_jump_started,
                                    "Ledge first jump was not accepted"
                                );
                            }
                            world.player_controller_tick(&mut animation, &mut player)?;
                            if tick == 0 {
                                anyhow::ensure!(!player.grounded, "Probe did not lose support");
                                anyhow::ensure!(
                                    world.player_graph.machines[&root].current != idle,
                                    "Support loss retained idle animation"
                                );
                            }
                            for node in &world.player_graph.plan.nodes {
                                if let Some(clip) = world.player_graph.state.clips.get(&node.object)
                                {
                                    fall_clips.insert(clip.settings.animation_name.clone());
                                }
                            }
                        }
                        if air_jump {
                            anyhow::ensure!(
                                fall_clips.iter().any(|name| name == "Mickey_Jump.hkx"),
                                "Ledge jump did not activate the original first-jump clip"
                            );
                        }
                        anyhow::ensure!(
                            player.grounded && world.player_graph.machines[&root].current == idle,
                            "Contact-loss animation failed to land and return to idle"
                        );
                        Ok(())
                    })();
                    report[if air_jump {
                        "ledge_jump_animation_probe"
                    } else {
                        "contact_loss_animation_probe"
                    }] = serde_json::json!({
                  "observed_clips":fall_clips,"error":fall.err().map(|e|format!("{e:#}")),
                  "synthetic_support_loss":true,"reference_validated":false});
                }
                for (airborne, moving) in [(false, false), (true, false), (false, true)] {
                    let mut spin_clips = std::collections::BTreeSet::new();
                    let mut stop_ticks = Vec::new();
                    let mut resumed_spin_run_tick = None;
                    let spin: Result<()> = (|| {
                        anyhow::ensure!(player.grounded, "Spin probe must start grounded");
                        if airborne {
                            player.step(&world.collision, [0.; 3], (true, true), STEP);
                            world.player_controller_tick(&mut animation, &mut player)?;
                        }
                        let event_names = world
                            .player_behavior
                            .single("hkbBehaviorGraphStringData")?["eventNames"]
                            .as_array()
                            .context("Missing animation event names")?;
                        let stop_id = event_names
                            .iter()
                            .position(|v| v.as_str() == Some("EVENT_Stop_SpinAttack"))
                            .context("Missing spin stop event")?
                            as i32;
                        let settings = world
                            .report
                            .spin_settings
                            .as_ref()
                            .context("Missing imported spin settings")?;
                        // Explicit initial Paint upgrade / neutral playstyle scenario;
                        // live progression and ability ownership are still unported.
                        let wait = settings.initial_preview_wait();
                        let start_ids: Vec<_> = event_names
                            .iter()
                            .enumerate()
                            .filter_map(|(i, v)| {
                                matches!(
                                    v.as_str(),
                                    Some("EVENT_Start_SpinAttack" | "EVENT_Start_SpinAttack_Air")
                                )
                                .then_some(i as i32)
                            })
                            .collect();
                        let mut starts = 0;
                        animation.request_spin();
                        for tick in 0..120 {
                            if (1..8).contains(&tick) {
                                animation.request_spin();
                            }
                            player.step(
                                &world.collision,
                                if moving && (2..70).contains(&tick) {
                                    [0., 0., -1.]
                                } else {
                                    [0.; 3]
                                },
                                false,
                                STEP,
                            );
                            world.player_controller_tick(&mut animation, &mut player)?;
                            for event in &animation.delivered_events {
                                if start_ids.contains(&event.id) {
                                    starts += 1;
                                }
                                if event.id == stop_id {
                                    stop_ticks.push(tick);
                                    anyhow::ensure!(
                                        animation.spin_lifecycle.cooldown == wait,
                                        "Stop did not apply imported cooldown"
                                    );
                                }
                            }
                            for node in &world.player_graph.plan.nodes {
                                if let Some(clip) = world.player_graph.state.clips.get(&node.object)
                                {
                                    spin_clips.insert(clip.settings.animation_name.clone());
                                    if moving
                                        && !stop_ticks.is_empty()
                                        && clip.settings.animation_name
                                            == "Mickey_mov_run_brush.hkx"
                                    {
                                        resumed_spin_run_tick.get_or_insert(tick);
                                    }
                                }
                            }
                        }
                        // Both authored root wildcards (events 63/67) target state
                        // 25 / generator 2:54464, using different transition effects.
                        anyhow::ensure!(
                            spin_clips.contains("Mickey_attack_Spin.hkx"),
                            "Spin clip did not activate"
                        );
                        anyhow::ensure!(starts == 1, "Repeated presses restarted active spin");
                        anyhow::ensure!(
                            !stop_ticks.is_empty() && !animation.spin_lifecycle.active,
                            "Spin animation did not stop the ability lifecycle"
                        );
                        anyhow::ensure!(
                            animation.spin_lifecycle.clone().start(0, true).is_some(),
                            "Spin did not become available after cooldown"
                        );
                        if moving {
                            let resumed = resumed_spin_run_tick
                                .context("Moving spin failed to resume running")?;
                            anyhow::ensure!(
                                resumed <= stop_ticks[0] + 3,
                                "Run did not resume after spin locomotion reset"
                            );
                        }
                        Ok(())
                    })();
                    report[if moving {
                        "moving_spin_animation_probe"
                    } else if airborne {
                        "air_spin_animation_probe"
                    } else {
                        "spin_animation_probe"
                    }] = serde_json::json!({"observed_clips":spin_clips,
                  "stop_ticks":stop_ticks,
                  "resumed_run_tick":resumed_spin_run_tick,
                  "error":spin.err().map(|e|format!("{e:#}")),"reference_validated":false,
                  "scope":"Shared interactive initial-upgrade preview lifecycle; repeated active requests rejected. Native ownership and combat remain unimplemented"});
                }
                let mut cycles = 0;
                let repeated: Result<()> = (|| {
                    let root = world.player_behavior.single("hkbBehaviorGraph")?["rootGenerator"]
                        ["ref"]
                        .as_str()
                        .context("Missing root generator")?
                        .to_owned();
                    let idle = world.player_graph.state.machines[&root]
                        .settings
                        .start_state_id;
                    for cycle in 0..6 {
                        for tick in 0..150 {
                            let direction = if (10..40).contains(&tick) {
                                [0., 0., if cycle % 2 == 0 { 1. } else { -1. }]
                            } else {
                                [0.; 3]
                            };
                            player.step(
                                &world.collision,
                                direction,
                                (tick == 70, (70..76).contains(&tick)),
                                STEP,
                            );
                            world.player_controller_tick(&mut animation, &mut player)?;
                        }
                        anyhow::ensure!(player.grounded, "Cycle {cycle} did not land");
                        anyhow::ensure!(
                            world.player_graph.machines[&root].current == idle,
                            "Cycle {cycle} did not return to initial idle state"
                        );
                        anyhow::ensure!(
                            world
                                .player_graph
                                .transition_stacks
                                .values()
                                .all(|s| s.records.is_empty()),
                            "Cycle {cycle} retained transition instances after settling"
                        );
                        cycles += 1;
                    }
                    Ok(())
                })();
                report["repeated_animation_probe"] = serde_json::json!({"completed_cycles":cycles,
                  "requested_cycles":6,"ticks_per_cycle":150,
                  "error":repeated.err().map(|e|format!("{e:#}")),"reference_validated":false});
                if cycles == 6 {
                    let mut recoveries = 0;
                    let recovery: Result<()> = (|| {
                        let root = world.player_behavior.single("hkbBehaviorGraph")?
                            ["rootGenerator"]["ref"]
                            .as_str()
                            .context("Missing root generator")?
                            .to_owned();
                        let idle = world.player_graph.state.machines[&root]
                            .settings
                            .start_state_id;
                        for automatic in [false, true] {
                            player.step(&world.collision, [0.; 3], (true, true), STEP);
                            world.player_controller_tick(&mut animation, &mut player)?;
                            anyhow::ensure!(
                                world.player_graph.machines[&root].current != idle,
                                "Recovery probe failed to enter jump animation"
                            );
                            let before = player.respawns;
                            if automatic {
                                player.position[1] = world.collision.min_y - 20.;
                                player.step(&world.collision, [0.; 3], false, STEP);
                            } else {
                                player.respawn();
                            }
                            anyhow::ensure!(
                                player.respawns == before + 1,
                                "Recovery did not reset the player"
                            );
                            world.player_controller_tick(&mut animation, &mut player)?;
                            anyhow::ensure!(
                                world.player_graph.machines[&root].current == idle,
                                "Recovery retained an airborne animation state"
                            );
                            anyhow::ensure!(
                                world
                                    .player_graph
                                    .transition_stacks
                                    .values()
                                    .all(|s| s.records.is_empty()),
                                "Recovery retained old animation transitions"
                            );
                            for tick in 0..120 {
                                player.step(
                                    &world.collision,
                                    [0.; 3],
                                    (tick == 40, (40..46).contains(&tick)),
                                    STEP,
                                );
                                world.player_controller_tick(&mut animation, &mut player)?;
                            }
                            anyhow::ensure!(
                                player.grounded
                                    && world.player_graph.machines[&root].current == idle,
                                "Jump/landing after recovery failed"
                            );
                            recoveries += 1;
                        }
                        Ok(())
                    })();
                    report["preview_respawn_probe"] = serde_json::json!({"completed_recoveries":recoveries,
                      "requested_recoveries":2,"error":recovery.err().map(|e|format!("{e:#}")),
                      "scope":"Preview R reset and fall recovery, not retail death/checkpoints",
                      "reference_validated":false});
                }
            }
        }
        if o.smoke {
            let mut animation = authored_preview::Preview::new(&mut world)?;
            animation.advance(&mut world, 15)?;
            report["ear_lifecycle_probe"] = {
                let mut ears = ears::Pose::new(&animation.pose)?;
                let actor = world.draws
                    [*world.player_draws.first().context("Missing Mickey draw")?]
                .trans_mat;
                let mut steps = vec![];
                for name in ["EVENT_Set_Ears_Static", "EVENT_Set_Ears_Dynamic"] {
                    animation.queue_named_event(&world.player_behavior, name)?;
                    animation.advance(&mut world, 1)?;
                    ears.animation_events(&world.player_behavior, &animation.delivered_events)?;
                    ears.update(
                        world
                            .report
                            .ear_settings
                            .as_ref()
                            .context("Missing ear settings")?,
                        &animation.pose,
                        actor,
                        ears::EarTransform {
                            rotation: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
                            position: [actor[12], actor[13] + 2., actor[14] + 5.],
                        },
                    )?;
                    let authored = skin::model_pose(&animation.pose, &[])?;
                    let overridden = skin::model_pose(&animation.pose, &ears.overrides)?;
                    let matches_authored = authored == overridden;
                    anyhow::ensure!(
                        matches_authored == (name == "EVENT_Set_Ears_Static"),
                        "Ear mode did not follow {name}"
                    );
                    let mut draws = world.draws.clone();
                    clip_preview::apply_model_pose(&world, &overridden, &mut draws)?;
                    steps.push(
                        serde_json::json!({"event":name,"matches_authored":matches_authored,
                        "override_count":ears.overrides.len()}),
                    );
                }
                serde_json::json!({"error":null,"reference_validated":false,"steps":steps})
            };
        }
        let smoke_failures = if o.smoke {
            smoke_validation::failures(&report)
        } else {
            Vec::new()
        };
        if o.smoke {
            report["smoke_validation"] = serde_json::json!({"passed":smoke_failures.is_empty(),"failures":smoke_failures,
            "scope":"Executed preview probes only; not full gameplay or original reference parity"});
        }
        std::fs::write(path, serde_json::to_vec_pretty(&report)?)?;
        anyhow::ensure!(
            smoke_failures.is_empty(),
            "Smoke validation failed; report saved to {}: {}",
            path.display(),
            smoke_failures.join("; ")
        );
        println!(
            "Blotworx Engine: {} meshes, {} collision triangles, {} textures. Report: {}",
            world.report.meshes,
            world.report.collision_triangles,
            world.report.textures,
            path.display()
        );
        return Ok(());
    }
    let capture_expected = o.capture.is_some();
    let (capture_tx, capture_rx) = mpsc::channel();
    log::info!(target: "blotworx_phase", "Starting application window and graphics context");
    eframe::run_native(
        "Blotworx Engine",
        eframe::NativeOptions {
            // Recorded captures own their fixed simulation clock. Waiting for
            // display refresh adds no fidelity and can suspend an unattended
            // replay behind presentation backpressure. Interactive play keeps
            // the normal synchronized presentation policy.
            glow_options: egui_glow::GlowConfiguration {vsync: !capture_expected,..Default::default()},
            #[cfg(target_os = "android")]
            android_app,
            depth_buffer: 24,
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280., 800.])
                .with_min_inner_size([800., 500.]),
            ..Default::default()
        },
        Box::new(move |cc| {
            log::info!(target: "blotworx_phase", "Application window and graphics context ready");
            let mut engine = Engine::new(cc, o);
            engine.capture_result = Some(capture_tx);
            Ok(Box::new(engine))
        }),
    )
    .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    if capture_expected {
        capture_rx
            .try_recv()
            .map_err(|_| anyhow::anyhow!("Capture interrupted before an image was saved"))?
            .map_err(anyhow::Error::msg)?;
    }
    Ok(())
}
/// Read-only authored graph inspection for asset tools. Parsing is not a claim
/// of complete behavior execution or original runtime compatibility.
pub fn inspect_behavior_bytes(bytes: &[u8]) -> Result<serde_json::Value> {
    Ok(serde_json::to_value(behavior::Document::load(bytes)?)?)
}
fn smoke(world: &World) -> Result<serde_json::Value> {
    let mut p = Player::new(world.report.spawn, world.report.motion.clone());
    for _ in 0..240 {
        p.step(&world.collision, [0.; 3], false, STEP);
    }
    anyhow::ensure!(
        p.grounded && p.respawns == 0,
        "Authored spawn did not settle onto imported collision: {:?}",
        p.position
    );
    let ground = p.position;
    let mut apex = ground[1];
    for i in 0..240 {
        p.step(&world.collision, [0.; 3], (i == 0, true), STEP);
        apex = apex.max(p.position[1]);
    }
    anyhow::ensure!(
        apex - ground[1] > 1.0 && p.grounded && p.respawns == 0,
        "Jump/landing failed: ground={ground:?}, apex={apex}, position={:?}, grounded={}, respawns={}", p.position,p.grounded,p.respawns
    );
    let mut distances = vec![];
    // Same fixed simulation scheduled by 30, 60 and 144 Hz presentation.
    for hz in [30, 60, 144] {
        let mut c = controller::Controller::new(Player::new(ground, world.report.motion.clone()));
        c.camera_yaw = std::f32::consts::FRAC_PI_2;
        for _ in 0..hz {
            c.advance(
                &world.collision,
                1. / hz as f64,
                true,
                controller::Input {
                    movement: [0., 1.],
                    ..Default::default()
                },
            );
        }
        distances.push((hz, c.ticks, c.player.position));
    }
    anyhow::ensure!(
        distances.iter().all(|d| d.1 == 30 && d.2 == distances[0].2),
        "Render rate changed playable controller result"
    );
    p.position[1] = world.collision.min_y - 20.;
    p.step(&world.collision, [0.; 3], false, STEP);
    anyhow::ensure!(
        p.respawns == 1 && p.position == p.spawn,
        "Fall recovery failed"
    );
    Ok(
        serde_json::json!({"spawn_grounded":true,"settled_spawn":ground,"jump_rise":apex-ground[1],"render_rate_runs":distances,"fall_respawn":true}),
    )
}
fn preview_entries(world: &assets::World) -> Vec<(String, [f32; 3])> {
    world
        .entities
        .iter()
        .filter(|entity| {
            let leaf = entity
                .name
                .rsplit('.')
                .next()
                .unwrap_or(&entity.name)
                .to_ascii_lowercase();
            leaf.starts_with("startposition")
                || leaf == "cp_marker"
                || leaf.contains("mickeystart")
                || leaf.starts_with("playerstart")
        })
        .map(|entity| {
            (
                entity.name.clone(),
                entity.transform[12..15].try_into().unwrap(),
            )
        })
        .collect()
}
fn select_preview_entry(
    world: &mut assets::World,
    controller: &mut controller::Controller,
    name: &str,
) -> Result<()> {
    let target = world
        .entities
        .iter()
        .position(|entity| entity.name == name)
        .context("Preview entry is not placed in this level")?;
    let request = entity_relocation::Request {
        entity: world.player_entity,
        target,
        reset: true,
        probe: true,
    };
    world.with_live_player(&mut controller.player, |world| {
        world.move_player_to_entity(request)
    })?;
    // This manual demo selection is also the R-reset destination. The original
    // TeleportToEntity API itself does not change checkpoints or respawn count.
    controller.player.spawn = controller.player.position;
    Ok(())
}
type PhaseTiming = performance::Timing;
fn record_phase(
    phases: &mut std::collections::BTreeMap<&'static str, PhaseTiming>,
    name: &'static str,
    start: Instant,
) {
    phases.entry(name).or_default().record(start.elapsed());
}
enum LevelLoad {
    Transfer,
    Clean,
    NewCampaign,
}

#[cfg(not(target_os = "horizon"))]
struct Engine {
    setup: disc_setup::Setup,
    updater: updater::Updater,
    frame_phases: std::collections::BTreeMap<&'static str, PhaseTiming>,
    save_directory: Option<PathBuf>,
    save_slot: usize,
    rtc_seconds: Option<u32>,
    capture_realtime: bool,
    diagnostic_serial_hud: bool,
    original_movie_hud: bool,
    diagnostic_eager_hud: bool,
    diagnostic_no_view_culling: bool,
    diagnostic_no_hud_reuse: bool,
    diagnostic_hud_driver_validation: bool,
    diagnostic_checkpoint_timing: bool,
    diagnostic_counter_impact: Option<(String, f32)>,
    particle_asset_preview: Option<String>,
    particle_materials: bool,
    preview_entry: Option<String>,
    render_probe: Option<[f32; 2]>,
    render_probe_done: bool,
    preview_runtime: bool,
    strict_npc_ai: bool,
    capture_hide_hose: bool,
    diagnostic_no_model_shadow: bool,
    capture_hide_decals: bool,
    input_playback: Option<input_playback::Playback>,
    playback_trace: Option<input_playback::TraceWriter>,
    playback_trace_error: Option<String>,
    diagnostic_pickup_commands: Option<String>,
    diagnostic_relocation: Option<(String, [f32; 3])>,
    live_actor: Option<(String, String)>,
    runtime_script: Option<PathBuf>,
    script_console: String,
    actor_focus: Option<String>,
    diagnostic_morph: Option<assets::MorphPreview>,
    diagnostic_hose: Option<usize>,
    diagnostic_hose_ticks: Option<u64>,
    diagnostic_aim: [f32; 2],
    capture_camera_target: Option<[f32; 3]>,
    capture_jigsaw_endpoint: Option<bool>,
    capture_jigsaw_animate: bool,
    diagnostic_strafe: bool,
    diagnostic_shuffle: Option<bool>,
    splat_preview: decal_preview::Preview,
    hose_active: bool,
    brush_action_respawns: u32,
    hose_phase: Option<brush::Firing>,
    hose_clock: brush::EmissionClock,
    hose_emitters: [bool; 2],
    ink_hold: Option<(usize, f32)>,
    blast_cooldown: [f32; 2],
    ear_pose: Option<ears::Pose>,
    clip_preview: Option<clip_preview::ClipPreview>,
    player_skin_frame: clip_preview::SkinFrame,
    authored_preview: Option<authored_preview::Preview>,
    root: String,
    level: String,
    level_catalog: Vec<assets::LevelEntry>,
    level_catalog_root: String,
    level_filter: String,
    show_unavailable_levels: bool,
    receiver: Option<mpsc::Receiver<Result<World>>>,
    loading_save: Option<(save_service::Service, u64, u32)>,
    loading_screen: Option<loading_screen::Screen>,
    usable_button: Option<egui::TextureHandle>,
    usable_prompt_target: Option<usize>,
    world: Option<World>,
    movie: Option<movie::Player>,
    last_movie: Option<serde_json::Value>,
    #[cfg(feature = "blotworx-ui-render")]
    title: Option<ui_movie::Title>,
    #[cfg(feature = "blotworx-ui-render")]
    main_menu: Option<ui_movie::Menu>,
    #[cfg(feature = "blotworx-ui-render")]
    hud: Option<ui_movie::GameplayWorker>,
    controller: Option<controller::Controller>,
    error: Option<String>,
    camera: Viewport3DState,
    zoom: f32,
    last: Instant,
    paused: bool,
    pause_menu_opened: Option<Instant>,
    playback_pause: Option<(u64, u16)>,
    simulation_active: bool,
    details: bool,
    debug: debug_view::DebugView,
    preferences_dirty: bool,
    options_open: bool,
    options_tab: usize,
    preferences: options_menu::Preferences,
    retail_lighting: bool,
    jump_pending: bool,
    spin_preview_pending: bool,
    anvil_root_pending: bool,
    capture: Option<PathBuf>,
    capture_tick: Option<u64>,
    capture_movie_frame: Option<u32>,
    capture_switch_level: Option<String>,
    loaded_frames: u32,
    capture_requested: bool,
    capture_result: Option<mpsc::Sender<std::result::Result<(), String>>>,
    pad: input::Pad,
    pad_aim: [f32; 2],
    #[cfg(feature = "blotworx-ui-render")]
    pause_pointer: ui_movie::PausePointer,
    previous_buttons: u16,
    pad_edges: u16,
    multiplayer: multiplayer::Multiplayer,
}
#[cfg(not(target_os = "horizon"))]
impl Engine {
    fn new(cc: &eframe::CreationContext<'_>, o: Options) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let mut style = (*cc.egui_ctx.style_of(egui::Theme::Dark)).clone();
        style
            .text_styles
            .insert(egui::TextStyle::Heading, egui::FontId::proportional(22.));
        style
            .text_styles
            .insert(egui::TextStyle::Body, egui::FontId::proportional(15.));
        style
            .text_styles
            .insert(egui::TextStyle::Small, egui::FontId::proportional(13.));
        cc.egui_ctx.set_style_of(egui::Theme::Dark, style);
        let mut setup = disc_setup::Setup::default();
        let root = o.root.or_else(disc_setup::installed)
            .and_then(|path| {
                if path.is_file() {
                    setup.start(path);
                    None
                } else {
                    Some(path)
                }
            })
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default();
        let (playback_trace, playback_trace_error) =
            match (o.capture.as_deref(), o.input_playback.as_ref()) {
                (Some(capture), Some(playback)) => {
                    match input_playback::TraceWriter::create(capture, playback) {
                        Ok(writer) => (Some(writer), None),
                        Err(error) => (
                            None,
                            Some(format!("Could not start playback trace: {error:#}")),
                        ),
                    }
                }
                _ => (None, None),
            };
        let mut s = Self {
            setup,
            updater: updater::Updater::on_launch(o.updates && !cfg!(target_os = "android")),
            frame_phases: Default::default(),
            save_directory: o.save_directory,
            save_slot: o.save_slot,
            rtc_seconds: o.rtc_seconds,
            capture_realtime: o.capture_realtime,
            original_movie_hud: o.original_movie_hud,
            diagnostic_serial_hud: o.diagnostic_serial_hud,
            diagnostic_eager_hud: o.diagnostic_eager_hud,
            diagnostic_no_view_culling: o.diagnostic_no_view_culling,
            diagnostic_no_hud_reuse: o.diagnostic_no_hud_reuse,
            diagnostic_hud_driver_validation: o.diagnostic_hud_driver_validation,
            diagnostic_checkpoint_timing: o.diagnostic_checkpoint_timing,
            particle_asset_preview: o.particle_asset_preview,
            particle_materials: o.particle_materials,
            preview_entry: o.preview_entry,
            render_probe: o.render_probe,
            render_probe_done: false,
            preview_runtime: o.preview_runtime,
            strict_npc_ai: o.strict_npc_ai,
            input_playback: o.input_playback,
            playback_trace,
            playback_trace_error,
            diagnostic_pickup_commands: o.diagnostic_pickup_commands,
            diagnostic_counter_impact: o.diagnostic_counter_impact,
            diagnostic_relocation: o.diagnostic_relocation,
            live_actor: o.live_actor,
            runtime_script: o.runtime_script,
            script_console: String::new(),
            actor_focus: o.actor_focus,
            diagnostic_morph: o.diagnostic_morph,
            capture_hide_hose: o.capture_hide_hose,
            diagnostic_no_model_shadow: o.diagnostic_no_model_shadow,
            capture_hide_decals: o.capture_hide_decals,
            diagnostic_hose: o.diagnostic_hose,
            diagnostic_hose_ticks: o.diagnostic_hose_ticks,
            diagnostic_aim: o.diagnostic_aim,
            capture_camera_target: o.capture_camera_target,
            capture_jigsaw_endpoint: o.capture_jigsaw_endpoint,
            capture_jigsaw_animate: o.capture_jigsaw_animate,
            splat_preview: Default::default(),
            hose_active: false,
            brush_action_respawns: 0,
            hose_phase: None,
            hose_clock: Default::default(),
            hose_emitters: [false; 2],
            ink_hold: None,
            blast_cooldown: [0.; 2],
            diagnostic_strafe: o.diagnostic_strafe,
            diagnostic_shuffle: o.diagnostic_shuffle,
            clip_preview: None,
            player_skin_frame: Default::default(),
            authored_preview: None,
            ear_pose: None,
            root,
            level: o.level.clone(),
            level_catalog: Vec::new(),
            level_catalog_root: String::new(),
            level_filter: String::new(),
            show_unavailable_levels: false,
            receiver: None,
            loading_save: None,
            loading_screen: None,
            usable_button: None,
            usable_prompt_target: None,
            world: None,
            #[cfg(feature = "blotworx-ui-render")]
            hud: None,
            movie: None,
            last_movie: None,
            #[cfg(feature = "blotworx-ui-render")]
            title: None,
            #[cfg(feature = "blotworx-ui-render")]
            main_menu: None,
            controller: None,
            error: None,
            camera: Viewport3DState {
                view_culling: !o.diagnostic_no_view_culling,
                limitations_overlay: true,
                show_limitations: false,
                pitch: 0.28,
                yaw: if o.diagnostic_hose.is_some() {
                    0.7
                } else {
                    1.6
                },
                distance: 5.,
                show_grid: false,
                show_markers: false,
                show_labels: false,
                backface_culling: true,
                render_mode: RenderMode::Lit,
                lighting_mode: LightingMode::Headlight,
                ..Default::default()
            },
            zoom: 5.,
            last: Instant::now(),
            paused: false,
            pause_menu_opened: None,
            playback_pause: None,
            simulation_active: false,
            details: false,
            debug: debug_view::DebugView::new(o.debug),
            preferences_dirty: false,
            options_open: o.show_options,
            options_tab: 0,
            preferences: options_menu::Preferences::load(),
            retail_lighting: o.game_rendering,
            jump_pending: false,
            spin_preview_pending: false,
            anvil_root_pending: false,
            capture: o.capture,
            capture_tick: o.capture_tick,
            capture_movie_frame: o.capture_movie_frame,
            capture_switch_level: o.capture_switch_level,
            loaded_frames: 0,
            capture_requested: false,
            capture_result: None,
            pad: input::Pad::default(),
            pad_aim: [0.5; 2],
            #[cfg(feature = "blotworx-ui-render")]
            pause_pointer: Default::default(),
            previous_buttons: 0,
            pad_edges: 0,
            multiplayer: multiplayer::Multiplayer::new(&o.level, o.join),
        };
        if let Some(profile)=o.graphics_profile{s.preferences.profile=profile;}
        s.zoom=s.preferences.distance;
        if s.preferences.fullscreen {cc.egui_ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));}
        if let (Some(rig), Some(clip)) = (&o.diagnostic_rig, &o.diagnostic_clip) {
            match clip_preview::ClipPreview::load(rig, clip) {
                Ok(preview) => s.clip_preview = Some(preview),
                Err(e) => {
                    s.error = Some(format!("Clip preview: {e:#}"));
                    return s;
                }
            }
        }
        if !s.root.is_empty() {
            if o.continue_checkpoint {
                s.continue_checkpoint(&cc.egui_ctx);
            } else {
                s.load(&cc.egui_ctx);
            }
        }
        s
    }
    fn original_pause_available(&self) -> bool {
        #[cfg(feature = "blotworx-ui-render")]
        { self.hud.is_some() }
        #[cfg(not(feature = "blotworx-ui-render"))]
        { false }
    }
    fn continue_checkpoint(&mut self, ctx: &egui::Context) {
        let result = (|| -> Result<_> {
            let directory = self.save_directory.clone()
                .map(Ok).unwrap_or_else(save_service::default_directory)?;
            if let Some(world) = self.world.as_mut() {
                if let Some(service) = &mut world.checkpoint.service {
                    service.flush(
                        world.frame_clock.hardware_sample(),
                        world.frame_clock.clock.rate.ticks_per_second.get(),
                    )?;
                }
            } else if let Some((service, sample, frequency)) = &self.loading_save {
                service.flush(*sample, *frequency)?;
            }
            let snapshot = save_service::Service::read_checkpoint(&directory, self.save_slot)?;
            Ok(snapshot)
        })();
        match result {
            Ok(snapshot) => match snapshot.completed_playthrough() {
                Ok(true) => {
                    let root = assets::files_root(std::path::Path::new(&self.root));
                    match endgame::campaign_start(&root) {
                        Ok(level) => self.load_campaign_slot(ctx, snapshot, level, score_state::CampaignResume::NextPlaythrough),
                        Err(error) => self.error = Some(format!("Start next playthrough: {error:#}")),
                    }
                }
                Ok(false) => self.resume_snapshot(ctx, snapshot),
                Err(error) => self.error = Some(format!("Continue game: {error:#}")),
            },
            Err(error) => self.error = Some(format!("Continue game: {error:#}")),
        }
    }
    fn resume_snapshot(&mut self, ctx: &egui::Context, snapshot: std::sync::Arc<checkpoint::Snapshot>) {
        match snapshot.level() {
            Ok(level) => self.load_campaign_slot(ctx, snapshot, level, score_state::CampaignResume::Checkpoint),
            Err(error) => self.error = Some(format!("Restore checkpoint: {error:#}")),
        }
    }
    fn load_campaign_slot(
        &mut self,
        ctx: &egui::Context,
        snapshot: std::sync::Arc<checkpoint::Snapshot>,
        level: String,
        resume: score_state::CampaignResume,
    ) {
        self.level = level;
        self.preview_entry = None;
        self.actor_focus = None;
        self.options_open = false;
        self.load_with_checkpoint(ctx, Some((snapshot, resume)), LevelLoad::Transfer);
    }
    fn load(&mut self, ctx: &egui::Context) {
        self.load_with_checkpoint(ctx, None, LevelLoad::Transfer);
    }
    fn load_with_checkpoint(
        &mut self,
        ctx: &egui::Context,
        snapshot: Option<(std::sync::Arc<checkpoint::Snapshot>, score_state::CampaignResume)>,
        mode: LevelLoad,
    ) {
        self.root = assets::files_root(std::path::Path::new(&self.root))
            .to_string_lossy()
            .into_owned();
        let root = std::path::Path::new(&self.root);
        if em2::is_install(root) {
            let started = std::env::current_exe().and_then(|exe| {
                std::process::Command::new(exe).arg("--data").arg(root).spawn()
            });
            match started {
                Ok(_) => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
                Err(error) => self.error = Some(format!("Open game folder: {error}")),
            }
            return;
        }
        if !root.join("packfiles").is_dir() {
            self.error=Some(format!("No game packfiles found in {}. Choose the extracted game, DATA, files or packfiles folder.",self.root));
            return;
        }
        self.error = None;
        self.simulation_active = false;
        self.previous_buttons = 0;
        self.jump_pending = false;
        self.spin_preview_pending = false;
        self.anvil_root_pending = false;
        let root = PathBuf::from(&self.root);
        let level = self.level.clone();
        log::info!(target: "blotworx_phase", "Loading level {level}");
        let diagnostic_morph = self.diagnostic_morph.clone();
        let particle_asset_preview = self.particle_asset_preview.clone();
        let particle_materials = self.particle_materials;
        let live_actor = self.live_actor.clone();
        let diagnostic_relocation = self.diagnostic_relocation.clone();
        let diagnostic_pickup_commands = self.diagnostic_pickup_commands.clone();
        let diagnostic_counter_impact = self.diagnostic_counter_impact.clone();
        let preview_runtime = self.preview_runtime;
        let strict_npc_ai = self.strict_npc_ai;
        // Loading a replacement level can allocate several hundred MiB of
        // decoded textures, meshes and native collision resources. Retaining
        // the old playable world as a rollback copy made peak memory approach
        // the sum of both complete levels. Preserve only the small progression
        // state extracted below; the old world is released before the worker
        // starts constructing its replacement.
        if let Some(world) = &mut self.world {
            world.frame_clock.suspend_for_loading();
        }
        let mut scores = if let Some(world) = self.world.as_ref().filter(|_| snapshot.is_some()) {
            score_state::InitialState {
                frame_clock: Some(world.frame_clock),
                checkpoint: checkpoint::State {
                    service: world.checkpoint.service.clone(),
                    ..Default::default()
                },
                ..Default::default()
            }
        } else if let Some(world) = self.world.as_ref().filter(|_| !matches!(mode, LevelLoad::Transfer)) {
            let mut playtime = checkpoint_time::Playtime::default();
            if matches!(mode, LevelLoad::NewCampaign) {
                playtime.set(0, &world.frame_clock.clock);
            }
            score_state::InitialState {
                playtime,
                frame_clock: Some(world.frame_clock),
                checkpoint: world.checkpoint.clone(),
                ..Default::default()
            }
        } else if let Some(world) = self.world.as_mut() {
            let session=match level_session::Session::capture(world) {
                Ok(session)=>session,
                Err(error)=> { self.error=Some(format!("Lua session transfer: {error:#}")); return; }
            };
            score_state::InitialState {
                session: Some(session),
                resume: None,
                playstyle_points: world.playstyle_points,
                playthrough: world.playthrough,
                playtime: world.playtime,
                checkpoint: world.checkpoint.clone(),
                frame_clock: Some(world.frame_clock),
            }
        } else {
            let frame_clock = frame_clock::Context::boot(self.rtc_seconds);
            let mut playtime = checkpoint_time::Playtime::default();
            // Direct boot uses the same playtime reset as a new campaign.
            playtime.set(0, &frame_clock.clock);
            let service = match &self.loading_save {
                Some((service, _, _)) => Ok(service.clone()),
                None => self
                    .save_directory
                    .clone()
                    .map(Ok)
                    .unwrap_or_else(save_service::default_directory)
                    .and_then(|directory| {
                        save_service::Service::open(directory, self.save_slot, frame_clock.hardware_sample())
                    }),
            };
            let service = match service {
                Ok(service) => service,
                Err(error) => {
                    self.receiver = None;
                    self.error = Some(format!("Blotworx save storage: {error:#}"));
                    return;
                }
            };
            score_state::InitialState {
                playtime,
                frame_clock: Some(frame_clock),
                checkpoint: checkpoint::State {
                    service: Some(service),
                    ..Default::default()
                },
                ..Default::default()
            }
        };
        if let Some((snapshot, resume)) = snapshot {
            if let Some(service) = &scores.checkpoint.service {
                service.record_memory_checkpoint(&snapshot);
            }
            scores.resume = Some(resume);
            scores.checkpoint.latest = Some(snapshot);
        }
        if let Some(clock) = &mut scores.frame_clock {
            clock.suspend_for_loading();
        }
        // Loading suspends the old world's simulation. Prepare the replacement
        // with its shared C rand state; commit it only with the successful world.
        let game_random = self
            .world
            .as_ref()
            .map(|world| world.game_random)
            .unwrap_or_default();
        let render_options = self
            .world
            .as_ref()
            .map(|world| world.render_options)
            .unwrap_or_default();
        // A diagnostic script belongs to the level the run started in.
        let runtime_script = self.runtime_script.take();
        let initial = self.world.is_none();
        self.loading_screen = Some(loading_screen::Screen::load(ctx, &root, (!initial).then_some(level.as_str())));
        self.usable_button = usable_prompt::load(ctx, &root).inspect_err(|e| log::warn!("Interaction artwork: {e:#}")).ok();
        self.usable_prompt_target = None;
        self.loading_save = scores.checkpoint.service.clone().zip(scores.frame_clock.as_ref()).map(|(service, clock)| {
            (service, clock.hardware_sample(), clock.clock.rate.ticks_per_second.get())
        });
        let (tx, rx) = mpsc::channel();
        self.receiver = Some(rx);
        drop(self.world.take());
        self.controller = None;
        self.ear_pose = None;
        self.authored_preview = None;
        self.splat_preview = Default::default();
        self.hose_active = false;
        self.hose_phase = None;
        self.hose_emitters = [false; 2];
        self.hose_clock = Default::default();
        self.ink_hold = None;
        self.blast_cooldown = [0.; 2];
        self.movie = None;
        self.last_movie = None;
        #[cfg(feature = "blotworx-ui-render")]
        {
            self.title = None;
            self.main_menu = None;
            self.hud = None;
            viewport3d::clear_gpu_cache();
        }
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut world = assets::load_with_bootstrap_mode(
                    &root,
                    &level,
                    &ctx,
                    diagnostic_morph.as_ref(),
                    particle_asset_preview.as_deref(),
                    game_random,
                    render_options,
                    scores,
                    preview_runtime,
                    strict_npc_ai,
                )?;
                if particle_materials {
                    if let Some(preview) = &mut world.particle_asset_preview {
                        preview.initialize_materials(&world.scripts)?;
                    }
                }
                if preview_runtime {
                    if !world.demo_story.enabled {
                        world.enable_demo_story()?;
                    }
                    world.demo_story.npc_fallback.enabled = !strict_npc_ai;
                    world.recover_failed_preview_script(0);
                }
                if let Some(name) = &diagnostic_pickup_commands {
                    world.replay_pickup_commands(name)?;
                }
                if let Some((name, amount)) = &diagnostic_counter_impact {
                    let owner = world
                        .scripts
                        .resolve_name(name)
                        .context("Counter-impact diagnostic entity not found")?;
                    anyhow::ensure!(
                        world.activate_counter_impact(owner, *amount)?,
                        "Counter-impact diagnostic effect is disabled or declined"
                    );
                    world.report.unsupported.push(format!(
                        "Diagnostic counter impact: {name}; color feedback only, not a combat hit"
                    ));
                }
                if let Some((name, position)) = &diagnostic_relocation {
                    let index = world
                        .scripts
                        .resolve_name(name)
                        .context("Relocation probe entity not found")?;
                    world.relocate_entity(
                        index,
                        *position,
                        world.entities[index].runtime_rotation,
                    )?;
                }
                if let Some((actor, sequence)) = &live_actor {
                    world.play_actor(actor, sequence, true)?;
                }
                if let Some(path) = &runtime_script {
                    world
                        .scripts
                        .load(&path.to_string_lossy(), &std::fs::read(path)?)?;
                }
                Ok(world)
            }))
            .unwrap_or_else(|_| {
                Err(anyhow::anyhow!(
                    "An asset parser panicked while loading this level"
                ))
            })
            .with_context(|| format!("Loading {level} from {}", root.display()));
            let _ = tx.send(result);
            ctx.request_repaint();
        });
    }
    fn play(&mut self, ui: &mut egui::Ui) {
        // The retained world is for rollback, not presentation during import.
        // Its pose publication or audio errors must not contaminate the new
        // request (or pause it before it has even finished loading).
        if self.receiver.is_some() {
            return;
        }
        let mut elapsed = if self.capture.is_some() && !self.capture_realtime {
            f64::from(STEP)
        } else {
            self.last.elapsed().as_secs_f64()
        };
        self.last = Instant::now();
        let Some(world) = &mut self.world else { return };
        let Some(controller) = &mut self.controller else {
            return;
        };
        if self.debug.open && self.capture.is_none() {
            egui::Window::new("Scene tools")
                .default_width(360.)
                .show(ui.ctx(), |ui| {
                    let entries = preview_entries(world);
                    if !entries.is_empty() {
                        ui.horizontal(|ui| {
                            ui.label("Preview entry").on_hover_text("Uses a placed entry marker. This is manual exploration; automatic original projector/checkpoint entry selection remains incomplete.");
                            egui::ComboBox::from_id_salt("preview_level_entry")
                                .selected_text(self.preview_entry.as_deref().unwrap_or("Serialized player position"))
                                .width(350.).show_ui(ui,|ui| {
                                    let mut destination=None;
                                    if ui.selectable_label(self.preview_entry.is_none(),"Serialized player position").clicked() {
                                        self.preview_entry=None;destination=Some(None);
                                    }
                                    for (name,_) in entries {
                                        if ui.selectable_label(self.preview_entry.as_deref()==Some(&name),&name).clicked() {
                                            self.preview_entry=Some(name.clone());destination=Some(Some(name));
                                        }
                                    }
                                    if let Some(entry)=destination {
                                        if let Some(name)=entry {
                                            if let Err(error)=select_preview_entry(world,controller,&name) {self.error=Some(format!("Preview entry: {error:#}"));}
                                        }else {controller.player.spawn=world.report.spawn;controller.respawn();}
                                        self.actor_focus=None;
                                        self.jump_pending=false;self.spin_preview_pending=false;
                                    }
                                });
                        });
                    }
                    ui.horizontal(|ui| {
                        let mut request=None;
                        egui::ComboBox::from_id_salt("live_actor_sequence").selected_text("Request actor sequence").show_ui(ui,|ui| {
                            for (&entity,resources) in &world.actor_sequences {
                                if !world.actor_bindings.contains_key(&entity) {continue;}
                                ui.menu_button(&world.entities[entity].name,|ui| {
                                    for (_,sequence) in resources.sequences() {
                                        if let Some(name)=&sequence.name {
                                            if ui.button(name).clicked() {request=Some((world.entities[entity].name.clone(),name.clone()));ui.close();}
                                        }
                                    }
                                });
                            }
                        });
                        if let Some((actor,sequence))=request {
                            self.actor_focus=Some(actor.clone());
                            if let Err(error)=world.entities.iter().position(|e|e.name==actor).context("Actor missing").and_then(|entity|world.request_actor_sequence(entity,&sequence)) {self.error=Some(format!("Actor playback: {error:#}"));}
                        }
                        if self.actor_focus.is_some() && ui.button("Follow Mickey").clicked() {self.actor_focus=None;}
                        if ui.button("Place anvil root").on_hover_text("Prototype: original animated model and physics. Impact callbacks, inventory, child effects and lifetime remain incomplete. Shortcut: 2.").clicked() {
                            self.anvil_root_pending=true;
                        }
                        #[cfg(not(target_os = "android"))]
                        if ui.button("Export 3DS Bundle").on_hover_text("Repackages the current level into a 3DS-optimized .bw3d bundle for Old 3DS / New 3DS (sdmc:/blotworx/levels/).").clicked() {
                            if let Some(dir) = rfd::FileDialog::new().set_title("Choose 3DS Bundle Output Folder (sdmc:/blotworx)").pick_folder() {
                                let out = dir.join("levels").join(export_3ds::level_bundle_filename(&world.report.level));
                                if let Err(error) = export_3ds::export_world_to_bw3d(world, &out) {
                                    self.error = Some(format!("3DS Export: {error:#}"));
                                }
                            }
                        }
                        if let Some(playback)=&world.actor_playback {
                            ui.label(format!("{}: {} {:.3}s â€” manual playback",world.entities[playback.entity].name,playback.name,playback.time));
                            if ui.button("Stop animation").clicked() {world.actor_playback=None;}
                        }
                    });
                    ui.collapsing("Animation scripts", |ui| {
                        ui.label(format!(
                            "{} actors playing from authored settings or script requests",
                            world
                                .actors
                                .values()
                                .filter(|actor| actor.active_id().is_some())
                                .count()
                        ));
                        ui.add(
                            egui::TextEdit::multiline(&mut self.script_console)
                                .desired_rows(3)
                                .hint_text("AnimGBSequence(GetEntity('exact entity name'), 'Spawn')"),
                        );
                        if ui.button("Run Lua").clicked() {
                            if let Err(error) = world.run_script(&self.script_console) {
                                self.error = Some(format!("Game Lua: {error:#}"));
                            }
                        }
                        for error in world.report.unsupported.iter().filter(|error| {
                            error.starts_with("Game Lua:") || error.starts_with("Script animation request:")
                        }) {
                            ui.colored_label(egui::Color32::YELLOW, error);
                        }
                    });
                });
        }
        let viewport = ui.available_size();
        if viewport.x > 0. && viewport.y > 0. {
            if let Err(error) = world
                .camera_lens
                .set_renderer_aspect(viewport.x / viewport.y)
            {
                self.error = Some(format!("Application camera lens: {error:#}"));
                return;
            }
        }
        let focused = ui.input(|i| i.focused);
        // Gameplay continues running even when the window is unfocused (e.g. multiplayer / alt-tab);
        // live keyboard/gamepad input remains gated on `focused`.
        let capture_mode = self.capture.is_some();
        let active = !self.paused
            && self.pause_menu_opened.is_none()
            && (!self.options_open || capture_mode)
            && !self.capture_requested
            && self.receiver.is_none();
        // A newly focused/unpaused frame includes time spent suspended.
        // Discard that interval, while retaining all time during active stalls.
        if active && !self.simulation_active && (!capture_mode || self.capture_realtime) {
            elapsed = 0.;
        }
        self.simulation_active = active;
        let playback_frame = self
            .input_playback
            .as_ref()
            .and_then(|playback| playback.frames.get(controller.ticks as usize))
            .copied();
        if self.input_playback.is_some() {
            elapsed = 1. / 30.;
        }
        world.frame_clock.publish(
            active,
            (capture_mode && !self.capture_realtime) || self.input_playback.is_some(),
        );
        let mp_typing = self.multiplayer.wants_keyboard_input();
        let raw_requested_hose = if !active || mp_typing || (!focused && !capture_mode && playback_frame.is_none()) {
            None
        } else if let Some(frame) = playback_frame {
            if frame.paint {
                Some(0)
            } else if frame.thinner {
                Some(1)
            } else {
                None
            }
        } else if capture_mode {
            self.diagnostic_hose.filter(|_| {
                self.diagnostic_hose_ticks
                    .is_none_or(|limit| controller.ticks < limit)
            })
        } else if ui.input(|i| i.key_down(egui::Key::Q)) || self.pad.triggers[1] > 30 {
            Some(0)
        } else if ui.input(|i| i.key_down(egui::Key::E)) || self.pad.triggers[0] > 30 {
            Some(1)
        } else {
            None
        };
        let mut requested_hose = None;
        let mut input = controller::Input::default();
        if active && focused && !capture_mode && !mp_typing {
            let mut movement_pad = self.pad;
            let use_pressed = self.pad_edges & 0x5000 != 0
                || ui.input(|i| i.key_pressed(egui::Key::G));
            if use_pressed {
                if let Some(entity) = world.usable_target(controller.player.position) {
                    if self.pad_edges & 0x1000 != 0 {
                        self.jump_pending = false;
                        movement_pad.buttons &= !0x1000;
                        self.pad_edges &= !0x1000;
                    }
                    if let Err(error) = world.use_usable(entity) {
                        self.error = Some(format!("Interaction: {error:#}"));
                    }
                }
            }
            if ui.input(|i| i.key_pressed(egui::Key::R)) {
                controller.respawn();
                self.jump_pending = false;
                self.spin_preview_pending = false;
            }
            let mouse_look=ui.rect_contains_pointer(ui.available_rect_before_wrap());
            input=ui.input(|i|input::movement_controls(i,movement_pad,self.jump_pending,self.preferences.sensitivity,self.preferences.invert_y,mouse_look));
            self.spin_preview_pending |= ui.input(|i| i.key_pressed(egui::Key::F));
            self.anvil_root_pending |= ui.input(|i| i.key_pressed(egui::Key::Num2));
            if mouse_look {
                self.zoom =
                    (self.zoom - ui.input(|i| i.smooth_scroll_delta.y) * 0.008).clamp(2., 16.);
            }
        }
        if active {
            if let Some(frame) = playback_frame {
                if let Some(choice) = frame.dialogue_choice {
                    #[cfg(feature = "blotworx-ui-render")]
                    if choice == 0 && self.preferences.hud
                        && self.hud.as_ref().is_some_and(|hud| hud.scroll_dialogue().unwrap_or(false))
                    {
                        ui.ctx().request_repaint();
                    } else {
                        world.acknowledge_demo_story(choice);
                    }
                    #[cfg(not(feature = "blotworx-ui-render"))]
                    world.acknowledge_demo_story(choice);
                }
                if frame.interact_pressed {
                    if let Some(entity) = world.usable_target(controller.player.position) {
                        if let Err(error) = world.use_usable(entity) {
                            self.error = Some(format!("Interaction: {error:#}"));
                        }
                    }
                }
                if frame.respawn_pressed {
                    controller.respawn();
                    self.jump_pending = false;
                    self.spin_preview_pending = false;
                }
                input = frame.input;
                self.spin_preview_pending |= frame.spin_pressed;
                self.anvil_root_pending |= frame.anvil_root_pressed;
            }
        }
        if active && self.diagnostic_strafe {
            input.movement = [1., 0.];
        }
        if !active {
            self.spin_preview_pending = false;
        }
        if !active {
            self.anvil_root_pending = false;
        }
        let aim_rect = ui.available_rect_before_wrap();
        let capture_aim = playback_frame
            .and_then(|frame| frame.aim)
            .unwrap_or(self.diagnostic_aim);
        let pad_aiming = active && !capture_mode
            && (self.pad.triggers.iter().any(|&trigger| trigger > 30) || self.pad.buttons & 0x0100 != 0);
        if pad_aiming {
            let amount = (elapsed as f32).clamp(0., 0.1) * self.preferences.sensitivity;
            self.pad_aim[0] = (self.pad_aim[0] + self.pad.right[0] * amount).clamp(0., 1.);
            self.pad_aim[1] = (self.pad_aim[1] - self.pad.right[1] * amount).clamp(0., 1.);
            input.camera_stick = [0.; 2];
        }
        let aim_pointer = if capture_mode || pad_aiming {
            let aim = if pad_aiming { self.pad_aim } else { capture_aim };
            egui::pos2(
                aim_rect.left() + aim_rect.width() * aim[0],
                aim_rect.top() + aim_rect.height() * aim[1],
            )
        } else {
            ui.input(|i| i.pointer.hover_pos())
                .filter(|p| aim_rect.contains(*p))
                .unwrap_or(aim_rect.center())
        };
        let pixels = ui.ctx().pixels_per_point();
        let local = (aim_pointer - aim_rect.min) * pixels;
        let aim_pixel = [local.x as i32, local.y as i32];
        let aim_size = [
            (aim_rect.width() * pixels).round() as u32,
            (aim_rect.height() * pixels).round() as u32,
        ];
        #[cfg(feature = "blotworx-ui-render")]
        let hud_cursor = (active && !world.side_scroll_disables("SideScrollDisablePaint") && (raw_requested_hose.is_some() || pad_aiming
            || world.player_ability_enabled("Brush",brush::Ink::Paint).unwrap_or(false)
            || world.player_ability_enabled("Brush",brush::Ink::Thinner).unwrap_or(false))
            && (!world.demo_story.enabled || world.demo_story.reticle_visible)).then(|| [
                ((aim_pointer.x - aim_rect.left()) / aim_rect.width().max(1.)).clamp(0., 1.),
                ((aim_pointer.y - aim_rect.top()) / aim_rect.height().max(1.)).clamp(0., 1.),
            ]);
        let mut aim_target = None;
        let mut aim_ray = None;
        let simulation_started = Instant::now();
        {log::debug!(target:"blotworx_phase", "simulation begin ticks={} active={active}",controller.ticks);}
        let result=controller.advance_with_tick_phases_and_camera(world,|world|&world.collision,elapsed,active,input,
            |world|world.published_camera_frame().map(|frame|frame.map(|frame|frame.direction)),
            |phase,player,tick,camera_yaw,camera_pitch,world| -> Result<()> {
            {log::debug!(target:"blotworx_phase", "tick {tick} phase {phase:?}");}
            if phase==controller::TickPhase::SimulatePhysics {
                world.advance_player_physics(player,STEP)?;
                return Ok(());
            }
            if phase==controller::TickPhase::BeforeMovement {
                world.with_live_player(player, |world| world.prepare_gameplay_scene_requests())?;
                world.advance_demo_motion(STEP)?;
                world.with_live_player(player,|world|world.advance_demo_ai(STEP))?;
                let trigger_position=player.position;
                world.with_live_player(player,|world|world.advance_demo_triggers(trigger_position,STEP))?;
                world.advance_demo_counter_states()?;
                if let Some(spawn)=world.player_spawn.override_transform{player.spawn=spawn.position;}
                let dead = world.runtime_health.get(&world.player_entity).is_some_and(|s|s.health<=0);
                let authored_death = dead && world.player_checkpoint_death_available();
                if dead && !authored_death {
                    // Keep the player loop playable in every level even before
                    // authored checkpoint/death-sequence restoration is wired.
                    player.respawn();
                    if let Some(authored)=world.authored_health.get(&world.player_entity){
                        let restored=health::Runtime::new(authored);let value=restored.health;
                        world.runtime_health.insert(world.player_entity,restored);
                        if let Some(p)=world.entities[world.player_entity].components.iter_mut().find(|c|c.class_name=="HealthComponent").and_then(|c|c.properties.iter_mut().find(|p|p.name=="Health")){p.value=serde_json::json!(value);}
                    }
                }
                world.prepare_side_scroller(player);
                player.script_input_restricted=authored_death || world.cutscene_input.restricted || world.player_physics.as_ref().is_some_and(|b|b.suspended_motion.is_some());
                if player.script_input_restricted {self.spin_preview_pending=false;self.anvil_root_pending=false;}
                world.advance_pressure_plates(player, STEP)?;
                world.prepare_scene_motion(STEP)?;
                if std::mem::take(&mut self.anvil_root_pending) && !world.side_scroll_disables("SideScrollDisableSketches") {
                    // Explicit root-factory preview placement; the original
                    // sketch cursor/action/inventory pipeline remains pending.
                    let (s,c)=player.yaw.sin_cos();
                    world.spawn_sketch_root("paintanvil",[player.position[0]+s*2.5,player.position[1]+3.5,player.position[2]-c*2.5],
                        [c,0.,-s,0.,1.,0.,s,0.,c])?;
                }
                for cd in &mut self.blast_cooldown {
                    if *cd > 0. {
                        *cd = (*cd - STEP).max(0.);
                    }
                }
                let allowed_raw_ink = raw_requested_hose.filter(|_| {
                    !player.script_input_restricted
                        && !world.side_scroll_disables("SideScrollDisablePaint")
                });
                let mut tap_ink: Option<usize> = None;
                requested_hose = match allowed_raw_ink {
                    Some(ink) if capture_mode && self.diagnostic_hose.is_some() => Some(ink),
                    Some(ink) => {
                        let channel = if ink == 0 { brush::Ink::Paint } else { brush::Ink::Thinner };
                        let held_time = match self.ink_hold {
                            Some((prev, t)) if prev == ink => {
                                let next = t + STEP;
                                self.ink_hold = Some((ink, next));
                                next
                            }
                            _ => {
                                self.ink_hold = Some((ink, 0.));
                                0.
                            }
                        };
                        if held_time >= 0.16 && world.player_ability_enabled("Brush", channel)? {
                            Some(ink)
                        } else {
                            None
                        }
                    }
                    None => {
                        if let Some((ink, held_time)) = self.ink_hold.take() {
                            if held_time <= 0.25 && !self.hose_active {
                                tap_ink = Some(ink);
                            }
                        }
                        None
                    }
                };
                if let Some(ink) = tap_ink {
                    let channel = if ink == 0 { brush::Ink::Paint } else { brush::Ink::Thinner };
                    let airborne = !player.grounded
                        && !player.stairs_ground_retained
                        && !player.ledge.is_active();
                    if airborne {
                        let can_air = world.player_ability_enabled("Brush", channel)?
                            || world.player_ability_enabled("Blast", channel)?;
                        if can_air && player.air_ink_jump_available {
                            let cost = world.ammo.settings.channels[ink].jump_cost;
                            if world.ammo.available(ink) < cost {
                                world.post_audio_from_game(
                                    world.player_entity,
                                    brush_runtime::DRY_SOUND_EVENT,
                                );
                            } else if player.begin_ink_jump() {
                                world.ammo.change(ink, -cost, &mut |_, _, _| Ok(()))?;
                                let origin = [
                                    player.position[0],
                                    player.position[1] + 0.25,
                                    player.position[2],
                                ];
                                let ring = 1.0_f32;
                                for offset in [
                                    [0., -4.0, 0.],
                                    [ring, -4.0, 0.],
                                    [-ring, -4.0, 0.],
                                    [0., -4.0, ring],
                                    [0., -4.0, -ring],
                                ] {
                                    let target = [
                                        origin[0] + offset[0],
                                        origin[1] + offset[1],
                                        origin[2] + offset[2],
                                    ];
                                    world.projectile_preview.spawn(
                                        &world.report,
                                        channel,
                                        world.player_entity as u64,
                                        origin,
                                        target,
                                    )?;
                                }
                                if let Some(preview) = &mut self.authored_preview {
                                    let ev = if ink == 1 {
                                        "EVENT_Start_ThinnerJump"
                                    } else {
                                        "EVENT_Start_DoubleJump"
                                    };
                                    let _ = preview.queue_named_event(&world.player_behavior, ev);
                                }
                            }
                        }
                    } else if self.blast_cooldown[ink] <= 0.
                        && world.player_ability_enabled("Blast", channel)?
                    {
                        const BLAST_COST: f32 = 5.0;
                        if world.ammo.available(ink) < BLAST_COST {
                            world.post_audio_from_game(
                                world.player_entity,
                                brush_runtime::DRY_SOUND_EVENT,
                            );
                        } else {
                            world.ammo.change(ink, -BLAST_COST, &mut |_, _, _| Ok(()))?;
                            self.blast_cooldown[ink] = 0.6;
                            let origin = [
                                player.position[0],
                                player.position[1] + 0.75,
                                player.position[2],
                            ];
                            let (sy, cy) = player.yaw.sin_cos();
                            let fwd = [sy, 0., cy];
                            let right = [cy, 0., -sy];
                            // Spawn authored BlastConePaint / BlastConeThinner FX
                            // from _Dynamic.pak (ConfigFiles.ini [MickeyAbilities_Blast]).
                            let cone_name = if ink == 0 {
                                "BlastConePaint"
                            } else {
                                "BlastConeThinner"
                            };
                            let cone_pose: [f32; 13] = [
                                cy, 0., sy,
                                0., 1., 0.,
                                -sy, 0., cy,
                                origin[0], origin[1], origin[2],
                                1.0,
                            ];
                            if let Err(err) = world.start_palette_effect(
                                world.player_entity,
                                cone_name,
                                Some(&cone_pose),
                                Some(0.65),
                            ) {
                                log::warn!("Blast cone FX {cone_name} skipped: {err:#}");
                            }
                            let _ = world.dispatch_blast_cone(origin, fwd, channel);
                            for (side, up) in [
                                (0., 0.),
                                (0.22, 0.),
                                (-0.22, 0.),
                                (0., 0.14),
                                (0., -0.14),
                            ] {
                                let target = [
                                    origin[0] + (fwd[0] + right[0] * side) * 5.0,
                                    origin[1] + up * 5.0,
                                    origin[2] + (fwd[2] + right[2] * side) * 5.0,
                                ];
                                world.projectile_preview.spawn(
                                    &world.report,
                                    channel,
                                    world.player_entity as u64,
                                    origin,
                                    target,
                                )?;
                            }
                            if let Some(preview) = &mut self.authored_preview {
                                let _ = preview.queue_named_event(
                                    &world.player_behavior,
                                    "EVENT_Start_Blast",
                                );
                            }
                        }
                    }
                }
                if self.brush_action_respawns!=player.respawns {
                    if let Some(preview)=&mut self.authored_preview {
                        preview.synchronize_player(player,&world.player_behavior,
                            &mut world.player_graph,&mut world.player_animations)?;
                    }
                    // Preview recovery recreates the graph. Retaining the old
                    // action latch would suppress its new brush-start event.
                    player.end_brush_motion();
                    if let Some(previous)=self.hose_phase.take() {
                        self.hose_clock=previous.stop_effects(true,|ink| {
                            self.hose_emitters[usize::from(ink==brush::Ink::Thinner)]=false;Ok(())
                        })?;
                    }
                    self.hose_emitters=[false;2];
                    self.hose_active=false;
                    self.brush_action_respawns=player.respawns;
                }
                if requested_hose.is_some()!=self.hose_active {
                    if let Some(preview)=&mut self.authored_preview {
                        if let Err(e)=preview.brush_transition(requested_hose.is_some(),&world.player_behavior,&mut world.player_graph,&mut world.player_animations){
                            return Err(e.context("Brush animation"));
                        }
                    }
                    if requested_hose.is_some() {
                        if let Some(settings)=&world.report.brush_settings {
                            if let Err(e)=player.begin_brush_motion(settings.run_speed,settings.turn_rate){return Err(e);}
                        }
                    } else {player.end_brush_motion();}
                    if let Some(previous)=self.hose_phase.take() {
                        match previous.stop_effects(true,|ink| {
                            self.hose_emitters[usize::from(ink==brush::Ink::Thinner)]=false;Ok(())
                        }) {
                            Ok(clock)=>self.hose_clock=clock,
                            Err(error)=>{return Err(error.context("Brush shutdown"));}
                        }
                    }
                    self.hose_phase=requested_hose.map(|ink|brush::Firing::begin(if ink==0 {brush::Ink::Paint}else{brush::Ink::Thinner},std::mem::take(&mut self.hose_clock)));
                    self.hose_emitters=[false;2];
                    self.hose_active=requested_hose.is_some();
                }
                world.publish_player_ground_record(player)?;
                world.advance_player_knockback(player,STEP)?;
                return Ok(());
            }
            world.refresh_player_stairs_after_physics(player)?;
            aim_target=None;aim_ray=None;
            let script_report_start=world.report.unsupported.len();
            {log::debug!(target:"blotworx_phase", "tick {tick} model update begin");}
            world.with_live_player(player,|world|world.advance_contact_stimuli(STEP))?;
            world.with_live_player(player,|world|world.advance_ai_projectile_impacts())?;
            world.advance_player_models(player,tick as f32*STEP)?;
            {log::debug!(target:"blotworx_phase", "tick {tick} model update end");}
            if self.preview_runtime && world.recover_failed_preview_script(script_report_start) {
                player.script_input_restricted=false;
            }
            if player.script_input_restricted {requested_hose=None;self.spin_preview_pending=false;}
            if let Some(effect)=&mut world.particle_asset_preview {
                // Deliberate asset-inspection position, not native effect
                // spawning, following, physics or gameplay simulation.
                let position=physics::add(player.position,[2.,1.,0.]);
                effect.advance_attached(STEP,true,[1.,0.,0.,0.,0.,1.,0.,0.,0.,0.,1.,0.,position[0],position[1],position[2],1.])?;
            }
            world.brush_runtime.recharge(&mut world.ammo)?;
            if self.clip_preview.is_none() {
                if let Some(preview)=&mut self.authored_preview {
                    if std::mem::take(&mut self.spin_preview_pending) {
                        if world.player_ability_enabled("Spin",brush::Ink::Paint)? {preview.request_spin();}
                    }
                    let downward_spray = requested_hose.is_some() && player.can_spray_below();
                    preview.brush_jump_transition(downward_spray, &world.player_behavior)?;
                    if downward_spray {
                        preview.brush_aim(&mut world.player_graph.state, -1., 0.)?;
                    } else if let Some(ink)=requested_hose {
                        let camera=if let Some(frame)=world.published_camera_frame()? {frame.cursor_camera()}else {
                            let (sy,cy)=camera_yaw.sin_cos();let (sp,cp)=camera_pitch.sin_cos();
                            let direction=[cy*cp,sp,sy*cp];let center=add(player.position,[0.,1.,0.]);
                            let distance=world.camera_preview_distance(center,direction,self.zoom)?;
                            let extent=(self.camera.fov_degrees.to_radians()*0.5).tan();
                            let aspect=(aim_rect.width()/aim_rect.height().max(1.)).max(0.1);
                            let camera=cursor::Camera {
                                position:std::array::from_fn(|i|center[i]+direction[i]*distance),
                                left:[-sy,0.,cy],up:[-cy*sp,cp,-sy*sp],forward:[-cy*cp,-sp,-sy*cp],
                                viewport:cursor::Rect{left:0.,right:1.,top:1.,bottom:0.},
                                frustum:cursor::Rect{left:-extent*aspect,right:extent*aspect,top:extent,bottom:-extent},orthographic:false,
                            };
                            camera
                        };
                        if let Some(ray)=camera.window_ray(aim_pixel,aim_size)? {
                            aim_ray=Some([ray.origin,ray.direction]);
                            // Base upgrade and neutral playstyle are preview inputs,
                            // not a substitute for the missing gameplay profile.
                            let range=world.report.brush_settings.as_ref().context("Missing brush settings")?
                                .value(if ink==0 {brush::Ink::Paint}else{brush::Ink::Thinner},"Range",0,ability_value::NEUTRAL_PLAYSTYLE)?;
                            // Native Cursor3D world admission is still under
                            // comparison. Preserve the working brush preview
                            // until paintable scene-layer coverage is verified.
                            let target=brush_preview::target(&world.collision,ray,player.position,range)?;
                            let aim=brush::target_direction(player.position,target);
                            let yaw=native_angle::atan2(aim.direction[0] as f64,aim.direction[2] as f64) as f32;
                            player.set_aim_facing_target(yaw);
                            preview.brush_aim(&mut world.player_graph.state,aim.elevation_dot,brush::aim_yaw_delta(player.yaw,yaw))?;
                            aim_target=Some(world.report.brush_settings.as_ref().context("Missing brush settings")?.emission_target(target));
                        }
                    }
                    world.player_controller_tick(preview,player)?;
                    world.export_player_animation_frames(std::mem::take(&mut preview.playback.emitted_frames))?;
                    world.synchronize_player_knockback(player);
                    // Publish Mickey's current tick position and yaw BEFORE updating
                    // skeletal children so the paintbrush never lags 1 tick behind rotation.
                    world.entities[world.player_entity].set_controller_pose(player.position, player.yaw);
                    world.update_skeletal_children(Some(&preview.playback.pose))?;
                    for _ in 0..std::mem::take(&mut preview.pending_combat_queries) {
                        world.dispatch_live_spin(&mut preview.spin_hits)?;
                    }
                    if let Some(ears)=&mut self.ear_pose {
                        ears.animation_events(&world.player_behavior,&preview.delivered_events)?;
                        let actor=world.entities[world.player_entity].controller_matrix(player.position,player.yaw);
                        let camera=if let Some(frame)=world.published_camera_frame()? {frame.ear_camera()}else {
                            let (sy,cy)=camera_yaw.sin_cos();let (sp,cp)=camera_pitch.sin_cos();
                            let direction=[cy*cp,sp,sy*cp];
                            let target=add(player.position,[0.,1.,0.]);
                            let distance=world.camera_preview_distance(target,direction,self.zoom)?;
                            // Same provisional orbit camera as the viewport;
                            // Gamebryo columns are negative screen-right, up, forward.
                            let camera=ears::EarTransform {
                                rotation:[[-sy,-cy*sp,-cy*cp],[0.,cp,-sp],[cy,-sy*sp,-sy*cp]],
                                position:std::array::from_fn(|i|target[i]+direction[i]*distance),
                            };
                            camera
                        };
                        ears.update(world.report.ear_settings.as_ref().context("Missing ear settings")?,
                            &preview.pose,actor,camera)?;
                    }
                    let pose=skin::model_pose(&preview.pose,self.ear_pose.as_ref().map_or(&[],|e|e.overrides.as_slice()))?;
                    let actor=world.entities[world.player_entity].controller_matrix(player.position,player.yaw);
                    let origin=world.brush.emitter_position(&pose,actor,player.position)?;
                    if let Some(ink) = requested_hose.filter(|_| downward_spray) {
                        let channel = if ink == 0 { brush::Ink::Paint } else { brush::Ink::Thinner };
                        let range = world.report.brush_settings.as_ref().context("Missing brush settings")?
                            .value(channel, "Range", 0, ability_value::NEUTRAL_PLAYSTYLE)?;
                        aim_ray = Some([origin, [0., -1., 0.]]);
                        aim_target = Some([origin[0], origin[1] - range, origin[2]]);
                    }
                    if let Some(ribbon)=&mut world.spin_ribbon {
                        ribbon.advance(STEP,preview.spin_lifecycle.active,world.brush.emitter_transform(&pose,actor)?)?;
                    }
                    if let Some(particles)=&mut world.spin_particles {
                        particles.advance_attached(STEP,preview.spin_lifecycle.active,world.brush.emitter_transform(&pose,actor)?)?;
                    }
                    let direction=aim_target.map(|p|brush::target_direction(origin,p).direction).unwrap_or([player.yaw.sin(),0.,player.yaw.cos()]);
                    let mut funded_hose = None;
                    if let (Some(phase),Some(ink))=(&mut self.hose_phase,requested_hose) {
                        if let Some(target)=aim_target {
                            let settings=world.report.brush_settings.as_ref().context("Missing brush settings")?;
                            let ink=if ink==0 {brush::Ink::Paint}else{brush::Ink::Thinner};
                            // Same explicit base upgrade/neutral playstyle as projectile preview.
                            let cost=settings.value(ink,"Cost",0,ability_value::NEUTRAL_PLAYSTYLE)?;
                            let dry_before=world.brush_runtime.dry_sound_requests;
                            let mut effects=brush_runtime::Effects {runtime:&mut world.brush_runtime,
                                emitters:&mut self.hose_emitters,projectiles:&mut world.projectile_preview,
                                report:&world.report,owner:world.player_entity as u64,origin,target};
                            let firing=phase.update(ink,true,STEP,settings.projectile_interval,settings.burst,cost,
                                &mut brush::AmmoFiringHost {ammo:&mut world.ammo,effects:&mut effects})?;
                            if firing.debit.is_some() {
                                funded_hose = Some(usize::from(ink == brush::Ink::Thinner));
                            }
                            if world.brush_runtime.dry_sound_requests!=dry_before {
                                world.post_audio_from_game(world.player_entity,brush_runtime::DRY_SOUND_EVENT);
                            }
                        }
                    }
                    world.advance_brush_projectiles(STEP)?;
                    for index in 0..world.projectile_preview.step_contacts.len() {
                        let contact=world.projectile_preview.step_contacts[index].clone();
                        world.dispatch_brush_contact(&contact,player.position)?;
                    }
                    let mut random=world.game_random;
                    let decals=self.splat_preview.update(decal_preview::View {world,
                        contacts:&world.projectile_preview.step_contacts},STEP,&mut random);
                    world.game_random=random;
                    decals?;
                    for (index,hose) in world.brush.hoses.iter_mut().enumerate(){
                        // Keep the effect's startup state so it can resume after recharge.
                        // Only a funded firing tick can produce new spray.
                        hose.advance(STEP,self.hose_emitters[index] && funded_hose==Some(index),origin,direction)?;
                    }
                }
            }
            world.advance_jigsaw_effects(STEP)?;
            if self.playback_trace.is_some() {
                let events=self.authored_preview.as_ref().map(|preview|
                    preview.delivered_events.iter().map(|event|event.id).collect::<Vec<_>>()).unwrap_or_default();
                let clips=world.player_graph.plan.nodes.iter().filter_map(|node|
                    world.player_graph.state.clips.get(&node.object).map(|clip|clip.settings.animation_name.clone())).collect::<Vec<_>>();
                // Only the explicitly inspected actor is recorded. Query the
                // live owners after physics/animation; don't maintain a second
                // actor simulation or retain a growing history in the World.
                let focused_actor = self.actor_focus.as_deref()
                    .and_then(|name| world.scripts.resolve_name(name))
                    .map(|owner| -> anyhow::Result<serde_json::Value> {
                        let entity = &world.entities[owner];
                        let body = world.character_physics.get(&owner)
                            .and_then(|body| body.as_ref().ok());
                        let physics = body.map(|body| -> anyhow::Result<_> {
                            let pose = world.collision.simulation_pose(body.shape)?;
                            Ok(serde_json::json!({"position":pose.position,"velocity":pose.linear,
                                "motion_type":body.motion.current.motion_type(),
                                "in_world":world.collision.owner_in_world(body.owner)?}))
                        }).transpose()?;
                        let animation = world.authored_havok.contains(owner)
                            .then(|| world.authored_havok.snapshot(owner)).transpose()?;
                        let brain = world.ai_brains.get(&owner);
                        Ok(serde_json::json!({"owner":owner,"position":&entity.transform[12..15],
                            "physics":physics,"motion_suspended":world.basic_motion_suspended(owner),
                            "scripted_idle":brain.map(|b|b.motion_suspended),
                            "behavior":brain.and_then(|b|b.selector.current.and_then(|slot|b.behaviors.get(slot))),
                            "relocation":world.npc_relocations.get(&owner),"animation":animation}))
                    }).transpose()?;
                let trace=serde_json::json!({"tick":tick,"position":player.position,
                    "focused_actor":focused_actor,
                    "respawns":player.respawns,
                    "script_input_restricted":player.script_input_restricted,
                    "cutscene_restrictions":world.cutscene_input.count,
                    "brush_ownership":world.player_abilities["Brush"].owned,
                    "preview_runtime":self.preview_runtime,"requested_hose":requested_hose,
                    "aim_camera":world.nif_camera,"aim_ray":aim_ray,"aim_target":aim_target,
                    "application_camera_source":world.camera_pipeline.last_authored,
                    "application_camera_publication":world.camera_pipeline.publications,
                    "velocity":player.velocity,"yaw":player.yaw,"grounded":player.grounded,
                    "support_state":player.support_state.map(|state|state as u32),
                    "motion_ground":player.motion_ground,
                    "stairs_ground_retained":player.stairs_ground_retained,
                    "motion_surface_normal":player.motion_surface_normal,
                    "camera_feedback":world.camera_feedback,
                    "mickey_tracker":world.mickey_tracker.as_ref().map(|tracker|serde_json::json!({"owner":tracker.owner,"target":tracker.target,"state":tracker.state,"error":tracker.error})),
                    "motion_state":player.motion_state(),"animation_velocity":player.animation_velocity,
                    "ground_jump_started":player.ground_jump_started,"air_first_jump_started":player.air_first_jump_started,
                    "second_jump_started":player.second_jump_started,"camera_yaw":camera_yaw,"camera_pitch":camera_pitch,
                    "animation_events":events,"active_clips":clips,"hose_emitters":self.hose_emitters,
                    "mickey_idle":self.authored_preview.as_ref().map(|preview|preview.idle),
                    "native_static_contacts":world.collision.native_character_contact_counts(),
                    "shots":world.projectile_preview.emitted,"contacts":world.projectile_preview.contacts});
                if let Some(writer) = self.playback_trace.as_mut() {
                    if let Err(error) = writer.push(&trace) {
                        self.playback_trace_error = Some(format!("Could not write playback trace: {error:#}"));
                        self.playback_trace = None;
                    }
                }
            }
            Ok(())
        });
        {log::debug!(target:"blotworx_phase", "simulation end ticks={}",controller.ticks);}
        record_phase(&mut self.frame_phases, "simulation", simulation_started);
        world.entities[world.player_entity]
            .set_controller_pose(controller.player.position, controller.player.yaw);
        if let Err(e) = result {
            let message = format!("Authored animation: {e:#}");
            log::warn!("{message}");
            self.error = Some(message);
            if capture_mode {
                self.paused = true;
                return;
            }
        }
        //Native6f58 consumes the published frame clocks, not a character
        //catch-up delta. Update exactly once even when this frame had no ticks.
        if let Err(error) = world.advance_screen_fade_frame() {
            self.error = Some(format!("Screen fade: {error:#}"));
            self.paused = true;
            return;
        }
        self.jump_pending = false;
        let presentation_started = Instant::now();
        let position = controller.presentation_position();
        let player = &controller.player;
        self.camera.yaw = controller.camera_yaw;
        self.camera.pitch = controller.camera_pitch;
        let focus = self
            .actor_focus
            .as_ref()
            .and_then(|name| {
                world
                    .entities
                    .iter()
                    .position(|entity| &entity.name == name)
            })
            .or_else(|| {
                world
                    .actor_playback
                    .as_ref()
                    .map(|playback| playback.entity)
            });
        self.camera.camera_target = if let Some(entity) = focus {
            match world.entity_position_or_bound_center(entity) {
                Ok(Some(center)) => center,
                Ok(None) => add(position, [0., 1., 0.]),
                Err(error) => {
                    self.error = Some(format!("Inspection bounds: {error:#}"));
                    return;
                }
            }
        } else {
            add(position, [0., 1., 0.])
        };
        let shake = world.script_feedback.angles();
        self.camera.yaw += shake[0];
        self.camera.pitch += shake[1];
        let direction = [
            self.camera.yaw.cos() * self.camera.pitch.cos(),
            self.camera.pitch.sin(),
            self.camera.yaw.sin() * self.camera.pitch.cos(),
        ];
        if capture_mode {
            if let Some(target) = self.capture_camera_target {
                self.camera.camera_target = target;
            }
        }
        self.camera.camera_override = None;
        let authored_camera = match world.published_camera_frame() {
            Ok(frame) => frame,
            Err(error) => {
                self.error = Some(format!("Application camera: {error:#}"));
                return;
            }
        };
        let inspection_camera =
            focus.is_some() || (capture_mode && self.capture_camera_target.is_some());
        // Every view-dependent consumer must use the camera actually presented.
        // Retaining the authored frame here made render probes/debug picking
        // inspect a different room while an asset-focused camera was displayed.
        let authored_camera = authored_camera.filter(|_| !inspection_camera);
        self.camera.distance = if authored_camera.is_some() && !inspection_camera {
            self.zoom
        } else {
            match world.camera_preview_distance(self.camera.camera_target, direction, self.zoom) {
                Ok(distance) => distance,
                Err(error) => {
                    self.error = Some(format!("Camera obstruction: {error:#}"));
                    return;
                }
            }
        };
        if let Some(frame) = authored_camera {
            let projection = match frame.projection() {
                Ok(projection) => projection,
                Err(error) => {
                    self.error = Some(format!("NIF camera projection: {error:#}"));
                    return;
                }
            };
            self.camera.camera_override = Some(viewport3d::CameraOverride {
                eye: frame.eye,
                view: frame.view(),
                projection,
            });
        }
        self.camera.lighting_mode = if self.retail_lighting {
            LightingMode::SceneLights
        } else {
            LightingMode::Headlight
        };
        #[cfg(feature = "blotworx-ui-render")]
        let (mut pause_input, hud_cursor) = if self.pause_menu_opened.is_some() {
            let pointer_input = if !capture_mode && self.pause_menu_opened.is_some_and(|opened| opened.elapsed().as_millis() >= 150) {
                self.pause_pointer.input(ui, aim_rect, self.pad, self.pad_edges, elapsed as f32)
            } else {
                Vec::new()
            };
            if self.hud.is_some() && ui.input(|input| input.pointer.hover_pos().is_some_and(|point| aim_rect.contains(point))) {
                ui.ctx().set_cursor_icon(egui::CursorIcon::None);
            }
            (pointer_input, Some(self.pause_pointer.position))
        } else { (Vec::new(), hud_cursor) };
        #[cfg(feature = "blotworx-ui-render")]
        if !self.diagnostic_serial_hud {
            if let Some(hud) = &mut self.hud {
                if let Err(error) =
                    hud.prepare(ui.available_rect_before_wrap(), ui.ctx().pixels_per_point(), world, controller.ticks,
                        self.pause_menu_opened.map(|opened| opened.elapsed().as_secs_f64()), std::mem::take(&mut pause_input), hud_cursor)
                {
                    self.error = Some(format!("Gameplay HUD preparation: {error:#}"));
                    self.hud = None;
                }
            }
        }
        let mut draws = world.draws.clone();
        let player_matrix =
            world.entities[world.player_entity].controller_matrix(position, player.yaw);
        for &i in &world.player_draws {
            draws[i].trans_mat = player_matrix;
        }
        // Apply the same sub-tick presentation interpolation delta to Mickey's
        // attached children (e.g. paintbrush) so they stay locked in his hand at 60/144Hz.
        let interp_delta = [
            position[0] - player.position[0],
            position[1] - player.position[1],
            position[2] - player.position[2],
        ];
        if interp_delta.iter().any(|v| v.abs() > 1e-6) {
            let mut stack: Vec<usize> = Vec::new();
            if let Some(slots) = world.live_child_slots.get(&world.player_entity) {
                stack.extend(slots.iter().flatten().copied());
            }
            for (&child, local) in &world.transform_hierarchy {
                if local.parent == world.player_entity && !stack.contains(&child) {
                    stack.push(child);
                }
            }
            let mut visited = std::collections::HashSet::new();
            while let Some(child) = stack.pop() {
                if !visited.insert(child) {
                    continue;
                }
                if let Some(entity) = world.entities.get(child) {
                    for draw_idx in entity.draws.clone() {
                        if let Some(draw) = draws.get_mut(draw_idx) {
                            draw.trans_mat[12] += interp_delta[0];
                            draw.trans_mat[13] += interp_delta[1];
                            draw.trans_mat[14] += interp_delta[2];
                        }
                    }
                }
                if let Some(slots) = world.live_child_slots.get(&child) {
                    stack.extend(slots.iter().flatten().copied());
                }
            }
        }
        if !self.retail_lighting {
            for (index, draw) in draws.iter_mut().enumerate() {
                let Some(vertices) = world.inspection_vertices(index) else {
                    self.error = Some(format!("Inspection vertex cache is missing draw {index}"));
                    return;
                };
                draw.v_data = vertices;
                draw.baked = false;
                if !world.sketch_draws.contains(&index)
                    && !world.jigsaw_effect_values.contains_key(&index)
                {
                    draw.preview = None;
                }
            }
        }
        let (sy, cy) = self.camera.yaw.sin_cos();
        let (sp, cp) = self.camera.pitch.sin_cos();
        let billboard_right = self
            .camera
            .camera_override
            .as_ref()
            .map(|camera| [camera.view[0], camera.view[4], camera.view[8]])
            .unwrap_or([sy, 0., -cy]);
        let billboard_up = self
            .camera
            .camera_override
            .as_ref()
            .map(|camera| [camera.view[1], camera.view[5], camera.view[9]])
            .unwrap_or([-cy * sp, cp, -sy * sp]);
        let mut brush_range = None;
        if let Some(preview) = &mut self.clip_preview {
            if active {
                preview.advance(elapsed as f32);
            }
            if let Err(e) = preview.apply(world, &mut draws) {
                self.error = Some(format!("Clip preview: {e:#}"));
                self.paused = true;
                return;
            }
        } else if let Some(preview) = &self.authored_preview {
            let result=skin::model_pose(&preview.pose,self.ear_pose.as_ref().map_or(&[],|e|e.overrides.as_slice()))
                .and_then(|pose| {
                    self.player_skin_frame.apply(world,&pose,&mut draws)?;
                    if world.hidden_entities.contains(&world.player_entity) {return Ok(());}
                    let actor=draws[*world.player_draws.first().context("Missing Mickey draw")?].trans_mat;
                    let brush_start=draws.len();
                    let canonical=matches!(world.child_entity_by_name(world.player_entity,"magicbrush"),Ok(Some(child)) if !world.entities[child].draws.is_empty());
                    if !canonical {
                        world.brush.append(&pose,actor,&mut draws)?;
                        brush_range=Some(brush_start..draws.len());
                        if !self.retail_lighting {for draw in &mut draws[brush_start..] {draw.preview=None;}}
                    }

                    let (sy,cy)=self.camera.yaw.sin_cos();let (sp,cp)=self.camera.pitch.sin_cos();
                    if let Some(ribbon)=&world.spin_ribbon {
                        ribbon.append(billboard_right,billboard_up,&mut draws)?;
                    }
                    if let Some(particles)=&world.spin_particles {
                        particles.append(billboard_right,billboard_up,&mut draws)?;
                    }
                    for hose in world.brush.hoses.iter().filter(|_|!self.capture_hide_hose) {
                        hose.append(billboard_right,billboard_up,&mut draws)?;
                    }
                    Ok(())
                });
            if let Err(e) = result {
                self.error = Some(format!("Animated skin: {e:#}"));
                self.paused = true;
                return;
            }
        }
        if let Err(error) = world.apply_model_geometry(&mut draws) {
            self.error = Some(format!("Model presentation: {error:#}"));
            self.paused = true;
            return;
        }
        let model_eye=self.camera.camera_override.as_ref().map(|camera|camera.eye).unwrap_or(std::array::from_fn(|i|self.camera.camera_target[i]+self.camera.distance*direction[i]));
        if let Err(error)=world.apply_model_camera_transforms(&mut draws,model_eye,billboard_right,billboard_up) {
            self.error=Some(format!("Camera-facing model presentation: {error:#}"));self.paused=true;return;
        }
        let lighting_started = Instant::now();
        if self.retail_lighting {
            let (sy, cy) = self.camera.yaw.sin_cos();
            let (sp, cp) = self.camera.pitch.sin_cos();
            let eye = add(
                self.camera.camera_target,
                [
                    self.camera.distance * cy * cp,
                    self.camera.distance * sp,
                    self.camera.distance * sy * cp,
                ],
            );
            let view = self
                .camera
                .camera_override
                .as_ref()
                .map(|camera| camera.view)
                .unwrap_or_else(|| {
                    viewport3d::look_at_matrix(eye, self.camera.camera_target, [0., 1., 0.])
                });
            world.apply_spherical_lighting(&mut draws, &view);
            if let Some(range) = brush_range.clone() {
                if let Err(error) = world.brush.apply_lighting(
                    &mut draws[range],
                    &view,
                    &world.report.spherical_lights,
                ) {
                    self.error = Some(format!("Brush lighting: {error:#}"));
                    self.paused = true;
                    return;
                }
            }
        }
        // Lighting can replace the material program and its image bindings.
        // Publish NIF controller output against those final bindings.
        if let Err(error) = world.apply_model_materials(&mut draws) {
            self.error = Some(format!("Model material animation: {error:#}"));
            self.paused = true;
            return;
        }
        record_phase(&mut self.frame_phases, "lighting_cpu", lighting_started);
        // Filter only after animation/attachment updates, which use stable draw
        // indices. Geometry stays owned by World so surfaces can be restored.
        let mut draw_index = 0;
        if let Err(error) = world.apply_model_constant_colors(&mut draws) {
            self.error = Some(format!("Model color publication: {error:#}"));
            self.paused = true;
            return;
        }
        if let Err(error) = world
            .material_animation
            .apply(world.animation_time, &mut draws)
        {
            self.error = Some(format!("Material animation: {error:#}"));
            self.paused = true;
            return;
        }
        let projector_started = Instant::now();
        if self.retail_lighting && !self.diagnostic_no_model_shadow {
            match world.prepare_model_shadow(&mut draws, brush_range) {
                Ok(frame) => {
                    self.camera.model_shadow = if self.retail_lighting { frame } else { None }
                }
                Err(error) => {
                    self.camera.model_shadow = None;
                    world
                        .model_shadow
                        .errors
                        .insert("projector".into(), format!("{error:#}"));
                }
            }
        } else {
            self.camera.model_shadow = None;
        }
        record_phase(&mut self.frame_phases, "projector_cpu", projector_started);
        if let Some(error) = self
            .camera
            .model_shadow_report
            .lock()
            .unwrap()
            .error
            .clone()
        {
            world.model_shadow.errors.insert("GPU".into(), error);
        }
        if !self.render_probe_done
            && self
                .capture_tick
                .is_some_and(|tick| controller.ticks >= tick)
        {
            if let Some([x, y]) = self.render_probe {
                let (sy, cy) = self.camera.yaw.sin_cos();
                let (sp, cp) = self.camera.pitch.sin_cos();
                let origin = add(
                    self.camera.camera_target,
                    [
                        self.camera.distance * cy * cp,
                        self.camera.distance * sp,
                        self.camera.distance * sy * cp,
                    ],
                );
                let size = ui.available_size();
                let extent = (self.camera.fov_degrees.to_radians() * 0.5).tan();
                let vx = (x * 2. - 1.) * extent * size.x / size.y;
                let vy = (1. - y * 2.) * extent;
                let direction = physics::unit([
                    -cy * cp + sy * vx - cy * sp * vy,
                    -sp + cp * vy,
                    -sy * cp - cy * vx - sy * sp * vy,
                ]);
                let (origin, direction) = if let Some(frame) = authored_camera {
                    let scale = ui.ctx().pixels_per_point();
                    let pixel = [(x * size.x * scale) as i32, (y * size.y * scale) as i32];
                    let size = [
                        (size.x * scale).round() as u32,
                        (size.y * scale).round() as u32,
                    ];
                    match frame.cursor_camera().window_ray(pixel, size) {
                        Ok(Some(ray)) => (ray.origin, ray.direction),
                        result => {
                            self.error = Some(format!("Authored camera render probe: {result:?}"));
                            return;
                        }
                    }
                } else {
                    (origin, direction)
                };
                let mut report = render_probe::candidates(world, &draws, origin, direction);
                if let Some(name) = self.actor_focus.as_ref().or_else(|| {
                    self.diagnostic_counter_impact
                        .as_ref()
                        .map(|(name, _)| name)
                }) {
                    report["focus_geometry"] = render_probe::entity_geometry(world, &draws, name);
                }
                if let Some(path) = &self.capture {
                    if let Err(error) = std::fs::write(
                        path.with_extension("draw-probe.json"),
                        serde_json::to_vec_pretty(&report).unwrap(),
                    ) {
                        self.error = Some(error.to_string());
                    }
                }
                self.render_probe_done = true;
            }
        }
        let mp_ear_camera = authored_camera
            .map(|frame| frame.ear_camera())
            .unwrap_or_else(|| {
                let (sy, cy) = self.camera.yaw.sin_cos();
                let (sp, cp) = self.camera.pitch.sin_cos();
                let dir = [cy * cp, sp, sy * cp];
                ears::EarTransform {
                    rotation: [[-sy, -cy * sp, -cy * cp], [0., cp, -sp], [cy, -sy * sp, -sy * cp]],
                    position: std::array::from_fn(|i| {
                        self.camera.camera_target[i] + dir[i] * self.camera.distance
                    }),
                }
            });
        let mp_view = self
            .camera
            .camera_override
            .as_ref()
            .map(|camera| camera.view)
            .unwrap_or_else(|| {
                let (sy, cy) = self.camera.yaw.sin_cos();
                let (sp, cp) = self.camera.pitch.sin_cos();
                let eye = add(
                    self.camera.camera_target,
                    [
                        self.camera.distance * cy * cp,
                        self.camera.distance * sp,
                        self.camera.distance * sy * cp,
                    ],
                );
                viewport3d::look_at_matrix(eye, self.camera.camera_target, [0., 1., 0.])
            });
        self.multiplayer
            .apply_player_draws(world, &mut draws, mp_ear_camera, &mp_view);
        let mut draw_index = 0usize;
        draws.retain(|draw| {
            let visible = !world.hidden_jigsaw_draws.contains(&draw_index)
                && !world.hidden_entity_draws.contains(&draw_index)
                && !world.removed_draws.contains(&draw_index)
                && !draw.material.as_ref().and_then(|material| material.shader.as_deref())
                    .is_some_and(|shader| shader.eq_ignore_ascii_case("SPL_hiddenSurface"))
                && !world.demo_story.scene.culled_draws.contains(&draw_index);
            draw_index += 1;
            visible
        });
        if !self.capture_hide_decals {
            if let Err(error) = self.splat_preview.append(&mut draws) {
                self.error = Some(format!("Decal color publication: {error:#}"));
                return;
            }
        }
        if let Some(effect) = &world.particle_asset_preview {
            if let Err(error) = effect
                .particles
                .append(billboard_right, billboard_up, &mut draws)
            {
                self.error = Some(format!("Particle asset inspection: {error:#}"));
                return;
            }
        }
        for (&entity, drip) in &world.scene_particles.drips {
            if world.hidden_entities.contains(&entity) || world.removed_entities.contains(&entity) || !world.entity_scene_active(entity) { continue; }
            if let Err(error) = drip.ribbon.append(billboard_right, billboard_up, &mut draws) {
                self.error = Some(format!("Drip ribbon: {error:#}"));
                return;
            }
        }
        for (&entity, effect) in &world.scene_particles.live {
            if world.hidden_entities.contains(&entity) || world.removed_entities.contains(&entity) {
                continue;
            }
            if let Err(error) = effect
                .particles
                .append(billboard_right, billboard_up, &mut draws)
            {
                self.error = Some(format!("Scene particles: {error:#}"));
                return;
            }
        }
        for (&entity, effect) in &world.named_fx.live {
            if world.hidden_entities.contains(&entity) {
                continue;
            }
            let eye = self
                .camera
                .camera_override
                .as_ref()
                .map(|camera| camera.eye)
                .unwrap_or(std::array::from_fn(|i| {
                    self.camera.camera_target[i] + self.camera.distance * direction[i]
                }));
            if let Err(error) = effect.append(
                &world.entities[entity],
                eye,
                billboard_right,
                billboard_up,
                &world.report.spherical_lights,
                &mut draws,
            ) {
                self.error = Some(format!("Named effect {}: {error:#}", effect.template));
                return;
            }
        }
        if let Err(error) = world.refresh_shadow_volumes() {
            self.error = Some(format!("Shadow volume update: {error:#}"));
            return;
        }
        self.camera.volume_shadow_mesh = world.shadow_volumes.mesh.clone();
        self.camera
            .volume_shadows
            .clone_from(&world.shadow_volumes.matrices);
        let mut debug_lines = self.debug.world_lines(world, controller);
        if self.debug.open {
            world.projectile_preview.lines(&mut debug_lines);
        }
        record_phase(
            &mut self.frame_phases,
            "presentation_including_projector",
            presentation_started,
        );
        self.camera.finish=self.preferences.grade();
        self.camera.smooth_textures = self.preferences.smooth_textures;
        self.camera.surfaces=self.preferences.surface();
        self.camera.fov_degrees=self.preferences.fov;
        self.camera.pixel_probe=if self.capture_tick.is_some_and(|tick|controller.ticks>=tick.saturating_sub(2)){self.render_probe}else{None};
        let render_lights=if self.camera.surfaces.mode!=0 {match world.enhanced_scene_lights(self.camera.surfaces.mode){Ok(l)=>l,Err(e)=>{self.error=Some(format!("Lighting: {e:#}"));self.paused=true;return;}}}else{world.lights.clone()};
        let (rect, response, view_projection, eye) = viewport3d::render_viewport(
            ui,
            &mut self.camera,
            draws,
            debug_lines,
            render_lights,
            false,
        );
        #[cfg(feature = "blotworx-ui-render")]
        if let Some(hud) = &mut self.hud {
            let result = (|| -> Result<()> {
                if self.diagnostic_serial_hud {
                    hud.prepare(rect, ui.ctx().pixels_per_point(), world, controller.ticks,
                        self.pause_menu_opened.map(|opened| opened.elapsed().as_secs_f64()), std::mem::take(&mut pause_input), hud_cursor)?;
                }
                if self.preferences.hud || self.pause_menu_opened.is_some() { hud.present(ui, rect, world)?; }
                if self.capture.is_some()
                    && self
                        .capture_tick
                        .is_none_or(|tick| controller.ticks >= tick)
                {
                    hud.finish_previous(world)?;
                }
                Ok(())
            })();
            if let Err(error) = result {
                self.error = Some(format!("Gameplay HUD: {error:#}"));
                self.hud = None;
            }
        }
        #[cfg(feature = "blotworx-ui-render")]
        let missing_game_hud = self.hud.is_none();
        #[cfg(not(feature = "blotworx-ui-render"))]
        let missing_game_hud = true;
        if missing_game_hud {
            world.ability_hud_requests.clear();
        }
        // Scripted HideHud only owns the Flash HUD. The world prompt follows the
        // native interaction target, which already excludes dialogue and input locks.
        self.usable_prompt_target = if self.preferences.hud && self.pause_menu_opened.is_none()
            && !world.screen_fade.hide_gameplay_hud
        {
            self.usable_button.as_ref().and_then(|button| {
                let target = world.usable_target(player.position)?;
                let owner = &world.entities[target];
                usable_prompt::draw(ui, rect, &view_projection, button, owner).then_some(target)
            })
        } else {
            None
        };
        if requested_hose.is_some() && (missing_game_hud || !self.preferences.hud)
            && (!world.demo_story.enabled || world.demo_story.reticle_visible)
            && !world.screen_fade.hide_gameplay_hud
        {
            let color = if requested_hose == Some(0) {
                egui::Color32::from_rgb(90, 160, 255)
            } else {
                egui::Color32::from_rgb(100, 200, 65)
            };
            ui.painter().with_clip_rect(rect).circle_stroke(
                aim_pointer,
                5.,
                egui::Stroke::new(1.5, color),
            );
        }
        self.debug.clear_pointer();
        if self.debug.open {
            let pointer = if capture_mode {
                Some(rect.center())
            } else if response.hovered() {
                ui.input(|i| i.pointer.hover_pos())
            } else {
                None
            };
            if let Some(pointer) = pointer {
                let camera = if let Some(frame) = authored_camera {
                    frame.cursor_camera()
                } else {
                    let (sy, cy) = self.camera.yaw.sin_cos();
                    let (sp, cp) = self.camera.pitch.sin_cos();
                    let extent = (self.camera.fov_degrees.to_radians() * 0.5).tan();
                    let aspect = (rect.width() / rect.height().max(1.)).max(0.1);
                    let camera = cursor::Camera {
                        position: eye,
                        left: [-sy, 0., cy],
                        up: [-cy * sp, cp, -sy * sp],
                        forward: [-cy * cp, -sp, -sy * cp],
                        viewport: cursor::Rect {
                            left: 0.,
                            right: 1.,
                            top: 1.,
                            bottom: 0.,
                        },
                        frustum: cursor::Rect {
                            left: -extent * aspect,
                            right: extent * aspect,
                            top: extent,
                            bottom: -extent,
                        },
                        orthographic: false,
                    };
                    camera
                };
                let scale = ui.ctx().pixels_per_point();
                let local = (pointer - rect.min) * scale;
                self.debug.probe_pointer(
                    &camera,
                    [local.x as i32, local.y as i32],
                    [
                        (rect.width() * scale).round() as u32,
                        (rect.height() * scale).round() as u32,
                    ],
                    &world.collision,
                    authored_camera.map_or(self.camera.far_clip, |frame| frame.frustum[5]),
                );
                ui.painter().with_clip_rect(rect).circle_stroke(
                    pointer,
                    4.,
                    egui::Stroke::new(1., egui::Color32::from_rgb(100, 230, 255)),
                );
            }
        }
        self.loaded_frames += 1;
        if let Some(bars) = world.camera_lens.forced_letterbox_rectangles() {
            for [left, top, width, height] in bars {
                let bar = egui::Rect::from_min_size(
                    rect.min + egui::vec2(left * rect.width(), top * rect.height()),
                    egui::vec2(width * rect.width(), height * rect.height()),
                );
                ui.painter()
                    .with_clip_rect(rect)
                    .rect_filled(bar, 0., egui::Color32::BLACK);
            }
        }
        if world.screen_fade.opacity > 0. {
            let alpha = (world.screen_fade.opacity.min(1.) * 255.).round() as u8;
            ui.painter().with_clip_rect(rect).rect_filled(
                rect,
                0.,
                egui::Color32::from_black_alpha(alpha),
            );
        }
        if !capture_mode {
            let level_name = world.report.level.clone();
            let p_vel = player.velocity;
            let p_yaw = player.yaw;
            let p_grounded = player.grounded;
            let p_spinning = self.spin_preview_pending
                || world.spin_ribbon.is_some();
            let (anim_clip_hash, anim_time) =
                animation_resources::Resources::active_clip_and_time(&world.player_graph);
            self.multiplayer.tick(
                ui.ctx(),
                &level_name,
                position,
                p_vel,
                p_yaw,
                p_grounded,
                p_spinning,
                requested_hose,
                anim_clip_hash,
                anim_time,
                eye,
                billboard_right,
            );
            self.multiplayer.sync_world(world, position);
            self.multiplayer
                .draw_overlays(ui, rect, &view_projection, position);
            if let Some(target_level) = self.multiplayer.take_requested_level_switch() {
                self.level = target_level;
                self.load(ui.ctx());
            }
        }
    }
}
#[cfg(not(target_os = "horizon"))]
impl Engine {
    fn draw_options(&mut self,ctx:&egui::Context) {
        let before=serde_json::to_vec(&self.preferences).unwrap_or_default();
        let (mut open,mut tab)=(self.options_open,self.options_tab);
        options_menu::panel(ctx,&mut open,&mut tab,|ui,selected|{
                  match selected {
                   0=>self.preferences.graphics(ui),
                   1=>{if self.preferences.display_controls(ui,ctx){self.zoom=self.preferences.distance;}},
                   2=>{
            ui.collapsing("Game data", |ui| {
                ui.add_enabled_ui(self.receiver.is_none(), |ui| {
                    ui.text_edit_singleline(&mut self.root);
                    if ui.button("Choose game folderâ€¦").clicked() {
                        if let Some(path) = crate::file_dialog::FileDialog::new().pick_folder() {
                            self.root = path.to_string_lossy().into_owned();
                        }
                    }
                    if ui.button("Refresh level list").clicked() {
                        self.level_catalog_root.clear();
                    }
                });
            });
            if self.level_catalog_root != self.root {
                self.level_catalog_root = self.root.clone();
                match assets::level_catalog(std::path::Path::new(&self.root)) {
                    Ok(levels) => self.level_catalog = levels,
                    Err(error) => {
                        self.level_catalog.clear();
                        self.error = Some(format!("Level list: {error:#}"));
                    }
                }
            }
            ui.horizontal(|ui| {
                ui.label("Level");
                ui.add_enabled_ui(self.receiver.is_none(), |ui| {
                    if ui.button("Continue checkpoint").clicked() {
                        self.continue_checkpoint(ui.ctx());
                    }
                    if ui.button("Open levelâ€¦").clicked() {
                        if let Some(path) = crate::file_dialog::FileDialog::new()
                            .add_filter("Epic Mickey level", &["level"])
                            .set_directory(std::path::Path::new(&self.root).join("Levels"))
                            .pick_file()
                        {
                            self.level = path.to_string_lossy().into_owned();
                            self.actor_focus = None;
                            self.load(ui.ctx());
                        }
                    }
                    egui::ComboBox::from_id_salt("demo_level_picker")
                        .width(320.)
                        .selected_text(&self.level)
                        .show_ui(ui, |ui| {
                            ui.text_edit_singleline(&mut self.level_filter);
                            ui.checkbox(
                                &mut self.show_unavailable_levels,
                                "Show unavailable development levels",
                            );
                            ui.separator();
                            let filter = self.level_filter.to_ascii_lowercase();
                            for entry in &self.level_catalog {
                                if (self.show_unavailable_levels || entry.unavailable.is_none())
                                    && entry.path.to_ascii_lowercase().contains(&filter)
                                {
                                    ui.add_enabled_ui(entry.unavailable.is_none(), |ui| {
                                        let response = ui.selectable_value(
                                            &mut self.level,
                                            entry.path.clone(),
                                            &entry.path,
                                        );
                                        if let Some(reason) = &entry.unavailable {
                                            response.on_disabled_hover_text(reason);
                                        }
                                    });
                                }
                            }
                        });
                    let unavailable = self
                        .level_catalog
                        .iter()
                        .find(|entry| entry.path.eq_ignore_ascii_case(&self.level))
                        .and_then(|entry| entry.unavailable.as_deref());
                    let load = ui.add_enabled(
                        unavailable.is_none(),
                        egui::Button::new("Load selected level"),
                    );
                    let load = if let Some(reason) = unavailable {
                        load.on_disabled_hover_text(reason)
                    } else {
                        load
                    };
                    if load.clicked() {
                        self.actor_focus = None;
                        self.load(ui.ctx());
                    }
                });
            });
            if self.receiver.is_some() {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("Loading selected levelâ€¦");
                });
            } else if let Some(world) = &self.world {
                ui.small(format!("Loaded: {}", world.report.level));
            }

                   },
                   3=>{
                    ui.checkbox(&mut self.debug.open,"Debug overlay · F3");ui.checkbox(&mut self.details,"Level report");
                    ui.checkbox(&mut self.retail_lighting,"Game materials and lighting");
                    ui.checkbox(&mut self.camera.view_culling,"View culling");
                    ui.checkbox(&mut self.camera.backface_culling,"Backface culling");
                    ui.checkbox(&mut self.camera.show_limitations,"Material warnings");
                    ui.checkbox(&mut self.paused,"Pause simulation");
                    if let Some(world) = self.world.as_mut() {
                        if ui.button("Unlock preview abilities")
                            .on_hover_text("Grants brush, blast and spin. Scripted restrictions still apply.")
                            .clicked()
                        {
                            if let Err(error) = world.enable_demo_abilities() {
                                self.error = Some(format!("Unlock abilities: {error:#}"));
                            }
                        }
                        if world.demo_story.npc_fallback.enabled {
                            ui.weak("Temporary NPC navigation enabled");
                        }
                    }
                   }
                   4 => self.multiplayer.draw_lobby_contents(ui, ctx),
                   _ => {}
                  }
        });
        self.options_open=open;self.options_tab=tab;
        if serde_json::to_vec(&self.preferences).unwrap_or_default()!=before {self.preferences_dirty=true;}
        if self.preferences_dirty && !ctx.input(|i|i.pointer.any_down()) {
          match self.preferences.save(){Ok(())=>self.preferences_dirty=false,Err(error)=>self.error=Some(format!("Settings: {error:#}"))}
        }
    }
}
#[cfg(not(target_os = "horizon"))]
impl eframe::App for Engine {
    fn on_exit(&mut self, _gl: Option<&egui_glow::painter::Context>) {
        #[cfg(feature = "blotworx-ui-render")]
        if _gl.is_some() {
            if let Some(hud) = &mut self.hud {
                hud.destroy_graphics();
            }
            ui_movie::GameplayWorker::destroy_retired_graphics();
        }
        let save = if let Some(world) = &self.world {
            world.checkpoint.service.as_ref().map(|service| {
                (service, world.frame_clock.hardware_sample(), world.frame_clock.clock.rate.ticks_per_second.get())
            })
        } else {
            self.loading_save.as_ref().map(|(service, sample, frequency)| (service, *sample, *frequency))
        };
        if let Some((service, sample, frequency)) = save {
            if let Err(error) = service.shutdown(sample, frequency) {
                eprintln!("Blotworx checkpoint exit flush failed: {error:#}");
            }
        }
    }
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.updater.show(ui.ctx());
        if self.receiver.is_some() {
            if let Some(screen) = self.loading_screen.as_mut().filter(|screen| !screen.ready_to_dismiss()) {
                screen.paint(ui);
                return;
            }
        }
        if let Some(rx) = &self.receiver {
            let result=match rx.try_recv() {
                Ok(result)=>Some(result),
                Err(mpsc::TryRecvError::Empty)=>None,
                Err(mpsc::TryRecvError::Disconnected)=>Some(Err(anyhow::anyhow!(
                    "Level loading stopped unexpectedly. The previous level was released before loading to reduce peak memory; load it again to return."))),
            };
            if let Some(result) = result {
                self.receiver = None;
                match result {
                    Ok(mut world) => {
                        log::info!(target:"blotworx_phase", "Level import received");
                        let ear_pose = match ears::Pose::new(&world.player_animations.rig.bones) {
                            Ok(pose) => pose,
                            Err(e) => {
                                self.error = Some(format!("Mickey ears: {e:#}"));
                                return;
                            }
                        };
                        let authored_preview = match authored_preview::Preview::new(&mut world) {
                            Ok(preview) => preview,
                            Err(e) => {
                                self.error = Some(format!("Authored animation: {e:#}"));
                                return;
                            }
                        };
                        // Retain the previous playable world until the replacement has
                        // passed import, animation and physics initialization.
                        if let Err(error) = world.attach_player_body() {
                            self.error = Some(format!("Player physics: {error:#}"));
                            return;
                        }
                        let mut controller =
                            controller::Controller::new(world.take_startup_player());
                        if let Some(name) = &self.preview_entry {
                            if world.entities.iter().any(|entity| &entity.name == name) {
                                if let Err(error) =
                                    select_preview_entry(&mut world, &mut controller, name)
                                {
                                    self.error = Some(format!("Preview entry: {error:#}"));
                                    return;
                                }
                            } else if self.capture.is_some() {
                                self.error = Some(format!(
                                    "Preview entry {name} is not placed in this level"
                                ));
                                return;
                            } else {
                                self.preview_entry = None;
                            }
                        }
                        self.ear_pose = Some(ear_pose);
                        self.authored_preview = Some(authored_preview);
                        self.controller = Some(controller);
                        self.splat_preview = Default::default();
                        self.hose_active = false;
                        self.brush_action_respawns = 0;
                        self.hose_phase = None;
                        self.hose_clock = Default::default();
                        self.hose_emitters = [false; 2];
                        self.loaded_frames = 0;
                        self.paused = false;
                        self.pause_menu_opened = None;
                        self.playback_pause = None;
                        self.camera.yaw = 1.6;
                        self.camera.pitch = 0.28;
                        self.zoom = 5.;
                        if let Some(preview) = &mut self.clip_preview {
                            preview.time = 0.;
                        }
                        viewport3d::clear_gpu_cache();
                        self.multiplayer.on_level_loaded(&world.report.level);
                        if let Some(left) = self.diagnostic_shuffle {
                            let result = (|| -> Result<()> {
                                let preview = self
                                    .authored_preview
                                    .as_mut()
                                    .context("Missing animation preview")?;
                                preview.diagnostic_brush_direction = self.diagnostic_strafe;
                                preview.brush_transition(
                                    true,
                                    &world.player_behavior,
                                    &mut world.player_graph,
                                    &mut world.player_animations,
                                )?;
                                let origin = world.report.spawn;
                                let target = physics::add(origin, [0., 3., 4.]);
                                let aim = brush::target_direction(origin, target);
                                preview.brush_aim(
                                    &mut world.player_graph.state,
                                    aim.elevation_dot,
                                    0.,
                                )?;
                                let mut latch = brush::AimShuffle::default();
                                // Deliberate diagnostic stimulus, not gameplay target selection.
                                preview.brush_shuffle(
                                    &mut latch,
                                    if left { -0.5 } else { 0.5 },
                                    [0.; 3],
                                    [1., 4.],
                                    0.25,
                                    &world.player_behavior,
                                    &mut world.player_graph,
                                    &mut world.player_animations,
                                )?;
                                Ok(())
                            })();
                            if let Err(e) = result {
                                self.error = Some(format!("Brush diagnostic: {e:#}"));
                                return;
                            }
                        }
                        if let Some(painted) = self.capture_jigsaw_endpoint {
                            let owners: Vec<_> = world.jigsaw_owners.keys().copied().collect();
                            for owner in owners {
                                let change = if self.capture_jigsaw_animate {
                                    world
                                        .preview_jigsaw_contact(owner, !painted)
                                        .and_then(|_| world.brush_jigsaw_contact(owner, painted))
                                } else {
                                    world.preview_jigsaw_contact(owner, painted)
                                };
                                if let Err(e) = change {
                                    self.error = Some(format!("Jigsaw endpoint diagnostic: {e:#}"));
                                    return;
                                }
                            }
                            eprintln!("Jigsaw endpoint diagnostic: all pieces painted={painted}; this is not an input replay");
                        }
                        if self.diagnostic_hose.is_some() {
                            self.controller.as_mut().unwrap().camera_yaw = 0.7;
                        }
                        if self.diagnostic_shuffle.is_some() {
                            if let Some(settings) = world.report.brush_settings.as_ref() {
                                if let Err(e) = self
                                    .controller
                                    .as_mut()
                                    .unwrap()
                                    .player
                                    .begin_brush_motion(settings.run_speed, settings.turn_rate)
                                {
                                    self.error = Some(e.to_string());
                                    return;
                                }
                            }
                        }

                        if let Some(left) = self.diagnostic_shuffle {
                            self.controller
                                .as_mut()
                                .unwrap()
                                .player
                                .set_aim_facing_target(if left { -0.5 } else { 0.5 });
                        }
                        self.debug.reset_selection();
                        self.camera.camera_override = None;
                        self.error = None;
                        self.level = world.report.level.clone();
                        #[cfg(feature = "blotworx-ui-render")]
                        {
                            self.hud = if self.original_movie_hud {
                                match ui_movie::GameplayWorker::load_with_graphics(
                                    std::path::Path::new(&self.root),
                                    !self.diagnostic_no_hud_reuse,
                                    self.diagnostic_hud_driver_validation,
                                ) {
                                    Ok(mut hud) => {
                                        hud.eager_presentation = self.diagnostic_eager_hud;
                                        Some(hud)
                                    }
                                    Err(error) => {
                                        self.error = Some(format!("Gameplay HUD: {error:#}"));
                                        None
                                    }
                                }
                            } else {
                                None
                            };
                        }
                        self.movie = None;
                        self.last_movie = None;
                        self.world = Some(world);
                        self.loading_save = None;
                        log::info!(target:"blotworx_phase", "Level ready for playback");
                        self.last = Instant::now();
                    }
                    Err(e) => {
                        log::error!("Level load failed: {e:#}");
                        self.error = Some(format!(
                            "{e:#}\nThe previous level was released before loading to reduce peak memory; load it again to return."
                        ));
                    }
                }
            }
        }
        #[cfg(feature = "blotworx-ui-render")]
        if let (Some(hud), Some(world)) = (&mut self.hud, &mut self.world) {
            if let Err(error) = hud.finish_previous(world) {
                self.error = Some(format!("Gameplay HUD: {error:#}"));
                self.hud = None;
            }
        }
        if let Some(world) = self.world.as_mut().filter(|_| self.movie.is_none()) {
            if let Some(request) = world.movies.take_next() {
                if let Err(error) = world.audio_playback.set_game_audio(true, world.game_options.music_volume, world.game_options.sfx_volume) {
                    self.error = Some(format!("Movie audio handoff: {error:#}"));
                }
                self.movie = Some(movie::Player::new(
                    world.source_data_root().to_owned(),
                    request,
                    world.runtime_settings.get("UI_SkipMovie_SkipButtonHoldTime")
                        .and_then(|value|config_value::decimal_float(value).ok()),
                ));
                world.frame_clock.suspend_for_loading();
            }
        }
        if let Some(movie) = &mut self.movie {
            let skip_held=if self.capture.is_some() {
                self.controller.as_ref().and_then(|controller|self.input_playback.as_ref()?.frames.get(controller.ticks as usize))
                    .is_some_and(|frame|frame.movie_skip_held)
            }else {ui.input(|input|input.key_down(egui::Key::Space)) || input::poll().buttons&0x1000!=0};
            #[cfg(feature = "blotworx-ui-render")]
            movie.set_native_skip_prompt(self.hud.is_some());
            let frontend = movie.request.frontend_skip.is_some();
            if frontend {
                let accept = if self.capture.is_some() {
                    skip_held
                } else {
                    let buttons = input::poll().buttons;
                    let pressed = buttons & !self.previous_buttons & 0x1000 != 0;
                    self.previous_buttons = buttons;
                    pressed || ui.input(|input| input.key_pressed(egui::Key::Space) || input.key_pressed(egui::Key::Enter)
                        || (cfg!(target_os = "android") && input.pointer.any_click()))
                };
                movie.show_frontend(ui, accept);
            } else {
                movie.show(ui, skip_held);
            }
            #[cfg(feature = "blotworx-ui-render")]
            if let (Some(hud), Some(world), Some(controller)) = (&mut self.hud, &mut self.world, &self.controller) {
                let rect = ui.max_rect();
                match hud.present_movie(ui, rect, world, controller.ticks, movie.skip_hud(skip_held && !frontend)) {
                    Ok(()) if skip_held && !frontend => movie.draw_skip_progress(ui, rect),
                    Ok(()) => {}
                    Err(error) => {
                        self.error = Some(format!("Movie HUD: {error:#}"));
                        self.hud = None;
                    }
                }
            }
            self.last = Instant::now();
            self.simulation_active = false;
            let capture = self
                .capture_movie_frame
                .is_some_and(|frame| movie.frame.is_some_and(|current| current >= frame))
                || movie.error.is_some();
            if let Some(error) = &movie.error {
                #[cfg(target_os = "android")]
                {
                    let proceed = ui.vertical_centered(|ui| {
                        ui.label("This video could not be played.");
                        ui.weak(error);
                        ui.button("Continue").clicked()
                    }).inner;
                    if proceed || skip_held {
                        movie.continue_after_error();
                    }
                }
                #[cfg(not(target_os = "android"))]
                { self.error = Some(format!("Movie playback: {error}")); }
            }
            let completed = movie.completed;
            self.capture_frame(ui, capture);
            if completed {
                self.last_movie = self.movie.as_ref().map(movie::Player::summary);
                if let Some(world) = &mut self.world {
                    world.movies.completed += 1;
                    world.movies.last_completed = self
                        .movie
                        .as_ref()
                        .map(|movie| movie.request.filename.clone());
                    world.frame_clock.suspend_for_loading();
                }
                self.movie = None;
            }
            return;
        }
        #[cfg(feature = "blotworx-ui-render")]
        self.present_title(ui);
        #[cfg(feature = "blotworx-ui-render")]
        if self.receiver.is_some() {
            self.paint_loading_screen(ui);
            return;
        }
        #[cfg(feature = "blotworx-ui-render")]
        if self.title.is_some() || self.main_menu.is_some() {
            if self.capture.is_none() {
                loading_screen::report_button(ui.ctx(), Some(&self.level), self.error.as_deref(), None);
            }
            return;
        }
        // Automated captures must not spin forever when loading or simulation fails.
        if self.capture.is_some() {
            if let Some(error) = &self.error {
                eprintln!("Capture failed: {error}");
                if let Some(tx) = self.capture_result.take() {
                    let _ = tx.send(Err(error.clone()));
                }
                if let Some(path) = self.capture.take() {
                    let report = path.with_extension("error.txt");
                    if let Err(write_error) = std::fs::write(&report, error) {
                        eprintln!(
                            "Could not save capture error {}: {write_error}",
                            report.display()
                        );
                    }
                }
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                return;
            }
        }
        self.debug.record_frame();
        // Save jobs complete independently of gameplay pause or level import.
        if let Some(service) = self
            .world
            .as_mut()
            .and_then(|world| world.checkpoint.service.as_mut())
        {
            if let Err(error) = service.poll() {
                self.error = Some(format!("Save storage: {error:#}"));
            }
        }
        if let Some(world) = self.world.as_mut().filter(|_| self.receiver.is_none()) {
            log::debug!(target:"blotworx_phase", "audio frame begin");
            let (eye,forward,up)=if let Some(camera)=&self.camera.camera_override {
                (camera.eye,[-camera.view[2],-camera.view[6],-camera.view[10]],[camera.view[1],camera.view[5],camera.view[9]])
            }else {
                let (sy,cy)=self.camera.yaw.sin_cos();let (sp,cp)=self.camera.pitch.sin_cos();let direction=[cy*cp,sp,sy*cp];
                (std::array::from_fn(|i|self.camera.camera_target[i]+self.camera.distance*direction[i]),direction.map(|v|-v),[-cy*sp,cp,-sy*sp])
            };
            if let Err(error)=world.update_audio_view(eye,forward,up){self.error=Some(format!("Audio view: {error:#}"));}
            let music_paused = self.pause_menu_opened.is_some();
            if !music_paused {
                if let Err(error) = world.update_automatic_music(world.frame_clock.clock.scaled_seconds) {
                    self.error = Some(format!("Music state: {error:#}"));
                }
            }
            if let Err(error) = world.audio_playback.set_game_audio(music_paused, world.game_options.music_volume, world.game_options.sfx_volume) {
                self.error = Some(format!("Music controls: {error:#}"));
            }
            if let Err(error) = world.audio_playback.poll_with_random(&mut ||world.game_random.draw() as u16) {
                self.error = Some(format!("Audio output: {error:#}"));
            }
        }
        if ui.input(|i| i.key_pressed(egui::Key::F3)) {
            self.debug.open = !self.debug.open;
        }
        self.pad = input::poll();
        let edge = self.pad.buttons & !self.previous_buttons;
        self.pad_edges = edge;
        self.previous_buttons = self.pad.buttons;
        #[cfg(feature = "blotworx-ui-render")]
        if let Some(hud) = &mut self.hud {
            let (resume, quit) = hud.take_pause_exit();
            if resume { self.pause_menu_opened = None; }
            if quit { ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close); }
        }
        if let Some((_, remaining)) = &mut self.playback_pause {
            if *remaining > 0 && !self.capture_requested {
                *remaining -= 1;
                if *remaining == 0 { self.pause_menu_opened = None; }
            }
        }
        if let Some(controller) = &self.controller {
            let tick = controller.ticks;
            if let Some(frame) = self.input_playback.as_ref().and_then(|p| p.frames.get(tick as usize)) {
                if frame.pause_frames > 0 && self.playback_pause.is_none_or(|(old, _)| old != tick) {
                    self.pause_menu_opened = Some(Instant::now());
                    self.playback_pause = Some((tick, frame.pause_frames));
                }
            }
        }
        let escape = ui.input(|i| i.focused && i.events.iter().any(|event| matches!(event,
            egui::Event::Key { key: egui::Key::Escape, pressed: true, repeat: false, .. })));
        let start = edge & 0x10 != 0 && ui.input(|i| i.focused);
        let pause_pressed = (escape && !self.multiplayer.is_chat_open()) || start;
        if self.capture.is_none() && pause_pressed {
            if self.options_open {
                self.options_open = false;
            } else if self.world.is_some() && self.receiver.is_none()
                && (self.pause_menu_opened.is_none() || start || !self.original_pause_available()) {
                self.pause_menu_opened = self.pause_menu_opened.is_none().then(Instant::now);
                #[cfg(feature = "blotworx-ui-render")]
                { self.pause_pointer = Default::default(); }
                self.jump_pending = false;
                self.spin_preview_pending = false;
                self.anvil_root_pending = false;
                self.pad_edges = 0;
            }
        }
        let gameplay_input = !self.paused && self.pause_menu_opened.is_none() && !pause_pressed;
        if self.capture.is_none() && edge & 0x1000 != 0 && ui.input(|i| i.focused) && gameplay_input {
            self.jump_pending = true;
        }
        if self.capture.is_none() && edge & 0x2000 != 0 && ui.input(|i| i.focused) && gameplay_input {
            self.spin_preview_pending = true;
        }
        if self.receiver.is_some() {
            self.paint_loading_screen(ui);
            return;
        } else {
            self.loading_screen = None;
        }
        if self.world.is_none() {
            if let Some(root) = self.setup.show(ui, self.error.as_deref()) {
                self.root = root.to_string_lossy().into_owned();
                self.load(ui.ctx());
            }
        } else {
            self.draw_options(ui.ctx());
            if self.details {
                egui::Window::new("Level report").open(&mut self.details).resizable(true).default_width(530.).show(ui.ctx(),|ui|{if let Some(w)=&self.world{egui::ScrollArea::vertical().max_height(550.).show(ui,|ui|{ui.label(format!("{} entities Â· {} meshes Â· {} triangles Â· {} collision triangles",w.report.placed_entities,w.report.meshes,w.report.render_triangles,w.report.collision_triangles));ui.label(format!("Authored spawn: {:?}",w.report.spawn));ui.label(format!("Global definitions applied: {}",w.report.global_overrides));if ui.button("Export reportâ€¦").clicked(){if let Some(path)=crate::file_dialog::FileDialog::new().set_file_name("blotworx-report.json").save_file(){if let Err(e)=std::fs::write(path,serde_json::to_vec_pretty(&w.report).unwrap_or_default()){self.error=Some(e.to_string())}}}ui.separator();for line in &w.report.unsupported{ui.label(line);}ui.collapsing("Asset / material warnings",|ui|{for line in &w.report.warnings{ui.label(line);}});ui.collapsing("Components",|ui|{for (name,count) in &w.report.components{ui.label(format!("{count} Ã— {name}"));}});});}});
            }
            self.play(ui);
            #[cfg(feature = "blotworx-ui-render")]
            let original_dialogue = self.hud.is_some() && self.preferences.hud;
            #[cfg(not(feature = "blotworx-ui-render"))]
            let original_dialogue = false;
            if let Some(world) = self.world.as_mut().filter(|_| gameplay_input) {
                if let Some((dialogue, interactive)) = world.demo_story.presented_dialogue() {
                    let speaker = dialogue.speaker.clone();
                    let text = dialogue.text.clone();
                    let choices = dialogue
                        .choices
                        .iter()
                        .map(|(text, _)| text.clone())
                        .collect::<Vec<_>>();
                    let mut selected=world.demo_story.choice;
                    let answer = if original_dialogue && choices.len() <= 2 {
                        if !interactive {
                            None
                        } else if choices.len() == 2 && (self.pad_edges & 0x2000 != 0 || ui.input(|input| input.key_pressed(egui::Key::Backspace))) {
                            Some(1)
                        } else if self.pad_edges & 0x1000 != 0 || ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                            Some(0)
                        } else {
                            None
                        }
                    } else {
                        dialogue_panel::draw(ui,&speaker,&text,interactive.then_some(&choices),&mut selected,self.pad_edges)
                    };
                    world.demo_story.choice=selected;
                    if let Some(answer) = answer {
                        #[cfg(feature = "blotworx-ui-render")]
                        if answer == 0 && original_dialogue && choices.len() <= 2
                            && self.hud.as_ref().is_some_and(|hud| hud.scroll_dialogue().unwrap_or(false))
                        {
                            ui.ctx().request_repaint();
                        } else {
                            world.acknowledge_demo_story(answer);
                        }
                        #[cfg(not(feature = "blotworx-ui-render"))]
                        world.acknowledge_demo_story(answer);
                    }
                }
            }
        }
        if self.capture.is_none() && self.world.is_none() {
            loading_screen::report_button(ui.ctx(), Some(&self.level), self.error.as_deref(), None);
        }
        if self.debug.open {
            let mut lines = Vec::new();
            for (key, label) in [
                ("simulation", "Simulation"),
                ("presentation_including_projector", "Scene preparation"),
                ("lighting_cpu", "Lighting (within preparation)"),
                ("projector_cpu", "Projector (within preparation)"),
            ] {
                if let Some(timing) = self.frame_phases.get(key) {
                    performance::append_line(&mut lines, label, timing);
                }
            }
            #[cfg(feature = "blotworx-ui-render")]
            if let Some(hud) = &self.hud {
                lines.extend(hud.performance_lines());
            }
            self.debug.performance_lines = lines;
        }
        self.debug.show(
            ui.ctx(),
            self.world.as_ref(),
            self.controller.as_ref(),
            self.authored_preview.as_ref(),
            &self.pad,
            self.paused,
            self.error.as_deref(),
        );
        if std::mem::take(&mut self.debug.export_performance) {
            // Snapshot before the save dialog, which intentionally blocks UI.
            let mut report = serde_json::json!({"format":"blotworx-performance-v1",
                "scope":"Application callbacks and CPU/wait costs; not display presents or GPU occupancy. Nested/overlapping phases must not be added.",
                "level":self.world.as_ref().map(|world|&world.report.level),"paused":self.paused,"error":self.error,
                "player_position":self.controller.as_ref().map(|controller|controller.player.position),
                "tick":self.controller.as_ref().map(|controller|controller.ticks),
                "native_contact_probe_enabled":self.debug.native_contact_probe_enabled(),
                "callbacks":self.debug.frame_timing_summary(),"cpu_phases":self.frame_phases,
                "scene_render_work":&*self.camera.render_work.lock().unwrap()});
            #[cfg(feature = "blotworx-ui-render")]
            {
                report["hud"] = self
                    .hud
                    .as_ref()
                    .map(|hud| hud.performance_snapshot())
                    .unwrap_or_default();
            }
            if let Ok(executable) = std::env::current_exe().and_then(std::fs::read) {
                report["executable_sha256"] =
                    ring::digest::digest(&ring::digest::SHA256, &executable)
                        .as_ref()
                        .iter()
                        .map(|byte| format!("{byte:02x}"))
                        .collect::<String>()
                        .into();
            }
            if let Some(path) = crate::file_dialog::FileDialog::new()
                .add_filter("Performance report", &["json"])
                .set_file_name("blotworx-performance.json")
                .save_file()
            {
                let result = serde_json::to_vec_pretty(&report)
                    .map_err(anyhow::Error::from)
                    .and_then(|bytes| std::fs::write(path, bytes).map_err(anyhow::Error::from));
                if let Err(error) = result {
                    self.error = Some(format!("Save performance report: {error:#}"));
                }
            }
        }
        // Original Lua requests replace a world only after the current frame's
        // callbacks have returned. Reuse the same loader as manual selection.
        if self.receiver.is_none() && self.error.is_none() {
            if let Some(world) = &mut self.world {
                match world.checkpoint.take_restore() {
                    Ok(Some(snapshot)) => {
                        self.resume_snapshot(ui.ctx(), snapshot);
                        if self.receiver.is_some() {
                            self.paint_loading_screen(ui);
                        }
                        ui.ctx().request_repaint();
                        return;
                    }
                    Ok(None) => {}
                    Err(error) => {
                        self.error = Some(format!("Restore checkpoint: {error:#}"));
                        return;
                    }
                }
            }
            if let Some((level, clean)) = self.world.as_mut().and_then(|world| world.level_transition.take()) {
                if !self.multiplayer.request_party_level_transition(level.clone(), clean) {
                    self.level = level;
                    self.actor_focus = None;
                    self.preview_entry = None;
                    let mode = if clean { LevelLoad::Clean } else { LevelLoad::Transfer };
                    self.load_with_checkpoint(ui.ctx(), None, mode);
                    if self.receiver.is_some() {
                        self.paint_loading_screen(ui);
                    }
                    ui.ctx().request_repaint();
                    return;
                }
            }
        }
        // Exercise the interactive replacement path, including an existing
        // physics world and GPU cache, rather than testing fresh imports only.
        if self.capture.is_some()
            && self.receiver.is_none()
            && self.error.is_none()
            && self
                .controller
                .as_ref()
                .is_some_and(|controller| controller.ticks >= 2)
        {
            if let Some(level) = self.capture_switch_level.take() {
                // Exercise the shipped LoadLevel helper and native transition
                // queue, including session transfer, rather than UI selection.
                if let Some(world) = &mut self.world {
                    let destination = serde_json::to_string(&level).unwrap();
                    if let Err(error) = world.run_script(&format!("LoadLevel(GetPlayer(), {destination})")) {
                        self.error = Some(format!("Scripted level transition: {error:#}"));
                    }
                }
                ui.ctx().request_repaint();
                return;
            }
        }
        log::debug!(target:"blotworx_phase", "application frame end");
        self.capture_frame(ui, false);
        ui.ctx().request_repaint();
    }
}

#[cfg(not(target_os = "horizon"))]
impl Engine {
    #[cfg(feature = "blotworx-ui-render")]
    fn present_title(&mut self, ui: &mut egui::Ui) {
        if let Some(world) = &mut self.world {
            if world.movies.take_title() {
                log::info!(target: "blotworx_phase", "Loading title screen");
                match ui_movie::Title::load(world) {
                    Ok(title) => {
                        self.title = Some(title);
                        log::info!(target: "blotworx_phase", "Title screen ready");
                    }
                    Err(error) => {
                        log::error!("Title screen failed: {error:#}");
                        self.error = Some(format!("Title screen: {error:#}"));
                    }
                }
            }
        }
        if self.main_menu.is_some() {
            self.present_main_menu(ui);
            return;
        }
        let Some(title) = &mut self.title else { return; };
        let accept = if self.capture.is_some() {
            self.input_playback.as_ref().and_then(|playback| playback.frames.get(title.tick as usize))
                .is_some_and(|frame| frame.interact_pressed)
        } else {
            let buttons = input::poll().buttons;
            let pressed = buttons & !self.previous_buttons & 0x1010 != 0;
            self.previous_buttons = buttons;
            pressed || ui.input(|input| input.key_pressed(egui::Key::Enter) || input.key_pressed(egui::Key::Space)
                || (cfg!(target_os = "android") && input.pointer.any_click()))
        };
        match title.show(ui, accept) {
            Ok(true) => {
                let directory = self.save_directory.clone().map(Ok).unwrap_or_else(save_service::default_directory);
                let title_report = self.capture.is_some().then(|| title.summary());
                match directory.and_then(|directory| {
                    // Finish gameplay writes before the menu reads or deletes a slot.
                    if let Some(world) = &self.world {
                        if let Some(service) = &world.checkpoint.service {
                            service.flush(world.frame_clock.hardware_sample(), world.frame_clock.clock.rate.ticks_per_second.get())?;
                        }
                    }
                    ui_movie::Menu::load(std::path::Path::new(&self.root), directory, title.tick, title_report)
                }) {
                    Ok(menu) => {
                        self.main_menu = Some(menu);
                        self.title = None;
                        ui.ctx().request_repaint();
                        return;
                    }
                    Err(error) => self.error = Some(format!("Main menu: {error:#}")),
                }
            }
            Ok(_) => {}
            Err(error) => self.error = Some(format!("Title screen: {error:#}")),
        }
        let capture = self.main_menu.is_none() && self.capture_movie_frame.is_none()
            && title.tick >= self.capture_tick.unwrap_or(120);
        self.draw_options(ui.ctx());
        self.capture_frame(ui, capture);
    }

    #[cfg(feature = "blotworx-ui-render")]
    fn present_main_menu(&mut self, ui: &mut egui::Ui) {
        let menu = self.main_menu.as_mut().unwrap();
        let title_tick = menu.start_tick;
        let tick = title_tick + (menu.elapsed * 30.) as u64;
        let replay = self.input_playback.as_ref().and_then(|playback| playback.frames.get(tick as usize)).copied();
        let pad = input::poll();
        let buttons = pad.buttons;
        let edges = buttons & !self.previous_buttons;
        let released = self.previous_buttons & !buttons;
        self.previous_buttons = buttons;
        let pointer_stick = if pad.left == [0.; 2] { pad.right } else { pad.left };
        match menu.show(ui, edges, released, pointer_stick, replay) {
            Ok(Some(slot)) => {
                if let Err(error) = self.open_menu_slot(ui.ctx(), slot) {
                    self.error = Some(format!("Open save slot: {error:#}"));
                }
            }
            Ok(None) => {}
            Err(error) => self.error = Some(format!("Main menu: {error:#}")),
        }
        if self.receiver.is_some() {
            return;
        }
        self.draw_options(ui.ctx());
        self.capture_frame(ui, self.capture_movie_frame.is_none() && tick >= self.capture_tick.unwrap_or(title_tick + 160));
    }

    #[cfg(feature = "blotworx-ui-render")]
    fn open_menu_slot(&mut self, ctx: &egui::Context, slot: usize) -> Result<()> {
        let directory = self.save_directory.clone().map(Ok).unwrap_or_else(save_service::default_directory)?;
        let root = assets::files_root(std::path::Path::new(&self.root));
        let state = match save_service::Service::read_checkpoint(&directory, slot) {
            Ok(snapshot) => Some(snapshot),
            Err(error) if error.downcast_ref::<std::io::Error>().is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) => None,
            Err(error) => return Err(error),
        };
        let completed = state.as_ref().map(|save| save.completed_playthrough()).transpose()?.unwrap_or(false);
        let level = if let Some(snapshot) = state.as_ref().filter(|_| !completed) {
            snapshot.level()?
        } else {
            endgame::campaign_start(&root)?
        };
        let world = self.world.as_mut().context("Main menu world is unavailable")?;
        let now = world.frame_clock.hardware_sample();
        if let Some(service) = &world.checkpoint.service {
            service.flush(now, world.frame_clock.clock.rate.ticks_per_second.get())?;
        }
        let service = save_service::Service::open(directory, slot, now)?;
        world.checkpoint = checkpoint::State { service: Some(service), ..Default::default() };
        self.save_slot = slot;
        if let Some(snapshot) = state {
            let resume = if completed { score_state::CampaignResume::NextPlaythrough } else { score_state::CampaignResume::Checkpoint };
            self.load_campaign_slot(ctx, snapshot, level, resume);
        } else {
            self.level = level;
            self.preview_entry = None;
            self.actor_focus = None;
            self.load_with_checkpoint(ctx, None, LevelLoad::NewCampaign);
        }
        Ok(())
    }

    fn paint_loading_screen(&mut self, ui: &mut egui::Ui) {
        if let Some(screen) = &mut self.loading_screen {
            screen.paint(ui);
            if self.capture.is_none() {
                loading_screen::report_button(ui.ctx(), Some(&self.level), self.error.as_deref(), Some(screen.elapsed()));
            }
        }
    }

    fn capture_frame(&mut self, ui: &mut egui::Ui, force: bool) {
        if self.capture.is_some()
            && (force
                || self.error.is_some()
                || (self.capture_movie_frame.is_none()
                    && self.playback_pause.is_none_or(|(_, remaining)| remaining <= 1)
                    && self.capture_tick.map_or(self.loaded_frames > 90, |tick| {
                        self.controller.as_ref().is_some_and(|c| c.ticks >= tick
                            && (self.input_playback.as_ref().and_then(|p| p.frames.get(c.ticks as usize))
                                .is_none_or(|frame| frame.pause_frames == 0)
                                || self.playback_pause.is_some_and(|(opened, _)| opened == c.ticks)))
                    })))
            && !self.capture_requested
        {
            self.capture_requested = true;
            let diagnostic_timing = if self.diagnostic_checkpoint_timing && self.error.is_none() {
                self.world.as_mut().map(|world| {
                    let mut header=vec![0;save_game::HEADER_SIZE];
                    save_game::initialize_header(&mut header).and_then(|_|world.capture_owned_checkpoint_timing(&mut header))
                        .map(|timing|serde_json::json!({"timing":timing.summary(),"mutated_actual_timer":true,"complete_checkpoint":false}))
                        .unwrap_or_else(|error|serde_json::json!({"error":format!("{error:#}"),"complete_checkpoint":false}))
                })
            } else {
                None
            };
            if let (Some(path), Some(world)) = (&self.capture, &self.world) {
                // Memory accounting walks the live world, so capture it only
                // when producing an explicit diagnostic artifact. Normal
                // gameplay and frame pacing do not pay this traversal cost.
                let (geometry, images, collision, prefab_sources, inactive_scenes) =
                    debug_view::tracked_asset_bytes(world);
                let (entity_heap_bytes, component_heap_bytes, property_heap_bytes) =
                    debug_view::runtime_entity_heap_bytes(world);
                let (working_bytes, private_bytes) = debug_view::process_memory_bytes()
                    .map_or((None, None), |(working, private)| {
                        (Some(working), Some(private))
                    });
                let box3d_bytes = physics::box3d_allocated_bytes();
                let gl_asset_payload_bytes =
                    crate::render_support::viewport3d::cached_scene_asset_payload_bytes();
                let rust_heap = crate::memory_profile::snapshot();
                let process_heaps = debug_view::process_heap_bytes();
                let (actor_model_count, actor_model_bytes) =
                    world.authored_havok.cached_model_payload_summary();
                let (skin_model_count, skin_bytes) = world.skins.iter().fold(
                    (0usize, 0usize),
                    |(models, bytes), (_, skins)| {
                        (
                            models + 1,
                            bytes.saturating_add(
                                skins
                                    .iter()
                                    .map(skin::owned_buffer_bytes)
                                    .sum::<usize>(),
                            ),
                        )
                    },
                );
                let jigsaw_asset_count = world.jigsaws.values().map(Vec::len).sum::<usize>();
                let jigsaw_payload_bytes = world
                    .jigsaws
                    .values()
                    .flatten()
                    .map(jigsaw::Asset::owned_payload_bytes)
                    .sum::<usize>();
                let (source_collision_shape_count, source_collision_bytes) =
                    world.retained_source_collision_payload();
                let mut report = serde_json::json!({
                    "level":world.report.level,"ticks":self.controller.as_ref().map(|controller|controller.ticks),
                    "preview_runtime":self.preview_runtime,"runtime_error":self.error,
                    "game_random":world.game_random,
                    "deferred_entity_events":world.entity_event_queue.entries(),
                    "ai_manager_gate":world.ai_manager_enabled,
                    "render_options":world.render_options,
                    "nif_camera":world.nif_camera,
                    "nif_camera_frame":world.active_nif_camera_frame().ok().flatten(),

                    "nif_camera_transition":world.nif_camera_transition,
                    "nif_camera_native_projection_parity":false,
                    "ledge_controls":world.ledge_controls,
                    "ledge_detection_and_hanging_implemented":true,
                    "silhouette_pass_implemented":false,
                    "silhouette_candidates":world.render_objects.values().filter(|object|world.render_options.admits_silhouette(object.flags)).count(),
                    "counter_materials":world.counter_materials,
                    "paint_counters":world.paint_counters,
                    "audio_output_preview":{"submitted":world.audio_playback.submitted,"controlled_pitch_submitted":world.audio_playback.controlled_pitch_submitted,"completed":world.audio_playback.completed,"stop_actions":world.audio_playback.stop_actions,"stopped_voices":world.audio_playback.stopped_voices,
                        "rejected_post_count":world.audio_playback.rejected_post_count,
                        "rejected_posts":world.audio_playback.rejected_posts.values().collect::<Vec<_>>(),
                        "parameter_controls_prepared":world.audio_playback.parameter_controls_prepared,
                        "audio_tick":world.audio_playback.audio_tick(),"pending_actions":world.audio_playback.pending_audio_actions(),"full_game_mixer":false},
                    "script_errors":world.report.unsupported.iter().filter(|message|
                        message.starts_with("Game Lua:") || message.starts_with("Game script ") ||
                        message.starts_with("Preview recovery:") || message.starts_with("Authored IO ")).collect::<Vec<_>>(),
                    "fade":{"out":world.screen_fade.out,"opacity":world.screen_fade.opacity,
                        "busy":world.screen_fade.busy(),"held":world.screen_fade.held,
                        "deferred_updates":world.screen_fade.deferred_updates,
                        "restoring_hud":world.screen_fade.restoring_hud(),
                        "suppress_mapping_restore":world.screen_fade.suppress_mapping_restore,
                        "native_mapping_restore_implemented":false},
                    "spawners":world.spawners,
                    "component_control":world.component_control,
                    "input_restricted":world.cutscene_input.restricted,
                    "authored_havok_actor_count":world.authored_havok.len(),
                    "authored_havok_errors":world.report.unsupported.iter().filter(|message|message.starts_with("Authored Havok actor")).collect::<Vec<_>>(),
                    "animation_export_errors":world.report.unsupported.iter().filter(|message|
                        message.contains(" animation export ") || message.contains("no mapped native registration")).collect::<Vec<_>>(),
                    "animation_export_count":world.controller_feedback.exported_events,
                    "child_instance_count":world.report.geometry.iter().filter(|record|record["kind"]=="child_template_instance").count(),
                    "skeletal_child_count":world.transform_hierarchy.values().filter(|local|local.socket.is_some()).count(),
                    "child_errors":world.report.unsupported.iter().filter(|message|message.starts_with("Child instantiation ") || message.starts_with("Skeletal child ")).collect::<Vec<_>>(),
                    "scenes":world.report.scenes,"entity_count":world.entities.len(),
                    "pending_scene_slots":world.scene_requests.occupied(),
                    "guardians":{"enabled":world.guardians.enabled,"visible":world.guardians.visible,
                        "dirty":world.guardians.dirty,"owned_count":world.guardian_entities.entities().len(),
                        "audio_tick":world.guardian_audio.schedule.tick(),"pending_audio_actions":world.guardian_audio.schedule.pending()}
                });
                report["guardians"]["pending_points"] = serde_json::json!(world.guardians.pending_points);
                report["demo_npc_fallback"] = serde_json::to_value(&world.demo_story.npc_fallback).unwrap_or_default();
                report["memory"] = serde_json::json!({
                    "tracked_cpu_asset_bytes":{
                        "geometry":geometry,"images":images,"collision":collision,
                        "prefab_sources":prefab_sources,
                        "total":geometry.saturating_add(images).saturating_add(collision).saturating_add(prefab_sources)
                    },
                    "inactive_scene_render_and_collision_bytes":inactive_scenes,
                    "compressed_inactive_scene_asset_bytes":world
                        .cold_scene_assets
                        .values()
                        .map(assets::ColdSceneAssets::compressed_bytes)
                        .sum::<usize>(),
                    "runtime_entity_identity_bytes":entity_heap_bytes,
                    "runtime_component_storage_bytes":component_heap_bytes,
                    "runtime_component_property_bytes":property_heap_bytes,
                    "box3d_live_allocation_bytes":box3d_bytes,
                    "gl_cached_scene_asset_payload_bytes":gl_asset_payload_bytes,
                    "gl_opaque_texture_savings_bytes":crate::render_support::viewport3d::cached_opaque_texture_savings_bytes(),
                    "legacy_shadow_target_allocated":crate::render_support::viewport3d::legacy_shadow_target_allocated(),
                    "rust_allocator_live_bytes":rust_heap.map(|(live, _)|live),
                    "rust_allocator_peak_bytes":rust_heap.map(|(_, peak)|peak),
                    "startup_entity_payload_compacted_bytes":world.startup_entity_payload_compacted_bytes,
                    "player_animation_cache":world.player_animations.memory_summary(),
                    "authored_actor_model_cache":{"model_count":actor_model_count,"owned_cpu_bytes":actor_model_bytes},
                    "authored_actor_animation_cache":world.authored_havok.animation_payload_summary(),
                    "cooked_skin_cache":{"model_count":skin_model_count,"owned_cpu_bytes":skin_bytes},
                    "jigsaw_collision_cache":{"asset_count":jigsaw_asset_count,"owned_cpu_bytes":jigsaw_payload_bytes},
                    "retained_dynamic_and_phantom_collision_source":{"shape_count":source_collision_shape_count,"owned_cpu_bytes":source_collision_bytes},
                    "startup_unused_texture_bytes_released":world.startup_texture_cache_pruned_bytes,
                    "script_runtime":world.scripts.memory_summary(),
                    "process_heap_container_count":process_heaps.as_ref().map(|heaps|heaps.containers),
                    "process_heap_allocated_bytes":process_heaps.as_ref().map(|heaps|heaps.allocated),
                    "process_heap_committed_bytes":process_heaps.as_ref().map(|heaps|heaps.committed),
                    "process_heap_reserved_bytes":process_heaps.as_ref().and_then(|heaps|heaps.reserved),
                    "process_working_set_bytes":working_bytes,
                    "process_private_bytes":private_bytes,
                    "excludes":"GPU allocations, allocator overhead and most runtime state"
                });
                report["mickey_idle"] = serde_json::to_value(
                    self.authored_preview.as_ref().map(|preview| preview.idle),
                )
                .unwrap_or_default();
                report["player_knockback"] =
                    serde_json::to_value(&world.player_knockback).unwrap_or_default();
                report["player_depth_candidates"] = serde_json::json!(world
                    .render_objects
                    .iter()
                    .filter(|(_, object)| object.flags.is_player_depth_target())
                    .map(|(&(entity, node), _)| serde_json::json!({"entity":entity,"node":node}))
                    .collect::<Vec<_>>());
                report["player_depth_render_pass_connected"] = false.into();
                report["brush_cursor"] = world.brush_cursor_summary();
                report["demo_story"] = world.demo_story.summary();
                report["diagnostic_checkpoint_timing"] =
                    diagnostic_timing.unwrap_or(serde_json::Value::Null);
                report["map_info"] = serde_json::to_value(&world.map_info).unwrap_or_default();
                report["particle_asset_inspection"] = world
                    .particle_asset_preview
                    .as_ref()
                    .map(|preview| preview.summary())
                    .unwrap_or_default();
                report["named_effects"] = world.named_fx.summary();
                report["scene_particles"] = world.scene_particles.summary();
                report["material_animation"] = world.material_animation.summary();
                report["shadow_volumes"] = world.shadow_volumes.summary();
                report["input_presentation"] = serde_json::json!(&world.input_presentation);
                report["ai_disable_states"] = serde_json::json!(world.component_control.disabled.keys().filter_map(|&owner| {
                    let entity=&world.entities[owner];
                    entity.components.iter().any(|c|matches!(c.class_name.as_str(),"BasicAIControllerComponent"|"AIBrainComponent")).then(||
                        serde_json::json!({"entity":owner,"name":entity.name,
                            "controller":world.ai_component_disabled(owner,"BasicAIControllerComponent"),
                            "brain":world.ai_component_disabled(owner,"AIBrainComponent")}))
                }).collect::<Vec<_>>());
                report["ai_conversations"] = serde_json::json!(world.ai_brains.iter()
                    .filter(|(_,brain)|!brain.conversations.is_empty())
                    .map(|(&owner,brain)|serde_json::json!({"entity":owner,"name":world.entities[owner].name,
                        "states":brain.conversations,"current":brain.selector.current,
                        "selection_connected":false})).collect::<Vec<_>>());
                report["script_transformations"] = serde_json::json!(world.script_transformations);
                report["wall_walk"] = serde_json::json!({
                    "script_enabled": !world.wall_pushing_disabled.contains(&world.player_entity),
                    "controller_implemented": false,
                });
                report["script_transform_lod_resets"] = serde_json::json!(world.script_transform_lod_resets);
                report["parenting"] = serde_json::json!({
                    "changed_child_lists":world.live_child_slots.len(),
                    "forced_body_poses":world.parent_pose_forces,"max_body_position_error":world.parent_pose_error,
                    "dynamic_relations":world.dynamic_parent_bindings.iter().map(|((owner,component),parent)|
                        serde_json::json!({"entity":owner,"name":world.entities[*owner].name,"component":component,"parent":parent})).collect::<Vec<_>>()
                });
                report["logic_timers"] = serde_json::json!(world.logic_timers.iter().map(|(key,state)|serde_json::json!({"entity":key.0,"component":key.1,"name":world.entities[key.0].name,"state":state})).collect::<Vec<_>>());
                report["whole_model_paint"] = serde_json::json!(world.model_paint.iter().map(|(id,model)|serde_json::json!({"entity":id,"name":world.entities[*id].name,"painted":model.painted,"invulnerable":model.invulnerable,"meshes":model.states.len(),"sketch_meshes":model.states.keys().filter(|draw|world.sketch_draws.contains(draw)).count()})).collect::<Vec<_>>());
                report["jigsaw_paint"] = serde_json::json!(world.jigsaw_instances.iter().enumerate().map(|(id,instance)| {
                    let draws=instance.mesh_draws.values().flatten().copied().collect::<std::collections::BTreeSet<_>>();
                    serde_json::json!({"instance":id,"entity":instance.entity,"name":world.entities[instance.entity].name,
                        "node":instance.node,"pieces":world.entities[instance.entity].jigsaw_states[instance.node].painted,
                        "component_painted":world.entities[instance.entity].components.iter()
                            .find(|c|c.class_name=="PhysicsJigsawComponent")
                            .and_then(|c|c.properties.iter().find(|p|p.name=="Is Painted")).and_then(|p|p.value.as_bool()),
                        "draws":draws.len(),"transition_draws":instance.transition_states.len(),
                        "real":world.entities[instance.entity].components.iter()
                            .filter(|c|matches!(c.class_name.as_str(),"PhysicsRigidBodyStaticComponent"|"PhysicsRigidBodyDynamicComponent"))
                            .flat_map(|c|c.properties.iter()).find(|p|p.name=="Is Real").and_then(|p|p.value.as_bool()),
                        "sketch_draws":draws.iter().filter(|draw|world.sketch_draws.contains(draw)).count(),
                        "active_draws":draws.iter().filter(|draw|world.jigsaw_effect_values.contains_key(draw)).count()})
                }).collect::<Vec<_>>());
                report["model_shadow"] = world.model_shadow.summary();
                report["model_shadow_gpu"] =
                    serde_json::to_value(&*self.camera.model_shadow_report.lock().unwrap())
                        .unwrap_or_default();
                report["shadow_gpu"] =
                    serde_json::to_value(&*self.camera.volume_shadow_report.lock().unwrap())
                        .unwrap_or_default();
                report["scene_render_work"] =
                    serde_json::to_value(&*self.camera.render_work.lock().unwrap())
                        .unwrap_or_default();
                report["frame_cpu_phases"] =
                    serde_json::to_value(&self.frame_phases).unwrap_or_default();
                report["recent_frame_intervals"] = self.debug.frame_timing_summary();
                report["lighting_texture_work"] = serde_json::json!({"evaluated":world.spherical_properties.texture_evaluations,"reused_exact_coefficients":world.spherical_properties.coefficient_reuses});
                report["hud_submission_mode"] = serde_json::json!(if self.diagnostic_eager_hud {
                    "eager_pixel_readback_diagnostic"
                } else if self.diagnostic_serial_hud {
                    "serial_vm_preparation_diagnostic"
                } else {
                    "same_frame_scene_gpu_overlap"
                });
                report["view_culling_enabled"] =
                    serde_json::json!(!self.diagnostic_no_view_culling);
                report["interaction_prompt"] = serde_json::json!({
                    "texture_loaded": self.usable_button.is_some(),
                    "entity": self.usable_prompt_target.and_then(|target| world.entities.get(target))
                        .map(|entity| entity.name.as_str()),
                });
                #[cfg(feature = "blotworx-ui-render")]
                {
                    report["gameplay_hud"] = self
                        .hud
                        .as_ref()
                        .map(|hud| hud.summary())
                        .unwrap_or(serde_json::Value::Null);
                }
                report["player_spawn_override"] =
                    serde_json::to_value(&world.player_spawn).unwrap_or_default();
                report["nif_camera_completed_frame"] =
                    serde_json::to_value(&world.nif_camera_completed_frame).unwrap_or_default();
                report["application_camera_lens"] =
                    serde_json::to_value(&world.camera_lens).unwrap_or_default();
                report["multi_position_audio"] = serde_json::json!({"controllers":world.multi_position_audio.iter().map(|(&(owner,_),state)|serde_json::json!({"owner":owner,"name":world.entities[owner].name,"state":state,
                    "backend_positions":multi_position_audio::backend_positions(&state.positions)})).collect::<Vec<_>>(),
                    "spatial_backend_connected":false,"effect_ramping_connected":false,"trigger_plane_subscriptions_connected":false,"native_update_order_verified":false});
                report["audio_output_preview"]["spatial_post_rejections"] =
                    serde_json::json!(world.audio_playback.spatial_post_rejections);
                report["audio_output_preview"]["game_post_requests"] =
                    serde_json::json!(world.audio_playback.game_post_requests);
                report["audio_output_preview"]["switch_controls"] = world
                    .audio_playback
                    .switch_summary()
                    .unwrap_or_else(|error| serde_json::json!({"error":format!("{error:#}")}));
                report["audio_output_preview"]["event_flow"] = world.audio_playback.event_summary();
                report["audio_output_preview"]["music"] = world.audio_playback.music_summary();
                report["animation_pushed_emitters"] =
                    serde_json::json!(world.controller_feedback.pushed_emitters);
                report["audio_output_preview"]["registered_banks"] =
                    serde_json::json!(world.audio_playback.registered_banks());
                report["audio_output_preview"]["spatial_position_requests"] =
                    serde_json::json!(world.audio_playback.spatial_position_requests);
                report["player_ability_checkpoint_records"] = match world
                    .player_ability_save_summary()
                {
                    Ok(records) => {
                        serde_json::json!({"records":records,"registry":world.player_ability_registry,
                        "scope":"Native registration order/common ownership; derived action initialization and full checkpoint pending"})
                    }
                    Err(error) => serde_json::json!({"error":format!("{error:#}")}),
                };
                let catalog = &world.camera_profiles;
                report["camera_feedback"] =
                    serde_json::to_value(&world.camera_feedback).unwrap_or(serde_json::Value::Null);
                report["camera_feedback_target"] = serde_json::json!({"subscribed":world.camera_target.subscribed,"resolved":world.camera_target.resolved_target});
                report["extras"] = world.extras.summary();
                report["collectibles"] = world.collectibles.summary();
                report["quests"] = world.quests.summary();
                report["game_options"] = world.game_options.summary();
                report["playthrough"] = serde_json::to_value(world.playthrough).unwrap_or_default();
                report["playtime_owner"] = world.playtime.summary();
                report["cinematic"] = self
                    .movie
                    .as_ref()
                    .map(movie::Player::summary)
                    .or_else(|| self.last_movie.clone())
                    .into();
                #[cfg(feature = "blotworx-ui-render")]
                if let Some(title) = &self.title {
                    report["title_screen"] = title.summary();
                }
                #[cfg(feature = "blotworx-ui-render")]
                if let Some(menu) = &self.main_menu {
                    if let Some(title) = &menu.title_report {
                        report["title_screen"] = title.clone();
                    }
                    report["main_menu"] = menu.summary();
                }
                report["robot_arm_idle_requests"] =
                    serde_json::to_value(&world.robot_arm_idle_requests).unwrap();
                report["robot_arm_controls"] =
                    serde_json::to_value(&world.robot_arm_controls).unwrap();
                if let Some(owner) = self.actor_focus.as_ref().and_then(|name| {
                    world
                        .entities
                        .iter()
                        .position(|entity| &entity.name == name)
                }) {
                    report["focused_scene_particles"] = world.scene_particles.live.get(&owner).map(|effect|
                        serde_json::json!({"entity":owner,"name":world.entities[owner].name,
                            "enabled":effect.enabled,"scene_active":world.entity_scene_active(owner),
                            "hidden":world.hidden_entities.contains(&owner),"model":world.entities[owner].model_asset,
                            "entity_transform":world.entities[owner].transform,
                            "systems":effect.particles.render_probe()})).into();
                    report["actor_graph_snapshot"] = if let Some(actor) = world.actors.get(&owner) {
                        serde_json::json!({"kind":"Gamebryo","requested_sequence":actor.requested_id(),
                            "active_sequence":actor.active_id(),"idle":world.idle_actors.contains(&owner),
                            "transforms":actor.pose.as_ref().map(|pose|&pose.transforms)})
                    } else {
                        world.authored_havok.snapshot(owner)
                            .unwrap_or_else(|error|serde_json::json!({"error":format!("{error:#}")}))
                    };
                }
                report["movie_requests"] = world.movies.summary();
                report["shared_frame_clock"] = world.frame_clock.summary();
                report["checkpoint"] = world.checkpoint.summary();
                report["animation_variable_changes"] = world.animation_variable_changes.summary();
                report["instance_changes"] = world.instance_changes.summary();
                report["player_scalar_capture"] = player_scalar_capture(world);
                report["player_record_capture"] = player_record_capture(world);
                report["world_scene_capture"] =
                    world.scene_resources.summary(&world.scene_requests);
                report["level_resource"] = world.level_resource.summary();
                report["character_physics"] = world.character_physics_summary();
                report["countdown"] = world.countdown_snapshot();
                report["lua_world_transfer"] = world.report.geometry.iter().find(|record|record["kind"]=="lua_world_transfer").cloned().unwrap_or_default();
                report["pickup_updates"]=world.pickup_update_snapshot();
                report["entity_cleanup"]=world.cleanup_snapshot();
                report["death_rewards"]=serde_json::json!({
                    "dispatches":world.report.geometry.iter().filter(|record|record["kind"]=="death_reward_spawner_dispatch").collect::<Vec<_>>(),
                    "diagnostics":world.report.unsupported.iter().filter(|line|line.starts_with("Death reward") || line.starts_with("DeathRewardComponent")).collect::<Vec<_>>(),
                    "pickups":world.report.geometry.iter().filter(|record|record["kind"]=="death_reward_spawner_dispatch")
                        .filter_map(|record|record["spawned"].as_array()).flatten().filter_map(|id|id.as_u64()).map(|id|id as usize)
                        .filter(|entity|world.pickup_settings.contains_key(entity))
                        .map(|entity|serde_json::json!({"entity":entity,"name":world.entities[entity].name,
                            "position":&world.entities[entity].transform[12..15],
                            "parent":world.transform_hierarchy.get(&entity).map(|local|local.parent),
                            "collection_delay":world.pickup_update_states.get(&entity).map(|state|state.gate.duration),
                            "lifetime_remaining":world.runtime_lifetimes.get(&entity).map(|state|state.timer.remaining),
                            "attract_radius":world.pickup_settings[&entity].radius})).collect::<Vec<_>>(),
                    "attachments":world.report.geometry.iter().filter(|record|record["kind"]=="death_reward_attachment").collect::<Vec<_>>(),
                    "distributions":world.report.geometry.iter().filter(|record|record["kind"]=="death_reward_distribution").collect::<Vec<_>>(),
                    "dynamic_selection_connected":true,"selections":world.report.geometry.iter().filter(|record|record["kind"]=="dynamic_reward_selection").collect::<Vec<_>>(),"dynamic_selection":world.dynamic_reward_snapshot(),"native_factory_failure_reservations_verified":false,"distribution_attachment_connected":true,"native_attachment_placement_parity_verified":false
                });
                report["spawned_objects"] = serde_json::to_value(world.report.geometry.iter().filter(|record|record["kind"]=="spawned_object_clone").collect::<Vec<_>>()).unwrap_or_default();
                report["spawned_actor_states"] = serde_json::Value::Array(world.report.geometry.iter()
                    .filter(|record|record["kind"]=="spawned_object_clone")
                    .filter_map(|record|record["entity"].as_u64().map(|entity|entity as usize))
                    .filter(|&entity|world.authored_havok.contains(entity))
                    .map(|entity|match world.authored_havok.snapshot(entity) {
                        Ok(mut state)=>serde_json::json!({"entity":entity,"name":world.entities[entity].name,
                            "clips":state["clips"].take(),"machines":state["machines"].take()}),
                        Err(error)=>serde_json::json!({"entity":entity,"error":format!("{error:#}")}),
                    }).collect());
                report["npc_relocations"] = serde_json::to_value(&world.npc_relocations).unwrap();
                report["cutscene_actions"] = world.cutscene_actions.summary();
                report["deferred_event_runtime"] = serde_json::to_value(&world.deferred_event_runtime).unwrap_or_default();
                report["input_restriction_depth"] = serde_json::json!(world.cutscene_input.count);
                report["save_header_owners"] = {
                    let mut header = vec![0; save_game::HEADER_SIZE];
                    save_game::initialize_header(&mut header).and_then(|_|world.write_save_header_owners(&mut header))
                        .map(|_|serde_json::json!({"captured":["level_resource","map/tickets/collectible_totals","global_metadata","inventory_limits","EXTR","COLL","OPTN","MICK.playthrough"],
                            "global_metadata":checkpoint_metadata::Globals::from_header(&header).ok().map(|state|state.summary()),
                            "inventory_metadata":checkpoint_metadata::InventoryLimits::from_header(&header).ok().map(|state|state.summary()),
                            "metadata":checkpoint_metadata::Summary::from_header(&header).ok().map(|state|state.summary()),"options":save_options::State::from_header(&header).ok(),
                            "playthrough":score_state::Playthrough::from_header(&header).ok(),
                            "script_owner_payload_bytes":save_options::SCRIPT_SECTION.2.len(),"complete_checkpoint":false}))
                        .unwrap_or_else(|error|serde_json::json!({"error":format!("{error:#}"),"complete_checkpoint":false}))
                };
                report["physics_world"] = serde_json::json!(world.report.physics_world);
                report["attachment_modifiers"] = world.attachment_modifier_summary();
                report["rigid_layer_updates"] =
                    serde_json::to_value(&world.rigid_layer_updates).unwrap();
                report["presented_camera"] = serde_json::json!({
                    "source":if self.camera.camera_override.is_some(){"authored"}else{"viewport"},
                    "target":self.camera.camera_target,"yaw":self.camera.yaw,"pitch":self.camera.pitch,"distance":self.camera.distance,
                    "eye":self.camera.camera_override.as_ref().map(|camera|camera.eye).unwrap_or_else(|| {
                        let (sy,cy)=self.camera.yaw.sin_cos();let (sp,cp)=self.camera.pitch.sin_cos();
                        [self.camera.camera_target[0]+self.camera.distance*cy*cp,self.camera.camera_target[1]+self.camera.distance*sp,self.camera.camera_target[2]+self.camera.distance*sy*cp]
                    })});
                report["enhanced_light_registry"]=serde_json::to_value(&world.enhanced_light_registry).unwrap_or_default();
                report["user_graphics"] = serde_json::to_value(&self.preferences).unwrap_or_default();
                report["camera_inspection"] = serde_json::json!({"actor":self.actor_focus,
                    "capture_target":self.capture_camera_target,"manual_actor":self.live_actor,
                    "overrides_authored_view":self.actor_focus.is_some() || self.live_actor.is_some() || self.capture_camera_target.is_some()});
                report["source_data_root"] =
                    serde_json::to_value(world.source_data_root()).unwrap();
                report["ui_notifications"] = serde_json::to_value(&world.ui_notifications)
                    .unwrap_or(serde_json::Value::Null);
                report["gameplay_camera_targets"] =
                    serde_json::to_value(&world.gameplay_camera_targets).unwrap();
                report["gameplay_camera_pipeline"] = world.camera_pipeline.summary();
                report["side_scroller"] = serde_json::to_value(&world.side_scroller).unwrap();
                report["contact_stimuli"] = serde_json::to_value(&world.contact_stimuli).unwrap();
                report["motion_surface_normal"] = serde_json::to_value(
                    self.controller
                        .as_ref()
                        .and_then(|c| c.player.motion_surface_normal),
                )
                .unwrap();
                report["story_sequences"] = world.story_sequences.summary();
                report["player_status"] =
                    match entity_status::read(&world.entities[world.player_entity].components) {
                        Ok(invisible) => serde_json::json!({"invisible":invisible}),
                        Err(error) => serde_json::json!({"error":error.to_string()}),
                    };
                report["application_camera_frame"] = match world.published_camera_frame() {
                    Ok(frame) => serde_json::to_value(frame).unwrap(),
                    Err(error) => serde_json::json!({"error":format!("{error:#}")}),
                };
                report["camera_profile_catalog"] = serde_json::json!({
                    "controller_update_connected":false,"mode":catalog.controller.mode,
                    "constructor_current":0,
                    "profile_selection":{"current":catalog.controller.selection.current,"previous":catalog.controller.selection.previous,
                        "duration":catalog.controller.selection.duration,"elapsed":catalog.controller.selection.elapsed,"weight":catalog.controller.selection.weight},
                    "custom_enabled":catalog.controller.enabled,
                    "target_subscription":{"subscribed":world.camera_target.subscribed,"target":world.camera_target.resolved_target},
                    "original_names":catalog.controller.originals,"custom_names":catalog.controller.customs,
                    "profiles":catalog.controller.library.profiles.iter().zip(&catalog.extras).map(|(profile,extra)|serde_json::json!({
                        "name":profile.name,"kind":extra.kind,"scalar_properties":profile.properties,
                        "frame_yaw":extra.frame_yaw,"frame_pitch":extra.frame_pitch,
                        "flags":extra.flags,"cached_distance":extra.cached_distance,"target":extra.target})).collect::<Vec<_>>(),
                    "properties":catalog.controller.library.properties.iter().map(|p|serde_json::json!({
                        "text":p.text,"value":p.value,"minimum":p.minimum,"maximum":p.maximum})).collect::<Vec<_>>(),
                    "flags":catalog.flags.iter().map(|p|serde_json::json!({"text":p.text,"value":p.value})).collect::<Vec<_>>()});
                if let Err(error) = std::fs::write(
                    path.with_extension("runtime.json"),
                    serde_json::to_vec_pretty(&report).unwrap(),
                ) {
                    eprintln!("Could not save capture runtime report: {error}");
                }
            }
            if self.diagnostic_hose.is_some() {
                if let Some(world) = &self.world {
                    eprintln!(
                        "Decal preview: projected={} visible={} rejected={} empty={}",
                        self.splat_preview.projected,
                        self.splat_preview.visible(),
                        self.splat_preview.rejected,
                        self.splat_preview.empty
                    );
                    let trace = &world.projectile_preview;
                    eprintln!(
                        "Brush diagnostic: shots={} contacts={} expired={} active={}",
                        trace.emitted,
                        trace.contacts,
                        trace.expired,
                        trace.pool.entries.iter().filter(|p| p.active).count()
                    );
                    if let Some(contact) = &trace.last_contact {
                        eprintln!(
                            "Brush diagnostic impact: {:?} position={:?} normal={:?} owner={}",
                            contact.ink,
                            contact.hit.position,
                            contact.hit.normal,
                            contact.hit.owner
                        );
                    }
                }
            }
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Screenshot(egui::UserData::default()));
        }
        for event in ui.input(|i| i.events.clone()) {
            if let egui::Event::Screenshot { image, .. } = event {
                if let Some(path) = self.capture.take() {
                    // Preserve the opaque game window's RGB, not material
                    // alpha left in its framebuffer by native game draws.
                    let rgb: Vec<u8> = image.pixels.iter().flat_map(|p| [p.r(),p.g(),p.b()]).collect();
                    let result = image::save_buffer(
                        &path,
                        &rgb,
                        image.size[0] as u32,
                        image.size[1] as u32,
                        image::ColorType::Rgb8,
                    )
                    .map_err(|e| format!("Could not save capture {}: {e}", path.display()))
                    .and_then(|()| {
                        let Some(playback) = &self.input_playback else {
                            return Ok(());
                        };
                        let Some(trace) = self.playback_trace.take() else {
                            return Err(self.playback_trace_error.clone().unwrap_or_else(|| {
                                "Playback trace writer was unavailable".to_owned()
                            }));
                        };
                        trace
                            .finish(playback)
                            .map(|_| ())
                            .map_err(|error| format!("Could not save playback trace: {error:#}"))
                    });
                    if let Err(e) = &result {
                        eprintln!("Capture failed: {e}");
                    }
                    if let Some(tx) = self.capture_result.take() {
                        let _ = tx.send(result);
                    }
                    ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                }
            }
        }
    }
}

#[cfg(debug_assertions)]
pub fn validate_live_inventory() -> anyhow::Result<()> {
    save_player::validate_live_inventory()
}

#[cfg(debug_assertions)]
pub fn validate_player_handoff() -> anyhow::Result<()> {
    level_session::validate_player_handoff()
}

#[cfg(debug_assertions)]
pub fn validate_streamed_triggers() -> anyhow::Result<()> {
    demo_triggers::validate_streamed_triggers()
}

#[cfg(debug_assertions)]
pub fn validate_directional_combat() -> anyhow::Result<()> {
    vulnerability_direction::validate_directional_combat()
}

#[cfg(debug_assertions)]
pub fn validate_completed_spawner() -> anyhow::Result<()> {
    spawner_schedule::validate_completed_spawner()
}

#[cfg(debug_assertions)]
pub fn validate_relay() -> anyhow::Result<()> {
    logic_relay::validate_relay()
}

#[cfg(debug_assertions)]
pub fn validate_demo_progression() -> anyhow::Result<()> {
    visibility_children::validate_instructions()?;
    particle_mesh::validate_original_source_streams()?;
    particle_mesh::validate_center()?;
    brush_particles::validate_radial_fields()?;
    brush_particles::validate_controller_time()?;
    effect_runtime::validate_original_animation_emitter()?;
    stairs::validate_instructions()?;
    physics::validate_stairs_context()?;
    println!("PASS: original ground wrapper:480 stairs admissions,108 force and144 reprojection exact-bit cases; no double upward velocity or fabricated support");
    knockback::validate_instructions()?;
    demo_ai::tests::original_steam_callback_pushes_live_player_toward_its_marker_without_damage()?;
    mickey_idle::validate_original_fidgets()?;
    robot_arm::validate_original_idle()?;
    movie::validate_original_headers()?;
    rotator_runtime::validate_original_commands()?;
    nif_properties::validate_original_portal()?;
    demo_triggers::validate_original_pickups_and_effects()?;
    trigger_damage::tests::original_hazard_arrays_reach_the_live_player_on_the_first_overlap()?;
    demo_story::validate_progression()
}

#[cfg(debug_assertions)]
pub fn validate_idle_command()->anyhow::Result<()> {mickey_idle::validate_idle_command()}

#[cfg(debug_assertions)]
pub fn validate_io_admission()->anyhow::Result<()> {io_admission::validate()}

#[cfg(debug_assertions)]
pub fn validate_model_billboards()->anyhow::Result<()> {model_billboard::validate()}

#[cfg(debug_assertions)]
pub fn validate_wii_morph_mesh()->anyhow::Result<()> {wii_morph_validation::validate()}

#[cfg(debug_assertions)]
pub fn validate_usable_runtime()->anyhow::Result<()> {usable_runtime_validation::validate()}

#[cfg(debug_assertions)]
pub fn validate_random_dialogue()->anyhow::Result<()> {demo_story::validate_random_dialogue()}

#[cfg(debug_assertions)]
pub fn validate_streamed_story()->anyhow::Result<()> {assets::validate_streamed_story()}

#[cfg(debug_assertions)]
pub fn validate_story_teardown()->anyhow::Result<()> {demo_story::validate_story_teardown()}

#[cfg(debug_assertions)]
pub fn validate_story_rewards()->anyhow::Result<()> {story_rewards::validate_story_rewards()}
#[cfg(debug_assertions)]
pub fn validate_conversation_clones()->anyhow::Result<()> {assets::template_spawner::validate_conversation_clones()}
#[cfg(debug_assertions)]
pub fn validate_airlock_spawns()->anyhow::Result<()> {assets::template_spawner::validate_airlock_spawns()}

#[cfg(debug_assertions)]
pub fn validate_palette_root()->anyhow::Result<()> {assets::validate_palette_root()}

#[cfg(debug_assertions)]
pub fn validate_brush_puzzle() -> anyhow::Result<()> { demo_story::validate_brush_puzzle() }

#[cfg(debug_assertions)]
pub fn validate_gv_encounter()->anyhow::Result<()> {demo_story::validate_gv_encounter()}

#[cfg(debug_assertions)]
pub fn validate_owned_cleanup()->anyhow::Result<()> {assets::validate_owned_cleanup()}

#[cfg(debug_assertions)]
pub fn validate_inactive_motion()->anyhow::Result<()> {demo_gameplay::validate_inactive_motion()}

#[cfg(debug_assertions)]
pub fn validate_spline_events()->anyhow::Result<()> {demo_gameplay::validate_spline_events()}
mod ai_graph_io;
mod file_request_status;
mod ai_graph_admission;
mod ai_graph_linking;
mod ai_graph_link_passes;
mod ai_graph_rings;
mod ai_graph_memory_queries;
mod ai_actor_queries;
mod ai_segment_query;
mod ai_region_geometry;
mod ai_region_segment;
mod ai_edge_crossing;
mod ai_path_construction;
mod ai_spatial_queries;
mod ai_traversal_cost;
mod ai_traversal_exclusion;
mod ai_spatial_walk;
mod ai_cell_obstacles;
mod ai_radius_segment;
mod ai_cell_crossing;
mod ai_cell_trace;
mod ai_navigation_sweep;
mod ai_spatial_service;
mod ai_spatial_acceptance;
mod ai_spatial_construction;
mod ai_spatial_modifier;
mod ai_spatial_definition_binding;
mod ai_pathfinder_spatial_binding;
mod ai_pathfinder_spatial_construction;
mod ai_cell_search;

/// Read-only bootstrap Lua export audit. Binding does not establish parity.
pub fn audit_native_lua_bindings() -> anyhow::Result<serde_json::Value> {
    actor_scripts::Scripts::native_binding_audit()
}

mod pc_spring;

mod em2_ground_plane;

mod dialogue_panel;
mod em2_dialogue;
