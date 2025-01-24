use bevy::core::{Pod, Zeroable};
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, ExtractComponent};
use bevy::render::{render_graph, RenderApp};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer, BufferInitDescriptor, BufferUsages, CachedPipelineState, CachedRenderPipelineId, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Extent3d, FragmentState, IndexFormat, LoadOp, MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StoreOp, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::texture::BevyDefault;
use crate::GameOfLifeState;

pub struct ResetPipelinePlugin;


impl Plugin for ResetPipelinePlugin {

    fn build(&self, app: &mut App) {


        // ;

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

#[derive(Resource)]
struct  ResetPipeline{
    pub pipeline: CachedRenderPipelineId,
    pub clear_bind_group_layout: BindGroupLayout,
    pub base_bind_group_layout: BindGroupLayout,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub vertex_count: u32,
}

 #[derive(Component)]
struct ResetBindGroups {
    clear_bind_groups: Box<[BindGroup]>,
    base_bind_groups: Box<[BindGroup]>,
    sampler: Sampler,
}

#[derive(Resource)]
struct InitTexture{
     pub burns:Texture,
     pub density:Texture,
     pub pressure:Texture,
     pub velocity:Texture,
}

impl FromWorld for InitTexture {
    fn from_world(world: &mut World) -> Self {

        let render_device = world.resource::<RenderDevice>();
        let width=300;
        let height =300;
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
            view_formats: &[]
        };
        let burns = render_device.create_texture(&texture_descriptor);
        let density = render_device.create_texture(&texture_descriptor);
        let pressure = render_device.create_texture(&texture_descriptor);
        let velocity = render_device.create_texture(&texture_descriptor);

        Self { burns, density, pressure, velocity }
    }
}

// fn prepare_bloom_bind_groups(
//     clear_uniforms: Res<ComponentUniforms<ClearUniform>>,
//     base_uniforms: Res<ComponentUniforms<VertexInput>>,
//     render_device: Res<RenderDevice>,
//     reset_pipeline: Res<ResetPipeline>,
//     init_texture: Res<InitTexture>
// ){
//
//     let sampler=render_device.create_sampler(&SamplerDescriptor::default());
//     let burns_clear_bing_group=render_device.create_bind_group(
//         "clear_bind_group",
//         &reset_pipeline.clear_bind_group_layout,
//         &BindGroupEntries::sequential((
//             &init_texture.burns.create_view(&Default::default()),
//             &sampler,
//             clear_uniforms.binding().unwrap(),
//         )),
//     );
//     let burns_base_bing_group= render_device.create_bind_group(
//         "base_bind_group",
//         &reset_pipeline.base_bind_group_layout,
//         &BindGroupEntries::sequential((
//             &init_texture.burns.create_view(&Default::default()),
//             &sampler,
//             base_uniforms.binding().unwrap(),
//         )),
//     );
// }
impl ResetPipeline{
    pub fn render(&self,
                  world:&World,
                  render_context: &mut RenderContext,
                  state:&GameOfLifeState,
                  tex:Texture,
                  base_bind_group: &BindGroup,
                  bind_group: BindGroup,
    ){
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<ResetPipeline>();
        let tex_view = tex.create_view(&Default::default());
        let mut pass = render_context
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("reset_render_pass"),
                color_attachments: &[Some(
                    RenderPassColorAttachment {
                        view: &tex_view,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(Color::BLACK.into()),
                            store: StoreOp::default()
                        },
                    }
                )],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

        pass.set_bind_group(0, base_bind_group, &[]);
        pass.set_bind_group(1, &bind_group, &[]);

        // select the pipeline based on the current state
        match state {
            GameOfLifeState::Loading => {}
            GameOfLifeState::Init => {
                let init_pipeline = pipeline_cache
                    .get_render_pipeline(pipeline.pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.set_vertex_buffer(0, *pipeline.vertex_buffer.slice(..));
                pass.set_index_buffer(*pipeline.index_buffer.slice(..), IndexFormat::Uint16);
                pass.draw_indexed(0..pipeline.vertex_count, 0, 0..1);
                // pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            GameOfLifeState::Update => {
                let update_pipeline = pipeline_cache
                    .get_render_pipeline(pipeline.pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                // pass.dispatch_workgroups(SIZE.0 / WORKGROUP_SIZE, SIZE.1 / WORKGROUP_SIZE, 1);
            }
            _=>{}
        }
    }
}
impl FromWorld for ResetPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let asser_server = world.resource::<AssetServer>();

        // 创建顶点缓冲区数据（正方形的四个顶点）
        let vertex_data: [f32; 8] = [-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, 1.0, -1.0];

        let vertex_buffer=render_device.create_buffer_with_data(
            &BufferInitDescriptor {
                label:None,
                contents:bytemuck::cast_slice(vertex_data.as_slice()),
                usage: BufferUsages::VERTEX,
            }
        );

        // 创建索引缓冲区数据（两个三角形的索引）
        let index_data: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer=render_device.create_buffer_with_data(&BufferInitDescriptor {
            label:None,
            contents:bytemuck::cast_slice(index_data.as_slice()),
            usage: BufferUsages::INDEX,
        });

