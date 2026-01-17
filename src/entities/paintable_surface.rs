use avian3d::prelude::{
    Collider, CollisionEventsEnabled, Sensor, SpatialQuery, SpatialQueryFilter,
};
use bevy::{
    app::{App, Update},
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Dir3, Vec3, primitives::Circle},
    mesh::{Mesh, Mesh3d},
    pbr::{MeshMaterial3d, StandardMaterial},
    time::{Time, Timer, TimerMode},
    transform::components::{GlobalTransform, Transform},
};

use crate::systems::despawn_entity::DespawnEntity;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, paint_surface);
}

#[derive(Component, Debug)]
#[require(Collider, Sensor, CollisionEventsEnabled, Transform)]
pub struct Paint;

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

#[derive(Component, Debug)]
#[require(Collider)]
pub struct PaintableSurface;

fn paint_surface(
    mut commands: Commands,
    spatial_query: SpatialQuery,
    mut painting_objects: Query<(&mut PaintingObject, &GlobalTransform), With<PaintingObject>>,
    paintable_surfaces: Query<&PaintableSurface>,
    paint: Query<&MeshMaterial3d<StandardMaterial>, With<Paint>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    let paint_plane = meshes.add(Circle::new(0.1));

    for (mut painting_object, painting_object_transform) in painting_objects.iter_mut() {
        painting_object.timer.tick(time.delta());

        if let Some(ray_hit_data) = spatial_query.cast_ray_predicate(
            (painting_object_transform.translation() + Vec3::new(0., 0.5, 0.)).into(),
            Dir3::NEG_Y,
            5.,
            false,
            &SpatialQueryFilter::default(),
            &|entity| paintable_surfaces.contains(entity) || paint.contains(entity),
        ) && painting_object.timer.just_finished()
        {
            if let Ok(mesh_material) = paint.get(ray_hit_data.entity)
                && let Some(material) = materials.get(mesh_material.0.id())
                && material.base_color != painting_object.colour
            {
                commands.entity(ray_hit_data.entity).insert(DespawnEntity);
            }

            let paint_material = materials.add(StandardMaterial {
                base_color: painting_object.colour.clone(),
                ..Default::default()
            });

            commands.spawn((
                Paint,
                Mesh3d(paint_plane.clone()),
                MeshMaterial3d(paint_material.clone()),
                Transform::from_xyz(
                    painting_object_transform.translation().x,
                    0.1,
                    painting_object_transform.translation().z,
                ),
            ));
        }
    }
}
