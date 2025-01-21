#![allow(clippy::type_complexity)]

mod boot;
mod convert_svg;
mod species;
mod category;
mod utils;
mod render;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::{PresentMode, WindowResolution};





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
        //local plugins


        .add_systems(Startup, setup);



    app.run();
}

fn setup(mut commands: Commands, mut time: ResMut<Time<Fixed>>) {
    time.set_timestep_hz(58.);

    let mut camera = Camera2dBundle::default();
    camera.camera.hdr = true;
    camera.transform.scale.x = 0.23;
    camera.transform.scale.y = 0.23;

    commands.spawn(camera);
}

