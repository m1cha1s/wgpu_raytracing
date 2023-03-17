use bevy::{
    prelude::*,
    render::{
        extract_resource::ExtractResourcePlugin, render_graph::RenderGraph, RenderApp, RenderSet,
    },
};

use super::{image::RtImage, node::RtNode, pipeline::RtPipeline, queue_bind_group};

pub struct RtPlugin;

impl Plugin for RtPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractResourcePlugin::<RtImage>::default());
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .init_resource::<RtPipeline>()
            .add_system(queue_bind_group.in_set(RenderSet::Queue));

        let mut render_graph = render_app.world.resource_mut::<RenderGraph>();
        render_graph.add_node("rt", RtNode::default());
    }
}
