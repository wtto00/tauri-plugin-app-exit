use tauri::{command, AppHandle, Runtime};

use crate::models::ExitAppRequest;
use crate::AppExitExt;
use crate::Result;

#[command]
pub(crate) async fn exit_app<R: Runtime>(app: AppHandle<R>, code: Option<i32>) -> Result<()> {
    app.app_exit()
        .exit_app(app.clone(), ExitAppRequest { code })
}
