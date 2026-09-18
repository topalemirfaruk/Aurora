#![allow(dead_code)]

mod application;
mod config;
mod models;
mod package_managers;
mod process;
mod security;
mod services;
mod state;
mod ui;
mod utils;

use application::Application;

fn main() -> glib::ExitCode {
    // Tracing / Loglama başlat
    tracing_subscriber::fmt::init();

    tracing::info!("Aurora Linux Uygulama Merkezi başlatılıyor...");

    // Tokio runtime'ı başlat ve reactor bağlamını GTK ana iş parçacığına bağla
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio çalışma zamanı başlatılamadı");

    let _guard = rt.enter();

    let app = Application::new();
    app.run()
}
