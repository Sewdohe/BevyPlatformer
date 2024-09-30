
use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entity_util::PhysObjectBundle;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
pub struct Wall;
#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub gravity: GravityScale,
    pub velocity: Velocity,
    pub rotation_constraints: LockedAxes,
    pub friction: Friction,
    pub damping: Damping,
    pub mass: ColliderMassProperties,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct PlatformBundle {
    pub collider: Collider,
    #[from_int_grid_cell]
    cell_instance: IntGridCell,
}

#[derive(Default, Bundle, LdtkIntCell)]
pub struct Platform {
    #[from_int_grid_cell]
    platform_bundle: PlatformBundle
}

#[derive(Clone, Default, Component)]
pub struct GroundDetection {
    pub on_ground: bool,
}

#[derive(Clone, Default, Bundle)]
pub struct SpringChild {
    pub transform: Transform,
    pub rigid_body: RigidBody
}

#[derive(Component)]
pub struct GroundSensor {
    pub ground_detection_entity: Entity,
    pub intersecting_ground_entities: HashSet<Entity>,
}


#[derive(Clone, Default, Bundle, LdtkIntCell)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

impl From<IntGridCell> for PlatformBundle {
    fn from(int_grid_cell: IntGridCell) -> PlatformBundle {
        println!("Registered platform");
        PlatformBundle {
            collider: Collider::cuboid(8., 8.),
            ..Default::default()
        }
    }
}
