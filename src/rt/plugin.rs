use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin,
        render_graph::RenderGraph,
        render_resource::{BufferDescriptor, BufferUsages},
        renderer::RenderDevice,
        RenderApp, RenderSet,
    },
};

use super::{
    image::RtImage,
    node::RtNode,
    pipeline::RtPipeline,
    queue_bind_group,
    sphere::{prepare_spheres, ExtractedSpheres, Sphere, SpheresMeta},
};

pub struct RtPlugin;

impl Plugin for RtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractResourcePlugin::<RtImage>::default());

        let render_device = app.world.resource::<RenderDevice>();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("spheres"),
            size: std::mem::size_of::<Sphere>() as u64,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        app.add_plugin(ExtractResourcePlugin::<ExtractedSpheres>::default());

        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<RtPipeline>()
            .insert_resource(SpheresMeta {
                buffer,
                bind_group: None,
            })
            .add_system(queue_bind_group.in_set(RenderSet::Queue))
            .add_system(prepare_spheres.in_set(RenderSet::Prepare));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("rt", RtNode::default());
    }
}
