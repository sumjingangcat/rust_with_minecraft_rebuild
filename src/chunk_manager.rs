use crate::chunk::{Chunk, BlockID};
use std::collections::HashMap;
use crate::shader::ShaderProgram;
use std::borrow::Borrow;

use nalgebra::{clamp, Matrix4, Vector3};
use nalgebra_glm::{pi, Vec2, vec2, vec3, IVec3};

pub struct ChunkManager{
    // 접근을 많이 하므로 Vec 대신 HashMap을 사용한다.
    pub loaded_chunks: HashMap<(i32, i32, i32), Chunk>,
}

impl ChunkManager{
    pub fn new() -> ChunkManager{
        ChunkManager{
            loaded_chunks: HashMap::new(),
        }
    }

    pub fn preload_some_chunks(&mut self){
        for y in 0..=2{
            for z in 0..=2{
                for x in 0..=2{
                    self.loaded_chunks.insert((x, y, z), Chunk::full_of_block(
                        if (x + y + z) % 2 == 0{
                            BlockID::COBBLESTONE
                        }else{
                            BlockID::DIRT
                        }
                    ));
                }
            }
        }
    }

    fn get_chunk_and_block_coords(x: i32, y: i32, z: i32) -> (i32, i32, i32, u32, u32, u32) {
        let chunk_x = if x < 0 {(x + 1) / 16 - 1 } else {x / 16};
        let chunk_y = if y < 0 {(y + 1) / 16 - 1 } else {y / 16};
        let chunk_z = if z < 0 {(z + 1) / 16 - 1 } else {z / 16};

        let block_x = x.rem_euclid(16) as u32;
        let block_y = y.rem_euclid(16) as u32;
        let block_z = z.rem_euclid(16) as u32;

        (chunk_x, chunk_y, chunk_z, block_x, block_y, block_z)
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<BlockID>{
        let (chunk_x, chunk_y, chunk_z, block_x, block_y, block_z) = ChunkManager::get_chunk_and_block_coords(x, y, z);

        self.loaded_chunks
            .get((chunk_x, chunk_y, chunk_z).borrow())
            .and_then(|chunk| Some(chunk.get_block(block_x as usize, block_y as usize, block_z as usize)))
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, block: BlockID){
        let (chunk_x, chunk_y, chunk_z, block_x, block_y, block_z) = ChunkManager::get_chunk_and_block_coords(x, y, z);

        self.loaded_chunks.get_mut((chunk_x, chunk_y, chunk_z).borrow()).map(|chunk|{
            chunk.set_block(block_x as usize, block_y as usize, block_z as usize, block);
        });
    }

    pub fn rebuild_dirty_chunks(&mut self, uv_map: &HashMap<BlockID, ((f32, f32), (f32, f32))>){
        for chunk in self.loaded_chunks.values_mut(){
            if chunk.dirty{
                chunk.regenerate_vbo(uv_map);
            }
        }
    }

    pub fn render_loaded_chunks(&mut self, program: &mut ShaderProgram){
        for ((x, y, z), chunk) in &self.loaded_chunks{
            let model_matrix = {
                let translate_matrix = Matrix4::new_translation(&vec3(
                    *x as f32, *y as f32, *z as f32,
                ).scale(16.0));
                let rotate_matrix = Matrix4::from_euler_angles(
                    0.0, 0.0, 0.0,
                );
                let scale_matrix = Matrix4::new_nonuniform_scaling(&vec3(1.0f32, 1.0f32, 1.0f32));

                translate_matrix * rotate_matrix * scale_matrix
            };

            gl_call!(gl::BindVertexArray(chunk.vao));
            program.set_uniform_matrix4fv("model", model_matrix.as_ptr());
            gl_call!(gl::DrawArrays(gl::TRIANGLES, 0, chunk.vertices_drawn as i32));
        }
    }
}
