use std::collections::HashMap;

extern crate wry;

use wry::{
    application::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
    },
};

mod config;
use config::{Config, WindowConfig};

mod browser_window;

mod settings_window;

mod shared_events;
use shared_events::UserEvents;

fn main()  -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::load()?;

    let event_loop = EventLoop::<UserEvents>::with_user_event();

    let mut webviews = HashMap::new();

    let proxy = event_loop.create_proxy();

    let id = settings_window::create_settings_window(&event_loop, proxy.clone());
    webviews.insert(id.0, id.1);

    // webview elements so you can refresh it's size
    for window_config in config.windows {
        let window = browser_window::build_window(window_config, &event_loop, proxy.clone());
        webviews.insert(window.0, window.1);
    }

    event_loop.run(move |event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event, window_id, ..
            } => match event {
                WindowEvent::CloseRequested => {
                    webviews.remove(&window_id);
                    if webviews.is_empty() {
                        *control_flow = ControlFlow::Exit
                    }
                }
                WindowEvent::Resized(_) => {
                    let _ = webviews[&window_id].resize();
                }
                _ => (),
            },
            Event::UserEvent(UserEvents::NewWindow()) => {
              let new_window = browser_window::build_window(WindowConfig{
                always_on_top: true,
                height : 400,
                width : 300,
                window_title: format!("Asd"),
                url: format!("http://google.es"),
              },
                &event_loop,
                proxy.clone(),
              );
              webviews.insert(new_window.0, new_window.1);
            }
            Event::UserEvent(UserEvents::CloseWindow(id)) => {
                webviews.remove(&id);
                if webviews.is_empty() {
                    *control_flow = ControlFlow::Exit
                }
            }
            _ => (),
        }
    });
}
