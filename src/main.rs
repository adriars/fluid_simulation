use bevy::{asset::RenderAssetUsages, prelude::*, render::{extract_resource::ExtractResource, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages}}};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/render.wgsl";

const DISPLAY_FACTOR: u32 = 4;
const SIZE: (u32, u32) = (1280 / DISPLAY_FACTOR, 720 / DISPLAY_FACTOR);
const WORKGROUP_SIZE: u32 = 8;

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).run();
}

// Creates a image texture to draw the particles on and spawns a sprite to render the texture
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {

    // Create a image texture
    let mut image = Image::new_fill(
        Extent3d {
            width: SIZE.0,
            height: SIZE.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::R32Float,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Set up the usages of the image texture
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::STORAGE_BINDING | TextureUsages::TEXTURE_BINDING;

    // Adds the image to the assets server
    let image = images.add(image);

    // Spawns a sprite with the image texture to render the image
    commands.spawn((
        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::new(SIZE.0 as f32, SIZE.1 as f32)),
            ..default()
        },
        Transform::from_scale(Vec3::splat(DISPLAY_FACTOR as f32)),
    ));

    // Spawns a 2D camera
    commands.spawn(Camera2d);

    // Inserts the FluidSimulationImage resource to the world
    commands.insert_resource(FluidSimulationImage {
        texture: image,
    });
}

// Resource containing the image texture to draw the particles on and to render with the sprite, this resource is extracted to the render_world every frame
#[derive(Resource, Clone, ExtractResource)]
struct FluidSimulationImage {
    texture: Handle<Image>,
}