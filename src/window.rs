use std::sync::mpsc::Receiver;

use crate::constants::{OPENGL_MAJOR_VERSION, OPENGL_MINOR_VERSION};
use glfw::{self, Context, CursorMode, Glfw, OpenGlProfileHint, Window, WindowEvent, WindowHint};

pub fn create_window(
    width: u32,
    height: u32,
    title: &str,
) -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    // glfw 초기화
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    // glfw 힌트
    glfw.window_hint(WindowHint::ContextVersion(
        OPENGL_MAJOR_VERSION,
        OPENGL_MINOR_VERSION,
    ));
    glfw.window_hint(WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlDebugContext(true));

    // 수직 동기화(Vsync)
    // Uncomment the following line to disable vsync
    // unsafe { glfwSwapInterval(0) };

    // 윈도우 창 생성
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // 윈도우의 context 설정
    window.make_current();
    // 이벤트 poll 설정
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_raw_mouse_motion(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);

    (glfw, window, events)
}
