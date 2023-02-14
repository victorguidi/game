use glfw::{Action, Context, Key};
use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    // Create new window with settings
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    // Load gl functions
    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        gl::load_with(|symbol| self.window_handle.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.glfw.poll_events();
        self.process_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}

// TODO: 1. Entry Point
fn main() {
    let mut window = Window::new(800, 600, "Hello World");

    window.init_gl();

    while !window.should_close() {
        window.update();
    }
}

// TODO: 2. Application Layer - What keeps the application open and rendering frames
// TODO: 3. Window Layer - What creates the window (OS Dependent so we probably will use something
// like OPENGL or Vulkan) and also handle inputs and events
// TODO: 4. Rendering Layer - What renders the frames
