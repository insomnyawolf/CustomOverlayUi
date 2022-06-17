use wry::application::window::Fullscreen;
use wry::{
    application::{
        event_loop::{EventLoopProxy, EventLoopWindowTarget},
        window::{WindowBuilder, WindowId},
    },
    webview::{WebView, WebViewBuilder},
};

use crate::config::WindowConfig;
use crate::shared_events::UserEvents;

pub fn build_window(
    window_config: WindowConfig,
    event_loop: &EventLoopWindowTarget<UserEvents>,
    proxy: EventLoopProxy<UserEvents>,
) -> (WindowId, WebView) {
    let window = WindowBuilder::new()
        .with_title(window_config.window_title)
        .build(event_loop)
        .unwrap();

    let window_id = window.id();

    window.set_always_on_top(window_config.always_on_top);

    let mut window_size = window_config.size;

    if window_config.fullscreen {
        window.set_fullscreen(Some(Fullscreen::Borderless(None)));
        // Adjust to fullscreen resolution
        match window.current_monitor() {
            Some(monitor_info) =>{
                window_size = monitor_info.size();
            },
            None =>{
                // So nothing
            }
        };     
    }

    window.set_inner_size(window_size);

    window.set_outer_position(window_config.position);

    window.set_resizable(window_config.resizable);

    let webview = WebViewBuilder::new(window)
        .unwrap()
        .with_url(&window_config.url)
        .unwrap()
        .build()
        .unwrap();

    (window_id, webview)
}
