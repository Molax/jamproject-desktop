use std::sync::Mutex;

use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use serde::Deserialize;

const DISCORD_CLIENT_ID: &str = "1500279117923225630";

pub struct PresenceState {
    client: Mutex<Option<DiscordIpcClient>>,
}

impl PresenceState {
    pub fn new() -> Self {
        let mut client = DiscordIpcClient::new(DISCORD_CLIENT_ID).ok();
        if let Some(c) = client.as_mut() {
            let _ = c.connect();
        }
        Self {
            client: Mutex::new(client),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PresencePayload {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub start_timestamp: Option<i64>,
    pub end_timestamp: Option<i64>,
}

pub fn set_activity(
    state: &PresenceState,
    p: PresencePayload,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = state.client.lock().map_err(|_| "presence mutex poisoned")?;
    let Some(client) = guard.as_mut() else {
        return Ok(());
    };

    let mut activity = Activity::new();
    if let Some(d) = p.details.as_deref() {
        activity = activity.details(d);
    }
    if let Some(s) = p.state.as_deref() {
        activity = activity.state(s);
    }

    let mut assets = Assets::new();
    if let Some(img) = p.large_image.as_deref() {
        assets = assets.large_image(img);
    }
    if let Some(t) = p.large_text.as_deref() {
        assets = assets.large_text(t);
    }
    if let Some(img) = p.small_image.as_deref() {
        assets = assets.small_image(img);
    }
    if let Some(t) = p.small_text.as_deref() {
        assets = assets.small_text(t);
    }
    activity = activity.assets(assets);

    let mut ts = Timestamps::new();
    if let Some(start) = p.start_timestamp {
        ts = ts.start(start);
    }
    if let Some(end) = p.end_timestamp {
        ts = ts.end(end);
    }
    activity = activity.timestamps(ts);

    client.set_activity(activity)?;
    Ok(())
}

pub fn clear_activity(state: &PresenceState) -> Result<(), Box<dyn std::error::Error>> {
    let mut guard = state.client.lock().map_err(|_| "presence mutex poisoned")?;
    let Some(client) = guard.as_mut() else {
        return Ok(());
    };
    client.clear_activity()?;
    Ok(())
}
