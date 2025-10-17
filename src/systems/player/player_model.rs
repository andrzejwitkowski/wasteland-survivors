use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::components::player::player::{Player, PlayerAnimation, PlayerModel};

const PLAYER_MODEL: &str = "models/dummy/dummy.glb#Scene0";
const PLAYER_RUN: &str = "models/dummy/dummy.glb#Animation0";
const PLAYER_WALK: &str = "models/dummy/dummy.glb#Animation1";
const PLAYER_SCALE: f32 = 5.0;

pub fn init_player_model(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    player_query: Query<Entity, (With<Player>, Without<PlayerModel>)>,
) {
    let player_model = asset_server.load(PLAYER_MODEL);

    let (graph, index) =
        AnimationGraph::from_clips([asset_server.load(PLAYER_WALK), asset_server.load(PLAYER_RUN)]);

    // Store the animation graph as an asset.
    let graph_handle = animation_graphs.add(graph);

    // Create a component that stores a reference to our animation.
    let walk = PlayerAnimation { index: index[0] };

    let run = PlayerAnimation { index: index[1] };

    if let Ok(player) = player_query.single() {
        info!("Player model added to player");

        let scene_root = SceneRoot(player_model.clone());
        commands
            .entity(player)
            .insert(Transform::from_scale(Vec3::splat(PLAYER_SCALE)))
            .insert(scene_root)
            .insert(PlayerModel {
                model: player_model.clone(),
                graph_handle: graph_handle,
                walk_clip: Some(walk.clone()),
                run_clip: Some(run),
            })
            .observe(play_animation_when_ready);

        info!("Player model initialized");
    }
}

fn play_animation_when_ready(
    scene_ready: On<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    player_query: Query<&PlayerModel>,
) {
    if let Ok(player) = player_query.get(scene_ready.entity) {
        for child in children.iter_descendants(scene_ready.entity) {
            commands.entity(child).insert(AnimationGraphHandle(player.graph_handle.clone()));
        }
    }
}

pub fn play_player_animation(
    mut animation_players: Query<&mut AnimationPlayer>,
    player_query: Query<&PlayerModel>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Play walk animation on 1 key
    if keyboard.just_pressed(KeyCode::Digit1) {
        for mut animation_player in animation_players.iter_mut() {
            if let Ok(player) = player_query.single() {
                animation_player.stop_all();
                animation_player.play(player.walk_clip.as_ref().unwrap().index).repeat();
                info!("Playing walk animation");
            }
        }
    }

    // Play walk animation on 2 key
    if keyboard.just_pressed(KeyCode::Digit2) {
        for mut animation_player in animation_players.iter_mut() {
            if let Ok(player) = player_query.single() {
                animation_player.stop_all();
                animation_player.play(player.run_clip.as_ref().unwrap().index).repeat();
                info!("Playing run animation");
            }
        }
    }
}
