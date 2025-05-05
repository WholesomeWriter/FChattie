use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
              update(handle).await.unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
        ;
}

async fn update(app: tauri::AppHandle) -> tauri_plugin_updater::Result<()> {
    let has_update = app.updater()?.check().await?;
    match has_update {
        Some(update) => {
            let mut downloaded = 0;
  
            // alternatively we could also call update.download() and update.install() separately
            update
              .download_and_install(
                |chunk_length, content_length| {
                  downloaded += chunk_length;
                  println!("downloaded {downloaded} from {content_length:?}");
                },
                || {
                  println!("download finished");
                },
              )
              .await?;
        
            println!("update installed");
            app.restart();
        }
        None => {
            println!("no update available");
        }
    }
  
    Ok(())
  }