use bevy::render::{
    render_graph,
    render_resource::{CachedPipelineState, ComputePassDescriptor, PipelineCache},
};

use crate::{HEIGHT, WIDTH};

use super::{image::RtImageBindGroup, pipeline::RtPipeline, state::RtState, WORKGROUP_SIZE};

pub struct RtNode {
    pub state: RtState,
}

impl Default for RtNode {
    fn default() -> Self {
        Self {
            state: RtState::Loading,
        }
    }
}

impl render_graph::Node for RtNode {
    fn update(&mut self, world: &mut bevy::prelude::World) {
        let pipeline = world.resource::<RtPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        match self.state {
            RtState::Loading => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.init_pipeline)
                {
                    self.state = RtState::Init;
                }
            }
            RtState::Init => {
                if let CachedPipelineState::Ok(_) =
                    pipeline_cache.get_compute_pipeline_state(pipeline.update_pipeline)
                {
                    self.state = RtState::Update;
                }
            }
            RtState::Update => {}
        }
    }
    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext,
        world: &bevy::prelude::World,
    ) -> Result<(), render_graph::NodeRunError> {
        let texture_bind_group = &world.resource::<RtImageBindGroup>().0;
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = world.resource::<RtPipeline>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        pass.set_bind_group(0, texture_bind_group, &[]);

        match self.state {
            RtState::Loading => {}
            RtState::Init => {
                let init_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.init_pipeline)
                    .unwrap();
                pass.set_pipeline(init_pipeline);
                pass.dispatch_workgroups(
                    WIDTH as u32 / WORKGROUP_SIZE,
                    HEIGHT as u32 / WORKGROUP_SIZE,
                    1,
                );
            }
            RtState::Update => {
                let update_pipeline = pipeline_cache
                    .get_compute_pipeline(pipeline.update_pipeline)
                    .unwrap();
                pass.set_pipeline(update_pipeline);
                pass.dispatch_workgroups(
                    WIDTH as u32 / WORKGROUP_SIZE,
                    HEIGHT as u32 / WORKGROUP_SIZE,
                    1,
                );
            }
        }

        Ok(())
    }
}
