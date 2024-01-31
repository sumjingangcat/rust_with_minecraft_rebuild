use std::collections::HashMap;
use std::ffi::c_void;

use crate::debugging;
use crate::shapes::unit_cube_array;
use gl::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE;
use rand::prelude::Distribution;
use rand::distributions::Standard;

use crate::gl_call;

const CHUNK_SIZE: u32 = 16;

const CHUNK_VOLUME: u32 = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

fn create_vao_vbo() -> (u32, u32){
    let mut vao = 0;
    gl_call!(gl::CreateVertexArrays(1, &mut vao));

    // pos
    gl_call!(gl::EnableVertexArrayAttrib(vao, 0));
    gl_call!(gl::VertexArrayAttribFormat(vao, 0, 3_i32, gl::FLOAT, gl::FALSE, 0));
    gl_call!(gl::VertexArrayAttribBinding(vao, 0, 0));

    // texture
    gl_call!(gl::EnableVertexArrayAttrib(vao, 1));
    gl_call!(gl::VertexArrayAttribFormat(vao, 1, 2_i32, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as u32));
    gl_call!(gl::VertexArrayAttribBinding(vao, 1, 0));

    let mut vbo = 0;
    gl_call!(gl::CreateBuffers(1, &mut vbo));
    gl_call!(gl::NamedBufferData(vbo, (180 * CHUNK_VOLUME as usize * std::mem::size_of::<f32>()) as isize, std::ptr::null(), gl::DYNAMIC_DRAW));

    gl_call!(gl::VertexArrayVertexBuffer(vao, 0, vbo, 0, (5 * std::mem::size_of::<f32>()) as i32));

    (vao, vbo)
}


#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlockID{
    DIRT,
    AIR,
    COBBLESTONE,
    OBSIDIAN,
}

impl Distribution<BlockID> for Standard{
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockID{
        match rng.gen_range(0..4){
            0 => BlockID::DIRT,
            1 => BlockID::AIR,
            2 => BlockID::COBBLESTONE,
            3 => BlockID::OBSIDIAN,
            _ => BlockID::AIR,
        }
    }

}

pub struct Chunk {
    blocks: [BlockID; CHUNK_VOLUME as usize],
    pub vao: u32,
    vbo: u32,
    pub vertices_drawn: u32,
    pub dirty: bool,
}

impl Chunk{
    pub fn empty() -> Chunk{
        let (vao, vbo) = create_vao_vbo();

        Chunk{
            blocks: [BlockID::AIR; CHUNK_VOLUME as usize],
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: false,
        }
    }
    
    pub fn full_of_block(block: BlockID) -> Chunk{
        let (vao, vbo) = create_vao_vbo();

        Chunk{
            blocks: [block; CHUNK_VOLUME as usize],
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: false,
        }
    }

    pub fn random() -> Chunk{
        let (vao, vbo) = create_vao_vbo();

        let mut blocks = [BlockID::AIR; CHUNK_VOLUME as usize];
        for i in 0..CHUNK_VOLUME as usize{
            blocks[i] = rand::random();
        }

        Chunk{
            blocks,
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: true,
        }
    }

    #[inline]
    fn coords_to_index(x: usize, y: usize, z: usize) -> usize{
        y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + z * CHUNK_SIZE as usize + x
    }

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockID {
        self.blocks[Self::coords_to_index(x, y, z)]
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockID){
        self.blocks[Self::coords_to_index(x, y, z)] = block;
        self.dirty = true;
    }

    pub fn regenerate_vbo(&mut self, uv_map: &HashMap<BlockID, ((f32, f32), (f32, f32))>) {
        let mut idx = 0;
        self.vertices_drawn = 0;

        for y in 0..CHUNK_SIZE as usize{
            for z in 0..CHUNK_SIZE as usize{
                for x in 0..CHUNK_SIZE as usize{
                    let block = self.get_block(x,y,z);

                    if block == BlockID::AIR{
                        continue;
                    }

                    let (uv_bl, ub_tr) = uv_map.get(&block).unwrap().clone();
                    let cube_array = unit_cube_array(x as f32, y as f32, z as f32, uv_bl, ub_tr, true, true, true, true, true, true);

                    gl_call!(gl::NamedBufferSubData(
                        self.vbo,
                        (idx * std::mem::size_of::<f32>()) as isize,
                        (cube_array.len() * std::mem::size_of::<f32>()) as isize,
                        cube_array.as_ptr() as *mut c_void,
                    ));

                    self.vertices_drawn += cube_array.len() as u32 / 5;
                    idx += cube_array.len()
                }
            }
        }

        self.dirty = false;
    }
}