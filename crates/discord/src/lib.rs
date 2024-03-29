use bevy::{
    app::{App, Plugin, Startup, Update},
    log::error,
    prelude::{DetectChanges, Res, ResMut, Resource},
};
use discord_rich_presence::{activity::Activity, DiscordIpc, DiscordIpcClient};

mod state;

pub use state::ActivityState;

pub struct DiscordConfig {
    pub app_id: u64,
    pub show_time: bool,
}

#[derive(Resource)]
pub struct DiscordClient {
    client: Option<DiscordIpcClient>,
}

impl DiscordClient {
    pub fn new(app_id: u64) -> Self {
        let client = DiscordIpcClient::new(&app_id.to_string()).ok();
        Self { client }
    }

    pub fn connect(&mut self) {
        if let Some(ref mut client) = self.client {
            if client.connect().is_err() {
                self.client = None;
                error!("Failed to connect to Discord IPC");
            }
        }
    }

    pub fn update_activity(&mut self, activity: &ActivityState) {
        if let Some(ref mut client) = self.client {
            if let Err(e) = client.set_activity(Activity::from(activity)) {
                error!("Failed to update Discord activity: {}", e);
            }
        }
    }
}

pub struct DiscordPlugin {
    pub config: DiscordConfig,
}

impl DiscordPlugin {
    pub fn new(app_id: u64, show_time: bool) -> Self {
        Self {
            config: DiscordConfig { app_id, show_time },
        }
    }
}

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource::<DiscordClient>(DiscordClient::new(self.config.app_id))
            .init_resource::<ActivityState>()
            .add_systems(Startup, start_discord_client)
            .add_systems(Update, check_activity_changed);
    }

    fn name(&self) -> &str {
        "Discord RPC"
    }
}

fn start_discord_client(mut client: ResMut<DiscordClient>) {
    client.connect();
    client.update_activity(&ActivityState::default());
}

fn check_activity_changed(activity: Res<ActivityState>, mut client: ResMut<DiscordClient>) {
    if activity.is_changed() {
        client.update_activity(&*activity);
    }
}
