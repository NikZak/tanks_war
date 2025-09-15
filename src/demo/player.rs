//! Player-specific behavior.

use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::{
    AppSystems, PausableSystems,
    asset_tracking::LoadResource,
    demo::tank_movement::{ScreenWrap, TankMovementController, record_tank_input},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<Turret>();
    app.register_type::<TurretController>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record tank input as movement controls.
    app.add_systems(
        Update,
        (
            record_tank_input,
            record_turret_input,
            apply_turret_movement,
        )
            .chain()
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Player"),
        Player,
        Sprite {
            image: player_assets.tank.clone(),
            ..default()
        },
        Transform::from_scale(Vec2::splat(0.8).extend(1.0))
            .with_rotation(Quat::from_rotation_z(f32::to_radians(90.0))),
        TankMovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        children![turret(player_assets)],
    )
}

/// The turret entity that sits on top of the tank.
fn turret(player_assets: &PlayerAssets) -> impl Bundle {
    (
        Name::new("Turret"),
        Turret,
        Sprite {
            image: player_assets.turret.clone(),
            ..default()
        },
        Transform::from_scale(Vec2::splat(0.8).extend(1.0))
            .with_rotation(Quat::from_rotation_z(f32::to_radians(90.0))),
        TurretController::default(),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Turret;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TurretController {
    /// The rotation intent (-1.0 to 1.0).
    /// Positive values rotate clockwise, negative values rotate counter-clockwise.
    pub rotation_intent: f32,

    /// The speed at which the turret rotates in radians per second.
    pub rotation_speed: f32,
}

impl Default for TurretController {
    fn default() -> Self {
        Self {
            rotation_intent: 0.0,
            rotation_speed: f32::to_radians(180.0), // 180 degrees per second
        }
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    tank: Handle<Image>,
    #[dependency]
    turret: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            tank: assets.load_with_settings(
                "images/player_tank-sheet0.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            turret: assets.load_with_settings(
                "images/player_turret-sheet0.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}

/// System to record turret input from keyboard.
pub fn record_turret_input(
    input: Res<ButtonInput<KeyCode>>,
    mut turret_query: Query<&mut TurretController>,
) {
    // Collect turret rotation input (Left/Right arrow keys)
    let mut rotation_intent = 0.0;
    if input.pressed(KeyCode::ArrowLeft) {
        rotation_intent += 1.0; // Counter-clockwise
    }
    if input.pressed(KeyCode::ArrowRight) {
        rotation_intent -= 1.0; // Clockwise
    }

    // Apply input to all turret controllers
    for mut controller in &mut turret_query {
        controller.rotation_intent = rotation_intent;
    }
}

/// System to apply turret rotation based on controller input.
fn apply_turret_movement(
    time: Res<Time>,
    mut turret_query: Query<(&TurretController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut turret_query {
        // Apply rotation based on rotation intent
        let rotation_delta =
            controller.rotation_intent * controller.rotation_speed * time.delta_secs();
        transform.rotate_z(rotation_delta);
    }
}
