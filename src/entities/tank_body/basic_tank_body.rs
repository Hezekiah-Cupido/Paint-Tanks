use avian3d::prelude::{Collider, Friction, Mass, RigidBody};
use bevy::{
    app::{App, Startup},
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        resource::Resource,
        system::{Commands, EntityCommands, Res},
        world::World,
    },
    gltf::GltfAssetLabel,
    scene::{Scene, SceneRoot},
    transform::components::Transform,
};

use crate::entities::tank_body::{TankBody, TankBodySpawner};

trait BasicTankBodySpawner {
    fn spawn_basic_tank_body<'a>(&'a mut self, world: &World) -> Option<EntityCommands<'a>>;
}

#[derive(Resource)]
pub struct BasicTankBodyAsset(pub Option<Handle<Scene>>);

#[derive(Component)]
#[require(TankBody, Transform::from_xyz(0., 0.5, 0.))]
pub struct BasicTankBody;

impl TankBodySpawner for BasicTankBody {
    fn spawn<'a>(&self, commands: &'a mut Commands, world: &World) -> EntityCommands<'a> {
        commands.spawn_basic_tank_body(world).unwrap() // TODO: log failure
    }
}

impl BasicTankBodySpawner for Commands<'_, '_> {
    fn spawn_basic_tank_body(&mut self, world: &World) -> Option<EntityCommands<'_>> {
        if let Some(tank_body_asset) = world.get_resource::<BasicTankBodyAsset>()
            && let Some(tank_body) = tank_body_asset.0.clone()
        {
            return Some(self.spawn((
                BasicTankBody,
                RigidBody::Dynamic,
                Collider::cuboid(1., 1., 1.),
                Mass(100.),
                Friction::new(0.9),
                SceneRoot(tank_body),
            )));
        }

        None
    }
}

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, load_asset);
}

fn load_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tank_body = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_body.gltf"));

    commands.insert_resource(BasicTankBodyAsset(Some(tank_body)));
}
