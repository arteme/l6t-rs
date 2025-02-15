use std::sync::OnceLock;
use crate::appwindow::AppWindow;
use crate::prelude::*;

mod appwindow;
mod file;
mod loading;
mod prelude;
mod util;
mod pages;
mod group;
mod page;
mod model;

const APP_ID: &str = "io.github.arteme.l6t-rs.viewer";

pub fn resource(path: &str) -> String {
    static BASE_PATH: OnceLock<String> = OnceLock::new();
    let base_path = BASE_PATH.get_or_init(|| {
        String::from("/") + &APP_ID.replace('.', "/") + "/"
    });

    String::new() + base_path + path
}


fn main() -> Result<()> {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Trace)
        .env()
        .init()?;

    gio::resources_register_include!("compiled.gresource")
        .context("Failed to register compiled gresource")?;

    let app = gtk4::Application::builder()
        .application_id(APP_ID)
        .resource_base_path(resource(""))
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    app.connect_startup(|_| {
        AppWindow::static_type();
    });

    app.connect_activate(|app| {
        let theme = gtk4::IconTheme::default();
        theme.add_resource_path(&resource("icons"));

        let w = AppWindow::new(app);
        w.present();
    });

    app.run();

    Ok(())
}
