use miniquad::*;

use multi_client_socket_client_app::App;

struct Stage {
    app: App,
}
impl EventHandler for Stage {
    fn update(&mut self, _ctx: &mut Context) {
        self.app.update();
    }

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0., 1., 0., 1.)), None, None);
    }
}

fn main() {
    let app = App::new();
    miniquad::start(conf::Conf::default(), |_ctx| Box::new(Stage { app }));
}
