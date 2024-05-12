use crate::types::{UVCoords, UVFaces};

#[derive(Copy, Clone)]
pub enum BlockFaces<T> {
    All(T),
    Sides {
        sides: T,
        top: T,
        bottom: T,
    },
    Each {
        top: T,
        bottom: T,
        front: T,
        back: T,
        left: T,
        right: T,
    },
}

// Unpacks a BlockFaces<UVCoords> instance and returns a tuple of UV coordinates for each face ot the block
impl BlockFaces<UVCoords> {
    pub fn get_uv_of_every_faces(&self) -> UVFaces {
        match self {
            &BlockFaces::All(uv) => (uv, uv, uv, uv, uv, uv),
            &BlockFaces::Sides { sides, top, bottom } => (sides, sides, top, bottom, sides, sides),
            &BlockFaces::Each {
                top,
                bottom,
                front,
                back,
                left,
                right,
            } => (front, back, left, right, top, bottom),
        }
    }
}
