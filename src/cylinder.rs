//   Copyright Dzmitry Malyshau 2017
//
//   Licensed under the Apache License, Version 2.0 (the "License");
//   you may not use this file except in compliance with the License.
//   You may obtain a copy of the License at
//
//       http://www.apache.org/licenses/LICENSE-2.0
//
//   Unless required by applicable law or agreed to in writing, software
//   distributed under the License is distributed on an "AS IS" BASIS,
//   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//   See the License for the specific language governing permissions and
//   limitations under the License.

use std::f32::consts::PI;
use Vertex;
use super::{Quad, Polygon, Triangle, MapVertex};
use super::generators::{SharedVertex, IndexedPolygon};
use texture_coord::{UVRect, UVCircle};

/// this gap is used to avoid texture bleeding between faces
/// of the primitive.
const UV_GAP: f32 = 0.01;
const UV_TOP_CENTER: [f32; 2] = [0.25 - UV_GAP, 0.25 - UV_GAP];
const UV_BOTTOM_CENTER: [f32; 2] = [0.25 - UV_GAP, 0.75 + UV_GAP];
const UV_RADIUS: f32 = 0.25 - UV_GAP;

/// Represents a cylinder with radius of 1, height of 2,
/// and centered at (0, 0, 0) pointing up (to 0, 0, 1).
#[derive(Clone, Copy)]
pub struct Cylinder {
    idx: usize,
    sub_u: usize,
    sub_h: isize,
}

impl Cylinder {
    /// Create a new cylinder.
    /// `u` is the number of points across the radius.
    pub fn new(u: usize) -> Self {
        assert!(u > 1);
        Cylinder {
            idx: 0,
            sub_u: u,
            sub_h: 1,
        }
    }

    /// Create a new subdivided cylinder.
    /// `u` is the number of points across the radius.
    /// `h` is the number of segments across the height.
    pub fn subdivide(u: usize, h: usize) -> Self {
        assert!(u > 1 && h > 0);
        Cylinder {
            idx: 0,
            sub_u: u,
            sub_h: h as isize,
        }
    }

    fn vert(&self, u: usize, h: isize) -> Vertex {
        debug_assert!(u <= self.sub_u);
        let u_per = u as f32 / self.sub_u as f32;
        let h_per = u as f32 / self.sub_u as f32;
        let a = u_per * PI * 2.;
        let n = [a.cos(), a.sin(), 0.];
        let (hc, normal, uv) = if h < 0 {
            // bottom
            debug_assert_eq!(h, -1);
            let uv = UVCircle::new(UV_BOTTOM_CENTER, UV_RADIUS).coord(a);
            (0, [0., 0., -1.], uv)
        } else if h > self.sub_h {
            // top 
            debug_assert_eq!(h, self.sub_h + 1);
            let uv = UVCircle::new(UV_TOP_CENTER, UV_RADIUS).coord(a);
            (self.sub_h, [0., 0., 1.], uv)
        } else {
            // side walls
            let uv = UVRect::new([0., 0.5 + UV_GAP], [1.0, 0.5 - UV_GAP]).coord([u_per, h_per]);
            (h, n, uv)
        };
        let z = (hc as f32 / self.sub_h as f32) * 2. - 1.;
        Vertex {
            pos: [n[0], n[1], z],
            normal,
            uv,
        }
    }
}

impl Iterator for Cylinder {
    type Item = Polygon<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = self.indexed_polygon_count() - self.idx;
        (n, Some(n))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx < self.indexed_polygon_count() {
            let idx = self.idx;
            self.idx += 1;
            Some(self.indexed_polygon(idx)
                     .map_vertex(|i| self.shared_vertex(i)))
        } else {
            None
        }
    }
}

impl SharedVertex<Vertex> for Cylinder {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        if idx == 0 {
            Vertex {
                pos: [0., 0., -1.],
                normal: [0., 0., -1.],
                uv: UV_BOTTOM_CENTER,
            }
        } else if idx == self.shared_vertex_count() - 1 {
            Vertex {
                pos: [0., 0., 1.],
                normal: [0., 0., 1.],
                uv: UV_TOP_CENTER,
            }
        } else {
            // skip the bottom center
            let idx = idx - 1;
            let u = idx % self.sub_u;
            let h = (idx / self.sub_u) as isize - 1;
            self.vert(u, h)
        }
    }

    fn shared_vertex_count(&self) -> usize {
        (3 + self.sub_h) as usize * self.sub_u + 2
    }
}

impl IndexedPolygon<Polygon<usize>> for Cylinder {
    fn indexed_polygon(&self, idx: usize) -> Polygon<usize> {
        let u = idx % self.sub_u;
        let u1 = (idx + 1) % self.sub_u;
        let h = (idx / self.sub_u) as isize - 1;
        let base = 1 + idx - u;
        if h < 0 {
            let start = 0;
            Polygon::PolyTri(Triangle::new(base + u, start, base + u1))
        } else if h == self.sub_h {
            // We need to to select the next vertex loop over, which
            // has the correct normals.
            let base = base + self.sub_u;
            let end = self.shared_vertex_count() - 1;
            Polygon::PolyTri(Triangle::new(base + u, base + u1, end))
        } else {
            Polygon::PolyQuad(Quad::new(base + u,
                                        base + u1,
                                        base + u1 + self.sub_u,
                                        base + u + self.sub_u))
        }
    }

    fn indexed_polygon_count(&self) -> usize {
        (2 + self.sub_h) as usize * self.sub_u
    }
}
