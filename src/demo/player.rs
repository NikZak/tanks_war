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

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record tank input as movement controls.
    app.add_systems(
        Update,
        record_tank_input
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
        Transform::from_scale(Vec2::splat(0.8).extend(1.0)),
        TankMovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    tank: Handle<Image>,
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
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
