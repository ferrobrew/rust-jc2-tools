use godot::prelude::*;

use crate::{jc_class, jc_class_registry, jc_property_struct};

// TODO:
// CAIAnimationAction
// CAutoParachuteAction
// CClingToRoofAction
// CDeployParachuteWhileReelingAction
// CDisarmAction
// CEnterAndWaitForAnimSOAction
// CEnterCivilianSOAction
// CEnterDriverSeatFromClingAction
// CEnterFixedMountedGunAction
// CEnterMCAction
// CEnterMCFromRoofAction
// CEnterMGAction
// CExitCivilianSOAction
// CExitFixedMountedGunAction
// CExitMCAction
// CExitMGAction
// CFireHookAction
// CGrenadePickupAction
// CHealthStationAction
// CLadderBottomAction
// CLadderDropAction
// CLadderGripAction
// CLadderTopAction
// CMCStuntAction
// CPickupAction
// CPullGrappledAction
// CReeledInJumpAction
// CReeledInReleaseAction
// CReelInAction
// CReelToSOAction
// CStuntAction
// CStuntJumpAction
// CWeaponPickupAction

jc_class_registry!(
    JcEnterDriverSeatFromRoofAction,
    JcEnterDriverSeatAction,
    JcEnterPassengerSeatAction,
    JcExitDriverSeatAction,
    JcExitPassengerSeatAction,
    JcGrapplingJumpAction,
    JcParachuteFromDriverSeatAction,
    JcParachuteFromRoofAction,
    JcParachuteToSOAction,
    JcPassengerToDriverAction,
    JcRoofToGroundJumpAction,
    JcSmartObjectAction
);

jc_class! {
    #[JcClass("CEnterDriverSeatAction")]
    pub struct JcEnterDriverSeatFromRoofAction {
        data: JcEnterDriverSeatFromRoofActionData,
    }
}

jc_property_struct! {
    pub struct JcEnterDriverSeatActionData {
        base: JcEnterVehicleActionData,
    }
}

jc_class! {
    #[JcClass("CEnterDriverSeatFromRoofAction")]
    pub struct JcEnterDriverSeatAction {
        data: JcEnterDriverSeatActionData,
    }
}

jc_property_struct! {
    pub struct JcEnterDriverSeatFromRoofActionData {
        base: JcEnterVehicleActionData,

        #[group("Enter Driver Seat From Roof Action")]
        to_rooftop: bool,
    }
}

jc_class! {
    #[JcClass("CEnterPassengerSeatAction")]
    pub struct JcEnterPassengerSeatAction {
        data: JcEnterPassengerSeatActionData,
    }
}

jc_property_struct! {
    pub struct JcEnterPassengerSeatActionData {
        base: JcEnterVehicleActionData,
    }
}

jc_property_struct! {
    pub struct JcEnterVehicleActionData {
        base: JcSmartObjectActionData,

        #[group("Enter Vehicle Action")]
        seat: i32,
        animation_modifier: GString,
        player_locked: bool,
        player_lock_event: GString,
        player_unlock_event: GString,
        stunt_jump_to_anim_start: bool,
        struggle_enabled: bool = true,
    }
}

jc_class! {
    #[JcClass("CExitDriverSeatAction")]
    pub struct JcExitDriverSeatAction {
        data: JcExitDriverSeatActionData,
    }
}

jc_property_struct! {
    pub struct JcExitDriverSeatActionData {
        base: JcExitVehicleActionData,
    }
}

jc_class! {
    #[JcClass("CExitPassengerSeatAction")]
    pub struct JcExitPassengerSeatAction {
        data: JcExitPassengerSeatActionData,
    }
}

jc_property_struct! {
    pub struct JcExitPassengerSeatActionData {
        base: JcExitVehicleActionData,
    }
}

jc_property_struct! {
    pub struct JcExitVehicleActionData {
        base: JcSmartObjectActionData,

        #[group("Exit Vehicle Action")]
        seat: i32,
        teleport_pos: Vector3,
        exit_jumping: bool,
        animation_modifier: GString,
    }
}

