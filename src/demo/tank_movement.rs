//! Tank-style movement system with WASD controls.
//! A/D keys control rotation, W/S keys control forward/backward movement.

use bevy::prelude::*;

use crate::{AppSystems, PausableSystems};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<TankMovementController>();
    app.register_type::<ScreenWrap>();

    app.add_systems(
        Update,
        (apply_tank_movement, apply_screen_wrap)
            .chain()
            .in_set(AppSystems::Update)
            .in_set(PausableSystems),
    );
}

/// Tank movement controller that handles rotation and forward/backward movement.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TankMovementController {
    /// The forward/backward movement intent (-1.0 to 1.0).
    /// Positive values move forward, negative values move backward.
    pub forward_intent: f32,

    /// The rotation intent (-1.0 to 1.0).
    /// Positive values rotate clockwise, negative values rotate counter-clockwise.
    pub rotation_intent: f32,

    /// Maximum forward/backward speed in world units per second.
    pub max_speed: f32,

    /// The speed at which the tank rotates in radians per second.
    pub rotation_speed: f32,
}

impl Default for TankMovementController {
    fn default() -> Self {
        Self {
            forward_intent: 0.0,
            rotation_intent: 0.0,
            max_speed: 400.0,
            rotation_speed: f32::to_radians(180.0), // 180 degrees per second
        }
    }
}

/// Screen wrap component to keep entities within screen bounds.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScreenWrap;

fn apply_tank_movement(
    time: Res<Time>,
    mut movement_query: Query<(&TankMovementController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut movement_query {
        // Apply rotation based on rotation intent
        let rotation_delta =
            controller.rotation_intent * controller.rotation_speed * time.delta_secs();
        transform.rotate_z(rotation_delta);

        // Apply forward/backward movement based on current rotation
        if controller.forward_intent != 0.0 {
            // Get the tank's forward direction (X axis in local space since sprite is rotated 90 degrees)
            let forward_direction = transform.rotation * Vec3::X;
            let movement_distance =
                controller.forward_intent * controller.max_speed * time.delta_secs();
            let translation_delta = forward_direction * movement_distance;
            transform.translation += translation_delta;
        }
    }
}

fn apply_screen_wrap(
    window: Query<&Window, With<bevy::window::PrimaryWindow>>,
    mut wrap_query: Query<&mut Transform, With<ScreenWrap>>,
) {
    if let Ok(window) = window.single() {
        let size = window.size() + 256.0;
        let half_size = size / 2.0;
        for mut transform in &mut wrap_query {
            let position = transform.translation.xy();
            let wrapped = (position + half_size).rem_euclid(size) - half_size;
            transform.translation = wrapped.extend(transform.translation.z);
        }
    }
}

/// System to record tank input from keyboard.
/// This should be called from the player module.
pub fn record_tank_input(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut TankMovementController>,
) {
    // Collect forward/backward input (W/S keys)
    let mut forward_intent = 0.0;
    if input.pressed(KeyCode::KeyW) {
        forward_intent += 1.0;
    }
    if input.pressed(KeyCode::KeyS) {
        forward_intent -= 1.0;
    }

    // Collect rotation input (A/D keys)
    let mut rotation_intent = 0.0;
    if input.pressed(KeyCode::KeyA) {
        rotation_intent += 1.0; // Counter-clockwise
    }
    if input.pressed(KeyCode::KeyD) {
        rotation_intent -= 1.0; // Clockwise
    }

    // Apply input to all tank movement controllers
    for mut controller in &mut controller_query {
        controller.forward_intent = forward_intent;
        controller.rotation_intent = rotation_intent;
    }
}
