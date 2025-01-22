use std::num::NonZeroU64;
use bevy::core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state;
use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingResource, BindingType, BufferBinding, BufferBindingType, BufferSize, CachedRenderPipelineId, ColorTargetState, ColorWrites, Extent3d, FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, SamplerBindingType, ShaderStage, ShaderStages, ShaderType, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureViewDimension, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use crate::render::load_shader::LoadFont;
struct FluidPlugin;

impl Plugin for FluidPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup,
                         setup
            )
            .add_systems(
                Update,
                (update_gui,handle_pointer_input)
            )
        ;
    }
}

// FluidConfig 和 FluidState:
//
// FluidConfig 用于存储流体模拟的配置信息，如纹理下采样、密度扩散、速度扩散、压力扩散、压力迭代次数等。
// FluidState 存储了一个 splat_stack，用于存储涂抹事件（splat）的堆栈。
// Pointer:
//
// Pointer 用于处理鼠标指针事件（位置、速度、按下状态等）。
// setup 和 setup_gui:
//
// 在 setup 函数中，我们设置了 Fluid 配置并启动了 GUI。
// setup_gui 创建了一个简单的按钮，用户可以点击这个按钮触发 "Random splats" 的操作，模拟涂抹事件。
// button_click_handler:
//
// 这是一个简单的按钮点击处理函数，当按钮被点击时，fluid_state.splat_stack 中会加入一个随机值，模拟涂抹效果。
// handle_pointer_input:
//
// 处理鼠标输入，计算鼠标的移动并存储到 pointers 中，同时记录鼠标按下状态。
// 动态更新 GUI:
//
// 在 update_gui 中，你可以设置其他控制参数（如下采样、扩散率等），这可以通过 Bevy 的 UI 系统进行动态调整。
#[derive(Resource)]
struct FluidConfig {
    texture_downsample: f32,
    density_dissipation: f32,
    velocity_dissipation: f32,
    pressure_dissipation: f32,
    pressure_iterations: u32,
    curl: f32,
    splat_radius: f32,
}
#[derive(Default)]
struct Pointer {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    down: bool,
    moved: bool,
}

#[derive(Default,Resource)]
struct PointerVec{
    pointer_vec: Vec<Pointer>,
}
#[derive(Default,Resource)]
struct FluidState {
    splat_stack: Vec<i32>,
}
#[derive(Resource)]
struct FluidTextures {
    velocity: Handle<Texture>,
    density: Handle<Texture>,
    divergence: Handle<Texture>,
    curl: Handle<Texture>,
    pressure: Handle<Texture>,
    burns: Handle<Texture>,
    cells: Handle<Texture>,
}

fn setup(
        mut commands: Commands,
        asset_server: &mut ResMut<Assets<Texture>,
            load_font:Res<LoadFont>
) {
    commands.insert_resource(FluidConfig {
        texture_downsample: 0.0,
        density_dissipation: 0.98,
        velocity_dissipation: 0.99,
        pressure_dissipation: 0.8,
        pressure_iterations: 25,
        curl: 15.0,
        splat_radius: 0.005,
    });
    commands.init_resource::<FluidState>();
    setup_gui(&mut commands,load_font);

    // 创建和加载帧缓冲区纹理
    let velocity_texture = create_texture(&mut asset_server, 512, 512, TextureFormat::Rgba16Float);
    let density_texture = create_texture(&asset_server, 512, 512, TextureFormat::Rgba16Float);

    commands.insert_resource(FluidTextures {
        velocity: velocity_texture,
        density: density_texture,
        divergence: create_texture(&asset_server, 512, 512, TextureFormat::R16Float),
        curl: create_texture(&asset_server, 512, 512, TextureFormat::R16Float),
        pressure: create_texture(&asset_server, 512, 512, TextureFormat::R16Float),
        burns: create_texture(&asset_server, 512, 512, TextureFormat::Rgba8Unorm),
        cells: create_texture(&asset_server, 512, 512, TextureFormat::Rgba8Unorm),
    });
}
fn setup_gui(commands: &mut Commands,load_font:Res<LoadFont>) {
    let mut gui = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..Default::default()
        },
        ..Default::default()
    });

    gui.with_children(|parent| {
        parent.spawn(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(Name::new("Random Splats"))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Random splats",
                        TextStyle {
                            font: load_font.font1.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        }
                    )
                );
            })
        ;
    });
}

