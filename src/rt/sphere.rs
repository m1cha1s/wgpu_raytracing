use bevy::{
    core::cast_slice,
    prelude::{Res, ResMut, Resource},
    render::{
        extract_resource::ExtractResource,
        render_resource::{BindGroup, Buffer},
        renderer::RenderQueue,
    },
};
use bytemuck;

const MAX_SPHERES: usize = 20;

#[repr(C)]
#[derive(Resource, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
}

#[derive(Resource, Default, Clone)]
pub struct Spheres {
    pub spheres: Vec<Sphere>,
}

#[repr(C)]
#[derive(Resource, Default, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ExtractedSpheres {
    pub sphere_count: u32,
    pub spheres: [Sphere; MAX_SPHERES],
}

impl ExtractResource for ExtractedSpheres {
    type Source = Spheres;

    fn extract_resource(source: &Self::Source) -> Self {
        let mut spheres: [Sphere; MAX_SPHERES] = [Sphere {
            center: [0.0; 3],
            radius: 0.0,
        }; MAX_SPHERES];
        let mut i = 0;

        for sphere in source.spheres.iter() {
            spheres[i] = sphere.clone();
            i += 1;
        }

        Self {
            spheres,
            sphere_count: i as u32,
        }
    }
}

#[derive(Resource)]
pub struct SpheresMeta {
    pub buffer: Buffer,
    pub bind_group: Option<BindGroup>,
}

pub fn prepare_spheres(
    spheres: Res<ExtractedSpheres>,
    spheres_meta: ResMut<SpheresMeta>,
    render_queue: Res<RenderQueue>,
) {
    render_queue.write_buffer(&spheres_meta.buffer, 0, cast_slice(&[spheres.clone()]))
}
