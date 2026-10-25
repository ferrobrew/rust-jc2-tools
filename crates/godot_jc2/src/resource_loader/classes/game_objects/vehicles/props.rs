use godot::{classes::node::InternalMode, prelude::*};
use jc2_file_formats::property_container::PropertyContainer;
use jc2_hashing::HashString;

use crate::{
    jc_class, jc_class_registry, jc_property_struct,
    resource_loader::classes::{GodotPropertyValueExtension, JcClass},
};

// TODO:
// CGenericProp
// CPointProp
// CVehicleProp
// CNullProp
// CSharedAssetProp
// CExcludeProp
// CPaperdollSettingsProp

jc_class_registry!(
    JcCollectionProp,
    JcEffectProp,
    JcFamilyProp,
    JcPartProp,
    JcReferenceProp
);

jc_class! {
    #[JcClass("CCollectionProp")]
    pub struct JcCollectionProp {
        data: JcCollectionPropData,
    }
}

jc_property_struct! {
    pub struct JcCollectionPropData {
        name("_name"): GString,

        #[group("Collection Prop")]
        collection_type: GString,
    }
}

jc_class! {
    #[JcClass("CEffectProp")]
    pub struct JcEffectProp {
        data: JcEffectPropData,
    }
}

jc_property_struct! {
    pub struct JcEffectPropData {
        #[group("Effect Prop")]
        effect: i32 = -1,
        // Unused,Explosion,EngineIdle,EngineRunning,Contrail,WaterExplosion,ShotdownEffect=8,WaterFrontDrive,WaterFrontCollision,WaterSideTrails,EngineSmoking,EngineBurning,EngineSmokeBurst,EnginePostExplosionFire
        usage: i32, // enum?
    }
}

jc_class! {
    #[JcClass("CFamilyProp")]
    pub struct JcFamilyProp {
        data: JcFamilyPropData,
    }
}

jc_property_struct! {
    pub struct JcFamilyPropData {
        base: JcCollectionPropData,

        #[group("Family Prop")]
        vehicle_id("VehicleID"): GString,
    }
}

jc_class! {
    pub struct JcPartProp {
        data: JcPartPropData,
    }
}

impl JcClass for JcPartProp {
    const NAME: &str = "CPartProp";

    fn import(
        &mut self,
        properties: &PropertyContainer,
        thread: &mut crate::resource_loader::JcResourceThread,
    ) -> crate::resource_loader::JcResourceResult<()> {
        // TODO: Most things need to write ortho scale, but also, don't support scaling to begin with... so their scale *should be one*
        self.data.read(properties);

        let mut scale = Vector3::ONE;
        if self.data.is_wheel {
            scale.read(properties, "tyre_scale");
        } else if self.data.is_rotor {
            scale.read(properties, "rotor_scale");
        }
        self.base_mut().set_scale(scale);

        if !self.data.model_shrt.is_empty() {
            let model = thread.create_resource_from_path(self.data.model_shrt.clone())?;
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

        let scale = self.base().get_scale();
        if self.data.is_wheel {
            scale.write(properties, "tyre_scale");
        } else if self.data.is_rotor {
            scale.write(properties, "rotor_scale");
        }

        todo!("write model path");

        Ok(())
    }
}

jc_property_struct! {
    pub struct JcPartPropData {
        name("_name"): GString,
        guid("_GUID"): i32,
        local: Transform3D,
        world: Transform3D,

        #[group("Part Prop")]
        model: GString,
        model_shrt: GString,
        pfx: GString,
        pfx_shrt: GString,
        #[hint(RANGE, "0,1000,1,prefer_slider")]
        mass: i32 = 100, // TODO: Irrelevant?
        part_type: GString, // enum?
        is_static("static"): bool,
        skin_to_parent: bool,
        ignore_palette: bool,

        #[subgroup("Wheel")]
        is_wheel: bool,
        wheel_index("wheel_slot"): i32,
        is_suspension: bool,
        is_static_suspension("is_staticsuspension"): bool,

        #[subgroup("Rotor")]
        is_rotor: bool,
        rotor_index("rotor_slot"): i32,
        #[hint(ENUM, "Still,Blurred,Axle")]
        rotor_part_type: i32,

        #[subgroup("Steering")]
        is_steering: bool,
        is_upper_steering("is_uppersteering"): bool,
    }
}

jc_class! {
    #[JcClass("CReferenceProp")]
    pub struct JcReferenceProp {
        data: JcReferencePropData,
    }
}

jc_property_struct! {
    pub struct JcReferencePropData {
        #[group("Reference Prop")]
        name("_name"): GString,
        guid("_GUID"): i32,
        ref_guid("_refGUID"): i32,
    }
}

jc_property_struct! {
    pub struct JcVehiclePropData {
        #[group("Vehicle Prop")]
        name("_name"): GString,
        vehicle_type("_vehicletype"): VehicleType,
        nof_wheels: i32,
        nof_rotors: i32,
    }
}

#[derive(GodotConvert, Var, Export, Default, Clone, Copy)]
#[godot(via = u8)]
pub enum VehicleType {
    #[default]
    Car,
    Helicopter,
    Boat,
    Airplane,
    Motorbike,
}

impl GodotPropertyValueExtension for VehicleType {
    fn read(&mut self, properties: &PropertyContainer, hash: impl Into<HashString>) -> bool {
        if let Some(value) = properties.get_value(hash) {
            *self = match value {
                1i32 => VehicleType::Helicopter,
                2i32 => VehicleType::Boat,
                3i32 => VehicleType::Airplane,
                4i32 => VehicleType::Motorbike,
                _ => VehicleType::Car,
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
