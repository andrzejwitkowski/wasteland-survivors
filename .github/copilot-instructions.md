# Bevy 0.17.1 Development Rules for AI Assistant

## Core Expertise
- **Bevy Version**: 0.17.1 (CRITICAL - API changes significantly between versions)
- **Rust Expert**: Idiomatic Rust patterns, ownership, lifetimes, trait bounds
- **Pattern Expert**: Builder patterns, type-state patterns, ECS patterns
- **DSL Preference**: Favor Kotlin-like fluent/chainable APIs when possible

## Critical: Always Check Current API First
⚠️ **BEFORE answering ANY Bevy question:**
1. Use `context7` MCP server to verify current API
2. Bevy APIs change rapidly - trained data is likely outdated
3. Check official Bevy 0.17.1 docs, examples, and migration guides
4. Verify component names, plugin names, and system signatures

## Bevy 0.17.1 Specific Changes to Remember

### Rendering & Mesh Changes
```rust
// OLD (pre 0.14)
PbrBundle {
    mesh: meshes.add(shape::Plane::default()),
    material: materials.add(Color::RED),
    ..default()
}

// NEW (0.17.1)
(
    Mesh3d(meshes.add(Plane3d::default().mesh().size(1.0, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
    Transform::default(),
)
Camera Changes
// OLD
Camera3dBundle::default()

// NEW (0.17.1)
(
    Camera3d::default(),
    Transform::from_xyz(0.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
)
Color Space
// Always use sRGB color space in 0.17.1
Color::srgb(1.0, 0.0, 0.0)       // For opaque colors
Color::srgba(1.0, 0.0, 0.0, 0.5) // For transparent colors
Picking System (bevy_picking)
// Integrated into core in 0.17.1
use bevy::picking::prelude::*;

// Add plugin
app.add_plugins(MeshPickingPlugin);

// Events
Pointer<Click>  // Click events
Pointer<Over>   // Hover enter
Pointer<Out>    // Hover exit
Pointer<Move>   // Pointer movement

// Message readers (not EventReader)
mut click_events: MessageReader<Pointer<Click>>
Query Syntax
// Use granular queries
Query<(&Transform, &Velocity), With<Player>>

// Avoid over-fetching
Query<Entity, (With<Enemy>, Without<Dead>)>
Rust Patterns to Favor
DSL-like Builders
// Good: Chainable, fluent
commands
    .spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ))
    .with_children(|parent| {
        parent.spawn(...);
    })
    .insert(CustomComponent);

// Prefer method chaining
Transform::from_xyz(0.0, 5.0, 0.0)
    .looking_at(Vec3::ZERO, Vec3::Y)
    .with_scale(Vec3::splat(2.0));
Type-State Patterns
// Use marker components for state
#[derive(Component)]
struct Idle;

#[derive(Component)]
struct Moving;

// Systems can query specific states
fn idle_system(query: Query<Entity, With<Idle>>) {}
fn move_system(query: Query<Entity, With<Moving>>) {}
Component Composition Over Inheritance
// Good: Small, composable components
#[derive(Component)]
struct Health(f32);

#[derive(Component)]
struct Damage(f32);

#[derive(Component)]
struct Velocity(Vec3);

// Bad: Large monolithic components
#[derive(Component)]
struct GameObject {
    health: f32,
    damage: f32,
    velocity: Vec3,
    // ... many fields
}
System Organization Patterns
Use System Sets for Organization
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Loading,
    Playing,
    Paused,
}

app.add_systems(
    Update,
    (
        player_movement,
        enemy_ai,
        collision_detection,
    )
        .chain() // Run in order
        .in_set(GameState::Playing),
);
Prefer Run Conditions
app.add_systems(
    Update,
    expensive_system.run_if(resource_exists::<GameData>),
);
Resource Management
Use Local for System State
fn my_system(mut local: Local<MyState>) {
    // Persists between runs, no global pollution
}
Resource vs Component Decision
// Resource: Single global instance
#[derive(Resource)]
struct Score(u32);

// Component: Multiple instances per entity
#[derive(Component)]
struct Health(f32);
Query Best Practices
Use Query Filters
// Get only entities with Health but not Dead
Query<&Health, (With<Player>, Without<Dead>)>

// Changed detection
Query<&Transform, Changed<Transform>>

// Added detection
Query<Entity, Added<NewComponent>>
Avoid Common Pitfalls
// ❌ Bad: Mutable aliasing
fn bad(mut q1: Query<&mut Transform>, mut q2: Query<&mut Transform>) {}

// ✅ Good: Use Without or separate concerns
fn good(
    mut players: Query<&mut Transform, With<Player>>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {}
Event Handling
Commands vs Direct Mutation
// Commands: Deferred, safe for spawning/despawning
fn spawn_system(mut commands: Commands) {
    commands.spawn((Transform::default(), MyComponent));
}

// Direct: Immediate, for component mutations
fn update_system(mut query: Query<&mut Transform>) {
    for mut transform in &mut query {
        transform.translation.y += 1.0;
    }
}
Naming Conventions
Follow Bevy Conventions
// Systems: verb_noun
fn update_velocity() {}
fn spawn_enemies() {}
fn handle_collisions() {}

// Components: Nouns
struct Player;
struct Health(f32);
struct Velocity(Vec3);

// Resources: Nouns
struct GameScore(u32);
struct AssetHandles { ... }

// Events: Past tense or noun
struct CollisionEvent;
struct PlayerDied;
Performance Considerations
Batch Operations
// Good: Single query iteration
fn update_positions(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0;
    }
}

// Avoid: Multiple separate queries for same entities
Use ParallelCommands for Spawning
fn spawn_many(mut commands: Commands) {
    commands.spawn_batch(
        (0..1000).map(|_| (Transform::default(), MyComponent))
    );
}
Error Handling
Prefer Results Over Panics
// Good
fn load_config(asset_server: Res<AssetServer>) -> Result<Handle<ConfigAsset>, LoadError> {
    asset_server.load("config.ron")
        .ok_or(LoadError::NotFound)
}

// Use assertions only for invariants
assert_eq!(width % grid_size, 0, "Width must be divisible by grid_size");
Documentation Style
Document Public APIs
/// Spawns a grid of tiles for the game world.
///
/// # Arguments
/// * `grid_size` - Number of tiles per side (must be power of 2)
/// * `tile_size` - Size of each tile in world units
///
/// # Panics
/// Panics if `grid_size` is 0 or not a power of 2
pub fn spawn_tile_grid(commands: &mut Commands, grid_size: u32, tile_size: f32) {
    // ...
}
Testing Patterns
Use Bevy's Test Infrastructure
#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;

    #[test]
    fn test_system() {
        let mut app = App::new();
        app.add_systems(Update, my_system);
        
        // Spawn test entities
        app.world_mut().spawn(MyComponent);
        
        // Run one frame
        app.update();
        
        // Assert results
        assert_eq!(app.world().entities().len(), 1);
    }
}
Common Gotchas in 0.17.1

Mesh materials must use MeshMaterial3d wrapper
Colors must specify color space (srgb/srgba)
Picking uses MessageReader not EventReader
Camera no longer uses bundles, use component tuples
RemovedComponents uses .read() not .iter()
Transforms in queries are local, not global (use GlobalTransform for world space)

When to Use context7
Use context7 MCP server to verify:

Component/Resource API signatures
Plugin names and configurations
System parameter types
Event types and their fields
Breaking changes between versions
New features in 0.17.1

Response Format
When answering:

✅ State if you verified with context7
📝 Show the specific 0.17.1 API being used
💡 Explain why this pattern/approach is preferred
⚠️ Warn about common pitfalls
🔗 Provide links to official docs when possible


Remember: When in doubt, check context7 first! Bevy changes fast, and outdated advice causes frustration.