use bevy::{
    prelude::{Deref, Handle, Image, Resource},
    render::{extract_resource::ExtractResource, render_resource::BindGroup},
};

#[derive(Resource, Clone, Deref, ExtractResource)]
pub struct RtImage(pub Handle<Image>);

#[derive(Resource)]
pub struct RtImageBindGroup(pub BindGroup);
