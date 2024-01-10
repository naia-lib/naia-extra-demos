#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {

        extern crate log;

        mod app_loop;

        use multi_client_socket_client_app::App;

        use app_loop::start_loop;

        fn main() {
            simple_logger::SimpleLogger::new()
                .with_level(log::LevelFilter::Info)
                .init()
                .expect("A logger was already initialized");

            start_loop(App::new());
        }
    }
}
