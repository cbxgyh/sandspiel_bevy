use std::borrow::Cow;
use bevy::prelude::*;
use bevy::render::extract_component::{ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin};
use bevy::render::render_resource::{BindGroup, BindGroupDescriptor, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer, BufferInitDescriptor, BufferUsages, CachedRenderPipelineId, ColorTargetState, ColorWrites, Extent3d, FragmentState, ImageDataLayout, LoadOp, MultisampleState, Operations, PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, ShaderType, StoreOp, Texture, TextureDescriptor, TextureDimension, TextureFormat, TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode};
use bevy::render::render_resource::binding_types::{sampler, texture_2d, uniform_buffer};
use bevy::render::RenderApp;
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::texture::{BevyDefault, TextureFormatPixelInfo};
use bytemuck::{Pod, Zeroable};
use crate::universe::Universe;

pub struct  PipelineSandPlugin;


impl Plugin for  PipelineSandPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ExtractComponentPlugin::<SandUniform>::default(),
                UniformComponentPlugin::<SandUniform>::default(),

                ExtractComponentPlugin::<SanVertexInput>::default(),
                UniformComponentPlugin::<SanVertexInput>::default(),
                ))

        ;

    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            // Initialize the pipeline
            .init_resource::<PipelineSand>();
    }
}

#[repr(C)]
#[derive(Component, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
pub struct SandUniform {
    t: f32,
    dpi: f32,
    resolution: Vec2,
    // 0 1
    is_snapshot: u32
}

impl Default for  SandUniform {
    fn default() -> Self {
        Self{
            t:0.,
            dpi:0.,
            resolution:Vec2::new(300.,300.),
            is_snapshot:0
        }
    }
}

#[repr(C)]
#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType, Pod, Zeroable)]
pub struct SanVertexInput {
    aPosition : Vec2,
}

#[derive(Resource)]
pub struct  PipelineSand{
    pub pipeline: CachedRenderPipelineId,
    pub sand_bind_layout: BindGroupLayout,
    pub sand_vertex_bind_layout: BindGroupLayout,
    pub texture_view: TextureView,
    pub sampler: Sampler,
}
impl PipelineSand {
    pub fn render(&self,
                  world:&World,
                  render_context: &mut RenderContext,

    ){
        let sand_uniforms = world.resource::<ComponentUniforms<SandUniform>>();
        let sand_data_uniforms = world.resource::<ComponentUniforms<SanVertexInput>>();
        let render_device = world.resource::<RenderDevice>();

        let pipeline_cache = world.resource::<PipelineCache>();
        // println!("PipelineSand111");
        let Some(sand_vertex_binding) = sand_uniforms.uniforms().binding() else {
            return;
        };
        let Some(sand_binding) = sand_data_uniforms.uniforms().binding() else {
            return;
        };
        println!("PipelineSand222");
        let sand_vertex_bind=render_device.create_bind_group(
            "sand_vertex_bind",
            &self.sand_vertex_bind_layout,
            &BindGroupEntries::sequential((
                sand_vertex_binding,
            ))
        );

        let sand_bind=render_device.create_bind_group(
            "sand_bind",
            &self.sand_bind_layout,
            &BindGroupEntries::sequential((
                &self.texture_view,
                &self.texture_view,
                &self.sampler,
                sand_binding
            ))
        );

        let sand_pipeline = pipeline_cache
            .get_render_pipeline(self.pipeline)
            .unwrap();
        let mut pass = render_context
            .command_encoder()
            .begin_render_pass(&RenderPassDescriptor {
                label: Some("reset_render_pass"),
                color_attachments: &[Some(
                    RenderPassColorAttachment {
                        view: &self.texture_view,
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
        pass.set_pipeline(sand_pipeline);
        pass.set_bind_group(0, &sand_vertex_bind, &[]);
        pass.set_bind_group(1, &sand_bind, &[]);

    }
}
impl FromWorld for PipelineSand {
    fn from_world(world: &mut World) -> Self {

        let render_device=world.resource::<RenderDevice>();
        let asser_server = world.resource::<AssetServer>();
        let render_queue =world.resource::<RenderQueue>();


        // let universe=world.resource::<Universe>();

        // let texture_data =universe.cells();
        // let width=universe.width as u32;
        // let height=universe.height as u32;
        let width=300.0 as u32;
        let height=300. as u32;
        let sampler1=render_device.create_sampler(&SamplerDescriptor::default());

        let data_texture = render_device.create_texture(&TextureDescriptor {
            label: Some("Data Texture"),
            size: Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // let cell_count = (width * height * 4) as usize;
        // let format = TextureFormat::bevy_default();
        let mut image = Image::default();
        // image.data = vec![255 as u8; cell_count];
        let format_size = image.texture_descriptor.format.pixel_size();
        render_queue.write_texture(
            data_texture.as_image_copy(),
            &image.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(image.width() * format_size as u32),
                rows_per_image: None,
            },
            image.texture_descriptor.size,

        );
        let texture_view = data_texture.create_view(&TextureViewDescriptor::default());
        // let sampler = render_device.create_sampler(&SamplerDescriptor::default());

        let frag_shader = asser_server.load("shader/sand.wgsl");
        let vert_shader = asser_server.load("shader/sandVertex.wgsl");

        let sand_layout = render_device.create_bind_group_layout(
            "sand_layout",
            &BindGroupLayoutEntries::sequential(
                // The layout entries will only be visible in the fragment stage
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<SandUniform>(false),
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
                    uniform_buffer::<SanVertexInput>(false),
                ),
            ),
        );
        let sand_buffer_layout =
                VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, vec![
                    // Position
                    VertexFormat::Float32x2,
                ]);




        let pipeline_cache =world
            .resource_mut::<PipelineCache>();
        let pipeline = pipeline_cache
            // This will add the pipeline to the cache and queue it's creation
            .queue_render_pipeline(RenderPipelineDescriptor {
                label: Some("clear_pipeline".into()),
                layout: vec![sand_vertex_layout.clone(),sand_layout.clone()],
                // This will setup a fullscreen triangle for the vertex state
                vertex: VertexState {
                    shader: vert_shader.clone(),  // 传递片段着色器作为顶点着色器
                    shader_defs: vec![],
                    entry_point: "main".into(),  // 顶点着色器的入口点
                    buffers: vec![sand_buffer_layout.clone()],  // 没有顶点数据，因此缓冲区为空
                },
                fragment: Some(FragmentState {
                    shader: frag_shader.clone(),
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
            pipeline,
            sand_bind_layout:sand_layout,
            sand_vertex_bind_layout:sand_vertex_layout,
            texture_view,
            sampler:sampler1

        }

    }
}