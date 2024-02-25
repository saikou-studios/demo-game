use bevy::prelude::Resource;
use discord_rich_presence::activity::{Activity, Timestamps};
use std::time::SystemTime;

#[derive(Resource)]
pub struct ActivityState {
    pub state: Option<String>,
    pub details: Option<String>,
    pub timestamps: Option<Timestamps>,
}

impl Default for ActivityState {
    fn default() -> Self {
        Self {
            state: None,
            details: Some("In Main Menu".to_string()),
            timestamps: Some(
                Timestamps::new().start(
                    SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                ),
            ),
        }
    }
}

impl<'a> From<&'a ActivityState> for Activity<'a> {
    fn from(state: &'a ActivityState) -> Self {
        let mut activity = Activity::new();
        if let Some(ref state) = state.state {
            activity = activity.state(state);
        }
        if let Some(ref details) = state.details {
            activity = activity.details(&details);
        }
        if let Some(ref timestamps) = state.timestamps {
            activity = activity.timestamps(timestamps.clone());
        }
        activity
    }
}
