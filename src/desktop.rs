use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::ExitAppRequest;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<AppExit<R>> {
    Ok(AppExit(app.clone()))
}

/// Access to the app-exit APIs.
pub struct AppExit<R: Runtime>(AppHandle<R>);

impl<R: Runtime> AppExit<R> {
    pub fn exit_app(&self, app: AppHandle<R>, payload: ExitAppRequest) -> crate::Result<()> {
        app.exit(payload.code.unwrap_or(0));
        Ok(())
    }
}