fn button_click_handler(
    mut fluid_state: ResMut<FluidState>,
    mut interaction_query: Query<(&Interaction, &mut Text), With<Button>>,
) {
    for (interaction, _) in &mut interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            fluid_state.splat_stack.push(rand::random::<i32>() % 20 + 5);
        }
    }
}

fn update_gui(mut config: ResMut<FluidConfig>, mut commands: Commands) {
    // Handle dynamic GUI updates (use sliders, checkboxes, etc)
    // For example, setting values for texture downsampling, density dissipation, etc.
    // Bevy's UI system allows for reactive and interactive settings.
}

fn handle_pointer_input(
    mut pointers:   ResMut<PointerVec>,
    mut fluid_state: ResMut<FluidState>,
    mut mouse_input: ResMut<ButtonInput<MouseButton>>,
    mut mouse_motion:  EventReader<CursorMoved>,
) {
    for enent in mouse_motion.read() {
        let mut pointer = Pointer::default();
        pointer.x = enent.position.x;
        pointer.y = enent.position.y;

        // Handle mouse button press/release
        if mouse_input.pressed(MouseButton::Left) {
            pointer.down = true;
        } else {
            pointer.down = false;
        }

        // Add pointer movement logic
        if pointer.moved {
            pointer.dx = pointer.x - pointer.dx;
            pointer.dy = pointer.y - pointer.dy;
        }

        pointers.pointer_vec.push(pointer);
    }

}

