use godot::{classes::node::InternalMode, prelude::*};

use jc2_file_formats::property_container::PropertyContainer;
use jc2_hashing::HashString;

use crate::{
    jc_class, jc_class_registry, jc_property_struct,
    resource_loader::classes::{GodotPropertyValueExtension, JcClass},
};

pub mod characters;
pub mod vehicles;

// TODO:
// CDamageableObject
// CLandmark
// CPlantedTree

jc_class_registry!(
    characters,
    vehicles,
    JcDamageControl,
    JcGeometryObject,
    JcSimpleRigidObject,
    JcSmartObject,
    JcCivilianSmartObjectTrigger,
    JcSmartObjectTrigger,
    JcAiCoverGameObject
);

jc_class! {
    pub struct JcDamageControl {
        data: JcDamageControlData,
    }
}

impl JcClass for JcDamageControl {
    const NAME: &str = "CDamageControl";

    fn import(
        &mut self,
        properties: &jc2_file_formats::property_container::PropertyContainer,
        _thread: &mut crate::resource_loader::JcResourceThread,
    ) -> crate::resource_loader::JcResourceResult<()> {
        if let (Some(timer), Some(event)) = (
            properties.get_value::<&str>("timer0"),
            properties.get_value::<&str>("timer_event0"),
        ) {
            if let Ok(timer) = timer.parse::<f32>() {
                self.base_mut()
                    .add_child(&super::components::JcTimerComponent::new(timer, event));
            }
        }

        Ok(self.data.read(properties))
    }

    fn export(
        &self,
        properties: &mut jc2_file_formats::property_container::PropertyContainer,
        _thread: &mut crate::resource_loader::JcResourceThread,
    ) -> crate::resource_loader::JcResourceResult<()> {
        Ok(self.data.write(properties))
    }
}

jc_property_struct! {
    pub struct JcDamageControlData {
        base: JcDamageableData,

        #[group("Damage Control")]
        master_object: GString,
        slave_object: GString,
        auto_show: bool,
        auto_hide: bool,
        enable_timers: bool,
        hit_indication: bool = true,
        ignore_collision_if_destroyed_by_vehicle: bool = true,
    }
}

jc_property_struct! {
    pub struct JcDamageableData {
        base: JcPhysicsGameObjectData,

        #[group("Damageable")]
        health: f32 = 100.0,
        max_health: f32 = 100.0,
        is_invulnerable("invulnerable"): bool,

        #[subgroup("Events", "event_")]
        event_force_destroy("force_destroy"): GString,
        event_restore_health("restore_health"): GString,
        event_make_invulnerable("make_invulnerable"): GString,
        event_make_vulnerable("make_vulnerable"): GString,
        event_on_hit("on_hit"): GString,
        event_on_hit_damaged("on_hit_damaged"): GString,
        event_on_hit_not_damaged("on_hit_not_damaged"): GString,
        event_on_destroy("on_destroy"): GString,
        event_on_removed("on_removed"): GString,

        #[subgroup("Armor")]
        bullet_armor_type: ArmorType,
        bullet_protection: f32,
        explosion_armor_type: ArmorType,
        explosion_protection: f32,
        impact_armor_type: ArmorType,
        impact_protection: f32,
        fire_armor_type: ArmorType,
        fire_protection: f32,
        melee_armor_type: ArmorType,
        melee_protection: f32,
        electricity_armor_type: ArmorType,
        electricity_protection: f32,
    }
}

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = u8)]
pub enum ArmorType {
    Threshold,
    #[default]
    Subtraction,
}

impl GodotPropertyValueExtension for ArmorType {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(value) = properties.get_value(hash) {
            *self = match value {
                1i32 => ArmorType::Subtraction,
                _ => ArmorType::Threshold,
            };
            true
        } else {
            false
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, *self as i32);
    }
}

jc_class! {
    #[JcClass("CGeometryObject")]
    pub struct JcGeometryObject {
        data: JcGeometryObjectData,
    }
}

// TODO: inherits from physics object, but only uses game object properties; re-base?
jc_property_struct! {
    pub struct JcGeometryObjectData {
        base: JcPhysicsGameObjectData,

        #[group("Geometry Object")]
        save: bool,
        mission_name: GString,
        missionhack: GString,
        disable: GString,
        reinit: GString,
        recreate_constraints: GString,
    }
}

jc_class! {
    pub struct JcSimpleRigidObject {
        data: JcSimpleRigidObjectData,
    }
}

impl JcClass for JcSimpleRigidObject {
    const NAME: &str = "CSimpleRigidObject";

    fn import(
        &mut self,
        properties: &PropertyContainer,
        thread: &mut crate::resource_loader::JcResourceThread,
    ) -> crate::resource_loader::JcResourceResult<()> {
        // TODO: Most things need to write ortho scale, but also, don't support scaling to begin with... so their scale *should be one*
        self.data.read(properties);

        if !self.data.model_resource.is_empty() {
            let model = thread.create_resource_from_path(self.data.model_resource.clone())?;
            self.base_mut()
                .add_child_ex(&model.cast::<Node>())
                .internal(InternalMode::FRONT)
                .done();
        }

        Ok(())
    }

    fn export(
        &self,
        properties: &mut PropertyContainer,
        _thread: &mut crate::resource_loader::JcResourceThread,
    ) -> crate::resource_loader::JcResourceResult<()> {
        self.data.write(properties);

        todo!("write model path");

        Ok(())
    }
}

