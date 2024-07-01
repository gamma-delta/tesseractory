use ultraviolet::Vec4;

use crate::{
  godot_bridge::{vec4_to_gd, GdPlayerCamera},
  math::hexadecitree::BrickRef,
};

use super::{Brick, BrickPtrRepr, Hexadecitree};

impl Hexadecitree {
  pub const GPU_BRICK_PTRS_COUNT: u32 = Self::TOTAL_BRICK_COUNT;
  pub const GPU_COMPOSITE_BRICKS_COUNT: u32 = 256;

  pub const GPU_BRICK_PTRS_BYTES: usize =
    Self::GPU_BRICK_PTRS_COUNT as usize * std::mem::size_of::<BrickPtrRepr>();
  pub const GPU_COMPOSITE_BRICKS_BYTES: usize =
    Self::GPU_COMPOSITE_BRICKS_COUNT as usize * std::mem::size_of::<Brick>();

  pub const GPU_TOTAL_BYTES: usize =
    Self::GPU_BRICK_PTRS_BYTES + Self::GPU_COMPOSITE_BRICKS_BYTES;

  /// Each pixel is 4 bytes; RF encoding means each pixel is 1 4-bit float,
  /// "representing" a monochrome red. This is the side length of the image
  /// allowed, in pixels.
  /// One byte equals one foxel, so one pixel equals 4 foxels kind of
  pub const GPU_TRANSFER_IMAGE_SIZE: usize =
    (Self::GPU_TOTAL_BYTES / 4).isqrt().next_power_of_two();
  pub const GPU_TRANSFER_IMAGE_SIZE_SQ: usize =
    Self::GPU_TRANSFER_IMAGE_SIZE.pow(2);

  pub fn upload(&self, bytes: &mut [u8], cam: &GdPlayerCamera) {
    debug_assert!(
      Hexadecitree::GPU_TOTAL_BYTES
        <= Hexadecitree::GPU_TRANSFER_IMAGE_SIZE_SQ * 4
    );

    let mut gpu_brick_ptrs = Vec::<u16>::new();
    let mut gpu_composite_bricks = Vec::new();

    for (corner, brick_repr) in self.brick_ptrs() {
      let brick_ref = self.brick_repr_to_ref(brick_repr).unwrap();
      match brick_ref {
        BrickRef::Solid(_) => {
          gpu_brick_ptrs.push(brick_repr.0);
        }
        BrickRef::Ref(brick_ref) => {
          if gpu_composite_bricks.len()
            >= Self::GPU_COMPOSITE_BRICKS_COUNT as usize
          {
            continue;
          }

          // Check if the brick is actually in ambit
          let player_to_brick = vec4_to_gd(corner.into()) - cam.pos;
          let player_forward_vec = vec4_to_gd(cam.rot * Vec4::unit_y());
          if player_to_brick.is_zero_approx()
            || player_to_brick.normalized().dot(player_forward_vec)
              >= 1.0 - cam.fov
          {
            gpu_composite_bricks.push(brick_ref.clone());
            gpu_brick_ptrs.push(gpu_composite_bricks.len() as u16 - 1);
          }
        }
      }
    }

    (&mut bytes[..Hexadecitree::GPU_BRICK_PTRS_BYTES])
      .copy_from_slice(bytemuck::cast_slice(gpu_brick_ptrs.as_slice()));
    let composite_bricks_bytes: &[u8] =
      bytemuck::cast_slice(gpu_composite_bricks.as_slice());
    (&mut bytes[Hexadecitree::GPU_BRICK_PTRS_BYTES
      ..Hexadecitree::GPU_BRICK_PTRS_BYTES + composite_bricks_bytes.len()])
      .copy_from_slice(composite_bricks_bytes);

    // bytes.clear();
    // bytes.extend_from_slice(bytemuck::cast_slice(&*self.grid));
    // debug_assert_eq!(bytes.len(), Hexadecitree::BRICKS_BYTES);

    // bytes.extend_from_slice(bytemuck::cast_slice(
    //   self.composite_bricks.as_slice(),
    // ));
    // if bytes.len() > Hexadecitree::MAX_UPLOAD_BYTE_COUNT {
    //   panic!("tried to ship {} bytes to the gpu but that was more than the allowed {}", bytes.len(), Hexadecitree::MAX_UPLOAD_BYTE_COUNT);
    // }
    // resize it to the size of the image, mandatory
    // because the image is floats, ... ughh
    // bytes.resize(Hexadecitree::TRANSFER_IMAGE_SIZE_SQ * 4, 0);
  }
}