        let clear_shader=asser_server.load("shader/clear.wgsl");
        let base_vertex_shader=asser_server.load("shader/baseVertex.wgsl");
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
        let formats = vec![
            // Position
            VertexFormat::Float32x2,
            // texel_size
            VertexFormat::Float32x2,
        ];
        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        let pipeline_cache =world
            .resource_mut::<PipelineCache>();
        let pipeline = pipeline_cache
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

        Self{
            index_buffer,
            vertex_buffer,
            pipeline,
            clear_bind_group_layout:clear_layout,
            base_bind_group_layout:base_vertex_layout,
            vertex_count:index_data.len() as u32,
        }
    }
}


struct GameOfLifeNode {
    state: GameOfLifeState,
}
impl Default for GameOfLifeNode {
    fn default() -> Self {
        Self {
            state: GameOfLifeState::Loading,
        }
    }
}



impl render_graph::Node for GameOfLifeNode {
    fn update(&mut self, world: &mut World) {
        let pipeline = world.resource::<ResetPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        // if the corresponding pipeline has loaded, transition to the next stage
        match self.state {
            GameOfLifeState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_render_pipeline_state(pipeline.pipeline)
                {
                    self.state = GameOfLifeState::Init;
                }
            }
            GameOfLifeState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_render_pipeline_state(pipeline.pipeline){
                        self.state = GameOfLifeState::Init;
                    }
            }
            GameOfLifeState::Reset => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_render_pipeline_state(pipeline.pipeline)
                {
                    self.state = GameOfLifeState::Reset;
                }
            }
            GameOfLifeState::Update => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        let pipeline = world.resource::<ResetPipeline>();
        let render_device = world.resource::<RenderDevice>();
        let clear_uniforms = world.resource::<ComponentUniforms<ClearUniform>>();
        let base_uniforms = world.resource::<ComponentUniforms<VertexInput>>();
        let width=300;
        let height=300;
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
            view_formats:&[]
        };
        let burns = render_device.create_texture(&texture_descriptor);
        let density = render_device.create_texture(&texture_descriptor);
        let pressure = render_device.create_texture(&texture_descriptor);
        let velocity = render_device.create_texture(&texture_descriptor);

        let sampler=render_device.create_sampler(&SamplerDescriptor::default());
        let burns_clear_bing_group=render_device.create_bind_group(
            "clear_bind_group",
            &pipeline.clear_bind_group_layout,
            &BindGroupEntries::sequential((
                &burns.create_view(&Default::default()),
                &burns.create_view(&Default::default()),
                &sampler,
                clear_uniforms.binding().unwrap(),
            )),
        );
        let burns_base_bing_group= render_device.create_bind_group(
            "base_bind_group",
            &pipeline.base_bind_group_layout,
            &BindGroupEntries::sequential((
                base_uniforms.binding().unwrap(),
            )),
        );
        let density_clear_bing_group=render_device.create_bind_group(
            "density_bind_group",
            &pipeline.clear_bind_group_layout,
            &BindGroupEntries::sequential((
                &density.create_view(&Default::default()),
                &density.create_view(&Default::default()),
                &sampler,
                clear_uniforms.binding().unwrap(),
            )),
        );
        let velocity_clear_bing_group=render_device.create_bind_group(
            "velocity_bind_group",
            &pipeline.clear_bind_group_layout,
            &BindGroupEntries::sequential((
                &velocity.create_view(&Default::default()),
                &velocity.create_view(&Default::default()),
                &sampler,
                clear_uniforms.binding().unwrap(),
            )),
        );
        let pressure_clear_bing_group=render_device.create_bind_group(
            "pressure_bind_group",
            &pipeline.clear_bind_group_layout,
            &BindGroupEntries::sequential((
                &pressure.create_view(&Default::default()),
                &pressure.create_view(&Default::default()),
                &sampler,
                clear_uniforms.binding().unwrap(),
            )),
        );

        pipeline.render(world,render_context,&self.state,burns,&burns_base_bing_group,burns_clear_bing_group);
        pipeline.render(world,render_context,&self.state,density,&burns_base_bing_group,density_clear_bing_group);
        pipeline.render(world,render_context,&self.state,pressure,&burns_base_bing_group,velocity_clear_bing_group);
        pipeline.render(world,render_context,&self.state,velocity,&burns_base_bing_group,pressure_clear_bing_group);
        Ok(())
    }
}

// fn prepare_bloom_bind_groups(
//     mut commands: Commands,
//     render_device: Res<RenderDevice>,
//     downsampling_pipeline: Res<BloomDownsamplingPipeline>,
//     upsampling_pipeline: Res<BloomUpsamplingPipeline>,
//     views: Query<(Entity, &BloomTexture)>,
//     uniforms: Res<ComponentUniforms<BloomUniforms>>,
// ) {

// let mut render_pass = TrackedRenderPass::new(&render_device, render_pass);
// if let Some(viewport) = camera.viewport.as_ref() {
// render_pass.set_camera_viewport(viewport);
// }