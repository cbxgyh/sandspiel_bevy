#![allow(clippy::type_complexity)]
extern crate core;

mod boot;
mod convert_svg;
mod species;
mod universe;
mod utils;
mod render;
mod pipeline_c;
mod pipeline_reset;
mod pipeline_sand;
mod pipeline_update;

use bevy::prelude::*;
use bevy::render::{RenderApp, RenderPlugin};
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{PresentMode, WindowResolution};
use crate::pipeline_reset::{ResetPipelinePlugin};
use crate::pipeline_sand::PipelineSandPlugin;
use crate::species::Species;
use crate::universe::Universe;

#[derive(Resource)]
struct FluidConfig {
    texture_downsample: u32,
    density_dissipation: f32,
    velocity_dissipation: f32,
    pressure_dissipation: f32,
    pressure_iterations: u32,
    curl: f32,
    splat_radius: f32,
}
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameOfLifeState {
    #[default]
    Loading,
    Init,
    Update,
    Reset
}
fn main() {
    let mut app = App::new();

    app
        .add_plugins(
            DefaultPlugins.set(RenderPlugin {
                render_creation: WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                    .into(),
                ..default()
            })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "BevyMark".into(),
                        // resolution: WindowResolution::new(120.0, 80.0)
                        //     .with_scale_factor_override(1.0),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
            ,

        )
        .init_state::<GameOfLifeState>()
        .insert_resource(FluidConfig {
            texture_downsample: 0,
            density_dissipation: 0.98,
            velocity_dissipation: 0.99,
            pressure_dissipation: 0.8,
            pressure_iterations: 25,
            curl: 15.0,
            splat_radius: 0.005,
        })
        //local plugins
        .insert_resource(Universe::default())
        .add_plugins((
                         ResetPipelinePlugin,
                         PipelineSandPlugin,
                     ))

        .add_systems(Startup, setup);


    app.run();
}

fn setup(mut commands: Commands,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<ColorMaterial>>,
         mut time: ResMut<Time<Fixed>>) {
    time.set_timestep_hz(58.);

    let mut camera = Camera2dBundle{
        camera:Camera{
            clear_color: Color::WHITE.into(),
            ..default()
        },
        ..default()
    };
    // camera.camera.hdr = true;
    // camera.transform.scale.x = 0.23;
    // camera.transform.scale.y = 0.23;

    commands.spawn(camera);
}

