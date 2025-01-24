// use bevy::prelude::*;
// use bevy::render::extract_component::ExtractComponent;
// use bevy::render::render_resource::{AddressMode, BindGroup, BindGroupLayout, BindGroupLayoutEntries, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferDescriptor, BufferInitDescriptor, BufferSize, BufferUsages, CachedRenderPipelineId, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Extent3d, FilterMode, FragmentState, LoadOp, MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StoreOp, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDimension, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
// use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
// use bevy::render::RenderApp;
// use bevy::render::renderer::{RenderDevice, RenderQueue};
// use bevy::render::texture::BevyDefault;
// use bytemuck::{Pod, Zeroable};
//
//
// pub struct PipelinesPlugin;
//
// impl Plugin for PipelinesPlugin {
//     fn build(&self, app: &mut App) {
//
//     }
//
//     fn finish(&self, app: &mut App) {
//         let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
//             return;
//         };
//
//         render_app
//             // Initialize the pipeline
//             .init_resource::<ResetPipeline>();
//
//     }
// }
//
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct ClearUniform {
//     value: f32,
// }
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct VertexInput {
//     aPosition : Vec2,
// }
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct SplatUniform {
//     aspect_ratio: f32,
//     color: Vec3,
//     point: Vec2,
//     radius: f32
// }
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct AdvectionUniform {
//     texel_size : Vec2,
//     dt : f32,
//     dissipation : f32
// }
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct VorticityUniform{
//     curl: f32,
//     dt: f32
// }
// #[repr(C)]
// #[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
// struct SandUniform {
//     t: f32,
//     dpi: f32,
//     resolution: Vec2,
//     // 0 1
//     is_snapshot: u32
// }
//
//
// #[derive(Resource)]
// pub(crate) struct ResetPipeline {
//     reset_bind_group_layout: BindGroupLayout,
//     display_bind_group_layout: BindGroupLayout,
//     velocity_out_bind_group_layout: BindGroupLayout,
//     splat_bind_group_layout: BindGroupLayout,
//     advection_bind_group_layout: BindGroupLayout,
//     divergence_bind_group_layout: BindGroupLayout,
//     curl_bind_group_layout: BindGroupLayout,
//     vorticity_bind_group_layout: BindGroupLayout,
//     pressure_bind_group_layout: BindGroupLayout,
//     gradient_subtract_bind_group_layout: BindGroupLayout,
//     sand_bind_group_layout: BindGroupLayout,
//     init_reset_pipeline: CachedRenderPipelineId,
//     init_display_pipeline: CachedRenderPipelineId,
//     init_velocity_out_pipeline: CachedRenderPipelineId,
//     init_splat_pipeline: CachedRenderPipelineId,
//     init_advection_pipeline: CachedRenderPipelineId,
//     init_divergence_pipeline: CachedRenderPipelineId,
//     init_curl_pipeline: CachedRenderPipelineId,
//     init_vorticity_pipeline: CachedRenderPipelineId,
//     init_pressure_pipeline: CachedRenderPipelineId,
//     init_gradient_subtract_pipeline: CachedRenderPipelineId,
//     init_sand_subtract_pipeline: CachedRenderPipelineId,
// }
//
// impl FromWorld for ResetPipeline {
//     fn from_world(world: &mut World) -> Self {
//         let render_device = world.resource::<RenderDevice>();
//         let asser_server = world.resource::<AssetServer>();
//         let entries=[
//                     BindGroupLayoutEntry {
//                         binding: 0,
//                         visibility: ShaderStages::FRAGMENT,
//                         ty: BindingType::Texture {
//                             sample_type: TextureSampleType::Float { filterable: true },
//                             view_dimension: TextureViewDimension::D2,
//                             multisampled: false,
//                         },
//                         count: None,
//                     },
//                     BindGroupLayoutEntry {
//                         binding: 1,
//                         visibility: ShaderStages::FRAGMENT,
//                         ty: BindingType::Texture {
//                             sample_type: TextureSampleType::Float { filterable: true },
//                             view_dimension: TextureViewDimension::D2,
//                             multisampled: false,
//                         },
//                         count: None,
//                     },
//                     BindGroupLayoutEntry{
//                         binding: 2,
//                         visibility: ShaderStages::VERTEX_FRAGMENT,
//                         ty: BindingType::Buffer {
//                             ty: BufferBindingType::Uniform,
//                             has_dynamic_offset: false,
//                             min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
//                         },
//                         count: None,
//                     },
//                 ];
//                 let layout = render_device.create_bind_group_layout(
//                     None,
//                     &entries,
//                 );
//                 let clear_shader = asser_server
//                     .load("shader/clear.wgsl");
//                 let base_vertex_shader = asser_server
//                     .load("shader/baseVertex.wgsl");
//                 let display_shader = asser_server
//                     .load("shader/display.wgsl");
//                 let velocity_out_shader = asser_server
//                     .load("shader/velocityOut.wgsl");
//
//                 let splat_out_shader = asser_server
//                     .load("shader/splat.wgsl");
//
//                 let advection_shader = asser_server
//                     .load("shader/advection.wgsl");
//
//                 let divergence_shader = asser_server
//                     .load("shader/divergence.wgsl");
//
//                 let curl_shader = asser_server
//                     .load("shader/curl.wgsl");
//                 let vorticity_shader = asser_server
//                     .load("shader/vorticity.wgsl");
//                 let pressure_shader = asser_server
//                     .load("shader/pressure.wgsl");
//
//                 let gradient_subtract_shader = asser_server
//                     .load("shader/gradientSubtract.wgsl");
//
//                 let clear_layout = render_device.create_bind_group_layout(
//                     "clear_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             uniform_buffer::<ClearUniform>(false),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//
//                 let base_vertex_layout =render_device.create_bind_group_layout(
//                     "base_vertex_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::VERTEX,
//                         (
//                             // The screen texture
//                             uniform_buffer::<VertexInput>(false),
//                         ),
//                     ),
//                 );
//                 let display_layout = render_device.create_bind_group_layout(
//                     "display_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//                 let velocity_out_layout = render_device.create_bind_group_layout(
//                     "velocity_out",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//                 let splat_layout = render_device.create_bind_group_layout(
//                     "splat_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                             uniform_buffer::<SplatUniform>(false),
//                         ),
//                     ),
//                 );
//
//
//                 let advection_layout = render_device.create_bind_group_layout(
//                     "advection_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                             uniform_buffer::<AdvectionUniform>(false),
//                         ),
//                     ),
//                 );
//                 let divergencen_layout = render_device.create_bind_group_layout(
//                     "divergencen_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//
//                 let curl_layout = render_device.create_bind_group_layout(
//                     "curl_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//                 let vorticity_layout = render_device.create_bind_group_layout(
//                     "vorticity_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             uniform_buffer::<VorticityUniform>(false),
//                             sampler(SamplerBindingType::Filtering),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//                 let pressure_layout = render_device.create_bind_group_layout(
//                     "pressure_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//
//                 let gradient_subtract_layout = render_device.create_bind_group_layout(
//                     "gradient_subtract_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture,
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                         ),
//                     ),
//                 );
//
//                 let sand_vertex_layout =render_device.create_bind_group_layout(
//                     "sand_vertex_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::VERTEX,
//                         (
//                             // The screen texture
//                             uniform_buffer::<VertexInput>(false),
//                         ),
//                     ),
//                 );
//                 let sand_layout =render_device.create_bind_group_layout(
//                     "sand_layout",
//                     &BindGroupLayoutEntries::sequential(
//                         // The layout entries will only be visible in the fragment stage
//                         ShaderStages::FRAGMENT,
//                         (
//                             // The screen texture
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             texture_2d(TextureSampleType::Float { filterable: true }),
//                             sampler(SamplerBindingType::Filtering),
//                             uniform_buffer::<SandUniform>(false),
//
//                         ),
//                     ),
//                 );
//                 let sand_shader = asser_server
//                     .load("shader/sand.wgsl");
//
//                 let sand_vertex_shader = asser_server
//                     .load("shader/sandVertex.wgsl");
//                 let formats = vec![
//                     // Position
//                     VertexFormat::Float32x2,
//                     // Color
//                     VertexFormat::Float32x2,
//                 ];
//                 let vertex_layout =
//                     VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);
//
//                 let pipeline_cache =world
//                     .resource_mut::<PipelineCache>();
//                 let init_reset_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("clear_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),clear_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: clear_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 let init_display_pipeline =pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("display_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),display_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: display_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 let init_velocity_out_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("velocity_out_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),velocity_out_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: velocity_out_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//                 let init_splat_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("splat_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),splat_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: splat_out_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//                 let init_advection_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_advection_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),advection_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: advection_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 let init_divergence_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_divergence_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),divergencen_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: divergence_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 let init_curl_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_curl_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),curl_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: curl_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//                 let init_vorticity_pipeline= pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_vorticity_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),vorticity_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: vorticity_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 let init_pressure_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_pressure_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),pressure_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: pressure_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//
//                 let init_gradient_subtract_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_gradient_subtract_pipeline".into()),
//                         layout: vec![base_vertex_layout.clone(),gradient_subtract_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: base_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![vertex_layout.clone()],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: gradient_subtract_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//
//
//
//
//
//                 let sand_buffer_layout =
//                     VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, vec![
//                         // Position
//                         VertexFormat::Float32x2,
//                     ]);
//
//                 let init_sand_subtract_pipeline = pipeline_cache
//                     // This will add the pipeline to the cache and queue it's creation
//                     .queue_render_pipeline(RenderPipelineDescriptor {
//                         label: Some("init_sand_subtract_pipeline".into()),
//                         layout: vec![sand_vertex_layout.clone(),sand_layout.clone()],
//                         // This will setup a fullscreen triangle for the vertex state
//                         vertex: VertexState {
//                             shader: sand_vertex_shader.clone(),  // 传递片段着色器作为顶点着色器
//                             shader_defs: vec![],
//                             entry_point: "main".into(),  // 顶点着色器的入口点
//                             buffers: vec![sand_buffer_layout],  // 没有顶点数据，因此缓冲区为空
//                         },
//                         fragment: Some(FragmentState {
//                             shader: sand_shader.clone(),
//                             shader_defs: vec![],
//                             // Make sure this matches the entry point of your shader.
//                             // It can be anything as long as it matches here and in the shader.
//                             entry_point: "main".into(),
//                             targets: vec![Some(ColorTargetState {
//                                 format: TextureFormat::bevy_default(),
//                                 blend: None,
//                                 write_mask: ColorWrites::ALL,
//                             })],
//                         }),
//                         // All of the following properties are not important for this effect so just use the default values.
//                         // This struct doesn't have the Default trait implemented because not all field can have a default value.
//                         primitive: PrimitiveState::default(),
//                         depth_stencil: None,
//                         multisample: MultisampleState::default(),
//                         push_constant_ranges: vec![],
//                     });
//
//                 ResetPipeline {
//                     reset_bind_group_layout: layout,
//                     display_bind_group_layout: display_layout,
//                     velocity_out_bind_group_layout: velocity_out_layout,
//                     splat_bind_group_layout: splat_layout,
//                     advection_bind_group_layout: advection_layout,
//                     divergence_bind_group_layout: divergencen_layout,
//                     curl_bind_group_layout: curl_layout,
//                     vorticity_bind_group_layout: vorticity_layout,
//                     pressure_bind_group_layout: pressure_layout,
//                     gradient_subtract_bind_group_layout:gradient_subtract_layout,
//                     sand_bind_group_layout: sand_layout,
//                     init_reset_pipeline,
//                     init_display_pipeline,
//                     init_velocity_out_pipeline,
//                     init_splat_pipeline,
//                     init_advection_pipeline,
//                     init_divergence_pipeline,
//                     init_curl_pipeline,
//                     init_vorticity_pipeline,
//                     init_pressure_pipeline,
//                     init_gradient_subtract_pipeline,
//                     init_sand_subtract_pipeline
//                 }
//     }
// }
//
// pub struct FrameBuffer{
//     pub texture: Texture,
//     pub texture_view: TextureView,
//     pub sampler: Sampler,
//     pub width: u32,
//     pub height: u32,
// }
// impl FrameBuffer {
//     fn init_frame_buffers(
//         render_device: &Res<RenderDevice>,
//         // queue: Res<RenderQueue>,
//         width: u32,
//         height: u32,
//         internal_format: TextureFormat,
//         filter_mode: FilterMode,
//     ) -> FrameBuffer {
//         let size = Extent3d {
//             width,
//             height,
//             ..default()
//         };
//
//
//         let tex = render_device.create_texture(
//             &TextureDescriptor {
//                 label: None,
//                 size,
//                 mip_level_count: 1,
//                 sample_count: 1,
//                 dimension: TextureDimension::D2,
//                 // format: TextureFormat::Rgba8UnormSrgb,
//                 format: internal_format,
//                 usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
//                 view_formats: &[],
//             }
//         );
//         let tex_view = tex.create_view(&Default::default());
//
//         let sampler = render_device.create_sampler(&SamplerDescriptor {
//             label: Some("framebuffer Sampler"),
//             address_mode_u: AddressMode::ClampToEdge,
//             address_mode_v: AddressMode::ClampToEdge,
//             address_mode_w: AddressMode::ClampToEdge,
//             mag_filter: filter_mode,
//             // mag_filter: FilterMode::Linear,
//             // min_filter: FilterMode::Linear,
//             min_filter: filter_mode,
//             mipmap_filter: FilterMode::Nearest,
//             ..Default::default()
//         });
//
//         let mut command_encoder = render_device.create_command_encoder(
//             &CommandEncoderDescriptor {
//                 label: Some("main_opaque_pass_3d_command_encoder"),
//             }
//         );
//
//         let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
//             label: Some("main_opaque_pass_3d"),
//             color_attachments: &[Some(
//                 RenderPassColorAttachment {
//                     view: &tex_view,
//                     resolve_target: None,
//                     ops: Operations {
//                         load: LoadOp::Clear(Color::BLACK.into()),
//                         store: StoreOp::default()
//                     },
//                 }
//             )],
//             depth_stencil_attachment: None,
//             timestamp_writes: None,
//             occlusion_query_set: None,
//         });
//         // queue.submit(Some(command_encoder.finish()));
//         // 设置视口
//         render_pass.set_viewport(0.0, 0.0, size.width as f32, size.height as f32, 0.0, 1.0);
//
//         FrameBuffer{
//             texture:tex,
//             texture_view:tex_view.clone(),
//             width,
//             height,
//             sampler
//         }
//     }
// }
// #[derive(Resource)]
// pub struct DoubleFrameBuffer {
//     pub fbo1: FrameBuffer,
//     pub fbo2: FrameBuffer,
// }
//
//
// impl DoubleFrameBuffer {
//     pub fn new(
//         render_device: &Res<RenderDevice>,
//         // queue: Res<RenderQueue>,
//         width: u32,
//         height: u32,
//         internal_format: TextureFormat,
//         filter_mode: FilterMode,
//     )->Self{
//         let fbo1 = FrameBuffer::init_frame_buffers(render_device, width, height, internal_format, filter_mode);
//         let fbo2 = FrameBuffer::init_frame_buffers(render_device, width, height, internal_format, filter_mode);
//
//         Self { fbo1, fbo2 }
//     }
//     pub fn swap(&mut self) {
//         std::mem::swap(&mut self.fbo1, &mut self.fbo2);
//     }
//
// }
//
//
// // pub struct WgpuProgram {
// //     pub pipeline: wgpu::RenderPipeline,
// //     pub bind_group_layout: wgpu::BindGroupLayout,
// //     pub bind_group: Option<wgpu::BindGroup>, // 用于存储 uniform 或资源绑定
// // }
// // pub fn bind<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
// //     render_pass.set_pipeline(&self.pipeline);
// //     if let Some(bind_group) = &self.bind_group {
// //         render_pass.set_bind_group(0, bind_group, &[]);
// //     }
// // }
// pub struct BlitProgram {
//     vertex_buffer: Buffer,
//     index_buffer: Buffer,
//     pipeline: RenderPipeline,
//     bind_group: Option<BindGroup>,
// }
//
// impl FromWorld for  BlitProgram {
//     fn from_world(world: &mut World) -> Self {
//
//
//     }
// }
//
// pub fn init_buffer(
//     render_device: &Res<RenderDevice>,
//     width: u32,
//     height: u32,
// ){
//     let velocity = DoubleFrameBuffer::new(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//     let density = DoubleFrameBuffer::new(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//
//
//     let divergence = FrameBuffer::init_frame_buffers(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//     let curl = FrameBuffer::init_frame_buffers(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//
//     let pressure = DoubleFrameBuffer::new(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//
//     let burns = FrameBuffer::init_frame_buffers(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//     let cells = FrameBuffer::init_frame_buffers(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//     let velocity_out = FrameBuffer::init_frame_buffers(render_device,width,height,TextureFormat::Rgba8UnormSrgb,FilterMode::Linear);
//
//
//
//     // impl BlitProgram {
// //     pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
// //         // 创建顶点缓冲区数据（正方形的四个顶点）
// //         let vertex_data: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];
// //         let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
// //             label: Some("Vertex Buffer"),
// //             contents: bytemuck::cast_slice(&vertex_data),
// //             usage: wgpu::BufferUsage::VERTEX,
// //         });
// //
// //         // 创建索引缓冲区数据（两个三角形的索引）
// //         let index_data: [u16; 6] = [0, 1, 2, 0, 2, 3];
// //         let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
// //             label: Some("Index Buffer"),
// //             contents: bytemuck::cast_slice(&index_data),
// //             usage: wgpu::BufferUsage::INDEX,
// //         });
// //
// //         // 着色器代码（顶点和片段）
// //         let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
// //             label: Some("Vertex Shader"),
// //             source: wgpu::ShaderSource::Wgsl(include_str!("vertex.wgsl").into()),
// //         });
// //
// //         let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
// //             label: Some("Fragment Shader"),
// //             source: wgpu::ShaderSource::Wgsl(include_str!("fragment.wgsl").into()),
// //         });
// //
// //         // 创建管线布局
// //         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
// //             label: Some("Pipeline Layout"),
// //             bind_group_layouts: &[],
// //             push_constant_ranges: &[],
// //         });
// //
// //         // 创建渲染管线
// //         let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
// //             label: Some("Render Pipeline"),
// //             layout: Some(&pipeline_layout),
// //             vertex: wgpu::VertexState {
// //                 module: &vertex_shader,
// //                 entry_point: "main",
// //                 buffers: &[wgpu::VertexBufferLayout {
// //                     array_stride: std::mem::size_of::<f32>() as wgpu::BufferAddress * 2,
// //                     step_mode: wgpu::InputStepMode::Vertex,
// //                     attributes: &[wgpu::VertexAttribute {
// //                         offset: 0,
// //                         format: wgpu::VertexFormat::Float2,
// //                         shader_location: 0,
// //                     }],
// //                 }],
// //             },
// //             fragment: Some(wgpu::FragmentState {
// //                 module: &fragment_shader,
// //                 entry_point: "main",
// //                 targets: &[wgpu::ColorTargetState {
// //                     format: wgpu::TextureFormat::Bgra8Unorm,
// //                     blend: Some(wgpu::BlendState::REPLACE),
// //                     write_mask: wgpu::ColorWrites::ALL,
// //                 }],
// //             }),
// //             primitive: wgpu::PrimitiveState {
// //                 topology: wgpu::PrimitiveTopology::TriangleList,
// //                 strip_index_format: None,
// //                 front_face: wgpu::FrontFace::Ccw,
// //                 cull_mode: Some(wgpu::Face::Back),
// //                 unclipped_depth: false,
// //                 polygon_mode: wgpu::PolygonMode::Fill,
// //                 conservative: false,
// //             },
// //             depth_stencil: None,
// //             multisample: wgpu::MultisampleState {
// //                 count: 1,
// //                 mask: !0,
// //                 alpha_to_coverage_enabled: false,
// //             },
// //             multiview: None,
// //         });
// //
// //         Self {
// //             vertex_buffer,
// //             index_buffer,
// //             pipeline,
// //             bind_group: None,
// //         }
// //     }
// //
// //     pub fn blit(&self, device: &wgpu::Device, queue: &wgpu::Queue, frame: &wgpu::TextureView, width: u32, height: u32) {
// //         // 创建渲染通道
// //         let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
// //             label: Some("Command Encoder"),
// //         });
// //
// //         // 设置渲染目标
// //         let render_pass_descriptor = wgpu::RenderPassDescriptor {
// //             label: Some("Render Pass"),
// //             color_attachments: &[wgpu::RenderPassColorAttachment {
// //                 view: frame,
// //                 resolve_target: None,
// //                 ops: wgpu::Operations {
// //                     load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
// //                     store: true,
// //                 },
// //             }],
// //             depth_stencil_attachment: None,
// //         };
// //
// //         let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
// //
// //         // 绑定管线和缓冲区
// //         render_pass.set_pipeline(&self.pipeline);
// //         render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
// //         render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
// //
// //         // 执行绘制操作
// //         render_pass.draw_indexed(0..6, 0, 0..1);
// //
// //         // 提交命令
// //         queue.submit(Some(encoder.finish()));
// //     }
// // }
//
//
//
//
//
// // use wgpu::util::DeviceExt;
// // use wgpu::Color;
// //
// // pub struct SplatProgram {
// //     program: wgpu::RenderPipeline,
// //     uniforms: SplatUniforms,
// //     bind_group: Option<wgpu::BindGroup>,
// // }
// //
// // pub struct SplatUniforms {
// //     u_target: wgpu::BindGroupLayoutEntry,
// //     aspect_ratio: wgpu::BindGroupLayoutEntry,
// //     point: wgpu::BindGroupLayoutEntry,
// //     color: wgpu::BindGroupLayoutEntry,
// //     radius: wgpu::BindGroupLayoutEntry,
// // }
// //
// // impl SplatProgram {
// //     pub fn new(device: &wgpu::Device, width: u32, height: u32) -> Self {
// //         let splat_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
// //             label: Some("Splat Shader"),
// //             source: wgpu::ShaderSource::Wgsl(include_str!("splat_shader.wgsl").into()),
// //         });
// //
// //         let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
// //             label: Some("Splat Bind Group Layout"),
// //             entries: &[
// //                 // uTarget
// //                 wgpu::BindGroupLayoutEntry {
// //                     binding: 0,
// //                     visibility: wgpu::ShaderStage::FRAGMENT,
// //                     ty: wgpu::BindingType::Texture {
// //                         sample_type: wgpu::TextureSampleType::Float,
// //                         view_dimension: wgpu::TextureViewDimension::D2,
// //                         multisampled: false,
// //                     },
// //                     count: None,
// //                 },
// //                 // aspectRatio
// //                 wgpu::BindGroupLayoutEntry {
// //                     binding: 1,
// //                     visibility: wgpu::ShaderStage::FRAGMENT,
// //                     ty: wgpu::BindingType::UniformBuffer {
// //                         dynamic: false,
// //                         min_binding_size: None,
// //                     },
// //                     count: None,
// //                 },
// //                 // point
// //                 wgpu::BindGroupLayoutEntry {
// //                     binding: 2,
// //                     visibility: wgpu::ShaderStage::FRAGMENT,
// //                     ty: wgpu::BindingType::UniformBuffer {
// //                         dynamic: false,
// //                         min_binding_size: None,
// //                     },
// //                     count: None,
// //                 },
// //                 // color
// //                 wgpu::BindGroupLayoutEntry {
// //                     binding: 3,
// //                     visibility: wgpu::ShaderStage::FRAGMENT,
// //                     ty: wgpu::BindingType::UniformBuffer {
// //                         dynamic: false,
// //                         min_binding_size: None,
// //                     },
// //                     count: None,
// //                 },
// //                 // radius
// //                 wgpu::BindGroupLayoutEntry {
// //                     binding: 4,
// //                     visibility: wgpu::ShaderStage::FRAGMENT,
// //                     ty: wgpu::BindingType::UniformBuffer {
// //                         dynamic: false,
// //                         min_binding_size: None,
// //                     },
// //                     count: None,
// //                 },
// //             ],
// //         });
// //
// //         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
// //             label: Some("Splat Pipeline Layout"),
// //             bind_group_layouts: &[&bind_group_layout],
// //             push_constant_ranges: &[],
// //         });
// //
// //         let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
// //             label: Some("Splat Render Pipeline"),
// //             layout: Some(&pipeline_layout),
// //             vertex: wgpu::VertexState {
// //                 module: &splat_shader,
// //                 entry_point: "main",
// //                 buffers: &[],
// //             },
// //             fragment: Some(wgpu::FragmentState {
// //                 module: &splat_shader,
// //                 entry_point: "main",
// //                 targets: &[wgpu::ColorTargetState {
// //                     format: wgpu::TextureFormat::Bgra8Unorm,
// //                     blend: Some(wgpu::BlendState::REPLACE),
// //                     write_mask: wgpu::ColorWrites::ALL,
// //                 }],
// //             }),
// //             primitive: wgpu::PrimitiveState {
// //                 topology: wgpu::PrimitiveTopology::TriangleList,
// //                 strip_index_format: None,
// //                 front_face: wgpu::FrontFace::Ccw,
// //                 cull_mode: Some(wgpu::Face::Back),
// //                 unclipped_depth: false,
// //                 polygon_mode: wgpu::PolygonMode::Fill,
// //                 conservative: false,
// //             },
// //             depth_stencil: None,
// //             multisample: wgpu::MultisampleState {
// //                 count: 1,
// //                 mask: !0,
// //                 alpha_to_coverage_enabled: false,
// //             },
// //             multiview: None,
// //         });
// //
// //         Self {
// //             program: render_pipeline,
// //             uniforms: SplatUniforms {
// //                 u_target: bind_group_layout.entries[0].clone(),
// //                 aspect_ratio: bind_group_layout.entries[1].clone(),
// //                 point: bind_group_layout.entries[2].clone(),
// //                 color: bind_group_layout.entries[3].clone(),
// //                 radius: bind_group_layout.entries[4].clone(),
// //             },
// //             bind_group: None,
// //         }
// //     }
// //
// //     pub fn splat(
// //         &mut self,
// //         device: &wgpu::Device,
// //         queue: &wgpu::Queue,
// //         frame: &wgpu::TextureView,
// //         velocity: &VelocityField,
// //         density: &DensityField,
// //         x: f32,
// //         y: f32,
// //         dx: f32,
// //         dy: f32,
// //         color: [f32; 3],
// //         size: f32,
// //     ) {
// //         // Bind to the splat shader program
// //         let tex_unit = 0;
// //         let mut bind_group_entries = vec![
// //             wgpu::BindGroupEntry {
// //                 binding: 0,
// //                 resource: wgpu::BindingResource::TextureView(&velocity.read[0]),
// //             },
// //             wgpu::BindGroupEntry {
// //                 binding: 1,
// //                 resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
// //                     buffer: &self.uniforms.aspect_ratio,
// //                     offset: 0,
// //                     size: None,
// //                 }),
// //             },
// //             wgpu::BindGroupEntry {
// //                 binding: 2,
// //                 resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
// //                     buffer: &self.uniforms.point,
// //                     offset: 0,
// //                     size: None,
// //                 }),
// //             },
// //             wgpu::BindGroupEntry {
// //                 binding: 3,
// //                 resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
// //                     buffer: &self.uniforms.color,
// //                     offset: 0,
// //                     size: None,
// //                 }),
// //             },
// //             wgpu::BindGroupEntry {
// //                 binding: 4,
// //                 resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
// //                     buffer: &self.uniforms.radius,
// //                     offset: 0,
// //                     size: None,
// //                 }),
// //             },
// //         ];
// //
// //         // Create bind group
// //         self.bind_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
// //             layout: &self.program.layout,
// //             entries: &bind_group_entries,
// //             label: Some("Splat Bind Group"),
// //         }));
// //
// //         // Create command encoder and render pass
// //         let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
// //             label: Some("Splat Command Encoder"),
// //         });
// //
// //         let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
// //             label: Some("Splat Render Pass"),
// //             color_attachments: &[wgpu::RenderPassColorAttachment {
// //                 view: frame,
// //                 resolve_target: None,
// //                 ops: wgpu::Operations {
// //                     load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
// //                     store: true,
// //                 },
// //             }],
// //             depth_stencil_attachment: None,
// //         });
// //
// //         render_pass.set_pipeline(&self.program);
// //         render_pass.set_bind_group(0, &self.bind_group.as_ref().unwrap(), &[]);
// //
// //         // Execute splat operation
// //         render_pass.draw(0..6, 0..1);
// //
// //         // Submit command to queue
// //         queue.submit(Some(encoder.finish()));
// //
// //         // Swap velocity and density textures (not shown in this example)
// //         velocity.swap();
// //         density.swap();
// //     }
// // }