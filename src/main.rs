use bevy::{
    prelude::*,
    render::{
        settings::{PowerPreference, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_ecs_ldtk::prelude::*;
mod entity;
mod level;
pub use crate::entity::entity_util;
pub use crate::level::level_utils;
pub use crate::phys::components;

mod phys;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: false,
                    render_creation: WgpuSettings {
                        power_preference: PowerPreference::LowPower,
                        ..Default::default()
                    }
                    .into(),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(level_utils::MapLoaderPlugin)
        .add_plugins(phys::PhysicsPlugin)
        .add_plugins(entity_util::EntityPlugin)
        .insert_resource(LevelSelection::index(0))
        .run();
}
