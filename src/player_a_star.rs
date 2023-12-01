use bevy::prelude::*;
use pyo3::prelude::*;
use pyo3::types::PyTuple;
use simple_matrix::Matrix;

pub struct AStarPlayerPlugin;

impl Plugin for AStarPlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_a_star_player);
    }
}

pub fn spawn_a_star_player(
    mut commands: Commands,
) {

}