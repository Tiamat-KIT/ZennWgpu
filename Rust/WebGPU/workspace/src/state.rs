pub struct RenderState<'a> {
    pub window: &'a winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>
}

impl<'a> RenderState<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();
        Self {
            window,
            size
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            // self.config.width = new_size.width;
            // self.config.height = new_size.height;
            // self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, event: &winit::event::WindowEvent) -> bool {
        false// todo!()
    }

    pub fn update(&mut self) {
        // todo!()
    }
}