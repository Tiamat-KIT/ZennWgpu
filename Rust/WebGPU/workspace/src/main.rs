mod state;

use state::RenderState;
use winit::{
    event::{self, *},
    event_loop::{self, EventLoop},
    keyboard::{KeyCode,PhysicalKey},
    window::WindowBuilder
};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() {
    pollster::block_on(run())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    
    
    let event_loop = EventLoop::new()
        .expect("EventLoop Create Error");

    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Window Build Error");

    #[cfg(target_arch = "wasm32")] {
        use winit::dpi::PhysicalSize;

        window.request_inner_size(PhysicalSize::new(450,400));
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("canvas")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("body要素にcanvas要素を追加することに失敗しました")
    }
    let mut render_state = RenderState::new(&window).await;

    #[warn(unused_must_use)]
    event_loop.run(
        move |
        event,
        control_flow
        | {
            match event {
                Event::WindowEvent { 
                    window_id, 
                    ref event 
                } if window_id == render_state.window.id() => {
                    if !render_state.input(event) {
                        match event {
                            WindowEvent::CloseRequested | WindowEvent::KeyboardInput { 
                                event: KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                                ..
                            } => control_flow.exit(),
                            WindowEvent::Resized(physical_size) => {
                                render_state.resize(*physical_size);
                            }
                            WindowEvent::RedrawRequested => {
                                render_state.window.request_redraw();
                                render_state.update();
                                /* match render_state.render() {
                                    Ok(_) => {}
                                    Err(
                                        wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated
                                    ) => render_state.resize(render_state.size),
                                    Err(wgpu::SurfaceError::OutOfMemory) => {
                                        control_flow.exit();
                                    }
                                    Err(wgpu::SurfaceError::Timeout) => {

                                    }
                                } */
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        });
}