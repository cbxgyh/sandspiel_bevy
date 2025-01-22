use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    // 创建一个新的 Mesh
    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);


    // 顶点数据
    let vertices: Vec<[f32; 2]> = vec![[-1.0, -1.0], [-1.0, 1.0], [1.0, 1.0], [1.0, -1.0]];
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vertices,
    );


    // 索引数据
    let indices: Vec<u32> = vec![0, 1, 2, 0, 2, 3];
    mesh.insert_indices(Indices::U32(indices));


    // 将 Mesh 添加到资产中
    let mesh_handle = meshes.add(mesh);


    // 创建一个用于渲染的纹理
    let texture = images.add(Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 512, // 可以根据需要调整宽度
                height: 512, // 可以根据需要调整高度
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    });


    // 创建一个 RenderTarget
    let render_target = RenderTarget::from(texture.clone());


    // 生成一个实体并添加 Mesh 组件和 Transform 组件
    commands.spawn(PbrBundle {
        mesh: mesh_handle,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: StandardMaterial {
            base_color_texture: Some(texture),
            ..default()
        },
        ..default()
    });


    // 自定义系统用于帧缓冲操作
    commands.add_systems(framebuffer_operation);
}


fn framebuffer_operation(
    mut commands: Commands,
    mut query: Query<Entity, With<Handle<Mesh>>>,
    mut render_targets: ResMut<Assets<RenderTarget>>,
) {
    for entity in query.iter_mut() {
        // 获取关联的 RenderTarget
        if let Some(render_target) = render_targets.get_mut(entity) {
            // 这里可以进行帧缓冲的操作
            // 例如，将当前渲染结果拷贝到另一个帧缓冲
            // 以下是一个示例，你可以根据具体需求修改
            // 假设你想将渲染结果复制到另一个 RenderTarget 或纹理
            let destination = RenderTarget::default();
            // 目前 Bevy 没有直接对应 gl.drawElements 的方法，你可以通过 RenderGraph 等更高级的方式来实现渲染操作
            // 以下是一个简单的示例，可能需要根据具体需求扩展
            commands.entity(entity).insert(PbrBundle {
                mesh: mesh_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                material: StandardMaterial {
                    base_color_texture: Some(destination.texture.clone()),
                    ..default()
                },
                ..default()
            });
        }
    }
}