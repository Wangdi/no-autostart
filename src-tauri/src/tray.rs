use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

pub fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    // Create menu items with Chinese labels
    let show_item = MenuItem::with_id(app, "show", "打开主窗口", true, None::<&str>)?;
    let auto_start_item = MenuItem::with_id(app, "auto_start", "开机自启", true, None::<&str>)?;
    let auto_close_item = MenuItem::with_id(app, "auto_close", "启动时自动关闭", true, None::<&str>)?;
    let execute_item = MenuItem::with_id(app, "execute", "执行自动关闭列表", true, None::<&str>)?;
    let about_item = MenuItem::with_id(app, "about", "关于 NoAutoStart", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // Build the context menu
    let menu = Menu::with_items(app, &[
        &show_item,
        &auto_start_item,
        &auto_close_item,
        &execute_item,
        &about_item,
        &quit_item,
    ])?;

    // Build and spawn the tray icon
    let _tray = TrayIconBuilder::new()
        .icon(Image::from_bytes(include_bytes!("../icons/icon.png"))?)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "auto_start" => {
                // TODO: Toggle auto start setting
                // Will be connected to config store in later integration
            }
            "auto_close" => {
                // TODO: Toggle auto close on start setting
                // Will be connected to config store in later integration
            }
            "execute" => {
                // TODO: Execute auto close list
                // Will trigger process closure based on auto-close list
            }
            "about" => {
                // TODO: Show about dialog
                // Could open a modal or navigate to about page
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