fn create_texture(mut textures: ResMut<Assets<Texture>>,
                  width: u32, height: u32,
                  format: TextureFormat,
                  render_device:RenderDevice,
) -> Handle<Texture> {
    textures.add(render_device.create_texture(&TextureDescriptor {
        label: None,
        size: Extent3d {
            width: width,
            height: height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: format,  // 纹理格式
        usage: TextureUsages::COPY_DST,  // 纹理的使用方式，采样和复制目标
        view_formats: &[],   // 视图格式（通常不需要设置）
        ..Default::default()
    }))

}
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct ClearUniform {
    value: f32,
}
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct VertexInput {
    position : Vec2
}
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct SplatUniform {
    aspect_ratio: f32,
    color: Vec3,
    point: Vec2,
    radius: f32
}
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct AdvectionUniform {
    texel_size : Vec2,
    dt : f32,
    dissipation : f32
}
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
struct SandUniform {
    t: f32,
    dpi: f32,
    resolution: Vec2,
    // 0 1
    is_snapshot: u32
}
#[derive(Resource)]
struct ResetPipeline {
    reset_bind_group_layout: BindGroupLayout,
    init_reset_pipeline: CachedRenderPipelineId,
    init_display_pipeline: CachedRenderPipelineId,
    init_velocity_out_pipeline: CachedRenderPipelineId,
    init_splat_pipeline: CachedRenderPipelineId,
    init_advection_pipeline: CachedRenderPipelineId,
    init_divergence_pipeline: CachedRenderPipelineId,
    init_curl_pipeline: CachedRenderPipelineId,
    init_vorticity_pipeline: CachedRenderPipelineId,
    init_pressure_pipeline: CachedRenderPipelineId,
    init_gradient_subtract_pipeline: CachedRenderPipelineId,
    init_sand_subtract_pipeline: CachedRenderPipelineId,
}
impl FromWorld for ResetPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let entries=[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry{
                binding: 2,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                },
                count: None,
            },
        ];
        let layout = render_device.create_bind_group_layout(
            None,
            &entries,
        );
        let clear_shader = world
            .resource::<AssetServer>()
            .load("clear.wgsl");
        let base_vertex_shader = world
            .resource::<AssetServer>()
            .load("baseVertex.wgsl");
        let display_shader = world
            .resource::<AssetServer>()
            .load("display.wgsl");
        let velocity_out_shader = world
            .resource::<AssetServer>()
            .load("velocityOut.wgsl");

        let splat_out_shader = world
            .resource::<AssetServer>()
            .load("splat.wgsl");
        let advection_shader = world
            .resource::<AssetServer>()
            .load("advection.wgsl");

        let divergence_shader = world
            .resource::<AssetServer>()
            .load("divergence.wgsl");
        let advection_shader = world
            .resource::<AssetServer>()
            .load("advection.wgsl");

        let divergence_shader = world
            .resource::<AssetServer>()
            .load("divergence.wgsl");

        let curl_shader = world
            .resource::<AssetServer>()
            .load("curl.wgsl");

        let pressure_shader = world
            .resource::<AssetServer>()
            .load("pressure.wgsl");

        let gradient_subtract_shader = world
            .resource::<AssetServer>()
            .load("gradientSubtract.wgsl");

        let clear_layout = render_device.create_bind_group_layout(
            "clear_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    uniform_buffer::<ClearUniform>(false),
                ),
            ),
        );

        let base_vertex_layout =render_device.create_bind_group_layout(
            "base_vertex_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::VERTEX,
                (
                    // The screen texture
                    uniform_buffer::<VertexInput>(false),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                ),
            ),
        );
        let display_layout = render_device.create_bind_group_layout(
            "display_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );
        let velocity_out_layout = render_device.create_bind_group_layout(
            "velocity_out",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );
        let splat_layout = render_device.create_bind_group_layout(
            "splat_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<SplatUniform>(false),
                ),
            ),
        );
        let advection_layout = render_device.create_bind_group_layout(
            "advection_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<AdvectionUniform>(false),
                ),
            ),
        );
        let divergencen_layout = render_device.create_bind_group_layout(
            "divergencen_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );
        let advection_layout = render_device.create_bind_group_layout(
            "advection_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<AdvectionUniform>(false),
                ),
            ),
        );
        let divergencen_layout = render_device.create_bind_group_layout(
            "divergencen_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                ),
            ),
        );

        let curl_layout = render_device.create_bind_group_layout(
            "curl_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                ),
            ),
        );

        let pressure_layout = render_device.create_bind_group_layout(
            "pressure_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                ),
            ),
        );

        let gradient_subtract_layout = render_device.create_bind_group_layout(
            "gradient_subtract_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture,
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                ),
            ),
        );
        let formats = vec![
            // Position
            VertexFormat::Float32x2,
            // Color
            VertexFormat::Float32x2,
        ];
        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);
        let init_reset_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("clear_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),clear_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: clear_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "fragment".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let init_display_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("display_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),display_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: display_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let init_velocity_out_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("velocity_out_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),velocity_out_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: velocity_out_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });
        let init_splat_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("splat_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),splat_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: splat_out_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });
        let init_advection_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_advection_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),advection_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: advection_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let init_divergence_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_divergence_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),divergencen_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: divergence_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let init_curl_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_curl_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),curl_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: curl_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let init_pressure_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_pressure_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),pressure_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: pressure_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });


        let init_gradient_subtract_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_gradient_subtract_pipeline".into()),
                layout: vec![base_vertex_layout.clone(),gradient_subtract_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![vertex_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: gradient_subtract_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        let sand_vertex_layout =render_device.create_bind_group_layout(
            "sand_vertex_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::VERTEX,
                (
                    // The screen texture
                    uniform_buffer::<VertexInput>(false),
                ),
            ),
        );
        let sand_layout =render_device.create_bind_group_layout(
            "sand_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    // The screen texture
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    uniform_buffer::<SandUniform>(false),
                ),
            ),
        );

        let sand_shader = world
            .resource::<AssetServer>()
            .load("sand.wgsl");

        let sand_vertex_shader = world
            .resource::<AssetServer>()
            .load("sandVertex.wgsl");


        let sand_buffer_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, vec![
                // Position
                VertexFormat::Float32x2,
            ]);

        let init_sand_subtract_pipeline = world
            .resource_mut::<PipelineCache>()
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("init_sand_subtract_pipeline".into()),
                layout: vec![sand_vertex_layout.clone(),sand_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: sand_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![sand_buffer_layout],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: sand_shader.clone(),
                    shader_defs: vec![],
                    // Make sure this matches the entry point of your shader.
                    // It can be anything as long as it matches here and in the shader.
                    entry_point: "main".into(),
                    targets: vec![Some(ColorTargetState {
                        format: TextureFormat::bevy_default(),
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                // All of the following properties are not important for this effect so just use the default values.
                // This struct doesn't have the Default trait implemented because not all field can have a default value.
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                push_constant_ranges: vec![],
            });

        ResetPipeline {
            reset_bind_group_layout: layout,
            init_reset_pipeline,
            init_display_pipeline,
            init_velocity_out_pipeline,
            init_splat_pipeline,
            init_advection_pipeline,
            init_divergence_pipeline,
            init_curl_pipeline,
            init_vorticity_pipeline,
            init_pressure_pipeline,
            init_gradient_subtract_pipeline,
            init_sand_subtract_pipeline

        }
    }
}
#[derive(Resource)]
struct DisplayPipeline {
    display_bind_group_layout: BindGroupLayout,
    init_display_pipeline: CachedRenderPipelineId,
}
impl FromWorld for crate::render::fluid::ResetPipeline {
    fn from_world(world: &mut World) -> Self {


    }
}


