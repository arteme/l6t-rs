use anyhow::*;
use gtk4::gio;
use gtk4::prelude::ApplicationExtManual;
use crate::app::App;

mod widgets;
mod appwindow;
mod app;
mod file;
mod loading;
mod prelude;

#[tokio::main]
async fn main() -> Result<()> {

    gio::resources_register_include!("compiled.gresource")
        .context("Failed to register compiled gresource")?;

    println!("Starting...");
    let app = App::new();
    app.run();

    Ok(())
}
