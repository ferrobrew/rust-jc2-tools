use godot::prelude::*;

use crate::resource_loader::classes::{GodotPropertyValueExtension, JcClass};
use crate::{jc_class, jc_class_registry, jc_property_struct};

jc_class_registry!(generic, car);

mod generic {
    use godot::{
        classes::{
            EditorNode3DGizmo, EditorNode3DGizmoPlugin, EditorPlugin, IEditorNode3DGizmoPlugin,
            IEditorPlugin, TriangleMesh,
        },
        obj::NewGd,
    };

    use super::*;

    jc_class_registry!(
        JcAiModule,
        JcBrakeModule,
        JcBuoyancyModule,
        JcGroupModule,
        JcPartDamageModule,
        JcSuspensionModule,
        JcVehicleDamageModule,
        JcWheelModule
    );

    jc_class! {
        #[JcClass("CAiModule")]
        pub struct JcAiModule {
            data: JcAiModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcAiModuleData {
            name("_name"): GString,

            #[group("AI Module")]
            #[subgroup("Turning PID Controller", "turning_")]
            turning_proportional("turn_p"): f32 = 1.95,
            turning_integral("turn_i"): f32 = 0.2,
            turning_derivative("turn_d"): f32 = 0.05,

            #[subgroup("Acceleration PID Controller", "acceleration_")]
            acceleration_proportional("acc_p"): f32 = 0.2,
            acceleration_integral("acc_i"): f32 = 0.005,
            acceleration_derivative("acc_d"): f32 = 0.3,

            #[subgroup("Stopping PID Controller", "stopping_")]
            stopping_proportional("stop_p"): f32 = 0.7,
            stopping_integral("stop_i"): f32 = 0.005,
            stopping_derivative("stop_d"): f32 = 0.05,

            #[subgroup("Speed", "speed_")]
            speed_forward: f32 = 50.0,
            speed_backward: f32 = 10.0,
        }
    }

    jc_class! {
        #[JcClass("CBrakeModule")]
        pub struct JcBrakeModule {
            data: JcBrakeModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcBrakeModuleData {
            name("_name"): GString,

            #[group("Brake Module")]
            #[subgroup("Brakes")]
            max_brake_torque: f32,
            #[hint(RANGE, "0,100,1,prefer_slider")]
            power_distribution: i32 = 100,
            handbrake: bool,
            min_time_to_block: f32,
            antilock_braking_system("ABS"): bool,
        }
    }

    #[derive(GodotClass)]
    #[class(tool, init, base=EditorPlugin)]
    struct JcEditorPlugin {
        base: Base<EditorPlugin>,
        gizmos: Vec<Gd<EditorNode3DGizmoPlugin>>,
    }

    fn create_gizmo<T: NewGd + Inherits<EditorNode3DGizmoPlugin>>() -> Gd<EditorNode3DGizmoPlugin> {
        T::new_gd().upcast()
    }

    type GizmoPlugin = fn() -> Gd<EditorNode3DGizmoPlugin>;

    impl JcEditorPlugin {
        const GIZMO_PLUGINS: &[GizmoPlugin] = &[create_gizmo::<JcBuoyancyModuleGizmoPlugin>];
    }

    #[godot_api]
    impl IEditorPlugin for JcEditorPlugin {
        fn enter_tree(&mut self) {
            let mut gizmos = Vec::with_capacity(Self::GIZMO_PLUGINS.len());
            {
                let mut base = self.base_mut();
                for plugin in Self::GIZMO_PLUGINS {
                    let gizmo = plugin();
                    base.add_node_3d_gizmo_plugin(&gizmo);
                    gizmos.push(gizmo);
                }
            }
            self.gizmos = gizmos;
        }

        fn exit_tree(&mut self) {
            let gizmos = std::mem::take(&mut self.gizmos);
            let mut base = self.base_mut();
            for gizmo in &gizmos {
                base.remove_node_3d_gizmo_plugin(gizmo);
            }
        }
    }

    #[derive(GodotClass)]
    #[class(tool, base=EditorNode3DGizmoPlugin)]
    pub struct JcBuoyancyModuleGizmoPlugin {
        base: Base<EditorNode3DGizmoPlugin>,
    }

    #[godot_api]
    impl IEditorNode3DGizmoPlugin for JcBuoyancyModuleGizmoPlugin {
        fn init(base: Base<EditorNode3DGizmoPlugin>) -> Self {
            base.to_init_gd().create_material("wireframe", Color::WHITE);
            Self { base }
        }

        fn has_gizmo(&self, for_node_3d: Option<Gd<Node3D>>) -> bool {
            for_node_3d.is_some_and(|node| node.try_cast::<JcBuoyancyModule>().is_ok())
        }

        fn get_gizmo_name(&self) -> GString {
            JcBuoyancyModule::class_id().to_gstring()
        }

        fn redraw(&mut self, gizmo: Option<Gd<EditorNode3DGizmo>>) {
            let Some(mut gizmo) = gizmo else { return };
            let Some(node) = gizmo
                .get_node_3d()
                .and_then(|node| node.try_cast::<JcBuoyancyModule>().ok())
            else {
                return;
            };

            gizmo.clear();

            let data = &node.bind().data;
            let radius = data.radius;
            let extents = [radius, radius * data.side_area, radius * data.bottom_area];
            let transform = node
                .get_transform()
                .affine_inverse()
                .translated(node.get_position());
            let point = |i: usize| -> Vector3 {
                transform
                    * Vector3::new(
                        if i & 1 == 0 { -extents[0] } else { extents[0] },
                        if i & 2 == 0 { -extents[1] } else { extents[1] },
                        if i & 4 == 0 { -extents[2] } else { extents[2] },
                    )
            };

            let mut lines = PackedVector3Array::new();
            let mut vertices = PackedVector3Array::new();

            for i in 0..8 {
                for axis in 0..3 {
                    let bit = 1 << axis;
                    let u = (axis + 1) % 3;
                    let v = (axis + 2) % 3;

                    if i & bit == 0 {
                        if !extents[axis].is_zero_approx() {
                            lines.push(point(i));
                            lines.push(point(i | bit));
                        }
                    } else {
                        if !(extents[u] + extents[v]).is_zero_approx() {
                            let b = i | (1 << u);
                            let c = i | (1 << v);
                            let d = b | (1 << v);

                            let triangles = if axis == 1 {
                                [[i, b, c], [b, d, c]]
                            } else {
                                [[i, c, b], [b, c, d]]
                            };

                            for [a, b, c] in triangles {
                                vertices.push(point(a));
                                vertices.push(point(b));
                                vertices.push(point(c));
                            }
                        }
                    }
                }
            }

            gizmo.add_lines(&lines, self.base().get_material("wireframe").as_ref());
            gizmo.add_collision_segments(&lines);

            if !vertices.is_empty() {
                let mut mesh = TriangleMesh::new_gd();
                if mesh.create_from_faces(&vertices) {
                    gizmo.add_collision_triangles(&mesh);
                }
            }
        }
    }

    jc_class! {
        pub struct JcBuoyancyModule {
            data: JcBuoyancyModuleData,
        }
    }

    impl JcClass for JcBuoyancyModule {
        const NAME: &str = "CBoyancyModule";

        fn import(
            &mut self,
            properties: &jc2_file_formats::property_container::PropertyContainer,
            _thread: &mut crate::resource_loader::JcResourceThread,
        ) -> crate::resource_loader::JcResourceResult<()> {
            let mut position = Vector3::ZERO;
            if position.read(properties, "position") {
                self.base_mut().set_position(position);
            }
            self.data.read(properties);
            Ok(())
        }

        fn export(
            &self,
            properties: &mut jc2_file_formats::property_container::PropertyContainer,
            _thread: &mut crate::resource_loader::JcResourceThread,
        ) -> crate::resource_loader::JcResourceResult<()> {
            self.base().get_position().write(properties, "position");
            self.data.write(properties);
            Ok(())
        }
    }

    jc_property_struct! {
        pub struct JcBuoyancyModuleData {
            name("_name"): GString,

            #[group("Buoyancy Module")]
            #[subgroup("General")]
            drag: f32,
            #[update_gizmos(true)]
            radius: f32,
            #[update_gizmos(true)]
            side_area: f32,
            #[update_gizmos(true)]
            bottom_area: f32,
            up_velocity_cap: f32 = 20.0,
        }
    }

    jc_class! {
        #[JcClass("CGroupModule")]
        pub struct JcGroupModule {
            data: JcGroupModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcGroupModuleData {
            #[group("Group Module")]
            name("_name"): GString,

            #[subgroup("General")]
            export_grouped: bool,
            num_parts: i32,
        }
    }

    jc_class! {
        #[JcClass("CPartDamageModule")]
        pub struct JcPartDamageModule {
            data: JcPartDamageModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcPartDamageModuleData {
            name("_name"): GString,

            #[group("Part Damage Module")]
            #[subgroup("Health")]
            max_health: i32 = 100,
            damage_health: i32 = 50,

            #[subgroup("Settings")]
            break_at: i32 = -1,
            should_break: bool = false,
            always_enabled: bool = false,
            should_explode: bool = false,
        }
    }

    jc_class! {
        #[JcClass("CSuspensionModule")]
        pub struct JcSuspensionModule {
            data: JcSuspensionModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcSuspensionModuleData {
            name("_name"): GString,

            #[group("Suspension Module")]
            #[subgroup("Spring")]
            #[hint(RANGE, "0,70,1,prefer_slider")]
            strength: f32 = 30.0,
            #[hint(RANGE, "0,70,1,prefer_slider")]
            anti_roll_bar_strength: f32 = 0.0,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            length: f32 = 0.2,
            direction: Vector3 = Vector3::new(0.0, -1.0, 0.0),

            #[subgroup("Dampening")]
            #[hint(RANGE, "0,25,0.25,prefer_slider")]
            compression: f32 = 3.0,
            #[hint(RANGE, "0,25,0.25,prefer_slider")]
            relaxation: f32 = 2.0,
        }
    }

    jc_class! {
        #[JcClass("CVehicleDamageModule")]
        pub struct JcVehicleDamageModule {
            data: JcVehicleDamageModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcVehicleDamageModuleData {
            name("_name"): GString,

            #[group("Vehicle Damage Module")]
            #[subgroup("Health")]
            max_health: i32 = 200,

            #[subgroup("Armor")]
            explosion_threshold: i32 = 200,
            bullet_armor: i32 = 50,
            impact_armor: i32 = 50,
            explosion_armor: i32 = 50,

            #[subgroup("Deform")]
            deform_scale: f32 = 1.0 / 300.0,

            #[subgroup("Material Armor")]
            glass_armor: i32 = 10,
            metal_armor: i32 = 50,

            #[subgroup("Modifiers")]
            impact_damage_factor: f32 = 1.0,
        }
    }

    jc_class! {
        #[JcClass("CWheelModule")]
        pub struct JcWheelModule {
            data: JcWheelModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcWheelModuleData {
            name("_name"): GString,

            #[group("Wheel Module")]
            #[subgroup("Dimensions")]
            radius: f32 = 0.45,
            width: f32 = 0.30,
            mass: f32 = 20.0,

            #[subgroup("Friction")]
            friction: f32 = 1.0,
            max_friction: f32 = 2.0,
            viscosity: f32 = 0.04,
            sink_friction_coeff: f32,
            slip_angle: f32,

            #[subgroup("Damage")]
            bullet_health: f32 = 80.0,
            skew_health: f32 = 200.0,
            broken_radius_fraction: f32 = 0.7,
            broken_friction_fraction: f32 = 0.5,

            #[subgroup("Wheel Slot", "wheel_")]
            wheel_index("wheel_nr"): i32,
            #[hint(ENUM, "Front,Back")]
            wheel_axis: i32,
        }
    }
}

mod car {
    use super::*;

    jc_class_registry!(
        JcCarEngineModule,
        JcCarGlobalModule,
        JcCarPhysicalBaseModule,
        JcCarRaycastModule,
        JcCarSteeringModule,
        JcCarTransmissionModule,
    );

    jc_class! {
        #[JcClass("CCarEngineModule")]
        pub struct JcCarEngineModule {
            data: JcCarEngineModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarEngineModuleData {
            name("_name"): GString,

            #[group("Car Engine Module")]
            #[subgroup("RPM", "rpm_")]
            #[hint(RANGE, "0,10000,100,prefer_slider")]
            rpm_opt: f32 = 4500.0,
            #[hint(RANGE, "0,10000,100,prefer_slider")]
            rpm_min: f32 = 1000.0,
            #[hint(RANGE, "0,10000,100,prefer_slider")]
            rpm_max: f32 = 8000.0,
            #[hint(RANGE, "0,1000,10,prefer_slider")]
            rpm_powerband: i32 = 1000,
            #[hint(RANGE, "0,3000,100,prefer_slider")]
            rpm_clutch_slip("clutchsliprpm"): f32 = 0.0,

            #[subgroup("Torque", "torque_")]
            #[hint(RANGE, "0,10000,10,prefer_slider")]
            torque_opt: f32 = 350.0,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            torque_min("torque_at_min"): f32 = 0.75,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            torque_max("torque_at_max"): f32 = 75.0, // 0.75

            #[subgroup("Resistance", "resistance_")]
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            resistance_opt: f32 = 0.15,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            resistance_min: f32 = 0.1,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            resistance_max: f32 = 0.5,

            #[subgroup("Gearing")]
            downshift_rpm("gear_down_rpm"): GString = "1250, 1350, 1450, 1550".into(),
            upshift_rpm("gear_up_rpm"): GString = "2500, 2700, 2900, 3100".into(),
        }
    }

    jc_class! {
        #[JcClass("CCarGlobalModule")]
        pub struct JcCarGlobalModule {
            data: JcCarGlobalModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarGlobalModuleData {
            name("_name"): GString,

            #[group("Car Global Module")]
            #[subgroup("General")]
            #[hint(RANGE, "0,2,0.05,prefer_slider")]
            normal_spin_damping("NormalSpinDamping"): f32 = 0.25,
            handbrake_spin_damping("HandbrakeSpinDamping"): f32 = 3.0,
            #[hint(RANGE, "0,2,0.05,prefer_slider")]
            collision_spin_damping("CollisionSpinDamping"): f32 = 0.25,
            #[hint(RANGE, "0,10,0.1,prefer_slider")]
            collision_spin_threshold("CollisionSpinThreshold"): f32 = 4.0,
            #[hint(RANGE, "0,2,0.05,prefer_slider")]
            linear_damping("LinearDamping"): f32 = 0.0,
            #[hint(RANGE, "0,2,0.05,prefer_slider")]
            angular_damping("AngularDamping"): f32 = 0.25,
            #[hint(RANGE, "0,1,0.01,prefer_slider")]
            normal_clipping_angle("NormalClippingAngle"): f32 = 0.2,
            #[hint(RANGE, "1,2,0.1,prefer_slider")]
            air_density("AirDensity"): f32 = 1.3,
            #[hint(RANGE, "1,5,0.1,prefer_slider")]
            buoyancy("Boyancy"): f32 = 1.5,
            #[hint(RANGE, "-5,0,1,prefer_slider")]
            extra_gravity("ExtraGravity"): f32 = -4.0,

            #[subgroup("Analog Driver Input")]
            #[hint(RANGE, "0,1,0.05,prefer_slider")]
            slope_change_x("SlopeChangeX"): f32 = 0.75,
            #[hint(RANGE, "0,1,0.05,prefer_slider")]
            initial_slope("InitialSlope"): f32 = 0.5,
            #[hint(RANGE, "0,0.5,0.05,prefer_slider")]
            dead_zone("DeadZone"): f32 = 0.0,

            #[subgroup("Traction Control Input")]
            min_velocity_ratio("MinVelocityRatio"): f32 = 1.02,
            max_velocity_ratio("MaxVelocityRatio"): f32 = 1.1,
        }
    }

    jc_class! {
        #[JcClass("CCarPhysicalBaseModule")]
        pub struct JcCarPhysicalBaseModule {
            data: JcCarPhysicalBaseModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarPhysicalBaseModuleData {
            name("_name"): GString,

            #[group("Car Physical Base Module")]
            #[subgroup("General")]
            center_of_mass("CenterOfMass"): Vector3,
            #[hint(RANGE, "50,50000,50,prefer_slider")]
            chassis_mass("ChassiMass"): f32 = 2000.0,
            #[hint(RANGE, "0.1,10,0.05,prefer_slider")]
            inertia_pitch("InertiaPitch"): f32 = 2.0,
            #[hint(RANGE, "0.1,10,0.05,prefer_slider")]
            inertia_roll("InertiaRoll"): f32 = 2.0,
            #[hint(RANGE, "0.1,10,0.05,prefer_slider")]
            inertia_yaw("InertiaYaw"): f32 = 2.0,

            #[subgroup("Aerodynamics")]
            #[hint(RANGE, "0.1,10,0.05,prefer_slider")]
            frontal_area("FrontalArea"): f32 = 3.0,
            #[hint(RANGE, "0,2,0.01,prefer_slider")]
            drag_coefficient("DragCoefficent"): f32 = 1.0,
            #[hint(RANGE, "-2,2,0.01,prefer_slider")]
            lift_coefficient("LiftCoefficent"): f32 = 0.0,

            #[subgroup("Wheel")]
            #[hint(ENUM, "Normal,Sport,Offroad")]
            wheel_friction_type("wheelfrictiontype"): i32 = 0,
        }
    }

    jc_class! {
        #[JcClass("CCarRaycastModule")]
        pub struct JcCarRaycastModule {
            data: JcCarRaycastModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarRaycastModuleData {
            name("_name"): GString,

            #[group("Car Raycast Module")]
            #[subgroup("General")]
            #[hint(RANGE, "0,5,1,prefer_slider")]
            extra_torque_factor("ExtraTorqueFactor"): f32 = 2.0,
            #[hint(RANGE, "0,1,0.05,prefer_slider")]
            friction_equalizer("FrictionEqualizer"): f32 = 0.5,
            #[hint(RANGE, "0,10,1,prefer_slider")]
            max_velocity_for_positional_friction("MaxVelForPosFriction"): f32 = 1.0,
            #[hint(RANGE, "0,1.5,0.05,prefer_slider")]
            torque_factor_pitch("TorqueFactorPitch"): f32 = 0.75,
            #[hint(RANGE, "0,1.5,0.05,prefer_slider")]
            torque_factor_roll("TorqueFactorRoll"): f32 = 0.5,
            #[hint(RANGE, "0,1.5,0.05,prefer_slider")]
            torque_factor_yaw("TorqueFactorYaw"): f32 = 1.0,
        }
    }

    jc_class! {
        #[JcClass("CCarSteeringModule")]
        pub struct JcCarSteeringModule {
            data: JcCarSteeringModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarSteeringModuleData {
            name("_name"): GString,

            #[group("Car Steering Module")]
            #[subgroup("Steering")]
            #[hint(RANGE, "10,90,1,prefer_slider")]
            max_angle: f32 = 70.0,
            #[hint(RANGE, "0,100,1,prefer_slider")]
            max_speed_full_angle("maxspeed_full_angle"): f32 = 100.0,
        }
    }

    jc_class! {
        #[JcClass("CCarTransmissionModule")]
        pub struct JcCarTransmissionModule {
            data: JcCarTransmissionModuleData,
        }
    }

    jc_property_struct! {
        pub struct JcCarTransmissionModuleData {
            name("_name"): GString,

            #[group("Car Transmission Module")]
            #[subgroup("Ratios")]
            #[hint(RANGE, "0,1000,1,prefer_slider")]
            top_speed("TopSpeed"): f32 = 100.0,
            #[hint(RANGE, "0,1000,1,prefer_slider")]
            reverse_speed("ReverseSpeed"): f32 = 40.0,
            gear_ratios("GearRatios"): GString = "5.0, 4.0, 3.0, 1.0".into(),
            #[hint(RANGE, "0,6,1,prefer_slider")]
            gears("Gears"): i32 = 4,

            #[subgroup("GearBox")]
            #[hint(RANGE, "0,10,0.01,prefer_slider")]
            clutch_delay("ClutchDelay"): f32 = 0.2,
            #[hint(RANGE, "0,5,0.05,prefer_slider")]
            top_gear_ratio("topGearRatio"): f32 = 1.0,
            #[hint(RANGE, "0,100,1,prefer_slider")]
            fwd_ratio("FWDRatio"): f32 = 0.0,
        }
    }
}