fn reset(
    mut commands: Commands,
    fluid_textures: Res<FluidTextures>,
    fluid_state: Res<FluidState>,
    world: &mut World,

)
{
    let render_device = world.resource::<RenderDevice>();
    let entries=[
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: 1,
            visibility: ShaderStages::FRAGMENT,
            ty: BindingType::Texture {
                sample_type: TextureSampleType::Float { filterable: true },
                view_dimension: TextureViewDimension::D2,
                multisampled: false,
            },
            count: None,
        },
        BindGroupLayoutEntry{
            binding: 2,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
            },
            count: None,
        },
    ];
    let layout = render_device.create_bind_group_layout(
        None,
        &entries,
        );

    let clear_shader = Shader::from_wgsl(include_str!("clear.wgsl"));
    let render_pipeline_descriptor=RenderPipelineDescriptor{
        label: None,
        layout: vec![layout.clone()],
        fragment: Some(FragmentState {
            shader,
            shader_defs: vec![],
            entry_point: "fragment".into(),
            targets: vec![Some(ColorTargetState {
                format: TextureFormat::bevy_default(),
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
        }),
        ..Default::default()
    };
    let value_data: [f32; 1] = [0.0]; // 默认值
    let value_buffer = render_device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Value Buffer"),
        contents: bytemuck::cast_slice(&value_data),
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let x=render_device.create_bind_group(
            Some("Burns Bind Group"),
            &layout,
            &[
                BindGroupEntry {
                    binding:0,
                    resource: BindingResource::TextureView(fluid_textures.divergence),
                },
                BindGroupEntry {
                    binding:1,
                    resource: BindingResource::TextureView(fluid_textures.divergence),
                },
                BindGroupEntry {
                    binding: 2, // 绑定点 2
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: value_buffer, // 指向存储 uniform 的缓冲区
                        offset: 0,            // 偏移量，通常是 0
                        size: None,           // 绑定整个缓冲区
                    }),
                },
            ]

    );
    let burns_bind_group = BindGroupDescriptor {
        label: Some("Burns Bind Group"),
        entries: vec![
            // Example binding for the burns texture
            BindGroupEntry {
                binding: 0,
                resource: burns_texture.view().into(),
            },
            // Add more bindings for other textures...
        ],
        ..Default::default()
    };

    // Example: reset the burns texture
    commands.spawn((
        burns_texture.clone(),
        burns_bind_group,
        pipeline,
    ));

    // Repeat the same for density, pressure, and other textures...
}


