use wry::{
    application::{
        event_loop::{EventLoopProxy, EventLoopWindowTarget},
        window::{Window, WindowBuilder, WindowId},
    },
    webview::{WebView, WebViewBuilder},
};

use crate::shared_events::UserEvents;

pub fn create_settings_window(event_loop: &EventLoopWindowTarget<UserEvents>, proxy: EventLoopProxy<UserEvents>,) -> (WindowId, WebView) {

    let title = format!("Settings");

    let window = WindowBuilder::new()
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let window_id = window.id();
    let handler = move |window: &Window, req: String| match req.as_str() {
        "new-window" => {
            let _ = proxy.send_event(UserEvents::NewWindow());
        }
        "close" => {
            let _ = proxy.send_event(UserEvents::CloseWindow(window.id()));
        }
        _ if req.starts_with("change-title") => {
            let title = req.replace("change-title:", "");
            window.set_title(title.as_str());
        }
        _ => {}
    };

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_html(include_str!("./views/settings.html"),
        )
        .unwrap()
        .with_ipc_handler(handler)
        .build()
        .unwrap();
    (window_id, webview)
}