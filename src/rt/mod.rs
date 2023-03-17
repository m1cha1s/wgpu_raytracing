use bevy::{
    prelude::{Commands, Image, Res},
    render::{
        render_asset::RenderAssets,
        render_resource::{BindGroupDescriptor, BindGroupEntry, BindingResource},
        renderer::RenderDevice,
    },
};

use self::{
    image::{RtImage, RtImageBindGroup},
    pipeline::RtPipeline,
};

const WORKGROUP_SIZE: u32 = 8;

pub mod image;
pub mod node;
pub mod pipeline;
pub mod plugin;
pub mod state;

pub fn queue_bind_group(
    mut commands: Commands,
    pipeline: Res<RtPipeline>,
    gpu_images: Res<RenderAssets<Image>>,
    rt_image: Res<RtImage>,
    render_device: Res<RenderDevice>,
) {
    let view = &gpu_images[&rt_image.0];
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.texture_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: BindingResource::TextureView(&view.texture_view),
        }],
    });
    commands.insert_resource(RtImageBindGroup(bind_group));
}
