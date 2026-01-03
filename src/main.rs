use iced::Point;
use iced::time;

use iced::window::{self, Position};
use iced::{
    Element, Subscription, Task, application,
    widget::{Button, Column, Text},
};

use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use sysinfo::System;

static SYS: OnceLock<Mutex<System>> = OnceLock::new();

fn system() -> &'static Mutex<System> {
    SYS.get_or_init(|| {
        let mut sys = System::new();
        sys.refresh_all();
        Mutex::new(sys)
    })
}

#[derive(Debug, Clone)]
struct ConkyApp {
    cpu_usage: f32,
    used_memory_gb: f64,
    total_memory_gb: f64,
}

#[derive(Debug, Clone)]
enum Message {
    Tick,
    RefreshNow,
}

fn boot() -> ConkyApp {
    let mut sys = system().lock().unwrap();
    sys.refresh_all();

    ConkyApp {
        cpu_usage: sys.global_cpu_info().cpu_usage(),
        used_memory_gb: sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0),
        total_memory_gb: sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0),
    }
}

fn update(app: &mut ConkyApp, message: Message) -> Task<Message> {
    match message {
        Message::Tick | Message::RefreshNow => {
            let mut sys = system().lock().unwrap();
            sys.refresh_cpu();
            sys.refresh_memory();

            app.cpu_usage = sys.global_cpu_info().cpu_usage();
            app.used_memory_gb = sys.used_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
            app.total_memory_gb = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
        }
    }
    Task::none()
}

fn view(app: &ConkyApp) -> Element<Message> {
    let memory_percent = (app.used_memory_gb / app.total_memory_gb) * 100.0;

    Column::new()
        .padding(20)
        .spacing(10)
        .push(Text::new(format!("CPU: {:.1}%", app.cpu_usage)))
        .push(Text::new(format!(
            "Memory: {:.2} / {:.2} GB ({:.1}%)",
            app.used_memory_gb, app.total_memory_gb, memory_percent
        )))
        .push(Button::new("Refresh Now").on_press(Message::RefreshNow))
        .into()
}

fn subscription(_app: &ConkyApp) -> Subscription<Message> {
    time::every(Duration::from_secs(2)).map(|_| Message::Tick)
}

fn main() -> iced::Result {
    let screen_width: f32 = 1366.0;
    let window_width: f32 = 320.0;
    let right_margin: f32 = 20.0;
    let x: f32 = screen_width - window_width - right_margin;
    let y: f32 = 50.0;

    application(boot, update, view)
        .subscription(subscription)
        .window(window::Settings {
            size: (300, 600).into(),
            resizable: false,
            position: Position::Specific(Point::new(x, y)),
            decorations: false,
            transparent: true,
            ..window::Settings::default()
        })
        .run()
}
