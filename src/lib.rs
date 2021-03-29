use bevy::{
    prelude::{AppBuilder, IntoSystem, Plugin, Res, ResMut},
    window::Windows,
};
use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

type OnResizeSender = Sender<()>;
type OnResizeReceiver = Receiver<()>;

pub struct FullViewportPlugin;

impl Plugin for FullViewportPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let channel = std::sync::mpsc::channel();
        let resize_sender: OnResizeSender = channel.0;
        let resize_receiver: OnResizeReceiver = channel.1;

        app.add_resource(Mutex::new(resize_sender))
            .add_resource(Mutex::new(resize_receiver))
            .add_startup_system(setup_viewport_resize_system.system())
            .add_system(viewport_resize_system.system());
    }
}

fn get_viewport_size() -> (u32, u32) {
    let web_window = web_sys::window().expect("could not get window");

    let width = web_window.inner_width().unwrap().as_f64().unwrap();
    let height = web_window.inner_height().unwrap().as_f64().unwrap();

    (width, height)
}

fn setup_viewport_resize_system(resize_sender: Res<Mutex<OnResizeSender>>) {
    let web_window = web_sys::window().expect("could not get window");
    let local_sender = resize_sender.lock().unwrap().clone();

    local_sender.send(()).unwrap();

    gloo_events::EventListener::new(&web_window, "resize", move |_event| {
        local_sender.send(()).unwrap();
    })
    .forget();
}

fn viewport_resize_system(
    mut windows: ResMut<Windows>,
    resize_receiver: Res<Mutex<OnResizeReceiver>>,
) {
    if resize_receiver.lock().unwrap().try_recv().is_ok() {
        if let Some(window) = windows.get_primary_mut() {
            let size = get_viewport_size();
            window.set_resolution(size.0 / 2 * 2 as f32, size.1 / 2 * 2 as f32);
        }
    }
}
