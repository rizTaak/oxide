use super::{
    app::Application,
    window::{Window, WindowProps},
};

pub struct Oxide<A: Application, W: Window<A>> {
    app: A,
    window: W,
}

impl<A: Application, W: Window<A>> Oxide<A, W> {
    pub fn new(props: WindowProps) -> Oxide<A, W> {
        Oxide::<A, W> {
            window: W::new(props),
            app: A::new(),
        }
    }

    pub fn run(&mut self) {
        while !self.window.should_close() {
            // todo: this should be removed
            unsafe {
                gl::ClearColor(1., 0., 1., 1.);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            for layer in self.app.layers().stack.iter_mut() {
                layer.on_update();
            }
            self.window.on_update(&mut self.app);
        }
    }

    pub fn close(&mut self) {
        self.app.close();
    }
}
