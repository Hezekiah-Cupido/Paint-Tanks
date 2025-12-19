use avian3d::prelude::{Collider, Friction, Mass, RigidBody};
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, EntityCommands},
    },
    gltf::GltfAssetLabel,
    scene::SceneRoot,
};

use crate::entities::tank_body::{TankBody, TankBodySpawner};

trait BasicTankBodySpawner {
    fn spawn_basic_tank_body<'a>(&'a mut self, asset_server: &AssetServer) -> EntityCommands<'a>;
}

#[derive(Component)]
#[require(TankBody)]
pub struct BasicTankBody;

impl TankBodySpawner for BasicTankBody {
    fn spawn<'a>(
        &self,
        commands: &'a mut Commands,
        asset_server: &AssetServer,
    ) -> EntityCommands<'a> {
        commands.spawn_basic_tank_body(asset_server)
    }
}

impl BasicTankBodySpawner for Commands<'_, '_> {
    fn spawn_basic_tank_body(&mut self, asset_server: &AssetServer) -> EntityCommands<'_> {
        let tank_body = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tank_body.gltf"));

        return self.spawn((
            BasicTankBody,
            RigidBody::Dynamic,
            Collider::cuboid(1., 1.25, 1.),
            Mass(100.),
            Friction::new(0.9),
            SceneRoot(tank_body),
        ));
    }
}