jc_property_struct! {
    pub struct JcSimpleRigidObjectData {
        base: JcPhysicsGameObjectData,

        #[group("Simple Rigid Object")]
        name: GString,
        model_resource("filename"): GString,
        collision_resource("collision"): GString,
        mission_hack("missionhack"): GString,
        dynamic_pathfinding_mode("dynamic_pathfinding_mode"): DynamicPathfinding,

        #[subgroup("Flags")]
        visible("visible"): bool = true,
        fixed("fixed"): bool,
        dynamic("dynamic"): bool,
        pre_draw("pre_draw"): bool,
        keyframed("key_framed"): bool,
        soft_keyframe("soft_keyframe"): bool,
        magnetable("magnetable"): bool = true,
        cast_shadows("cast_shadows"): bool,
        clip("clip"): bool = true,
        reflect_in_water("reflect_in_water"): bool,
        far_collision("far_collision"): bool,

        #[subgroup("Events")]
        event_show("showonevent"): GString,
        event_hide("hideonevent"): GString,
        event_fade_out("fade_out"): GString,
        event_reload_physics("reload_physics_event"): GString,
        event_move_to_player("move_to_player_event"): GString,
        event_stop_keyframe("stop_key_frame"): GString,

        #[subgroup("Effects")]
        impact_effect_id("collision_effect_id_impact"): i32 = -1,
        impact_magnitude_input_id("collision_effect_input_size_id_impact"): i32 = -1,
        slide_effect_id("collision_effect_id_slide"): i32 = -1,
        slide_magnitude_input_id("collision_effect_input_size_id_slide"): i32 = -1,
        roll_effect_id("collision_effect_id_roll"): i32 = -1,
        roll_magnitude_input_id("collision_effect_input_size_id_roll"): i32 = -1,
    }
}

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = u8)]
pub enum DynamicPathfinding {
    Automatic,
    Enabled,
    #[default]
    Disabled,
}

impl GodotPropertyValueExtension for DynamicPathfinding {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(value) = properties.get_value(hash) {
            *self = match value {
                1i32 => DynamicPathfinding::Enabled,
                2i32 => DynamicPathfinding::Disabled,
                _ => DynamicPathfinding::Automatic,
            };
            true
        } else {
            false
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, *self as i32);
    }
}

jc_class! {
    #[JcClass("CSmartObject")]
    pub struct JcSmartObject {
        data: JcSmartObjectData,
    }
}

jc_property_struct! {
    pub struct JcSmartObjectData {
        base: JcPhysicsGameObjectData,

        name: GString,
        object_key("_object_key"): GString,
        local: Transform3D,
        world: Transform3D,
        model_resource("model_filename"): GString, // removed?

        #[group("Smart Object")]
        precomputed_ik("_precomputedIK"): bool,
        save: bool,
        mission_name: GString,
        enabled: bool,
        enable: GString,
        disable: GString,
        sound_perception_id: i32,
        on_connect_event: GString,
        enter_event: GString,
        exit_event: GString,
        player_enter_event: GString,
        player_exit_event: GString,
    }
}

jc_class! {
    #[JcClass("CCivilianSmartObjectTrigger")]
    pub struct JcCivilianSmartObjectTrigger {
        data: JcSmartObjectTriggerData,
    }
}

jc_class! {
    #[JcClass("CSmartObjectTrigger")]
    pub struct JcSmartObjectTrigger {
        data: JcSmartObjectTriggerData,
    }
}

jc_property_struct! {
    pub struct JcSmartObjectTriggerData {
        object_id("_object_id"): i32,
        object_key("_object_key"): GString,
        name: GString,
        local: Transform3D,
        world: Transform3D,

        #[group("Smart Object Trigger")]
        box_size: Vector3 = Vector3::new(1.0, 1.0, 1.0),
    }
}

// TODO: convert "_internal_type" objects?
jc_class! {
    #[JcClass("CAICoverGameObject")]
    pub struct JcAiCoverGameObject {
        data: JcAiCoverGameObjectData,
    }
}

jc_property_struct! {
    pub struct JcAiCoverGameObjectData {
        base: JcGameObjectData,

        #[group("AI Cover Game Object")]
        corner_max_rotation: f32,
        disable_when_parent_destructs: bool,
        universal_cover: bool,
        spawn_enabled: bool = true,
    }
}

jc_property_struct! {
    pub struct JcPhysicsGameObjectData {
        base: JcGameObjectData,

        #[group("Physics Game Object")]
        grapple_weight("grappling_weight_category"): GrappleWeight,
    }
}

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = u8)]
pub enum GrappleWeight {
    #[default]
    None,
    Light,
    Heavy,
}

impl GodotPropertyValueExtension for GrappleWeight {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(value) = properties.get_value(hash) {
            *self = match value {
                1i32 => GrappleWeight::Light,
                2i32 => GrappleWeight::Heavy,
                _ => GrappleWeight::None,
            };
            true
        } else {
            false
        }
    }

    fn write(&self, properties: &mut PropertyContainer, hash: impl Into<HashString>) {
        properties.insert(hash, *self as i32);
    }
}

jc_property_struct! {
    pub struct JcGameObjectData {
        #[group("Game Object")]
        object_id("_object_id"): i32,
        instance_id("_instance_id"): i32,
    }
}
