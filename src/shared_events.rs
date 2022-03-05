use wry::application::window::WindowId;

pub enum UserEvents {
    CloseWindow(WindowId),
    NewWindow(),
    RefreshView(WindowId),
}
