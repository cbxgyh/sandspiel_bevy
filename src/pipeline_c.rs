use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::{BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, MultisampleState, PipelineCache, PrimitiveState, RenderPipelineDescriptor, SamplerBindingType, ShaderStages, ShaderType, TextureFormat, TextureSampleType, TextureViewDimension, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::RenderApp;
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::BevyDefault;
use bytemuck::{Pod, Zeroable};


pub struct PipelinesPlugin;

impl Plugin for PipelinesPlugin {
    fn build(&self, app: &mut App) {

    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<ResetPipeline>();

    }
}

#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct ClearUniform {
    value: f32,
}
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct VertexInput {
    aPosition : Vec2,
}
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct SplatUniform {
    aspect_ratio: f32,
    color: Vec3,
    point: Vec2,
    radius: f32
}
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct AdvectionUniform {
    texel_size : Vec2,
    dt : f32,
    dissipation : f32
}
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct VorticityUniform{
    curl: f32,
    dt: f32
}
#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
struct SandUniform {
    t: f32,
    dpi: f32,
    resolution: Vec2,
    // 0 1
    is_snapshot: u32
}


#[derive(Resource)]
pub(crate) struct ResetPipeline {
    reset_bind_group_layout: BindGroupLayout,
    display_bind_group_layout: BindGroupLayout,
    velocity_out_bind_group_layout: BindGroupLayout,
    splat_bind_group_layout: BindGroupLayout,
    advection_bind_group_layout: BindGroupLayout,
    divergence_bind_group_layout: BindGroupLayout,
    curl_bind_group_layout: BindGroupLayout,
    vorticity_bind_group_layout: BindGroupLayout,
    pressure_bind_group_layout: BindGroupLayout,
    gradient_subtract_bind_group_layout: BindGroupLayout,
    sand_bind_group_layout: BindGroupLayout,
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
        let asser_server = world.resource::<AssetServer>();
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
                let clear_shader = asser_server
                    .load("shader/clear.wgsl");
                let base_vertex_shader = asser_server
                    .load("shader/baseVertex.wgsl");
                let display_shader = asser_server
                    .load("shader/display.wgsl");
                let velocity_out_shader = asser_server
                    .load("shader/velocityOut.wgsl");

                let splat_out_shader = asser_server
                    .load("shader/splat.wgsl");

                let advection_shader = asser_server
                    .load("shader/advection.wgsl");

                let divergence_shader = asser_server
                    .load("shader/divergence.wgsl");

                let curl_shader = asser_server
                    .load("shader/curl.wgsl");
                let vorticity_shader = asser_server
                    .load("shader/vorticity.wgsl");
                let pressure_shader = asser_server
                    .load("shader/pressure.wgsl");

                let gradient_subtract_shader = asser_server
                    .load("shader/gradientSubtract.wgsl");

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
                            sampler(SamplerBindingType::Filtering),
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

                let curl_layout = render_device.create_bind_group_layout(
                    "curl_layout",
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
                let vorticity_layout = render_device.create_bind_group_layout(
                    "vorticity_layout",
                    &BindGroupLayoutEntries::sequential(
                        // The layout entries will only be visible in the fragment stage
                        ShaderStages::FRAGMENT,
                        (
                            // The screen texture,
                            texture_2d(TextureSampleType::Float { filterable: true }),
                            texture_2d(TextureSampleType::Float { filterable: true }),
                            uniform_buffer::<VorticityUniform>(false),
                            sampler(SamplerBindingType::Filtering),
                            sampler(SamplerBindingType::Filtering),
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
                            sampler(SamplerBindingType::Filtering),
                            sampler(SamplerBindingType::Filtering),
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
                            sampler(SamplerBindingType::Filtering),
                        ),
                    ),
                );

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
                            sampler(SamplerBindingType::Filtering),
                            uniform_buffer::<SandUniform>(false),

                        ),
                    ),
                );
                let sand_shader = asser_server
                    .load("shader/sand.wgsl");

                let sand_vertex_shader = asser_server
                    .load("shader/sandVertex.wgsl");
                let formats = vec![
                    // Position
                    VertexFormat::Float32x2,
                    // Color
                    VertexFormat::Float32x2,
                ];
                let vertex_layout =
                    VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

                let pipeline_cache =world
                    .resource_mut::<PipelineCache>();
                let init_reset_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("clear_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),clear_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
                        },
                        fragment: Some(FragmentState {
                            shader: clear_shader.clone(),
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

                let init_display_pipeline =pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("display_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),display_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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

                let init_velocity_out_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("velocity_out_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),velocity_out_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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
                let init_splat_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("splat_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),splat_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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
                let init_advection_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_advection_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),advection_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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

                let init_divergence_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_divergence_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),divergencen_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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

                let init_curl_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_curl_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),curl_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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
                let init_vorticity_pipeline= pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_vorticity_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),vorticity_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
                        },
                        fragment: Some(FragmentState {
                            shader: vorticity_shader.clone(),
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

                let init_pressure_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_pressure_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),pressure_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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


                let init_gradient_subtract_pipeline = pipeline_cache
                    // This will add the pipeline to the cache and queue it's creation
                    .queue_render_pipeline(RenderPipelineDescriptor {
                        label: Some("init_gradient_subtract_pipeline".into()),
                        layout: vec![base_vertex_layout.clone(),gradient_subtract_layout.clone()],
                        // This will setup a fullscreen triangle for the vertex state
                        vertex: VertexState {
                            shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
                            shader_defs: vec![],
                            entry_point: "main".into(),  // 顶点着色器的入口点
                            buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
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






                let sand_buffer_layout =
                    VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, vec![
                        // Position
                        VertexFormat::Float32x2,
                    ]);

                let init_sand_subtract_pipeline = pipeline_cache
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
                    display_bind_group_layout: display_layout,
                    velocity_out_bind_group_layout: velocity_out_layout,
                    splat_bind_group_layout: splat_layout,
                    advection_bind_group_layout: advection_layout,
                    divergence_bind_group_layout: divergencen_layout,
                    curl_bind_group_layout: curl_layout,
                    vorticity_bind_group_layout: vorticity_layout,
                    pressure_bind_group_layout: pressure_layout,
                    gradient_subtract_bind_group_layout:gradient_subtract_layout,
                    sand_bind_group_layout: sand_layout,
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