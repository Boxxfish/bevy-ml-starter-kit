//! Defines various configurations our game can be in.

use bevy::prelude::*;

/// Handles core functionality for our game (e.g. gameplay logic).
pub struct CoreGamePlugin;

impl Plugin for CoreGamePlugin {
    fn build(&self, _app: &mut App) {}
}

/// Adds functionality required to make the game playable.
pub struct PlayablePlugin;

impl Plugin for PlayablePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Your Project (Game)".into(),
                ..default()
            }),
            ..default()
        }));
    }
}

/// The configuration for published builds.
pub struct ReleaseCfgPlugin;

impl Plugin for ReleaseCfgPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CoreGamePlugin, PlayablePlugin));
    }
}

/// The configuration for library builds (e.g. for machine learning).
pub struct LibCfgPlugin;

impl Plugin for LibCfgPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreGamePlugin);
    }
}
