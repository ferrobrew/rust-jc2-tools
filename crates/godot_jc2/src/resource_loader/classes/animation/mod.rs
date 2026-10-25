use godot::prelude::*;

use crate::{jc_class, jc_class_registry, jc_property_struct};

jc_class_registry!(JcHelperPoint, JcIkTarget);

jc_class! {
    #[JcClass("CHelperPoint")]
    pub struct JcHelperPoint {
        data: JcIkTargetData,
    }
}

jc_class! {
    #[JcClass("CIKTarget")]
    pub struct JcIkTarget {
        data: JcIkTargetData,
    }
}

jc_property_struct! {
    pub struct JcIkTargetData {
        name: i32,
        local: Transform3D,
        #[group("IK Target")]
        filter_type: i32, // enum?
        use_rotation: bool,
        skinned_part_name("_skinned_partname"): i32,
        start_pos: Vector3,
        end_pos: Vector3,
        index0("_index0"): i32,
        index1("_index1"): i32,
        index2("_index2"): i32,
        index3("_index3"): i32,
        weight0("_weight0"): f32,
        weight1("_weight1"): f32,
        weight2("_weight2"): f32,
        weight3("_weight3"): f32,
    }
}
