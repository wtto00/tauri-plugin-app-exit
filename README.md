# Tauri Plugin app-exit

A plugin for [tauri@v2](https://v2.tauri.app/zh-cn/) to exit app.

## Platform Supported

| Platform | Supported |
| -------- | :-------: |
| Linux    |    ✅     |
| Windows  |    ✅     |
| macOS    |    ✅     |
| Android  |    ✅     |
| iOS      |    ✅     |

This plugin is similar to [tauri-plugin-process](https://v2.tauri.app/plugin/process/), with the addition of support for `Android` and `iOS` platforms.

## Install

```shell
pnpm tauri add taur-plugin-app-exit-api

cargo add tauri-plugin-app-exit
```

## Usage

```js
import { exitApp } from tauri-plugin-app-exit-api";

exitApp().catch(err => {
  console.error(err)
})
```
