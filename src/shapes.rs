use crate::types::UVFaces;

pub fn quad(uv: (f32, f32, f32, f32)) -> Vec<f32> {
    (&[
        -0.5f32, -0.5, 0.0, uv.0, uv.3, 0.5, -0.5, 0.0, uv.2, uv.3, 0.5, 0.5, 0.0, uv.2, uv.1, 0.5,
        0.5, 0.0, uv.2, uv.1, -0.5, 0.5, 0.0, uv.0, uv.1, -0.5, -0.5, 0.0, uv.0, uv.3,
    ])
        .to_vec()
}

#[rustfmt::skip]
pub unsafe fn write_unit_cube_to_ptr(
    ptr: *mut f32,
    (x, y, z): (f32, f32, f32),
    (front_uv, back_uv, top_uv, bottom_uv, left_uv, right_uv): UVFaces,
    [right, left, top, bottom, front, back]: [bool; 6],
    ao: [[u8; 4]; 6],
) -> u32 {

    let vertex_size = 9; // 3 for position, 2 for uv, 3 for normal
    let vertex_per_face = 6; // 2 triangles * 3 vertices
    let face_size = vertex_size * vertex_per_face;

    let mut idx = 0;
    let mut copied_vertices = 0;

    if front {
        ptr.offset(idx).copy_from_nonoverlapping([
            0.0 + x, 0.0 + y, 1.0 + z, front_uv.0, front_uv.1, 0.0, 0.0, 1.0, ao[4][0] as f32,
            1.0 + x, 0.0 + y, 1.0 + z, front_uv.2, front_uv.1, 0.0, 0.0, 1.0, ao[4][1] as f32,
            1.0 + x, 1.0 + y, 1.0 + z, front_uv.2, front_uv.3, 0.0, 0.0, 1.0, ao[4][2] as f32,
            1.0 + x, 1.0 + y, 1.0 + z, front_uv.2, front_uv.3, 0.0, 0.0, 1.0, ao[4][2] as f32,
            0.0 + x, 1.0 + y, 1.0 + z, front_uv.0, front_uv.3, 0.0, 0.0, 1.0, ao[4][3] as f32,
            0.0 + x, 0.0 + y, 1.0 + z, front_uv.0, front_uv.1, 0.0, 0.0, 1.0, ao[4][0] as f32,
        ].as_ptr(), face_size);

        idx += face_size as isize;
        copied_vertices += vertex_per_face;
    }

    if back {
        ptr.offset(idx).copy_from_nonoverlapping([
            1.0 + x, 0.0 + y, 0.0 + z, back_uv.0, back_uv.1, 0.0, 0.0, -1.0, ao[5][0] as f32,
            0.0 + x, 0.0 + y, 0.0 + z, back_uv.2, back_uv.1, 0.0, 0.0, -1.0, ao[5][1] as f32,
            0.0 + x, 1.0 + y, 0.0 + z, back_uv.2, back_uv.3, 0.0, 0.0, -1.0, ao[5][2] as f32,
            0.0 + x, 1.0 + y, 0.0 + z, back_uv.2, back_uv.3, 0.0, 0.0, -1.0, ao[5][2] as f32,
            1.0 + x, 1.0 + y, 0.0 + z, back_uv.0, back_uv.3, 0.0, 0.0, -1.0, ao[5][3] as f32,
            1.0 + x, 0.0 + y, 0.0 + z, back_uv.0, back_uv.1, 0.0, 0.0, -1.0, ao[5][0] as f32,
        ].as_ptr(), face_size);

        idx += face_size as isize;
        copied_vertices += vertex_per_face;
    }

    if left {
        ptr.offset(idx).copy_from_nonoverlapping([
            0.0 + x, 0.0 + y, 0.0 + z, left_uv.0, left_uv.1, -1.0, 0.0, 0.0, ao[1][0] as f32,
            0.0 + x, 0.0 + y, 1.0 + z, left_uv.2, left_uv.1, -1.0, 0.0, 0.0, ao[1][1] as f32,
            0.0 + x, 1.0 + y, 1.0 + z, left_uv.2, left_uv.3, -1.0, 0.0, 0.0, ao[1][2] as f32,
            0.0 + x, 1.0 + y, 1.0 + z, left_uv.2, left_uv.3, -1.0, 0.0, 0.0, ao[1][2] as f32,
            0.0 + x, 1.0 + y, 0.0 + z, left_uv.0, left_uv.3, -1.0, 0.0, 0.0, ao[1][3] as f32,
            0.0 + x, 0.0 + y, 0.0 + z, left_uv.0, left_uv.1, -1.0, 0.0, 0.0, ao[1][0] as f32,
        ].as_ptr(), face_size);

        idx += face_size as isize;
        copied_vertices += vertex_per_face;
    }

    if right {
        ptr.offset(idx).copy_from_nonoverlapping([
            1.0 + x, 0.0 + y, 1.0 + z, right_uv.0, right_uv.1, 1.0, 0.0, 0.0, ao[0][0] as f32,
            1.0 + x, 0.0 + y, 0.0 + z, right_uv.2, right_uv.1, 1.0, 0.0, 0.0, ao[0][1] as f32,
            1.0 + x, 1.0 + y, 0.0 + z, right_uv.2, right_uv.3, 1.0, 0.0, 0.0, ao[0][2] as f32,
            1.0 + x, 1.0 + y, 0.0 + z, right_uv.2, right_uv.3, 1.0, 0.0, 0.0, ao[0][2] as f32,
            1.0 + x, 1.0 + y, 1.0 + z, right_uv.0, right_uv.3, 1.0, 0.0, 0.0, ao[0][3] as f32,
            1.0 + x, 0.0 + y, 1.0 + z, right_uv.0, right_uv.1, 1.0, 0.0, 0.0, ao[0][0] as f32,
        ].as_ptr(), face_size);

        idx += face_size as isize;
        copied_vertices += vertex_per_face;
    }

    if top {
        ptr.offset(idx).copy_from_nonoverlapping([
            0.0 + x, 1.0 + y, 1.0 + z, top_uv.0, top_uv.1, 0.0, 1.0, 0.0, ao[2][0] as f32,
            1.0 + x, 1.0 + y, 1.0 + z, top_uv.2, top_uv.1, 0.0, 1.0, 0.0, ao[2][1] as f32,
            1.0 + x, 1.0 + y, 0.0 + z, top_uv.2, top_uv.3, 0.0, 1.0, 0.0, ao[2][2] as f32,
            1.0 + x, 1.0 + y, 0.0 + z, top_uv.2, top_uv.3, 0.0, 1.0, 0.0, ao[2][2] as f32,
            0.0 + x, 1.0 + y, 0.0 + z, top_uv.0, top_uv.3, 0.0, 1.0, 0.0, ao[2][3] as f32,
            0.0 + x, 1.0 + y, 1.0 + z, top_uv.0, top_uv.1, 0.0, 1.0, 0.0, ao[2][0] as f32,
        ].as_ptr(), face_size);

        idx += face_size as isize;
        copied_vertices += vertex_per_face;
    }

    if bottom {
        ptr.offset(idx).copy_from_nonoverlapping([
            0.0 + x, 0.0 + y, 0.0 + z, bottom_uv.0, bottom_uv.1, 0.0, -1.0, 0.0, ao[3][0] as f32,
            1.0 + x, 0.0 + y, 0.0 + z, bottom_uv.2, bottom_uv.1, 0.0, -1.0, 0.0, ao[3][1] as f32,
            1.0 + x, 0.0 + y, 1.0 + z, bottom_uv.2, bottom_uv.3, 0.0, -1.0, 0.0, ao[3][2] as f32,
            1.0 + x, 0.0 + y, 1.0 + z, bottom_uv.2, bottom_uv.3, 0.0, -1.0, 0.0, ao[3][2] as f32,
            0.0 + x, 0.0 + y, 1.0 + z, bottom_uv.0, bottom_uv.3, 0.0, -1.0, 0.0, ao[3][3] as f32,
            0.0 + x, 0.0 + y, 0.0 + z, bottom_uv.0, bottom_uv.1, 0.0, -1.0, 0.0, ao[3][0] as f32,
        ].as_ptr(), face_size);

        copied_vertices += vertex_per_face;
    }

    copied_vertices as u32

}

