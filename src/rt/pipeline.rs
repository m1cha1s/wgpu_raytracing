use std::{borrow::Cow, mem::size_of, num::NonZeroU64};

use bevy::{
    prelude::{AssetServer, FromWorld, Resource},
    render::{
        render_resource::{
            BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
            BufferBindingType, BufferSize, CachedComputePipelineId, ComputePipelineDescriptor,
            PipelineCache, ShaderStages, StorageTextureAccess, TextureFormat, TextureViewDimension,
        },
        renderer::RenderDevice,
    },
};

use super::sphere::{ExtractedSpheres, Sphere};

#[derive(Resource)]
pub struct RtPipeline {
    pub texture_bind_group_layout: BindGroupLayout,
    pub init_pipeline: CachedComputePipelineId,
    pub update_pipeline: CachedComputePipelineId,
}

impl FromWorld for RtPipeline {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let texture_bind_group_layout =
            world
                .resource::<RenderDevice>()
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[
                        BindGroupLayoutEntry {
                            binding: 0,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::StorageTexture {
                                access: StorageTextureAccess::WriteOnly,
                                format: TextureFormat::Rgba8Unorm,
                                view_dimension: TextureViewDimension::D2,
                            },
                            count: None,
                        },
                        BindGroupLayoutEntry {
                            binding: 1,
                            visibility: ShaderStages::COMPUTE,
                            ty: BindingType::Buffer {
                                ty: BufferBindingType::Storage { read_only: true },
                                has_dynamic_offset: false,
                                min_binding_size: BufferSize::new(
                                    size_of::<ExtractedSpheres>() as u64
                                ),
                            },
                            count: None,
                        },
                    ],
                });

        let shader = world.resource::<AssetServer>().load("shaders/rt.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();

        let init_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: Cow::from("init"),
        });

        let update_pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: None,
            layout: vec![texture_bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader,
            shader_defs: vec![],
            entry_point: Cow::from("update"),
        });

        RtPipeline {
            texture_bind_group_layout,
            init_pipeline,
            update_pipeline,
        }
    }
}
