
use bevy::app::{App, Plugin};
use bevy::prelude::*;
struct LoadShaderPlugin;


impl Plugin for LoadShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            setup
        )

        ;
    }
}
fn setup ( mut commands: Commands, asset_server: Res<AssetServer>) {
    let base_vertex_shader = asset_server.load("baseVertex.wgsl");
    let clear_shader = asset_server.load("clear.wgsl");
    let display_shader = asset_server.load("display.wgsl");
    let splat_shader = asset_server.load("splat.wgsl");
    let advection_manual_filtering_shader = asset_server.load("advectionManualFilter.wgsl");
    let advection_shader = asset_server.load("advection.wgsl");
    let divergence_shader = asset_server.load("divergence.wgsl");
    let curl_shader = asset_server.load("curl.wgsl");
    let vorticity_shader = asset_server.load("vorticity.wgsl");
    let pressure_shader = asset_server.load("pressure.wgsl");
    let gradient_subtract_shader = asset_server.load("gradientSubtract.wgsl");
    let velocity_out_shader = asset_server.load("velocityOut.wgsl");


    // 将着色器存储在世界资源中，方便其他系统使用
    commands.insert_resource(LoadShaders{
        base_vertex_shader,
        clear_shader,
        display_shader,
        splat_shader,
        advection_manual_filtering_shader,
        advection_shader,
        divergence_shader,
        curl_shader,
        vorticity_shader,
        pressure_shader,
        gradient_subtract_shader,
        velocity_out_shader
    });
}
#[derive(Resource)]
struct LoadShaders {
    base_vertex_shader: Handle<Shader>,
    clear_shader: Handle<Shader>,
    display_shader: Handle<Shader>,
    splat_shader: Handle<Shader>,
    advection_manual_filtering_shader: Handle<Shader>,
    advection_shader: Handle<Shader>,
    divergence_shader: Handle<Shader>,
    curl_shader: Handle<Shader>,
    vorticity_shader: Handle<Shader>,
    pressure_shader: Handle<Shader>,
    gradient_subtract_shader: Handle<Shader>,
    velocity_out_shader: Handle<Shader>,
}

fn load_shader(path: &str) -> String {
    std::fs::read_to_string(path).expect("Failed to read shader file")
}
#[derive(Resource)]
pub struct LoadFont {
    pub font1: Handle<Font>,
    pub font2: Handle<Font>,
    pub font3: Handle<Font>,
    pub font4: Handle<Font>,
}

fn load_font(  mut commands: Commands,  asset_server: Res<AssetServer>) {
    commands.insert_resource(
        LoadFont{
            font1:asset_server.load("ChiKareGo2.ttf"),
            font2:asset_server.load("PPMondwest-Regular.otf"),
            font3:asset_server.load("PPNeueBit-Bold.otf"),
            font4:asset_server.load("PPNeueBit-Regular.otf"),
        }
    );
}
