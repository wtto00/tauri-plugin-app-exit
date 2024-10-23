package wang.tato.tauri_plugin_app_exit

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import kotlin.system.exitProcess

@InvokeArg
class ExitAppArgs {
    var code: Int? = 0
}

@TauriPlugin
class ExitAppPlugin(private val activity: Activity): Plugin(activity) {
    @Command
    fun exitApp(invoke: Invoke) {
        try {
            activity.finish()
            android.os.Process.killProcess(android.os.Process.myPid())
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject(e.message)
        }
    }
}
