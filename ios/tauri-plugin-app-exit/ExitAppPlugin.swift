//
//  ExitAppPlugin.swift
//  tauri-plugin-app-exit
//
//  Created by wtto on 2024/10/23.
//

import SwiftRs
import Tauri
import UIKit
import WebKit

class ExitAppArgs: Decodable {
  let code: Int32?
}

class ExitAppPlugin: Plugin {
  @objc public func exitApp(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(ExitAppArgs.self)
    exit(args.code ?? 0)
  }
}

@_cdecl("init_plugin_app_exit")
func initPlugin() -> Plugin {
  return ExitAppPlugin()
}
