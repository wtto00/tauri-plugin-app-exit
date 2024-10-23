use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::AppExit;
#[cfg(mobile)]
use mobile::AppExit;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the app-exit APIs.
pub trait AppExitExt<R: Runtime> {
  fn app_exit(&self) -> &AppExit<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AppExitExt<R> for T {
  fn app_exit(&self) -> &AppExit<R> {
    self.state::<AppExit<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("app-exit")
    .invoke_handler(tauri::generate_handler![commands::exit_app])
    .setup(|app, api| {
      #[cfg(mobile)]
      let app_exit = mobile::init(app, api)?;
      #[cfg(desktop)]
      let app_exit = desktop::init(app, api)?;
      app.manage(app_exit);
      Ok(())
    })
    .build()
}
