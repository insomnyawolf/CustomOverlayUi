use wry::application::dpi::PhysicalSize;
use wry::{
    application::{
        event_loop::{EventLoopProxy, EventLoopWindowTarget},
        window::{WindowBuilder, WindowId},
    },
    webview::{WebView, WebViewBuilder},
};

use crate::shared_events::{UserEvents};
use crate::config::{WindowConfig};

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

    let size: PhysicalSize<u32> = PhysicalSize::new(window_config.width, window_config.height);
    window.set_inner_size(size);

    let webview = WebViewBuilder::new(window).unwrap().with_url(&window_config.url).unwrap().build().unwrap();

    (window_id, webview)
}