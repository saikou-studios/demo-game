use crate::GameState;
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};

mod diagnostics;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), diagnostics::setup_diagnostics)
            .add_systems(
                Update,
                (
                    diagnostics::diagnostics_text_update,
                    diagnostics::diagnostics_show_hide,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}
