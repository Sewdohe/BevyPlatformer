pub mod entity_util {
    use bevy::prelude::*;
    use bevy_ecs_ldtk::prelude::*;
    use bevy_rapier2d::prelude::*;

    use crate::{phys::components::{ColliderBundle, GroundDetection}};

    pub struct EntityPlugin;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Component)]
    pub struct Player;


    #[derive(Bundle, Default, Clone, LdtkEntity)]
    pub struct PlatformBundle {

    }

    #[derive(Default, Component, Clone)]
    pub struct SpringPlatform;

    impl From<&EntityInstance> for ColliderBundle {
        fn from(entity_instance: &EntityInstance) -> ColliderBundle {
            let rotation_constraints = LockedAxes::ROTATION_LOCKED;
            match entity_instance.identifier.as_ref() {
                "Player" => ColliderBundle {
                    collider: Collider::cuboid(8., 8.),
                    rigid_body: RigidBody::Dynamic,
                    friction: Friction {
                        coefficient: 0.0,
                        combine_rule: CoefficientCombineRule::Min,
                    },
                    rotation_constraints,
                    ..Default::default()
                },
                "Crate" => ColliderBundle {
                    collider: Collider::cuboid(8., 8.),
                    rigid_body: RigidBody::Dynamic,
                    mass: ColliderMassProperties::Density(20.0),
                    friction: Friction {
                        coefficient: 3.0,
                        combine_rule: CoefficientCombineRule::Average,
                    },
                    ..Default::default()
                },
                "Spring_Platform" => ColliderBundle {
                    collider: Collider::cuboid(8., 8.),
                    rigid_body: RigidBody::Dynamic,
                    gravity: GravityScale(0.),
                    friction: Friction {
                        coefficient: 3.0,
                        combine_rule: CoefficientCombineRule::Average,
                    },
                    mass: ColliderMassProperties::Density(30.0),
                    damping: Damping {
                        linear_damping: 2.0,
                        angular_damping: 50.0
                    },
                    ..Default::default()
                },
                _ => ColliderBundle::default(),
            }
        }
    }

    #[derive(Bundle, Default, Clone, LdtkEntity)]
    pub struct PlayerBundle {
        // use the commented code to use a custom sprite
        // instead of the visual editors
        // #[sprite_bundle("slime.png")]
        // pub sprite_bundle: SpriteBundle,
        #[sprite_sheet_bundle]
        sprite_sheet_bundle: SpriteSheetBundle,
        #[from_entity_instance]
        pub collider_bundle: ColliderBundle,
        player: Player,
        pub ground_detection: GroundDetection,
        #[from_entity_instance]
        entity_instance: EntityInstance,
    }

    #[derive(Bundle, Default, Clone, LdtkEntity)]
    pub struct PhysObjectBundle {
        #[sprite_sheet_bundle]
        sprite_sheet_bundle: SpriteSheetBundle,
        #[from_entity_instance]
        pub collider_bundle: ColliderBundle,
        pub ground_detection: GroundDetection,
        #[from_entity_instance]
        entity_instance: EntityInstance,
        #[grid_coords]
        grid_coords: GridCoords,
    }

    #[derive(Bundle, Default, Clone, LdtkEntity)]
    pub struct SpringPlatformBundle {
        #[sprite_sheet_bundle]
        sprite_sheet_bundle: SpriteSheetBundle,
        #[from_entity_instance]
        pub collider_bundle: ColliderBundle,
        #[from_entity_instance]
        entity_instance: EntityInstance,
        #[grid_coords]
        grid_coords: GridCoords,
        spring_platform: SpringPlatform
    }



    // get an instance of any object with the "player" tag and check for input every frame.
    fn player_movement(
        _time: Res<Time>,                // grab delta time resource
        keys: Res<ButtonInput<KeyCode>>, // key keys resource to identify pressed keys
        mut player_entity: Query<(&mut Velocity, &GroundDetection, Option<&Player>)>, // instance of player's controller
    ) {
        for (mut velocity, ground_detection, mut _player) in &mut player_entity {
            let right = if keys.pressed(KeyCode::KeyD) { 1. } else { 0. };
            let left = if keys.pressed(KeyCode::KeyA) { 1. } else { 0. };


            if let Some(_player) = _player {
                // multiply constant by left/right value for movement.
                // cleaner than an if statement.
                velocity.linvel.x = (right - left) * 100.;

                if keys.just_pressed(KeyCode::Space) && ground_detection.on_ground {
                    // println!("jump!");
                    velocity.linvel.y = 300.;
                };
            }
        }
    }

    impl Plugin for EntityPlugin {
        fn build(&self, app: &mut App) {
            app
                .register_ldtk_entity::<PlayerBundle>("Player")
                .register_ldtk_entity::<PhysObjectBundle>("Crate")
                .register_ldtk_entity::<SpringPlatformBundle>("Spring_Platform")
                .add_systems(Update, player_movement);
        }
    }
}