// Reference : https://stackoverflow.com/questions/25195363/draw-cube-vertices-with-fewest-number-of-steps
pub fn block_outline() -> &'static [f32; 72] {
    // Groups of parallel lines for each dime
    &[
        0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
        1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0,
    ]
}

#[rustfmt::skip]
pub fn centered_unit_cube(x: f32, y: f32, z: f32, (front_uv, back_uv, top_uv, bottom_uv, left_uv, right_uv): UVFaces) -> Vec<f32> {
    // Position: 3 floats
    // UV coords: 2 floats
    // Normals: 3 floats
    [
        0.0 + x, 0.0 + y, 1.0 + z, front_uv.0, front_uv.1, 0.0, 0.0, 1.0,
        1.0 + x, 0.0 + y, 1.0 + z, front_uv.2, front_uv.1, 0.0, 0.0, 1.0,
        1.0 + x, 1.0 + y, 1.0 + z, front_uv.2, front_uv.3, 0.0, 0.0, 1.0,
        1.0 + x, 1.0 + y, 1.0 + z, front_uv.2, front_uv.3, 0.0, 0.0, 1.0,
        0.0 + x, 1.0 + y, 1.0 + z, front_uv.0, front_uv.3, 0.0, 0.0, 1.0,
        0.0 + x, 0.0 + y, 1.0 + z, front_uv.0, front_uv.1, 0.0, 0.0, 1.0,

        1.0 + x, 0.0 + y, 0.0 + z, back_uv.0, back_uv.1, 0.0, 0.0, -1.0,
        0.0 + x, 0.0 + y, 0.0 + z, back_uv.2, back_uv.1, 0.0, 0.0, -1.0,
        0.0 + x, 1.0 + y, 0.0 + z, back_uv.2, back_uv.3, 0.0, 0.0, -1.0,
        0.0 + x, 1.0 + y, 0.0 + z, back_uv.2, back_uv.3, 0.0, 0.0, -1.0,
        1.0 + x, 1.0 + y, 0.0 + z, back_uv.0, back_uv.3, 0.0, 0.0, -1.0,
        1.0 + x, 0.0 + y, 0.0 + z, back_uv.0, back_uv.1, 0.0, 0.0, -1.0,

        0.0 + x, 0.0 + y, 0.0 + z, left_uv.0, left_uv.1, -1.0, 0.0, 0.0,
        0.0 + x, 0.0 + y, 1.0 + z, left_uv.2, left_uv.1, -1.0, 0.0, 0.0,
        0.0 + x, 1.0 + y, 1.0 + z, left_uv.2, left_uv.3, -1.0, 0.0, 0.0,
        0.0 + x, 1.0 + y, 1.0 + z, left_uv.2, left_uv.3, -1.0, 0.0, 0.0,
        0.0 + x, 1.0 + y, 0.0 + z, left_uv.0, left_uv.3, -1.0, 0.0, 0.0,
        0.0 + x, 0.0 + y, 0.0 + z, left_uv.0, left_uv.1, -1.0, 0.0, 0.0,

        1.0 + x, 0.0 + y, 1.0 + z, right_uv.0, right_uv.1, 1.0, 0.0, 0.0,
        1.0 + x, 0.0 + y, 0.0 + z, right_uv.2, right_uv.1, 1.0, 0.0, 0.0,
        1.0 + x, 1.0 + y, 0.0 + z, right_uv.2, right_uv.3, 1.0, 0.0, 0.0,
        1.0 + x, 1.0 + y, 0.0 + z, right_uv.2, right_uv.3, 1.0, 0.0, 0.0,
        1.0 + x, 1.0 + y, 1.0 + z, right_uv.0, right_uv.3, 1.0, 0.0, 0.0,
        1.0 + x, 0.0 + y, 1.0 + z, right_uv.0, right_uv.1, 1.0, 0.0, 0.0,

        0.0 + x, 1.0 + y, 1.0 + z, top_uv.0, top_uv.1, 0.0, 1.0, 0.0,
        1.0 + x, 1.0 + y, 1.0 + z, top_uv.2, top_uv.1, 0.0, 1.0, 0.0,
        1.0 + x, 1.0 + y, 0.0 + z, top_uv.2, top_uv.3, 0.0, 1.0, 0.0,
        1.0 + x, 1.0 + y, 0.0 + z, top_uv.2, top_uv.3, 0.0, 1.0, 0.0,
        0.0 + x, 1.0 + y, 0.0 + z, top_uv.0, top_uv.3, 0.0, 1.0, 0.0,
        0.0 + x, 1.0 + y, 1.0 + z, top_uv.0, top_uv.1, 0.0, 1.0, 0.0,

        0.0 + x, 0.0 + y, 0.0 + z, bottom_uv.0, bottom_uv.1, 0.0, -1.0, 0.0,
        1.0 + x, 0.0 + y, 0.0 + z, bottom_uv.2, bottom_uv.1, 0.0, -1.0, 0.0,
        1.0 + x, 0.0 + y, 1.0 + z, bottom_uv.2, bottom_uv.3, 0.0, -1.0, 0.0,
        1.0 + x, 0.0 + y, 1.0 + z, bottom_uv.2, bottom_uv.3, 0.0, -1.0, 0.0,
        0.0 + x, 0.0 + y, 1.0 + z, bottom_uv.0, bottom_uv.3, 0.0, -1.0, 0.0,
        0.0 + x, 0.0 + y, 0.0 + z, bottom_uv.0, bottom_uv.1, 0.0, -1.0, 0.0,
    ].to_vec()
}
