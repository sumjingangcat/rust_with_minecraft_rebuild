#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod debugging;

pub mod aabb;
pub mod block_texture_sides;
pub mod chunk;
pub mod chunk_manager;
pub mod constants;
pub mod ecs;
pub mod physics;
pub mod raycast;
pub mod renderer;
pub mod shader;
pub mod shapes;
pub mod texture;
pub mod types;
pub mod util;
pub mod window;
pub mod texture_pack;
pub mod input;
pub mod player;

use crate::aabb::get_block_aabb;
use crate::constants::*;
use crate::texture_pack::*;
use crate::window::*;
use crate::input::InputCache;

use crate::debugging::*;
use crate::shader::{ShaderProgram};
use crate::util::Forward;

use crate::chunk::BlockID;
use crate::chunk_manager::ChunkManager;

use glfw::ffi::glfwSwapInterval;
use glfw::{Action, Context, Key, MouseButton, WindowHint};
use nalgebra::{Vector3};
use nalgebra_glm::{pi, vec3, IVec3, Vec2, Vec3};
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_void;

use crate::physics::{PhysicsManager};
use std::time;
use crate::player::{PlayerPhysicsState, PlayerProperties};


fn main() {
    let (mut glfw, mut window, events) = create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_NAME);

    gl_call!(gl::Enable(gl::DEBUG_OUTPUT));
    gl_call!(gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS));
    gl_call!(gl::DebugMessageCallback(
        Some(debug_message_callback),
        0 as *const c_void
    ));
    gl_call!(gl::DebugMessageControl(
        gl::DONT_CARE,
        gl::DONT_CARE,
        gl::DONT_CARE,
        0,
        0 as *const u32,
        gl::TRUE
    ));

    gl_call!(gl::Enable(gl::CULL_FACE));
    // Backface culling
    gl_call!(gl::CullFace(gl::BACK));
    // enable depth test (z-buffer)
    gl_call!(gl::Enable(gl::DEPTH_TEST));
    gl_call!(gl::Enable(gl::BLEND));
    gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA));
    gl_call!(gl::Viewport(
        0,
        0,
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32
    ));
    
    // Generate Texture Atlas
    let (atlas, uv_map) = generate_texture_atlas();
    let mut voxel_shader = ShaderProgram::compile("src/shaders/voxel.vert", "src/shaders/voxel.frag");

    let mut player_properties = PlayerProperties::new();
    let mut physics_manager = PhysicsManager::new(
        1.0 / 60.0,
        PlayerPhysicsState::new_at_position(vec3(0.0f32, 30.0, 0.0)),
    );

    gl_call!(gl::ActiveTexture(gl::TEXTURE0 + 0));
    gl_call!(gl::BindTexture(gl::TEXTURE_2D, atlas));

    let mut chunk_manager = ChunkManager::new();
    chunk_manager.generate_terrain();
    // chunk_manager.preload_some_chunks();

    let mut input_cache = InputCache::default();

    // 메인 루프
    while !window.should_close() {
        // 이벤트를 받고 처리
        glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            input_cache.handle_event(&event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true);
                }

                glfw::WindowEvent::CursorPos(_, _) => {
                    player_properties.rotate_camera(
                        input_cache.cursor_rel_pos.x as f32,
                        input_cache.cursor_rel_pos.y as f32,
                    );
                }

                glfw::WindowEvent::MouseButton(button, Action::Press, _) => {
                    let is_solid_block_at = |x: i32, y: i32, z: i32|
                        chunk_manager.is_solid_block_at(x, y, z);

                    let fw = player_properties.rotation.forward();
                    let player = physics_manager.get_current_state();

                    let block_hit = raycast::raycast(
                        &is_solid_block_at,
                        &player.get_camera_position(),
                        &fw.normalize(),
                        REACH_DISTANCE,
                    );

                    if let Some(((x, y, z), normal)) = block_hit {
                        match button {
                            MouseButton::Button1 => {
                                chunk_manager.set_block(x, y, z, BlockID::Air);
                                println!("Destroyed block at ({x} {y} {z})");
                            }
                            MouseButton::Button2 => {
                                let adjacent_block = IVec3::new(x, y, z) + normal;
                                let adjacent_block_aabb = get_block_aabb(&vec3(
                                    adjacent_block.x as f32,
                                    adjacent_block.y as f32,
                                    adjacent_block.z as f32,
                                ));

                                if !player.aabb.intersects(&adjacent_block_aabb) {
                                    chunk_manager.set_block(adjacent_block.x, adjacent_block.y, adjacent_block.z, BlockID::Debug2);
                                }
                            }
                            _ => {}
                        }
                        
                    }
                }

                _ => {}
            }
        }

        
        let player_physics_state = physics_manager.update_player_physics(&input_cache, &chunk_manager, &player_properties);
        let looking_dir = player_properties.rotation.forward();
        let view_matrix = {
            let camera_position = player_physics_state.get_camera_position();
            nalgebra_glm::look_at(
            &camera_position,
            &(camera_position + looking_dir),
            &Vector3::y(),
            )
        };

        let projection_matrix =
            nalgebra_glm::perspective(1.0, pi::<f32>() / 2.0, NEAR_PLANE, FAR_PLANE);

        chunk_manager.rebuild_dirty_chunks(&uv_map);

        voxel_shader.use_program();

        voxel_shader.set_uniform_matrix4fv("view", view_matrix.as_ptr());
        voxel_shader.set_uniform_matrix4fv("projection", projection_matrix.as_ptr());
        voxel_shader.set_uniform1i("atlas", 0);

        let (r, g, b, a) = BACKGROUND_COLOR;
        gl_call!(gl::ClearColor(r, g, b, a));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT));

        chunk_manager.render_loaded_chunks(&mut voxel_shader);

        // 프론트 버퍼와 백 버퍼 교체 - 프리징 방지
        window.swap_buffers();
    }
}
