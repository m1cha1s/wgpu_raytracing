use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    window::WindowResolution,
};
use rt::{
    image::RtImage,
    plugin::RtPlugin,
    sphere::{Sphere, Spheres},
};

mod rt;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 800.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                title: "Raytracing".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(RtPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
    );

    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    let image = images.add(image);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
            ..default()
        },
        texture: image.clone(),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(RtImage(image));

    commands.insert_resource(Spheres {
        spheres: vec![Sphere {
            center: [0.0, 0.0, -1.0],
            radius: 0.5,
        }],
    })
}