fn update(
    time: Res<Time>,
    mut fluid_textures: ResMut<FluidTextures>,
    fluid_state: Res<FluidState>,
    config: Res<FluidConfig>,
    mut commands: Commands,
    device: Res<wgpu::Device>,
    queue: Res<wgpu::Queue>,
) {
    // 时间差，用于控制每一帧的时间步长
    let dt = time.delta_seconds();

    // 获取流体模拟的纹理
    let velocity_texture = &fluid_textures.velocity;
    let density_texture = &fluid_textures.density;
    let pressure_texture = &fluid_textures.pressure;
    let burns_texture = &fluid_textures.burns;
    let cells_texture = &fluid_textures.cells;
    let divergence_texture = &fluid_textures.divergence;
    let curl_texture = &fluid_textures.curl;

    // 1. Advection - 速度和密度平流
    let advection_shader = Shader::from_wgsl(include_str!("advection.wgsl"));
    let advection_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        vertex: ShaderStage::from(advection_shader.clone()),
        fragment: Some(ShaderStage::from(advection_shader)),
        layout: Some(vec![
            // 绑定的 Layout: 用于绑定纹理和其他数据
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStage::FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float,
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            // 更多绑定项（如时间步长、参数等）
        ]),
        ..Default::default()
    });

    let advection_bind_group = BindGroup::new(device, &advection_pipeline, &vec![
        // 绑定纹理
        BindGroupEntry {
            binding: 0,
            resource: velocity_texture.view(),
        },
        BindGroupEntry {
            binding: 1,
            resource: velocity_texture.view(),
        },
        // 绑定时间步长
        BindGroupEntry {
            binding: 2,
            resource: BufferBinding {
                buffer: dt.into(),
                offset: 0,
                size: wgpu::BufferSize::new(),
            },
        },
    ]);

    // 执行平流计算，将结果写入到 `velocity.write[1]`
    commands.spawn_bundle(RenderPipeline::new(advection_pipeline))
        .insert(advection_bind_group);

    // 2. 更新燃烧纹理
    queue.write_texture(
        burns_texture,
        &fluid_state.burns_data,
        TextureDataLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: None,
        },
        TextureSize {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    // 更新细胞纹理
    queue.write_texture(
        cells_texture,
        &fluid_state.cells_data,
        TextureDataLayout {
            offset: 0,
            bytes_per_row: Some(width * 4),
            rows_per_image: None,
        },
        TextureSize {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );

    // 3. CURL 计算
    let curl_shader = Shader::from_wgsl(include_str!("curl.wgsl"));
    let curl_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        vertex: ShaderStage::from(curl_shader.clone()),
        fragment: Some(ShaderStage::from(curl_shader)),
        layout: Some(vec![/* BindGroup entries for curl */]),
        ..Default::default()
    });

    let curl_bind_group = BindGroup::new(device, &curl_pipeline, &vec![
        BindGroupEntry {
            binding: 0,
            resource: velocity_texture.view(),
        },
        BindGroupEntry {
            binding: 1,
            resource: curl_texture.view(),
        },
    ]);

    commands.spawn_bundle(RenderPipeline::new(curl_pipeline))
        .insert(curl_bind_group);

    // 4. Vorticity 计算
    let vorticity_shader = Shader::from_wgsl(include_str!("vorticity.wgsl"));
    let vorticity_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        vertex: ShaderStage::from(vorticity_shader.clone()),
        fragment: Some(ShaderStage::from(vorticity_shader)),
        layout: Some(vec![/* BindGroup entries for vorticity */]),
        ..Default::default()
    });

    let vorticity_bind_group = BindGroup::new(device, &vorticity_pipeline, &vec![
        BindGroupEntry {
            binding: 0,
            resource: velocity_texture.view(),
        },
        BindGroupEntry {
            binding: 1,
            resource: curl_texture.view(),
        },
    ]);

    commands.spawn_bundle(RenderPipeline::new(vorticity_pipeline))
        .insert(vorticity_bind_group);

    // 5. DIVERGENCE 计算
    let divergence_shader = Shader::from_wgsl(include_str!("divergence.wgsl"));
    let divergence_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        vertex: ShaderStage::from(divergence_shader.clone()),
        fragment: Some(ShaderStage::from(divergence_shader)),
        layout: Some(vec![/* BindGroup entries for divergence */]),
        ..Default::default()
    });

    let divergence_bind_group = BindGroup::new(device, &divergence_pipeline, &vec![
        BindGroupEntry {
            binding: 0,
            resource: velocity_texture.view(),
        },
        BindGroupEntry {
            binding: 1,
            resource: divergence_texture.view(),
        },
    ]);

    commands.spawn_bundle(RenderPipeline::new(divergence_pipeline))
        .insert(divergence_bind_group);

    // 6. PRESSURE 计算
    let pressure_shader = Shader::from_wgsl(include_str!("pressure.wgsl"));
    let pressure_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
        vertex: ShaderStage::from(pressure_shader.clone()),
        fragment: Some(ShaderStage::from(pressure_shader)),
        layout: Some(vec![/* BindGroup entries for pressure */]),
        ..Default::default()
    });

    let pressure_bind_group = BindGroup::new(device, &pressure_pipeline, &vec![
        BindGroupEntry {
            binding: 0,
            resource: divergence_texture.view(),
        },
        BindGroupEntry {
            binding: 1,
            resource: pressure_texture.view(),
        },
    ]);

    commands.spawn_bundle(RenderPipeline::new(pressure_pipeline))
        .insert(pressure_bind_group);

    // 7. Splat - 涂抹操作
    for pointer in fluid_state.splat_stack.iter() {
        splat(pointer.x, pointer.y, pointer.dx, pointer.dy, pointer.color);
    }
}

struct Splat {
    position: Vec2,
    velocity: Vec2,
    color: Color,
    radius: f32,
}

