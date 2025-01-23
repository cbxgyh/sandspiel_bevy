use bevy::prelude::*;


struct  UpdatePipeline{
    pub advection_pipeline: CachedRenderPipelineId,
    pub advection_bind_group_layout: BindGroupLayout,

    pub curl_pipeline: CachedRenderPipelineId,
    pub curl_bind_group_layout: BindGroupLayout,

    pub vorticity_pipeline: CachedRenderPipelineId,
    pub vorticity_bind_group_layout: BindGroupLayout,

    pub divergence_pipeline: CachedRenderPipelineId,
    pub divergence_bind_group_layout: BindGroupLayout,

    pub clear_pipeline: CachedRenderPipelineId,
    pub clear_bind_group_layout: BindGroupLayout,

    pub pressure_pipeline: CachedRenderPipelineId,
    pub pressure_bind_group_layout: BindGroupLayout,

    pub velocity_out_pipeline: CachedRenderPipelineId,
    pub velocity_out_bind_group_layout: BindGroupLayout,

    pub gradient_subtract_pipeline: CachedRenderPipelineId,
    pub gradient_subtract_bind_group_layout: BindGroupLayout,


    pub display_subtract_pipeline: CachedRenderPipelineId,
    pub display_subtract_bind_group_layout: BindGroupLayout,

    pub splat_subtract_pipeline: CachedRenderPipelineId,
    pub splat_subtract_bind_group_layout: BindGroupLayout,

    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vertex_count: u32,
}

impl FromWorld for UpdatePipeline {
    fn from_world(world: &mut World) -> Self {

        let width = 300;
        let height = 300;

        // 纹理数据
        let winds: Vec<u8> = vec![0; (width * height * 4) as usize]; // 替换为实际数据
        let burns_data: Vec<u8> = vec![0; (width * height * 4) as usize]; // 替换为实际数据
        let cells_data: Vec<u8> = vec![0; (width * height * 4) as usize]; // 替换为实际数据

        let texture_descriptor = TextureDescriptor {
            label: Some("Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT,
            ..default()
        };

        let winds_texture  = render_device.create_texture(&texture_descriptor);
        let winds_view  = winds_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let burns_texture  = render_device.create_texture(&texture_descriptor);
        let cells_texture  = render_device.create_texture(&texture_descriptor);

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

        Self{
            advection_pipeline:init_advection_pipeline,
            advection_bind_group_layout:advection_layout,

            curl_pipeline: init_curl_pipeline,
            curl_bind_group_layout: curl_layout,

            vorticity_pipeline: init_vorticity_pipeline,
            vorticity_bind_group_layout: vorticity_layout,

            divergence_pipeline: init_divergence_pipeline,
            divergence_bind_group_layout: divergencen_layout,

            clear_pipeline: init_curl_pipeline,
            clear_bind_group_layout: curl_layout,

            pressure_pipeline: init_pressure_pipeline,
            pressure_bind_group_layout: pressure_layout,

            velocity_out_pipeline: init_velocity_out_pipeline,
            velocity_out_bind_group_layout: velocity_out_layout,

            gradient_subtract_pipeline: init_gradient_subtract_pipeline,
            gradient_subtract_bind_group_layout: gradient_subtract_layout,

            display_subtract_pipeline: init_display_pipeline,
            display_subtract_bind_group_layout: display_layout,

            splat_subtract_pipeline: init_splat_pipeline,
            splat_subtract_bind_group_layout: splat_layout,

        }
    }
}

