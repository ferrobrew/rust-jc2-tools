use godot::prelude::*;

use crate::{jc_class, jc_class_registry, jc_property_struct};

jc_class_registry!(JcAiCoverNodeCorner, JcAiCoverNodeFireLine, JcTimerComponent);

jc_class! {
    #[JcClass("SAICoverNode_Corner")]
    pub struct JcAiCoverNodeCorner {
        data: JcAiCoverNodeCornerData,
    }
}

jc_property_struct! {
    pub struct JcAiCoverNodeCornerData {
        #[group("AI Cover Node (Corner)")]
        _object_id: i32,
        points: PackedVector4Array,
        use_left_stretch: bool = true,
        left_collision_adjustment: f32,
        use_right_stretch: bool = true,
        right_collision_adjustment: f32,
    }
}

jc_class! {
    #[JcClass("SAICoverNode_FireLine")]
    pub struct JcAiCoverNodeFireLine {
        data: JcAiCoverNodeFireLineData,
    }
}

jc_property_struct! {
    pub struct JcAiCoverNodeFireLineData {
        #[group("AI Cover Node (Fire Line)")]
        _object_id: i32,
        points: PackedVector4Array,
        collision_adjustment: f32,
    }
}

jc_class! {
    #[JcClass("STimerComponent")]
    pub struct JcTimerComponent {
        data: JcTimerComponentData,
    }
}

impl JcTimerComponent {
    pub fn new(timer: f32, event: impl Into<GString>) -> Gd<Self> {
        let mut result = Self::new_alloc();
        result.bind_mut().data = JcTimerComponentData {
            timer,
            event: event.into(),
        };
        result
    }
}

jc_property_struct! {
    pub struct JcTimerComponentData {
        #[group("Timer")]
        timer: f32,
        event("timer_event"): GString,
    }
}
