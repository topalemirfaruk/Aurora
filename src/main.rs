use aurora::application::Application;

fn main() -> glib::ExitCode {
    tracing_subscriber::fmt::init();
    tracing::info!("Aurora başlatılıyor...");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio çalışma zamanı başlatılamadı");

    let _guard = rt.enter();

    let app = Application::new();
    app.run()
}
