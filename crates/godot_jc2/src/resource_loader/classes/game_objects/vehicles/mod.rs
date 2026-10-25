use godot::prelude::*;

use godot_utils::property_bitflags;
use jc2_file_formats::property_container::PropertyContainer;
use jc2_hashing::HashString;

use super::JcDamageableData;
use crate::{
    jc_class, jc_class_registry, jc_property_struct,
    resource_loader::classes::GodotPropertyValueExtension,
};

pub mod modules;
pub mod props;

jc_class_registry!(
    modules,
    props,
    JcAirPlane,
    JcHelicopter,
    JcCar,
    JcMotorBike,
    JcBoat
);

jc_class! {
    #[JcClass("CAirPlane")]
    pub struct JcAirPlane {
        data: JcAirPlaneData,
    }
}

jc_property_struct! {
    pub struct JcAirPlaneData {
        base: JcAirVehicleData,
    }
}

jc_class! {
    #[JcClass("CHelicopter")]
    pub struct JcHelicopter {
        data: JcHelicopterData,
    }
}

jc_property_struct! {
    pub struct JcHelicopterData {
        base: JcAirVehicleData,

        #[group("Helicopter")]
        return_pitch_limit: f32,
        return_roll_limit: f32,
    }
}

jc_property_struct! {
    pub struct JcAirVehicleData {
        base: JcVehicleData,

        #[group("Air Vehicle")]
        #[subgroup("Ground Effect", "arve_groundeffect_")]
        arve_groundeffect_id: i32,
        arve_groundeffect_max_distance: f32,
        arve_groundeffect_min_velocity: f32,
        arve_groundeffect_offset_translation: Vector3,

        #[group("Sound")]
        preset("sound_preset_fx"): i32,

        #[group("Sound/RPM", "arve_engine_")]
        arve_engine_acceleration: f32,
        arve_engine_min_rpm: f32,
        arve_engine_max_rpm: f32,
        arve_engine_top_speed: f32,
        arve_engine_trust_coef: f32,
        arve_engine_velocity_coef: f32,
        arve_engine_idle_load: f32,
        arve_engine_load_hold: f32,
        arve_engine_load_release: f32,

        #[group("Handling")]
        turn_factor: f32,
        turn_smoothing("turn_smooth_pc"): f32,
        acceleration_smoothing("acc_smooth_pc"): f32,
        roll_decay("roll_return_pc"): f32,
        pitch_decay("pitch_return_pc"): f32,
    }
}

jc_class! {
    #[JcClass("CCar")]
    pub struct JcCar {
        data: JcCarData,
    }
}

jc_property_struct! {
    pub struct JcCarData {
        base: JcLandVehicleData,

        #[group("Car")]
        has_alarm: bool,
        has_sirens: bool,
        has_reverse_beep: bool,
    }
}

jc_class! {
    #[JcClass("CMotorBike")]
    pub struct JcMotorBike {
        data: JcMotorBikeData,
    }
}

jc_property_struct! {
    pub struct JcMotorBikeData {
        base: JcLandVehicleData,

        #[group("Motor Bike")]
        #[subgroup("Leaning", "lean_")]
        lean_min_turn_speed("min_turn_lean_speed"): f32,
        lean_max_turn_speed("max_turn_lean_speed"): f32,
        lean_max_amount("max_lean_amount"): f32,
        lean_smoothing("lean_smooth"): f32,

        #[subgroup("Fall Off", "fall_off_")]
        fall_off_velocity_threshold("falloff_treshold"): Vector3,
        fall_off_angular_velocity_treshold("angvel_falloff_treshold"): f32,
        fall_off_axis_angle("angvel_falloff_angle"): f32,

        #[subgroup("Wheelie")]
        stunt_offset: Vector3,
        stunt_rotation_force: f32,
        wheelie_speed: f32,
        min_rotation_limit: f32,
        max_rotation_limit: f32,
        min_center_of_mass_shift_limit("min_com_shift_limit"): f32,
        max_center_of_mass_shift_limit("max_com_shift_limit"): f32,
        damping_limit: f32,
        min_wheelie_velocity: f32,
    }
}

