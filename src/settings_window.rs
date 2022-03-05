// use wry::application::dpi::PhysicalSize;
// use wry::{
//     application::{
//         event::{Event, WindowEvent},
//         event_loop::{ControlFlow, EventLoop, EventLoopProxy, EventLoopWindowTarget},
//         window::{Window, WindowBuilder, WindowId},
//     },
//     webview::{WebView, WebViewBuilder},
// };

// pub fn create_settings_window(
//     title: String,
//     event_loop: &EventLoopWindowTarget<UserEvents>,
//     proxy: EventLoopProxy<UserEvents>,
// ) -> (WindowId, WebView) {
//     let window = WindowBuilder::new()
//         .with_title(title)
//         .build(event_loop)
//         .unwrap();
//     let window_id = window.id();
//     let handler = move |window: &Window, req: String| match req.as_str() {
//         "new-window" => {
//             let _ = proxy.send_event(UserEvents::NewWindow());
//         }
//         "close" => {
//             let _ = proxy.send_event(UserEvents::CloseWindow(window.id()));
//         }
//         _ if req.starts_with("change-title") => {
//             let title = req.replace("change-title:", "");
//             window.set_title(title.as_str());
//         }
//         _ => {}
//     };

//     let webview = WebViewBuilder::new(window)
//         .unwrap()
//         .with_html(
//             r#"
//         <button onclick="window.ipc.postMessage('new-window')">Open a new window</button>
//         <button onclick="window.ipc.postMessage('close')">Close current window</button>
//         <input oninput="window.ipc.postMessage(`change-title:${this.value}`)" />
//     "#,
//         )
//         .unwrap()
//         .with_ipc_handler(handler)
//         .build()
//         .unwrap();
//     (window_id, webview)
// }