fn splat(
    mut commands: Commands,
    splat_position: Vec2,
    splat_velocity: Vec2,
    splat_color: Color,
    velocity_texture: Handle<Texture>,
    density_texture: Handle<Texture>,
    ui_size: f32,  // 假设 UI 状态中的 size 用来表示喷溅的大小
    aspect_ratio: f32,  // 画布的宽高比
) {
    // 定义 Spray shader 程序
    let splat_shader = Shader::from_wgsl(include_str!("splat_shader.wgsl"));

    // 创建管道
    let splat_pipeline = commands.spawn_bundle(PipelineDescriptor {
        vertex: Shader::from_wgsl(include_str!("vertex_shader.wgsl")),
        fragment: Some(splat_shader),
        ..Default::default()
    }).id();

    // 创建绑定组（BindGroup）来绑定纹理和参数
    let splat_bind_group = BindGroupDescriptor {
        entries: vec![
            BindGroupEntry {
                binding: 0,  // 绑定速度纹理
                resource: velocity_texture.view().into(),
            },
            BindGroupEntry {
                binding: 1,  // 绑定密度纹理
                resource: density_texture.view().into(),
            },
            BindGroupEntry {
                binding: 2,  // 绑定喷溅位置（point）
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: splat_position.into(),
                    offset: 0,
                    size: wgpu::BufferSize::new(),
                }),
            },
            BindGroupEntry {
                binding: 3,  // 绑定喷溅速度（velocity）
                resource: splat_velocity.into(),
            },
            BindGroupEntry {
                binding: 4,  // 绑定喷溅颜色
                resource: splat_color.into(),
            },
            BindGroupEntry {
                binding: 5,  // 绑定喷溅的半径
                resource: ui_size.into(),
            },
            BindGroupEntry {
                binding: 6,  // 绑定画布的宽高比
                resource: aspect_ratio.into(),
            }
        ],
        ..Default::default()
    };

    // 执行 splat 计算
    commands.spawn_bundle(RenderPipeline::new(splat_pipeline))
        .insert(splat_bind_group)
        .insert(Splat {
            position: splat_position,
            velocity: splat_velocity,
            color: splat_color,
            radius: (ui_size + 1.0) / 600.0,  // 使用 UI 状态中的 size 参数
        });

    // 确保用更新后的纹理执行下一步的渲染
    velocity_texture.write();
    density_texture.write();
}

fn blit(
    mut commands: Commands,
    pipeline: Handle<PipelineDescriptor>,
    texture: Handle<Texture>,
    target_texture: Handle<Texture>,
) {
    // Create a full-screen quad mesh
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Quad { size: Vec2::new(2.0, 2.0), ..Default::default() })),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(texture),
            ..Default::default()
        }),
        ..Default::default()
    })
        .insert(RenderPipeline::new(pipeline.clone()))
        .insert(BindGroup {
            entries: vec![
                BindGroupEntry {
                    binding: 0,
                    resource: texture.clone(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: target_texture.clone(),
                },
            ],
            ..Default::default()
        });

    // Setup the render pipeline
    let pipeline = commands.spawn_bundle(PipelineDescriptor {
        vertex: Shader::from_wgsl(include_str!("full_screen_quad_vertex.wgsl")),
        fragment: Some(Shader::from_wgsl(include_str!("full_screen_quad_fragment.wgsl"))),
        ..Default::default()
    });
}


fn multiple_splats(
    mut commands: Commands,
    amount: usize,
    fluid_color: Res<FluidColor>,
    canvas_size: (f32, f32), // 画布宽高
    splat_radius: f32,
) {
    for _ in 0..amount {
        // 随机生成喷溅的位置和速度
        let x = canvas_size.0 * rand::random::<f32>();
        let y = canvas_size.1 * rand::random::<f32>();
        let dx = 1000.0 * (rand::random::<f32>() - 0.5);
        let dy = 1000.0 * (rand::random::<f32>() - 0.5);

        // 调用 splat 函数进行喷溅
        commands.spawn().insert(Splat {
            position: Vec2::new(x, y),
            velocity: Vec2::new(dx, dy),
            color: fluid_color.0,
            radius: splat_radius,
        });
    }
}


