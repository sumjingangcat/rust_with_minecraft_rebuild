use crate::chunk_manager::{CHUNK_SIZE, CHUNK_VOLUME};
use crate::debugging;
use crate::shapes::write_unit_cube_to_ptr;
use gl::FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::collections::{HashMap, HashSet};
use std::ffi::c_void;

use crate::gl_call;

fn create_vao_vbo() -> (u32, u32) {
    let mut vao = 0;
    gl_call!(gl::CreateVertexArrays(1, &mut vao));

    // pos
    gl_call!(gl::EnableVertexArrayAttrib(vao, 0));
    gl_call!(gl::VertexArrayAttribFormat(
        vao,
        0,
        3_i32,
        gl::FLOAT,
        gl::FALSE,
        0
    ));
    gl_call!(gl::VertexArrayAttribBinding(vao, 0, 0));

    // texture
    gl_call!(gl::EnableVertexArrayAttrib(vao, 1));
    gl_call!(gl::VertexArrayAttribFormat(
        vao,
        1,
        2_i32,
        gl::FLOAT,
        gl::FALSE,
        (3 * std::mem::size_of::<f32>()) as u32
    ));
    gl_call!(gl::VertexArrayAttribBinding(vao, 1, 0));

    let mut vbo = 0;
    gl_call!(gl::CreateBuffers(1, &mut vbo));
    gl_call!(gl::NamedBufferData(
        vbo,
        (180 * CHUNK_VOLUME as usize * std::mem::size_of::<f32>()) as isize,
        std::ptr::null(),
        gl::DYNAMIC_DRAW
    ));

    gl_call!(gl::VertexArrayVertexBuffer(
        vao,
        0,
        vbo,
        0,
        (5 * std::mem::size_of::<f32>()) as i32
    ));

    (vao, vbo)
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BlockID {
    Air,
    Dirt,
    Cobblestone,
    Obsidian,
    Grass,
}

impl BlockID {
    pub fn is_transparent(&self) -> bool {
        match self {
            BlockID::Air => true,
            _ => false,
        }
    }
}

impl Distribution<BlockID> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockID {
        match rng.gen_range(1..4) {
            // 0 => BlockID::AIR,
            1 => BlockID::Dirt,
            2 => BlockID::Cobblestone,
            3 => BlockID::Obsidian,
            _ => BlockID::Air,
        }
    }
}

pub struct Chunk {
    blocks: [BlockID; CHUNK_VOLUME as usize],
    pub vao: u32,
    pub vbo: u32,
    pub vertices_drawn: u32,
    pub dirty: bool,
    pub dirty_neighbours: HashSet<(i32, i32, i32)>,
}

impl Chunk {
    fn all_neighbours() -> HashSet<(i32, i32, i32)> {
        let mut hash_set = HashSet::new();

        hash_set.insert((1, 0, 0));
        hash_set.insert((-1, 0, 0));
        hash_set.insert((0, 1, 0));
        hash_set.insert((0, -1, 0));
        hash_set.insert((0, 0, 1));
        hash_set.insert((0, 0, -1));

        hash_set
    }

    pub fn empty() -> Chunk {
        let (vao, vbo) = create_vao_vbo();

        Chunk {
            blocks: [BlockID::Air; CHUNK_VOLUME as usize],
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: false,
            dirty_neighbours: Chunk::all_neighbours(),
        }
    }

    pub fn full_of_block(block: BlockID) -> Chunk {
        let (vao, vbo) = create_vao_vbo();

        Chunk {
            blocks: [block; CHUNK_VOLUME as usize],
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: false,
            dirty_neighbours: Chunk::all_neighbours(),
        }
    }

    pub fn random() -> Chunk {
        let (vao, vbo) = create_vao_vbo();

        let mut blocks = [BlockID::Air; CHUNK_VOLUME as usize];
        for i in 0..CHUNK_VOLUME as usize {
            blocks[i] = rand::random();
        }

        Chunk {
            blocks,
            vao,
            vbo,
            vertices_drawn: 0,
            dirty: true,
            dirty_neighbours: Chunk::all_neighbours(),
        }
    }

    #[inline]
    fn coords_to_index(x: usize, y: usize, z: usize) -> usize {
        y * (CHUNK_SIZE as usize * CHUNK_SIZE as usize) + z * CHUNK_SIZE as usize + x
    }

    #[inline]
    pub fn get_block(&self, x: usize, y: usize, z: usize) -> BlockID {
        self.blocks[Self::coords_to_index(x, y, z)]
    }

    #[inline]
    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockID) {
        self.blocks[Self::coords_to_index(x, y, z)] = block;
        self.dirty = true;

        if x == 0 {
            self.dirty_neighbours.insert((-1, 0, 0));
        } else if x == 15 {
            self.dirty_neighbours.insert((1, 0, 0));
        }

        if y == 0 {
            self.dirty_neighbours.insert((0, -1, 0));
        } else if y == 15 {
            self.dirty_neighbours.insert((0, 1, 0));
        }

        if z == 0 {
            self.dirty_neighbours.insert((0, 0, -1));
        } else if z == 15 {
            self.dirty_neighbours.insert((0, 0, 1));
        }
    }
}
