use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::camera::RenderTarget;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::view::RenderLayers;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::PrimaryWindow;


// 流体组件
#[derive(Component)]
struct FluidComponent {
    density: f32,
    velocity: Vec2,
    pressure: f32,
    divergence: f32,
    curl: f32,
    // 存储纹理句柄，用于流体的各种状态
    density_texture: Handle<Image>,
    velocity_texture: Handle<Image>,
    pressure_texture: Handle<Image>,
    // 存储其他纹理，如燃烧、细胞等
    burns_texture: Handle<Image>,
    cells_texture: Handle<Image>,
}


// 指针组件
#[derive(Component)]
struct PointerComponent {
    id: u64,
    position: Vec2,
    velocity: Vec2,
    color: Color,
    down: bool,
    moved: bool,
}


// 流体配置资源
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


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FluidConfig {
            texture_downsample: 0,
            density_dissipation: 0.98,
            velocity_dissipation: 0.99,
            pressure_dissipation: 0.8,
            pressure_iterations: 25,
            curl: 15.0,
            splat_radius: 0.005,
        })
        // .add_systems(setup)
        // .add_system(update_system)
        // // .add_system(event_handler_system)
        // .add_system(gui_system)
        .run();
}


fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {


    // 创建流体实体
    let size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };
    let density_texture = images.add(Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD
    //     TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
    ));
    let velocity_texture = images.add(Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD
    ));
    let pressure_texture = images.add(Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD
    ));
    let burns_texture = images.add(Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD
    ));
    let cells_texture = images.add(Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD
    ));


    commands.spawn(FluidComponent {
        density: 0.0,
        velocity: Vec2::ZERO,
        pressure: 0.0,
        divergence: 0.0,
        curl: 0.0,
        density_texture: density_texture.clone(),
        velocity_texture: velocity_texture.clone(),
        pressure_texture: pressure_texture.clone(),
        burns_texture: burns_texture.clone(),
        cells_texture: cells_texture.clone(),
    });


    // 创建初始指针
    commands.spawn(PointerComponent {
        id: 0,
        position: Vec2::ZERO,
        velocity: Vec2::ZERO,
        color: Color::WHITE,
        down: false,
        moved: false,
    });


    // 创建网格
    let quad_handle = meshes.add(
        Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0))));


    // 创建材质
    let material_handle = materials.add(ColorMaterial::from(Color::WHITE));


    // 创建渲染目标
    let render_target = RenderTarget::from(density_texture.clone());


    // 创建流体精灵
    commands.spawn(MaterialMesh2dBundle {
        mesh: quad_handle.clone().into(),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}


fn update_system(
    mut fluid_query: Query<&mut FluidComponent>,
    mut pointer_query: Query<&mut PointerComponent>,
    time: Res<Time>,
    mut images: ResMut<Assets<Image>>,
    config: Res<FluidConfig>,
) {
    let mut fluid = fluid_query.single_mut();
    let mut pointer = pointer_query.single_mut();


    // 时间差计算
    let dt = time.delta_seconds();


    // 流体模拟更新逻辑
    // 这里可以根据物理方程更新流体的速度、密度、压力等
    // 例如，简单的更新逻辑如下：
    fluid.velocity += Vec2::new(1.0, 1.0) * dt * config.velocity_dissipation;
    fluid.density += 0.1 * dt * config.density_dissipation;
    fluid.pressure += 0.2 * dt * config.pressure_dissipation;


    // 平流更新
    // 这里需要更复杂的计算，可能需要使用着色器和纹理采样
    // 暂时使用简单的更新作为示例
    if let Some(velocity_image) = images.get_mut(fluid.velocity_texture.clone()) {
        // 模拟平流更新
        for pixel in velocity_image.data.iter_mut() {
            *pixel = (*pixel as u32 * 254 / 255) as u8;
        }
    }


    if let Some(density_image) = images.get_mut(fluid.density_texture.clone()) {
        // 模拟密度更新
        for pixel in density_image.data.iter_mut() {
            *pixel = (*pixel as u32 * 253 / 255) as u8;
        }
    }


    if let Some(pressure_image) = images.get_mut(fluid.pressure_texture.clone()) {
        // 模拟压力更新
        for pixel in pressure_image.data.iter_mut() {
            *pixel = (*pixel as u32 * 252 / 255) as u8;
        }
    }


    // 旋度和散度更新
    fluid.curl += 0.1 * dt;
    fluid.divergence += 0.05 * dt;


    // 处理指针输入的影响，例如 splat 操作
    if pointer.down && pointer.moved {
        splat(
            &mut fluid,
            pointer.position,
            pointer.velocity,
            pointer.color,
            config.splat_radius,
            &mut images,
        );
        pointer.moved = false;
    }
}


fn splat(
    fluid: &mut FluidComponent,
    position: Vec2,
    velocity: Vec2,
    color: Color,
    radius: f32,
    images: &mut ResMut<Assets<Image>>,
) {
    // 这里需要实现更复杂的 splat 操作，可能需要使用着色器和纹理采样
    // 暂时使用简单的更新作为示例
    if let Some(density_image) = images.get_mut(fluid.density_texture.clone()) {
        let x = position.x as u32;
        let y = position.y as u32;
        let radius = (radius * 10.0) as u32;


        for i in (x - radius)..=(x + radius) {
            for j in (y - radius)..=(y + radius) {
                if i < density_image.width() && j < density_image.height() {
                    let index = (j * density_image.width() + i) * 4;
                    density_image.data[index as usize] = (color.r() * 255.0) as u8;
                    density_image.data[index as usize + 1 ] = (color.g() * 255.0) as u8;
                    density_image.data[index as usize + 2] = (color.b() * 255.0) as u8;
                    density_image.data[index as usize+ 3] = 255;
                }
            }
        }
    }


    if let Some(velocity_image) = images.get_mut(fluid.velocity_texture.clone()) {
        let x = position.x as u32;
        let y = position.y as u32;
        let radius = (radius * 10.0) as u32;


        for i in (x - radius)..=(x + radius) {
            for j in (y - radius)..=(y + radius) {
                if i < velocity_image.width() && j < velocity_image.height() {
                    let index = (j * velocity_image.width() + i) * 4;
                    velocity_image.data[index as usize] = (velocity.x * 255.0) as u8;
                    velocity_image.data[index as usize+ 1] = (velocity.y * 255.0) as u8;
                    velocity_image.data[index as usize+ 2] = 0;
                    velocity_image.data[index as usize+ 3] = 255;
                }
            }
        }
    }
}


// fn event_handler_system(
//     mut pointer_query: Query<&mut PointerComponent>,
//     mut mouse_button_input: EventReader<ButtonInput<MouseButton>>,
//     mut touch_input: EventReader<ButtonInput<TouchInput>>,
//     primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
// )
// {
//     let mut pointer = pointer_query.single_mut();
//
//
//     // 获取窗口尺寸
//     let window = primary_window.get_primary().unwrap();
//
//
//     // 处理鼠标事件
//     if mouse_button_input.just_pressed(MouseButton::Left) {
//         pointer.down = true;
//     } else if mouse_button_input.just_released(MouseButton::Left) {
//         pointer.down = false;
//     }
//
//
//     // 处理触摸事件
//     for touch in touch_input.get_just_pressed() {
//         pointer.down = true;
//         pointer.id = touch.id;
//     }
//     for touch in touch_input.get_just_released() {
//         if pointer.id == touch.id {
//             pointer.down = false;
//         }
//     }
//
//
//     // 处理鼠标和触摸的移动事件
//     if pointer.down {
//         if let Some(position) = window.cursor_position() {
//             let prev_position = pointer.position;
//             pointer.position = position;
//             pointer.velocity = position - prev_position;
//             pointer.moved = true;
//         }
//     }
// }


fn gui_system(
    // mut egui_context: ResMut<EguiContext>,
    mut config: ResMut<FluidConfig>,
) {
    // egui::Window::new("Fluid Settings")
    //     .show(egui_context.ctx_mut(), |ui| {
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.texture_downsample,
    //                 0..=2,
    //             )
    //                 .text("Texture Downsample"),
    //         );
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.density_dissipation,
    //                 0.9..=1.0,
    //             )
    //                 .text("Density Dissipation"),
    //         );
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.velocity_dissipation,
    //                 0.9..=1.0,
    //             )
    //                 .text("Velocity Dissipation"),
    //         );
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.pressure_dissipation,
    //                 0.0..=1.0,
    //             )
    //                 .text("Pressure Dissipation"),
    //         );
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.pressure_iterations,
    //                 1..=60,
    //             )
    //                 .text("Pressure Iterations"),
    //         );
    //         ui.add(
    //             egui::Slider::new(&mut config.curl, 0.0..=50.0)
    //                 .text("Curl"),
    //         );
    //         ui.add(
    //             egui::Slider::new(
    //                 &mut config.splat_radius,
    //                 0.0001..=0.01,
    //             )
    //                 .text("Splat Radius"),
    //         );
    //     });
}