jc_property_struct! {
    pub struct JcLandVehicleData {
        base: JcVehicleData,

        #[group("Land Vehicle")]
        has_unbreakable_wheels("unbreakable_wheels"): bool,
        static_suspension_offset: f32,
        side_compensate_factor: f32,

        #[subgroup("Handling")]
        turn_smoothing("turn_smooth"): f32,
        npc_turn_smoothing("turn_smooth_ai"): f32,
        turn_decay_smoothing("return_smooth"): f32,
        npc_turn_decay_smoothing("return_smooth_ai"): f32,
        acceleration_smoothing("acc_smooth"): f32,

        #[subgroup("Sound", "sound_")]
        sound_preset("sound_preset_fx"): i32,
        sound_siren_preset: i32,

        #[subgroup("Sound/RPM")]
        engine_load_mid("engineload_mid"): f32,
        engine_load_clutch_smoothing("engineload_clutch_smooth"): f32,
        engine_load_smoothing("engineload_smooth"): f32,

        #[subgroup("Sound/RPM", "rpm_")]
        rpm_gain_in_air: f32,
        rpm_gain_per_gear: f32,
        rpm_smoothing("rpm_smooth"): f32,
        rpm_random_deviation: f32,

        #[subgroup("Sound/RPM/Scaling", "rpm_scaling")]
        #[hint(GROUP_ENABLE)]
        rpm_scaling_enabled: bool,
        rpm_scaling_min: f32,
        rpm_scaling_max: f32,

        #[subgroup("Effects")]
        tire_burst("tire_explosion_effect_id"): i32,
        undercarriage_collision("undercarriage_collision_effect_id"): i32,
        tire_roll("wheel_effect_id_roll"): i32,
        tire_roll_threshold: f32,
        tire_skid("wheel_effect_id_skid"): i32,
        tire_skid_threshold: f32,
        tire_spin("wheel_effect_id_spin"): i32,
        tire_spin_threshold: f32,
        rim_sparks("wheel_effect_id_notyre"): i32,
        rim_sparks_threshold: f32,
        suspension("wheel_effect_id_suspension"): i32,
        suspension_threshold: f32,
    }
}

jc_class! {
    #[JcClass("CBoat")]
    pub struct JcBoat {
        data: JcBoatData,
    }
}

jc_property_struct! {
    pub struct JcBoatData {
        base: JcSeaVehicleData,
    }
}

jc_property_struct! {
    pub struct JcSeaVehicleData {
        base: JcVehicleData,

        #[group("Sea Vehicle")]
        velocity_to_rpm("seve_engine_vel_to_rpm"): f32,
        min_rpm("seve_engine_min_rpm"): f32,
        max_rpm("seve_engine_max_rpm"): f32,
        rpm_mix("seve_engine_rpm_mix"): f32,
        is_hover_craft("seve_hover_craft"): bool,
        turn_smoothing("turn_smooth"): f32,
        acceleration_smoothing("acc_smooth"): f32,
    }
}

jc_property_struct! {
    pub struct JcVehicleData {
        base: JcDamageableData,

        #[group("Vehicle")]
        name: GString,
        displayed_name: GString,
        receive_matrix_event: GString,
        explosion_delay: f32,
        post_explosion_burn_time: f32,
        engine_smoke_threshold: f32,
        engine_burn_threshold: f32,
        open_door_speed: f32,
        close_door_speed: f32,
        auto_aim_position: Vector3,
        attached_part_collision_effect_id_impact: i32,
        attached_part_collision_effect_id_slide: i32,
        ai_handling_params: GString,

        #[subgroup("Paper Doll")]
        paper_doll_resource("vdoll"): GString,
        rear_steering_resource("rear_steering"): GString,
        spawn_template_resource("sstemplate"): GString,
        spawn_template_selection("spawn_templateselection"): GString,
        palette_resource("palette"): GString,
        palette_selection("paletteselection"): GString,
        decal_resource("decal"): GString,
        decal_selection("decalselection"): GString,

        #[subgroup("")]
        is_bulk_vehicle("bulkvehicle"): bool,
        has_unique_pfx("uniquepfx"): bool,
        light_explosion_effect_id: i32,
        wheel_rotated: WheelFlags,
    }
}

property_bitflags! {
    #[derive(Clone, Copy, PartialEq, PartialOrd)]
    pub struct WheelFlags: u8 {
        const Wheel1 = 0b00000001;
        const Wheel2 = 0b00000010;
        const Wheel3 = 0b00000100;
        const Wheel4 = 0b00001000;
        const Wheel5 = 0b00010000;
        const Wheel6 = 0b00100000;
        const Wheel7 = 0b01000000;
        const Wheel8 = 0b10000000;
    }
}

impl Default for WheelFlags {
    fn default() -> Self {
        Self(0b10101010)
    }
}

impl GodotPropertyValueExtension for WheelFlags {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(value) = properties.get_value::<i32>(hash) {
            *self = WheelFlags::from_bits_truncate(value as u8);
            true
        } else {
            false
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, self.0 as i32);
    }
}