jc_class! {
    #[JcClass("CGrapplingJumpAction")]
    pub struct JcGrapplingJumpAction {
        data: JcGrapplingJumpActionData,
    }
}

jc_property_struct! {
    pub struct JcGrapplingJumpActionData {
        base: JcSmartObjectActionData,

        #[group("Grappling Jump Action")]
        #[hint(ENUM, "Roof,Cling,Area,Invalid,MotorBike")]
        goal_type: i32,
        camera_id_from_so("camera_id_from_SO"): i32,
    }
}

jc_class! {
    #[JcClass("CParachuteFromDriverSeatAction")]
    pub struct JcParachuteFromDriverSeatAction {
        data: JcParachuteFromDriverSeatActionData,
    }
}

jc_property_struct! {
    pub struct JcParachuteFromDriverSeatActionData {
        base: JcSmartObjectActionData,
    }
}

jc_class! {
    #[JcClass("CParachuteFromRoofAction")]
    pub struct JcParachuteFromRoofAction {
        data: JcParachuteFromRoofActionData,
    }
}

jc_property_struct! {
    pub struct JcParachuteFromRoofActionData {
        base: JcSmartObjectActionData,
    }
}

jc_class! {
    #[JcClass("CParachuteToSOAction")]
    pub struct JcParachuteToSOAction {
        data: JcParachuteToSOActionData,
    }
}

jc_property_struct! {
    pub struct JcParachuteToSOActionData {
        base: JcSmartObjectActionData,
    }
}

jc_class! {
    #[JcClass("CPassengerToDriverAction")]
    pub struct JcPassengerToDriverAction {
        data: JcPassengerToDriverActionData,
    }
}

jc_property_struct! {
    pub struct JcPassengerToDriverActionData {
        base: JcSmartObjectActionData,

        #[group("Passenger To Driver Action")]
        animation_modifier: GString,
    }
}

jc_class! {
    #[JcClass("CRoofToGroundJumpAction")]
    pub struct JcRoofToGroundJumpAction {
        data: JcRoofToGroundJumpActionData,
    }
}

jc_property_struct! {
    pub struct JcRoofToGroundJumpActionData {
        base: JcSmartObjectActionData,

        #[group("Roof To Ground Jump Action")]
        animation_modifier: GString,
    }
}

jc_class! {
    #[JcClass("CSmartObjectAction")]
    pub struct JcSmartObjectAction {
        data: JcSmartObjectActionData,
    }
}

jc_property_struct! {
    pub struct JcSmartObjectActionData {
        base: JcContextActionData,

        #[group("Smart Object Action")]
        send_done_event: GString,
        send_execute_event: GString,
        send_cancel_event: GString,
        send_player_done_event: GString,
        send_player_execute_event: GString,
        send_player_cancel_event: GString,
        enable_event: GString,
        disable_event: GString,
        animation_resource("animation_filename"): GString,
        change_state: bool,
        play_state: GString,
        end_state: GString,
        blend_time: f32,
        smart_object_goal: i32,
        smart_object_goal2: i32,
        smart_object_goal3: i32,
        smart_object_goal4: i32,
        smart_object_goal5: i32,
        attach_to_goal: bool,
        facing_angle: f32,
        allow_so_usage: bool,
        ai_action_identifier: GString,
        ai_teleport_action: bool,
        player_teleport_action: bool,
        dedicated_enter_action: bool,
        dedicated_exit_action: bool,
        show_floating_activation_button: bool,
    }
}

jc_property_struct! {
    pub struct JcContextActionData {
        object_id("_object_id"): i32,
        object_key("_object_key"): GString,
        name: GString,
        local: Transform3D,
        world: Transform3D,

        #[group("Context Action")]
        always_execute: bool,
        override_input("overideinput"): bool,
        priority: i32,
        execute_input: i32,
        camera_id: i32,
        activation_message: GString,
    }
}
