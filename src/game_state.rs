use crate::entities::tank_body::basic_tank_body::BasicTankBody;
use crate::entities::turret::basic_turret::BasicTurret;
use bevy::{
    app::{Plugin, Startup, Update},
    color::Color,
    ecs::{
        entity::Entity,
        message::{Message, MessageReader, MessageWriter},
        query::{With, Without},
        resource::Resource,
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    input::{ButtonInput, keyboard::KeyCode},
    prelude::Deref,
    state::{
        app::AppExtStates,
        state::{NextState, OnEnter, State, States},
    },
};

use crate::{
    maps::{Inactive, SpawnPoint, spawn_map},
    tank::{Player, SpawnTank, Team},
};

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_resource::<GameResources>()
            .init_state::<GameState>()
            .add_message::<SetGameState>()
            .add_systems(Startup, init_in_game_state)
            .add_systems(Update, (input_toggle_game_state, toggle_game_state))
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_map, spawn_players).chain(),
            );
    }
}

#[derive(Debug, Deref, Message)]
struct SetGameState(GameState);

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    EndScreen,
}

impl GameState {
    fn next(&self) -> Self {
        match *self {
            GameState::MainMenu => GameState::InGame,
            GameState::InGame => GameState::EndScreen,
            GameState::EndScreen => GameState::MainMenu,
        }
    }
}

fn init_in_game_state(mut commands: Commands) {
    commands.insert_resource(GameResources {
        teams: vec![
            TeamInfo {
                team: Team(Color::srgb(1., 0., 0.)),
                player_type: Player::User,
                is_active: false,
            },
            TeamInfo {
                team: Team(Color::srgb(0., 1., 0.)),
                player_type: Player::Program,
                is_active: false,
            },
            TeamInfo {
                team: Team(Color::srgb(0., 0., 1.)),
                player_type: Player::Program,
                is_active: false,
            },
        ],
    });
}

fn input_toggle_game_state(
    mut set_game_state_event_writer: MessageWriter<SetGameState>,
    current_state: Res<State<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        set_game_state_event_writer.write(SetGameState(current_state.next()));
    }
}

fn toggle_game_state(
    mut set_game_state_event_reader: MessageReader<SetGameState>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Some(game_state) = set_game_state_event_reader.read().nth(0) {
        next_state.set(game_state.0.clone());
        println!("Changing state...");
    }
}

#[derive(Resource, Default)]
struct GameResources {
    teams: Vec<TeamInfo>,
}

#[derive(Debug, Clone, Copy)]
struct TeamInfo {
    team: Team,
    player_type: Player,
    is_active: bool,
}

fn spawn_players(
    mut spawn_tank_event_writer: MessageWriter<SpawnTank>,
    mut game_state_res: ResMut<GameResources>,
    spawn_points: Query<Entity, (With<SpawnPoint>, Without<Inactive>)>,
) {
    game_state_res
        .teams
        .iter_mut()
        .zip(spawn_points.iter())
        .for_each(|(team_info, spawn_point)| {
            spawn_tank_event_writer.write(SpawnTank {
                player: team_info.player_type,
                team: team_info.team,
                turret: Box::new(BasicTurret {}),
                tank_body: Box::new(BasicTankBody {}),
                spawn_point: spawn_point,
            });

            team_info.is_active = true;
        });
}
