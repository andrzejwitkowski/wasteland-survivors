use crate::components::CameraController;
use crate::player::player::Player;
use crate::systems::camera_system::CameraFollow;
use bevy::prelude::*;
use bevy::scene::SceneInstanceReady;
use bevy_landmass::{Archipelago3d, ArchipelagoOptions, ArchipelagoRef, FromAgentRadius, Island};
use bevy_rerecast::debug::{DetailNavmeshGizmo, PolygonNavmeshGizmo};
use bevy_rerecast::prelude::*;
use bevy_rerecast::NavmeshSettings;
use landmass_rerecast::{Island3dBundle, NavMeshHandle3d};

static LEVEL_SCENE_FILE: &str = "level/01/level1.glb#Scene0";
static LEVEL_FILE: &str = "../../../assets/level/01/level.glb";

#[derive(Resource)]
struct ImportHandles {
    scene: Handle<Scene>,
    counted: bool,
}

#[derive(Resource)]
pub struct LevelSceneHandle(Handle<Scene>);

#[derive(Resource, Default)]
pub struct NavRes {
    handle: Option<Handle<Navmesh>>,
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Scene> = asset_server.load(LEVEL_SCENE_FILE);
    commands.spawn((
        SceneRoot(handle.clone()),
        Visibility::Visible,
        Transform::default(),
        Name::new("Level Scene"),
    ));

    info!("Requesting load of levels/level.glb#Scene0");
    commands.insert_resource(LevelSceneHandle(handle));
}

pub fn on_scene_loaded(
    trigger: On<SceneInstanceReady>,
    level_scene: Res<LevelSceneHandle>,
    scenes: Res<Assets<Scene>>,
    player: Query<(Entity, &Transform), With<Player>>,
    mut navmesh_gen: NavmeshGenerator,
    mut navres: ResMut<NavRes>,
    mut commands: Commands,
) {
    info!("Scene loaded");
    if let Some(scene) = scenes.get(&level_scene.0) {
        let world = &scene.world;

        info!("Scene loaded; inspecting entities and their GltfExtras…");

        for entity_ref in world.iter_entities() {
            let name = entity_ref.get::<Name>().map(|n| n.as_str().to_string());
            info!("Entity {:?}", name);

            if let Some(extras) = entity_ref.get::<GltfExtras>() {
                info!("Entity {:?}: extras={:?}", name, extras);
                if let Some(transform) = entity_ref.get::<Transform>() {
                    info!("Entity {:?}: transform={:?}", name, transform);
                }
            }
        }
    }

    if let Ok((player_entity, player_transform)) = player.single() {
        let camera_offset = Vec3::new(-65.36, 50.0, 105.36);
        let camera_position = player_transform.translation + camera_offset;

        // init camera 3d and look at player
        let camera_entity = (
            Transform::from_translation(camera_position)
                .looking_at(player_transform.translation, Vec3::Y),
            Camera3d::default(),
            Camera::default(),
            CameraFollow { offset: camera_offset, ..Default::default() },
            CameraController,
        );

        commands.spawn(camera_entity).insert(Name::new("Main Camera"));
    } else {
        warn!("No player found");
    }

    // generate navmesh once
    let settings = NavmeshSettings::from_agent_3d(0.6, 1.8);
    navres.handle = Some(navmesh_gen.generate(settings));
}

pub fn on_navmesh_ready(trigger: On<NavmeshReady>, mut commands: Commands, nav_res: Res<NavRes>) {
    info!("Navmesh ready: {:?}", trigger.event().0);
    let asset_id = trigger.event().0; // AssetId<Navmesh>
    if let Some(h) = &nav_res.handle {
        // check if this is the navmesh we're
        if h.id() == asset_id {

            info!("Spawning navmesh gizmo");
            commands.spawn(DetailNavmeshGizmo::new(asset_id));

            // spawn landmass island
            let archipelago_id =
                commands.spawn(Archipelago3d::new(ArchipelagoOptions::from_agent_radius(0.6))).id();

            commands.spawn(Island3dBundle {
                nav_mesh: NavMeshHandle3d(h.clone()),
                archipelago_ref: ArchipelagoRef::new(archipelago_id),
                island: Island,
            });
        }
    }
}
