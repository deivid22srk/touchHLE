/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

//! YUV to RGB conversion.

// This is a simple and not very efficient implementation.
// It could be optimized with SIMD or by using a shader on the GPU.
pub fn yuv_to_rgb(y_plane: &[u8], u_plane: &[u8], v_plane: &[u8], width: usize, height: usize, rgb: &mut [u8]) {
    let y_stride = width;
    let u_stride = width / 2;
    let v_stride = width / 2;

    for y in 0..height {
        for x in 0..width {
            let y_idx = y * y_stride + x;
            let u_idx = (y / 2) * u_stride + (x / 2);
            let v_idx = (y / 2) * v_stride + (x / 2);

            let y = y_plane[y_idx] as f32;
            let u = u_plane[u_idx] as f32 - 128.0;
            let v = v_plane[v_idx] as f32 - 128.0;

            let r = (y + 1.402 * v).round().clamp(0.0, 255.0) as u8;
            let g = (y - 0.344136 * u - 0.714136 * v).round().clamp(0.0, 255.0) as u8;
            let b = (y + 1.772 * u).round().clamp(0.0, 255.0) as u8;

            let rgb_idx = (y * width + x) * 3;
            rgb[rgb_idx] = r;
            rgb[rgb_idx + 1] = g;
            rgb[rgb_idx + 2] = b;
        }
    }
}
