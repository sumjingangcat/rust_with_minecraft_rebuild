use std::os::raw::c_void;

use glfw::WindowHint;
use image::ColorType;
use nalgebra::Matrix4;
use nalgebra_glm::vec3;

use crate::{shader::ShaderProgram, shapes::{block_outline, quad}, CROSSHAIR_SIZE, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn create_gui_icons_texture() -> u32 {
    let gui_icons_image = match image::open("textures/gui/icons.png") {
        Ok(img) => img,
        Err(err) => panic!("Filename : {}, error: {}", "textures/gui/icons.png", err),
    };

    match gui_icons_image.color() {
        ColorType::Rgba8 => {}
        _ => panic!("Texture fomrat not supported"),
    };

    // Upload the image to the GPU
    let mut id = 0;
    gl_call!(gl::CreateTextures(gl::TEXTURE_2D, 1, &mut id));

    gl_call!(gl::TextureParameteri(
        id,
        gl::TEXTURE_MIN_FILTER,
        gl::NEAREST_MIPMAP_NEAREST as i32
    ));

    gl_call!(gl::TextureParameteri(
        id,
        gl::TEXTURE_MAG_FILTER,
        gl::NEAREST as i32
    ));

    gl_call!(gl::TextureStorage2D(
        id,
        1,
        gl::RGBA8,
        gui_icons_image.width() as i32,
        gui_icons_image.height() as i32
    ));

    gl_call!(gl::TextureSubImage2D(
        id,
        0,
        0,
        0,
        gui_icons_image.width() as i32,
        gui_icons_image.height() as i32,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        gui_icons_image.as_bytes().as_ptr() as *mut c_void
    ));

    id
}

pub fn create_crosshair_vao() -> u32 {
    // Setup VAO
    let mut vao = 0;
    gl_call!(gl::CreateVertexArrays(1, &mut vao));

    // Position
    gl_call!(gl::EnableVertexArrayAttrib(vao, 0));
    gl_call!(gl::VertexArrayAttribFormat(
        vao,       // vao 객체
        0,         // 속성 번호
        3,         // 크기 (float 개수)
        gl::FLOAT, // 데이터 타입
        gl::FALSE, // 정규화 여부
        0          // VBO에서의 시작 위치
    ));
    gl_call!(gl::VertexArrayAttribBinding(vao, 0, 0));

    // Texture Coords
    gl_call!(gl::EnableVertexArrayAttrib(vao, 1));
    gl_call!(gl::VertexArrayAttribFormat(
        vao,
        1,
        3,
        gl::FLOAT,
        gl::FALSE,
        (3 * std::mem::size_of::<f32>()) as u32
    ));
    gl_call!(gl::VertexArrayAttribBinding(vao, 1, 0));

    // VBO
    let mut vbo = 0;
    gl_call!(gl::CreateBuffers(1, &mut vbo));

    gl_call!(gl::VertexArrayVertexBuffer(
        vao,
        0,
        vbo,
        0,
        (5 * std::mem::size_of::<f32>()) as i32
    ));

    gl_call!(gl::NamedBufferData(
        vbo,
        (5 * 6 * std::mem::size_of::<f32>()) as isize,
        quad((0.0, 0.0, 15.0 / 256.0, 15.0 / 256.0)).as_ptr() as *const c_void,
        gl::STATIC_DRAW
    ));

    vao
}

pub fn draw_crosshair(vao: u32, shader: &mut ShaderProgram) {
    let model_matrix = {
        let translate_matrix = Matrix4::new_translation(&vec3(
            WINDOW_WIDTH as f32 / 2.0,
            WINDOW_HEIGHT as f32 / 2.0,
            0.0,
        ));
        let scale_matrix =
            Matrix4::new_nonuniform_scaling(&vec3(CROSSHAIR_SIZE, CROSSHAIR_SIZE, 1.0));

        translate_matrix * scale_matrix
    };

    let projection_matrix = nalgebra_glm::ortho(
        0.0,
        WINDOW_WIDTH as f32,
        0.0,
        WINDOW_HEIGHT as f32,
        -5.0,
        5.0,
    );

    shader.use_program();
    unsafe {
        shader.set_uniform_matrix4fv("model", model_matrix.as_ptr());
        shader.set_uniform_matrix4fv("projection", projection_matrix.as_ptr());
    }
    shader.set_uniform1i("tex", 1);

    gl_call!(gl::BlendFunc(gl::ONE_MINUS_DST_COLOR, gl::ZERO));
    gl_call!(gl::BindVertexArray(vao));
    gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, 6));
}

pub fn create_block_outline_vao() -> u32 {
    // Setup VAO
    let mut vao = 0;
    gl_call!(gl::CreateVertexArrays(1, &mut vao));

    // Position
    gl_call!(gl::EnableVertexArrayAttrib(vao, 0));
    gl_call!(gl::VertexArrayAttribFormat(
        vao,       // vao 객체
        0,         // 속성 번호
        3,         // 크기 (float 개수)
        gl::FLOAT, // 데이터 타입
        gl::FALSE, // 정규화 여부
        0          // VBO에서의 시작 위치
    ));
    gl_call!(gl::VertexArrayAttribBinding(vao, 0, 0));

    // VBO
    let mut vbo = 0;
    gl_call!(gl::CreateBuffers(1, &mut vbo));

    gl_call!(gl::VertexArrayVertexBuffer(
        vao,
        0,
        vbo,
        0,
        (3 * std::mem::size_of::<f32>()) as i32
    ));

    gl_call!(gl::NamedBufferData(
        vbo,
        (72 * std::mem::size_of::<f32>()) as isize,
        block_outline().as_ptr() as *const c_void,
        gl::STATIC_DRAW
    ));

    vao
}