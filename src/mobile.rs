use crate::models::ExitAppRequest;
use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_app_exit);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AppExit<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("wang.tato.tauri_plugin_app_exit", "ExitAppPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_app_exit)?;
    Ok(AppExit(handle))
}

/// Access to the app-exit APIs.
pub struct AppExit<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AppExit<R> {
    pub fn exit_app(&self, _app: AppHandle<R>, payload: ExitAppRequest) -> crate::Result<()> {
        self.0
            .run_mobile_plugin("exitApp", payload)
            .map_err(Into::into)
    }
}
