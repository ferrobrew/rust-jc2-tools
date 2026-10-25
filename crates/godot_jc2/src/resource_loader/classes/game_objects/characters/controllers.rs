use godot::prelude::*;

use crate::{jc_class, jc_class_registry, jc_property_struct};

// TODO:
// CAirPlaneController
// CATVController
// CBellyStuntController
// CBoatController
// CCivilianSOController
// CClingStuntController
// CEventCharacterController
// CHelicopterController
// CLadderController
// CMCController
// CMountedGunController
// CReeledInController
// CRocketBabyController
// CStuntAreaController
// CThreeNodeClingController

jc_class_registry!(
    JcDriverSeatController,
    JcPassengerSeatController,
    JcRoofTopStuntController,
);

jc_class! {
    #[JcClass("CDriverSeatController")]
    pub struct JcDriverSeatController {
        data: JcDriverSeatControllerData,
    }
}

jc_property_struct! {
    pub struct JcDriverSeatControllerData {
        base: JcWeaponController,

        #[group("Driver Seat Controller")]
        camera_id_reverse: i32,
        duck_cooldown_time: f32,
        animation_modifier: GString,
    }
}

jc_property_struct! {
    pub struct JcWeaponController {
        base: JcCharacterController,

        #[group("Weapon Controller")]
        offset_target_position: i32,
        #[subgroup("Events", "event_")]
        event_fire_left_trigger: GString,
        event_fire_right_trigger: GString,
        event_send_up: GString,
        event_send_down: GString,
        event_send_left: GString,
        event_send_right: GString,
        event_send_aiming: GString,
    }
}

jc_class! {
    #[JcClass("CPassengerSeatController")]
    pub struct JcPassengerSeatController {
        data: JcPassengerSeatControllerData,
    }
}

jc_property_struct! {
    pub struct JcPassengerSeatControllerData {
        base: JcCharacterController,

        #[group("Passenger Seat Controller")]
        defend_vehicle_enabled: bool,
        main_body_animation_modifier: GString,
    }
}

jc_class! {
    #[JcClass("CRoofTopStuntController")]
    pub struct JcRoofTopStuntController {
        data: JcRoofTopStuntControllerData,
    }
}

jc_property_struct! {
    pub struct JcRoofTopStuntControllerData {
        base: JcCharacterController,

        #[group("Rooftop Stunt Controller")]
        main_body_animation_modifier: GString,
        upper_body_animation_modifier: GString,
    }
}

jc_property_struct! {
    pub struct JcCharacterController {
        object_id("_object_id"): i32,
        object_key("_object_key"): GString,
        local: Transform3D,
        world: Transform3D,

        #[group("Character Controller")]
        camera_id: i32,
        camera_id_aim: i32,
    }
}