fn pointer_movement(
    mut pointer_query: Query<&mut Pointer>,
    mouse_motion_events: EventReader<MouseMotion>,
    touch_events: EventReader<Touch>,
    fluid_color: Res<FluidColor>,
    mut commands: Commands,
) {
    for mut pointer in pointer_query.iter_mut() {
        // 处理鼠标移动事件
        for event in mouse_motion_events.iter() {
            let canvas_left = event.position.x; // 获取鼠标位置
            let canvas_top = event.position.y;

            // 更新指针的位置和移动状态
            if pointer.down {
                pointer.moved = true;
                pointer.dx = (canvas_left - pointer.x) * 10.0;
                pointer.dy = (canvas_top - pointer.y) * 10.0;
            }

            pointer.x = canvas_left;
            pointer.y = canvas_top;
        }

        // 处理触摸事件
        for touch in touch_events.iter() {
            let canvas_left = touch.position.x; // 获取触摸位置
            let canvas_top = touch.position.y;

            // 更新触摸指针的位置和移动状态
            if pointer.down {
                pointer.moved = true;
                pointer.dx = (canvas_left - pointer.x) * 10.0;
                pointer.dy = (canvas_top - pointer.y) * 10.0;
            }

            pointer.x = canvas_left;
            pointer.y = canvas_top;
        }

        // 判断是否在鼠标按下时进行喷溅
        if pointer.moved {
            pointer.color = fluid_color.0; // 使用流体颜色
            // 这里可以触发喷溅的逻辑
            // splat(pointer.x, pointer.y, pointer.dx, pointer.dy, pointer.color);
        }
    }
}

fn touch_start(
    mut pointer_query: Query<&mut Pointer>,
    touch_events: EventReader<Touch>,
    fluid_color: Res<FluidColor>,
) {
    // 处理触摸开始事件
    for touch in touch_events.iter() {
        // 查找是否有空闲的指针，如果没有，增加一个新指针
        let mut pointer = pointer_query.iter_mut().next().unwrap();

        // 更新指针位置
        pointer.id = Some(touch.id);
        pointer.down = true;
        pointer.x = touch.position.x;
        pointer.y = touch.position.y;
        pointer.color = fluid_color.0;
    }
}

fn touch_end(
    mut pointer_query: Query<&mut Pointer>,
    touch_end_events: EventReader<Touch>,
) {
    // 处理触摸结束事件
    for touch in touch_end_events.iter() {
        for mut pointer in pointer_query.iter_mut() {
            if pointer.id == Some(touch.id) {
                pointer.down = false;
            }
        }
    }
}

fn mouse_button_input(
    mut pointer_query: Query<&mut Pointer>,
    mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    // 处理鼠标按下和松开事件
    for event in mouse_button_input_events.iter() {
        if let MouseButton::Left = event.button {
            for mut pointer in pointer_query.iter_mut() {
                if event.state == ElementState::Pressed {
                    pointer.down = true;
                } else {
                    pointer.down = false;
                }
            }
        }
    }
}

fn pointer_movement(
    mut pointer_query: Query<&mut Pointer>,
    mouse_motion_events: EventReader<MouseMotion>,
    touch_events: EventReader<Touch>,
    fluid_color: Res<FluidColor>,
) {
    // 处理指针的移动事件（包括鼠标和触摸）
    for mut pointer in pointer_query.iter_mut() {
        // 处理鼠标移动
        for event in mouse_motion_events.iter() {
            let canvas_left = event.position.x; // 获取鼠标位置
            let canvas_top = event.position.y;

            // 更新指针位置和移动状态
            if pointer.down {
                pointer.dx = (canvas_left - pointer.x) * 10.0;
                pointer.dy = (canvas_top - pointer.y) * 10.0;
            }

            pointer.x = canvas_left;
            pointer.y = canvas_top;
        }

        // 处理触摸事件
        for touch in touch_events.iter() {
            let canvas_left = touch.position.x;
            let canvas_top = touch.position.y;

            // 更新触摸指针的位置和移动状态
            if pointer.down {
                pointer.dx = (canvas_left - pointer.x) * 10.0;
                pointer.dy = (canvas_top - pointer.y) * 10.0;
            }

            pointer.x = canvas_left;
            pointer.y = canvas_top;
        }

        // 判断是否需要喷溅
        if pointer.down {
            pointer.color = fluid_color.0;
            // 这里可以调用喷溅函数，进行流体模拟更新
            // splat(pointer.x, pointer.y, pointer.dx, pointer.dy, pointer.color);
        }
    }
}