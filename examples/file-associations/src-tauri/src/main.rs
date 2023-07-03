// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::env;
use tauri::{Manager, Runtime};

fn create_window<R: Runtime, M: Manager<R>>(app: &M, files: impl AsRef<str>) {
  if app.get_window("main").is_none() {
    tauri::WindowBuilder::new(app, "main", Default::default())
      .initialization_script(&format!("window.openedFile = `{}`", files.as_ref()))
      .build()
      .unwrap();
  }
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(any(windows, target_os = "linux")) {
        // Windows and Linux
        let argv = env::args().collect::<Vec<_>>();
        create_window(
          app,
          if argv.len() > 1 {
            // NOTICE: `argv` may include URL protocol (`your-app-protocol://`) or arguments (`--`) if app supports them.
            format!("{}", argv[1..].join(","))
          } else {
            "".into()
          },
        );
      } else {
        create_window(app, "");
      }
      #[cfg(any(windows, target_os = "linux"))]
      {}

      Ok(())
    })
    .build(tauri::generate_context!())
    .expect("error while running tauri application")
    .run(|app, event| {
      #[cfg(target_os = "macos")]
      if let tauri::RunEvent::Opened {
        event: tauri::OpenEvent::File(paths),
      } = event
      {
        let files = paths
          .iter()
          .map(|f| {
            percent_encoding::percent_decode(f.to_string_lossy().as_bytes())
              .decode_utf8_lossy()
              .into_owned()
          })
          .collect::<Vec<String>>()
          .join(",");
        if let Some(w) = app.get_window("main") {
          let _ = w.eval(&format!("window.onFileOpen(`{}`)", files));
        } else {
          create_window(app, files);
        }
      }
    });
}