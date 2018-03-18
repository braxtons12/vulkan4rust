use glfw;
use glfw::{Glfw,Window,WindowMode,WindowHint,ClientApiHint};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn app_main() {
    let mut app = HelloTriangleApplication::new();

    app.run();
}

struct HelloTriangleApplication {
    glfw: Glfw,
    window: Window,
}

impl HelloTriangleApplication {
    fn run(&mut self) {
        self.main_loop();
        self.cleanup();
    }

    fn main_loop(&mut self) {
        while !self.window.should_close() {
            self.glfw.poll_events();
        }
    }

    fn cleanup(&mut self) {
        // destroy window and terminate is not needed
        // because it is hadled by the library
    }

    fn new() -> HelloTriangleApplication {
        // initWindow stuff is here
        let (glfw, window) = init_window(WIDTH, HEIGHT);

        HelloTriangleApplication {
            glfw: glfw,
            window: window,
        }
    }
}

pub fn init_window(width: u32, height: u32) -> (Glfw,Window) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (window, _events) = glfw.create_window(width, height, "Vulkan", WindowMode::Windowed).unwrap();
    (glfw,window)
}


