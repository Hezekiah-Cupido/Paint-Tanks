use avian3d::prelude::{Collider, ColliderAabb, SpatialQuery, SpatialQueryFilter};
use bevy::{
    app::{App, Plugin, Update},
    asset::{Asset, Assets, Handle},
    camera::{Camera, Camera2d, RenderTarget, visibility::RenderLayers},
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        entity::Entity,
        hierarchy::Children,
        lifecycle::Discard,
        message::MessageReader,
        observer::On,
        prelude::ReflectComponent,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    image::Image,
    math::{Dir3, Vec3, primitives::Rectangle},
    mesh::{Mesh, Mesh2d},
    pbr::{MeshMaterial3d, StandardMaterial},
    prelude::Vec2,
    reflect::{Reflect, TypePath},
    render::{
        render_resource::{AsBindGroup, ShaderType, TextureFormat},
        storage::ShaderBuffer,
    },
    sprite_render::{Material2d, Material2dPlugin, MeshMaterial2d},
    time::{Time, Timer, TimerMode},
    transform::components::GlobalTransform,
    world_serialization::WorldInstanceReady,
};

use crate::{game_state::ClearWorld, systems::despawn_entity::DespawnEntity};

pub struct PaintableSurfacePlugin;

impl Plugin for PaintableSurfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((Material2dPlugin::<PaintMaterial>::default(),))
            .add_systems(Update, paint_surface)
            .add_observer(add_paint_material_to_map)
            .add_observer(clear_paintable_surface_data);
    }
}

const PAINT_SHADER_PATH: &str = "shaders/paint_material.wgsl";

#[derive(Component)]
struct PaintRenderSurface;

#[derive(Component)]
struct PaintRenderCamera(Entity);

#[derive(ShaderType, Clone, Copy, Debug, Default)]
pub struct PaintData {
    pub coordinates: Vec2,
    pub colour: LinearRgba,
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct PaintMaterial {
    #[storage(100, read_only)]
    pub paint_data_buffer: Handle<ShaderBuffer>,

    #[texture(0)]
    #[sampler(1)]
    pub texture: Handle<Image>,
    #[uniform(2)]
    pub mesh_surface_resolution: Vec2,
}

impl Material2d for PaintMaterial {
    fn fragment_shader() -> bevy::shader::ShaderRef {
        PAINT_SHADER_PATH.into()
    }
}

#[derive(Component, Debug)]
pub struct PaintingObject {
    colour: Color,
    timer: Timer,
}

impl PaintingObject {
    pub fn new(colour: Color) -> Self {
        Self {
            colour,
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
#[require(Collider)]
pub struct PaintableSurface;

fn paint_surface(
    spatial_query: SpatialQuery,
    mut painting_objects: Query<(&mut PaintingObject, &GlobalTransform), With<PaintingObject>>,
    paintable_surfaces: Query<&PaintableSurface>,
    mut paint_materials: ResMut<Assets<PaintMaterial>>,
    paint_render_target: Query<(&RenderTarget, &PaintRenderCamera)>,
    time: Res<Time>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    if let Some((_, paint_material)) = paint_materials.iter_mut().nth(0) {
        let mut data = Vec::new();

        let mut buffer = buffers.get_mut(&paint_material.paint_data_buffer).unwrap();

        for (mut painting_object, painting_object_transform) in painting_objects.iter_mut() {
            painting_object.timer.tick(time.delta());

            if let Some(ray_hit_data) = spatial_query.cast_ray_predicate(
                (painting_object_transform.translation() + Vec3::new(0., 0.5, 0.)).into(),
                Dir3::NEG_Y,
                5.,
                false,
                &SpatialQueryFilter::default(),
                &|entity| paintable_surfaces.contains(entity),
            ) && painting_object.timer.just_finished()
                && let Some((render_target, _)) = paint_render_target
                    .iter()
                    .filter(|(_, p)| p.0 == ray_hit_data.entity)
                    .nth(0)
                && let Some(image_handle) = render_target.as_image()
            {
                data.push(PaintData {
                    colour: painting_object.colour.into(),
                    coordinates: (
                        painting_object_transform.translation().x,
                        painting_object_transform.translation().z,
                    )
                        .into(),
                });

                paint_material.texture = image_handle.clone();
            }
        }

        buffer.set_data(data);
    }
}

fn add_paint_material_to_map(
    trigger: On<WorldInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    paintable_surfaces: Query<&ColliderAabb, With<PaintableSurface>>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
    mut paint_materials: ResMut<Assets<PaintMaterial>>,
    mut images: ResMut<Assets<Image>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
) {
    if let Ok(surface_collider_aabb) = paintable_surfaces.get(trigger.entity) {
        for descendant in children.iter_descendants(trigger.entity) {
            if matches!(
                mesh_materials
                    .get(descendant)
                    .ok()
                    .and_then(|id| asset_materials.get(id.id())),
                Some(_)
            ) {
                let surface_bounding_box = surface_collider_aabb.size();

                let image = Image::new_target_texture(
                    1024,
                    1024,
                    TextureFormat::Rgba8UnormSrgb,
                    Some(TextureFormat::Rgba8UnormSrgb),
                );

                let image_handle = images.add(image);

                let data = vec![PaintData {
                    coordinates: (0., 0.).into(),
                    colour: Color::linear_rgba(1., 1., 1., 1.).into(),
                }];

                let paint_data_buffer = buffers.add(ShaderBuffer::from(data));

                let paint_material = paint_materials.add(PaintMaterial {
                    paint_data_buffer: paint_data_buffer,
                    texture: image_handle.clone(),
                    mesh_surface_resolution: (surface_bounding_box.x, surface_bounding_box.z)
                        .into(),
                });

                let first_render_layer = RenderLayers::layer(1);

                commands.spawn((
                    PaintRenderSurface,
                    Mesh2d(meshes.add(Rectangle::from_size((1024., 1024.).into()))),
                    MeshMaterial2d(paint_material),
                    first_render_layer.clone(),
                ));

                commands.spawn((
                    Camera2d,
                    Camera {
                        order: -1,
                        ..Default::default()
                    },
                    RenderTarget::Image(image_handle.clone().into()),
                    PaintRenderCamera(trigger.entity.clone()),
                    first_render_layer,
                ));

                let material_handle = asset_materials.add(StandardMaterial {
                    base_color_texture: Some(image_handle),
                    unlit: false,
                    ..Default::default()
                });

                commands
                    .entity(descendant)
                    .insert(MeshMaterial3d(material_handle));
            }
        }
    }
}

fn clear_paintable_surface_data(
    _: On<Discard, PaintableSurface>,
    mut commands: Commands,
    clear_world_reader: MessageReader<ClearWorld>,
    paint_render_camera: Query<Entity, With<PaintRenderCamera>>,
    paint_render_surface: Query<Entity, With<PaintRenderSurface>>,
) {
    if !clear_world_reader.is_empty() {
        commands
            .entity(paint_render_camera.single().unwrap())
            .insert(DespawnEntity);

        commands
            .entity(paint_render_surface.single().unwrap())
            .insert(DespawnEntity);
    }
}
