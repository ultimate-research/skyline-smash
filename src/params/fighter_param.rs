#[repr(C)]
#[derive(Debug)]
pub struct FighterParamTable {
    pub fighter_kind: u64,
    pub walk_accel_mul: f32,
    pub walk_accel_add: f32,
    pub walk_speed_max: f32,
    pub walk_slow_speed_mul: f32,
    pub walk_middle_ratio: f32,
    pub walk_fast_ratio: f32,
    pub ground_brake: f32,
    pub dash_speed: f32,
    pub run_accel_mul: f32,
    pub run_accel_add: f32,
    pub run_speed_max: f32,
    pub jump_squat_frame: i32,
    pub jump_speed_x: f32,
    pub jump_speed_x_mul: f32,
    pub jump_speed_x_max: f32,
    pub jump_aerial_speed_x_mul: f32,
    pub jump_initial_y: f32,
    pub jump_y: f32,
    pub mini_jump_y: f32,
    pub jump_aerial_y: f32,
    pub air_accel_x_mul: f32,
    pub air_accel_x_add: f32,
    pub air_speed_x_stable: f32,
    pub air_brake_x: f32,
    pub air_accel_y: f32,
    pub air_speed_y_stable: f32,
    pub damage_fly_top_air_accel_y: f32,
    pub damage_fly_top_speed_y_stable: f32,
    pub air_brake_y: f32,
    pub dive_speed_y: f32,
    pub weight: f32,
    pub air_ground_speed_brake: f32,
    pub landing_attack_air_frame_n: f32,
    pub landing_attack_air_frame_f: f32,
    pub landing_attack_air_frame_b: f32,
    pub landing_attack_air_frame_hi: f32,
    pub landing_attack_air_frame_lw: f32,
    pub landing_frame_light: f32,
    pub landing_frame: f32,
    pub landing_heavy_frame: i32,
    pub scale: f32,
    pub shield_radius: f32,
    pub shield_break_y: f32,
    pub guard_speed_limit: f32,
    pub jostle_front: f32,
    pub jostle_back: f32,
    pub jostle_weight: f32,
    pub cliff_jump_x_speed: f32,
    pub cliff_jump_y: f32,
    pub attack_combo_max: i32,
    pub s3_combo_max: i32,
    pub s4_combo_max: i32,
    pub combo_attack_12_end: f32,
    pub combo_attack_13_end: f32,
    pub combo_attack_32_end: f32,
    pub combo_attack_33_end: f32,
    pub combo_attack_42_end: f32,
    pub attack100_type: bool,
    pub attack_100_enable_cnt: i32,
    pub attack_100_rebound_distance: f32,
    pub attack_100_rebound_count: i32,
    pub attack_lw3_rebound_distance: f32,
    pub attack_lw3_rebound_count: i32,
    pub attack_s4_smash_hold_attack_up: f32,
    pub attack_s4_hold_frame: i32,
    pub attack_s4_hold_keep_frame: i32,
    pub attack_hi4_smash_hold_attack_up: f32,
    pub attack_hi4_hold_frame: i32,
    pub attack_hi4_hold_keep_frame: i32,
    pub attack_lw4_smash_hold_attack_up: f32,
    pub attack_lw4_hold_frame: i32,
    pub attack_lw4_hold_keep_frame: i32,
    pub attack_4_hold_scale: f32,
    pub jump_count_max: i32,
    pub squat_walk_type: bool,
    pub wall_jump_type: bool,
    pub attach_wall_type: bool,
    pub x1d633fc2eb: f32,
    pub air_lasso_type: bool,
    pub x2ea659cf56: bool,
    pub x271984ee09: f32,
    pub x2bf4bef265: f32,
    pub tread_jump_speed_y_mul: f32,
    pub tread_mini_jump_speed_y_mul: f32,
    pub tread_fall_recovery_frame: i32,
    pub tread_range_offset_y: f32,
    pub tread_range_w: f32,
    pub tread_range_h: f32,
    pub tread_range_add_w: f32,
    pub tread_range_add_h: f32,
    pub entry_stand_scale: f32,
    pub cursor_offset_y: f32,
    pub dead_rare_se_rate: i32,
    pub special_s_brake: f32,
    pub wind_resistance: f32,
    pub height: f32,
    pub expand_height: f32,
    pub result_scale_mul: f32,
    pub bury_r_depth_node: u64,
    pub bury_r_x_offset: f32,
    pub bury_r_x_offset_robot: f32,
    pub bury_r_y_offset: f32,
    pub bury_r_rot_angle: f32,
    pub slope_top_angle_max: f32,
    pub slope_front_up_max: f32,
    pub slope_back_up_max: f32,
    pub slope_loose_angle: f32,
    pub passive_wall_x_speed: f32,
    pub passive_wall_jump_x_speed: f32,
    pub passive_wall_jump_y_speed: f32,
    pub wall_jump_enable_x_speed: f32,
    pub passive_ceil_x_speed: f32,
    pub damage_model_scale_x: f32,
    pub is_blend_motion_kind: f32,
    pub damage_model_scale_z: f32,
    pub damage_model_no_hit_frame: i32,
    pub se_pitch_big: f32,
    pub se_pitch_small: f32,
    pub se_pitch_big_mushroom: f32,
    pub se_pitch_small_mushroom: f32,
    pub se_pitch_big_mushd: f32,
    pub se_pitch_small_mushd: f32,
    pub se_pitch_big_thunder: f32,
    pub se_pitch_small_thunder: f32,
    pub final_start_turn_frame: i32,
    pub scroll_rate_change_frame: i32,
    pub scroll_rate: f32,
    pub x13e3bcfb18: bool,
    pub x18f0607a99: bool,
    pub x1408832504: bool,
    pub dead_end_final: bool,
    pub final_end_jump_y: f32,
    pub charge_final_attack_mul: f32,
    pub charge_final_attack_reaction_mul: f32,
    pub star_scale: f32,
    pub star_attack_power: i32,
    pub egg_scale: f32,
    pub egg_collision_offset_1_x: f32,
    pub egg_collision_offset_1_y: f32,
    pub egg_collision_offset_1_z: f32,
    pub egg_collision_offset_2_x: f32,
    pub egg_collision_offset_2_y: f32,
    pub egg_collision_offset_2_z: f32,
    pub egg_collision_size: f32,
    pub bitten_wario_adjust_scale_node: u64,
    pub bitten_wario_adjust_scale_value: f32,
    pub palutena_autoaimbullet_target_node: u64,
    pub palutena_autoaimbullet_target_offset_x: f32,
    pub palutena_autoaimbullet_target_offset_y: f32,
    pub palutena_autoaimbullet_target_offset_z: f32,
    pub psychobreak_offset_y: f32,
    pub damage_fall_center_offset_y: f32,
    pub zelda_triforce_scale: f32,
    pub zelda_triforce_offset_y: f32,
    pub ridley_stabbed_node: u64,
    pub ridley_stabbed_offset_x: f32,
    pub ridley_stabbed_offset_y: f32,
    pub ridley_stabbed_offset_z: f32,
    pub ridley_stabbed_rhombus_offset_y: f32,
    pub gamewatch_stockicon_scale: f32,
    pub packun_final_capture_offset_y: f32,
    pub packun_final_capture_scale: f32,
    pub x2ab2a23d0d: f32,
    pub x2a3504f64e: f32,
    pub jack_final_target_scale: f32,
    pub jack_final_target_rot_y: f32,
    pub jack_final_target_effect_offset_x: f32,
    pub jack_final_target_effect_offset_y: f32,
    pub jack_final_target_effect_offset_z: f32,
    pub buddy_final_target_offset_x: f32,
    pub buddy_final_target_offset_y: f32,
    pub buddy_final_target_offset_z: f32,
    pub buddy_final_target_scale: f32,
    pub dolly_super_special2_offset_x: f32,
    pub dolly_super_special2_offset_y: f32,
    pub dolly_super_special2_offset_z: f32,
    pub dolly_super_special2_offset_x_ground: f32,
    pub dolly_super_special2_offset_y_ground: f32,
    pub dolly_super_special2_offset_z_ground: f32,
    pub x23b5b18326: f32,
    pub x23c2b6b3b0: f32,
    pub x235bbfe20a: f32,
    pub x23576d985f: f32,
    pub x23206aa8c9: f32,
    pub x23b963f973: f32,
    pub x294bf073a1: f32,
    pub x293cf74337: f32,
    pub x29a5fe128d: f32,
    pub master_final_target_offset_x: f32,
    pub master_final_target_offset_y: f32,
    pub master_final_target_offset_z: f32,
    pub x1ec3673edd: u64,
    pub x2234f9e1e4: f32,
    pub x2243fed172: f32,
    pub x22daf780c8: f32,
    pub x2456dfb0c0: f32,
    pub x2421d88056: f32,
    pub x24b8d1d1ec: f32,
    pub x24b91ddbfe: f32,
    pub x24ce1aeb68: f32,
    pub x245713bad2: f32,
    pub item_lift_accel_mul: f32,
    pub item_lift_accel_add: f32,
    pub item_lift_speed_max: f32,
    pub throw_item_speed_mul: f32,
    pub throw_item_power_mul: f32,
    pub shoot_walk_accel_mul: f32,
    pub shoot_walk_accel_add: f32,
    pub shoot_walk_speed_max: f32,
    pub shoot_dash_speed_f: f32,
    pub shoot_dash_speed_b: f32,
    pub item_range_front: f32,
    pub item_range_front_max: f32,
    pub item_range_back: f32,
    pub item_range_h: f32,
    pub item_range_air_front: f32,
    pub item_range_air_back: f32,
    pub item_range_air_h: f32,
    pub item_grip_offset_x: f32,
    pub item_grip_offset_y: f32,
    pub item_grip_offset_z: f32,
    pub item_grip_offset_rot_x: f32,
    pub item_grip_offset_rot_y: f32,
    pub item_grip_offset_rot_z: f32,
    pub badge_scale: f32,
    pub superleaftail_scale: f32,
    pub backshield_offset_y: f32,
    pub bossgalaga_offset_y: f32,
    pub baitocrane_offset_y: f32,
    pub driver_offset_x: f32,
    pub driver_scale: f32,
    pub effect_scale: f32,
    pub x11b9282bf8: f32,
    pub x11d4f5cf13: f32,
    pub x11a49f3b9c: f32,
    pub damage_fly_smoke_node: u64,
    pub damage_fly_smoke_offset_x: f32,
    pub damage_fly_smoke_offset_y: f32,
    pub damage_fly_roll_smoke_node: u64,
    pub damage_fly_roll_smoke_offset_x: f32,
    pub damage_fly_roll_smoke_offset_y: f32,
    pub fire_hit_size: u64,
    pub ice_node: u64,
    pub ice_offset_y: f32,
    pub ice_offset_z: f32,
    pub ice_radius: f32,
    pub ice_jump_speed_y_mul: f32,
    pub ice_jump_speed_x: f32,
    pub x1a786b5238: u64,
    pub x2080da265f: u64,
    pub x204850f761: u64,
    pub photo_camera_depth: f32,
    pub photo_camera_depth_ortho: f32,
    pub camera_range_center_y: f32,
    pub camera_range_front: f32,
    pub camera_range_back: f32,
    pub camera_range_up: f32,
    pub camera_range_down: f32,
    pub camera_range_2nd_center_y: f32,
    pub camera_range_2nd_front: f32,
    pub camera_range_2nd_back: f32,
    pub camera_range_2nd_up: f32,
    pub camera_range_2nd_down: f32,
    pub zoom_camera_range_center_y: u64,
    pub zoom_camera_range_front: f32,
    pub zoom_camera_range_back: f32,
    pub zoom_camera_range_up: f32,
    pub zoom_camera_range_down: f32,
    pub clip_sphere_node: u64,
    pub clip_sphere_offset_x: f32,
    pub clip_sphere_offset_y: f32,
    pub clip_sphere_offset_z: f32,
    pub clip_sphere_radius: f32,
    pub pose_offset_y: f32,
    pub dead_up_camera_hit_offset: f32,
    pub dead_up_camera_hit_rot_min: f32,
    pub dead_up_camera_hit_rot_max: f32,
    pub dead_up_camera_hit_distance: f32,
    pub dead_up_camera_hit_offset_y: f32,
    pub finish_camera_zoom_mul: f32,
    pub finish_camera_zoom_mul_thrown: f32,
    pub critical_hit_camera_zoom_mul: f32,
    pub x12aff73dbe: u64,
    pub x1236fe6c04: u64,
    pub ladder_range_offset_y: f32,
    pub ladder_range_w: f32,
    pub ladder_range_h: f32,
    pub ladder_attack_lw_fall: bool,
    pub barrel_shoot_node: u64,
    pub barrel_smoke_offset_y: f32,
    pub barrel_attack_node: u64,
    pub barrel_attack_offset_x: f32,
    pub barrel_attack_offset_y: f32,
    pub barrel_attack_offset_z: f32,
    pub barrel_attack_size: f32,
    pub x0e0b4b30ac: bool,
    pub x1f897884d1: f32,
    pub x1ffe7fb447: f32,
    pub x228658238e: f32,
    pub x22f15f1318: f32,
    pub x259b7edbff: f32,
    pub x25ec79eb69: f32,
    pub dolly_stage_wall_check_dead_offset_x: f32,
    pub x2a23dea6e7: f32,
    pub damage_attack_size: f32,
    pub escape_air_landing_frame: i32,
    pub x17e10662a4: bool,
    pub fly_speed_y_mul: f32,
    pub attack_s4_turn: bool,
    pub x145620fd97: bool,
    pub x13042acb2d: bool,
    pub dead_up_fall_oh_ko: bool,
    pub x187608007b: bool,
    pub x13cf982a55: bool,
    pub lw4_combo_max: i32,
    pub high_speed_dash_attack_offset_x: f32,
    pub high_speed_dash_attack_offset_y: f32,
    pub high_speed_dash_attack_offset_z: f32,
    pub item_range_offset_y: f32,
    pub item_range_air_offset_y: f32,
    pub size: i32,
    pub skin_kind: i32,
    pub metal_sound_level: i32,
    pub bury_motion_height: i32,
    pub bury_r_motion_height: i32,
    pub attack_combo_type: i32,
    pub aerial_type: i32,
}

#[repr(C)]
#[derive(Debug)]
pub struct FighterParams {
    pub params: super::Params,
    pub fighter_param_table: *mut [FighterParamTable; 88],
    pad_00: [u64; 2],
}

impl super::Filepath for FighterParams {
    fn filepath() -> &'static str {
        "fighter/common/param/fighter_param.prc"
    }
}
