use avian3d::prelude::{Collider, SpatialQuery, SpatialQueryFilter};
use bevy::{
    app::{App, Update},
    asset::{Asset, Assets},
    color::{Color, LinearRgba},
    ecs::{
        component::Component,
        hierarchy::Children,
        observer::On,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Dir3, Vec3, vec3},
    pbr::{Material, MaterialPlugin, MeshMaterial3d, StandardMaterial},
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    scene::SceneInstanceReady,
    time::{Time, Timer, TimerMode},
    transform::components::GlobalTransform,
};

const PAINT_SHADER_PATH: &str = "shaders/paint_material.wgsl";

pub fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<PaintMaterial>::default())
        .add_systems(Update, paint_surface)
        .add_observer(add_paint_material_to_map);
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Debug)]
pub struct PaintMaterial {
    #[uniform(0)]
    pub coordinates: Vec3,
    #[uniform(1)]
    pub colour: LinearRgba,
}

impl Material for PaintMaterial {
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

#[derive(Component, Debug)]
#[require(Collider)]
pub struct PaintableSurface;

fn paint_surface(
    spatial_query: SpatialQuery,
    mut painting_objects: Query<(&mut PaintingObject, &GlobalTransform), With<PaintingObject>>,
    paintable_surfaces: Query<&PaintableSurface>,
    mut paint_materials: ResMut<Assets<PaintMaterial>>,
    time: Res<Time>,
) {
    for (mut painting_object, painting_object_transform) in painting_objects.iter_mut() {
        painting_object.timer.tick(time.delta());

        if let Some(_) = spatial_query.cast_ray_predicate(
            (painting_object_transform.translation() + Vec3::new(0., 0.5, 0.)).into(),
            Dir3::NEG_Y,
            5.,
            false,
            &SpatialQueryFilter::default(),
            &|entity| paintable_surfaces.contains(entity),
        ) && painting_object.timer.just_finished()
        {
            let paint_material = paint_materials.iter_mut().nth(0).unwrap().1;

            paint_material.colour = painting_object.colour.into();
            paint_material.coordinates = vec3(
                painting_object_transform.translation().x,
                0.,
                painting_object_transform.translation().z,
            );
        }
    }
}

fn add_paint_material_to_map(
    trigger: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    paintable_surfaces: Query<&PaintableSurface>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
    mut paint_materials: ResMut<Assets<PaintMaterial>>,
) {
    if let Ok(_) = paintable_surfaces.get(trigger.entity) {
        for descendant in children.iter_descendants(trigger.entity) {
            if let Some(_) = mesh_materials
                .get(descendant)
                .ok()
                .and_then(|id| asset_materials.get_mut(id.id()))
            {
                let paint_material = paint_materials.add(PaintMaterial {
                    coordinates: vec3(0., 0., 0.),
                    colour: Color::linear_rgba(1., 1., 1., 1.).into(),
                });

                commands
                    .entity(descendant)
                    .insert(MeshMaterial3d(paint_material));
            }
        }
    }
}
