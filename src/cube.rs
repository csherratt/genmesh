//   Copyright GFX Developers 2014-2017
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

use std::ops::Range;
use {Normal, Position, Vertex, UV};
use super::{MapVertex, Quad};
use super::generators::{SharedVertex, IndexedPolygon};
use super::texture_coord::UVRect;

/// A perfect cube, centered at (0, 0, 0) with each face starting at 1/-1 away from the origin
#[derive(Clone)]
pub struct Cube {
    range: Range<usize>,
}

impl Cube {
    /// create a new cube generator
    pub fn new() -> Self {
        Cube { range: 0..6 }
    }

    fn vert(&self, idx: usize) -> Position {
        let x = match idx {
            0...5 | 14...17 | 20...21 => 1.,
            _ => -1.,
        };
        let y = match idx {
            2...9 | 17...18 | 20 | 23 => 1.,
            _ => -1.,
        };
        let z = match idx {
            0 | 3...4 | 7...8 | 11...12 | 15...19 => 1.,
            _ => -1., 
        };
        [x, y, z]
    }

    /// fetches the uv coordinate for each vertex based on the following
    /// vertex layout.
    ///
    ///          x-----x
    ///          |16 19|
    ///          |  4  | +z
    ///          |17 18|
    ///  <-x-----x-----x-----x-----x->
    ///  15|0   3|4   7|8  11|12 15|0
    ///    |  0  |  1  |  2  |  3  |
    ///  15|1   2|5   6|9  10|13 14|1
    ///  <-x-----x-----x-----x-----x->
    ///       +x |20 23|
    ///          |  5  |
    ///          |21 22|
    ///          x-----x
    fn uv(&self, idx: usize) -> UV {
        const WIDTH: f32 = 1. / 4.;
        const HEIGHT: f32 = 1. / 3.;

        let rect = UVRect::new(match idx {
                                   0...3 => [HEIGHT, 0.],
                                   4...7 => [HEIGHT, WIDTH],
                                   8...11 => [HEIGHT, WIDTH * 2.],
                                   12...15 => [HEIGHT, WIDTH * 3.],
                                   16...19 => [HEIGHT * 2., WIDTH],
                                   20...23 => [0., WIDTH],
                                   _ => return [0., 0.],
                                   //_ => panic!("idx {} is out of range 0..24", idx)
                               },
                               [WIDTH, HEIGHT]);

        match idx % 4 {
            0 => rect.coord([0., 0.]),
            1 => rect.coord([1., 0.]),
            2 => rect.coord([1., 1.]),
            3 => rect.coord([0., 1.]),
            _ => unreachable!(),
        }
    }

    fn face_indexed(&self, idx: usize) -> (Normal, Quad<usize>) {
        match idx {
            0 => ([1., 0., 0.], Quad::new(0, 1, 2, 3)),
            1 => ([0., 1., 0.], Quad::new(4, 5, 6, 7)),
            2 => ([-1., 0., 0.], Quad::new(8, 9, 10, 11)),
            3 => ([0., -1., 0.], Quad::new(12, 13, 14, 15)),
            4 => ([0., 0., 1.], Quad::new(16, 17, 18, 19)),
            5 => ([0., 0., -1.], Quad::new(20, 21, 22, 23)),
            idx => panic!("{} face is higher then 6", idx),
        }
    }

    fn face(&self, idx: usize) -> Quad<Vertex> {
        let (no, quad) = self.face_indexed(idx);
        quad.map_vertex(|i| {
                            Vertex {
                                pos: self.vert(i),
                                normal: no,
                                uv: self.uv(i),
                            }
                        })
    }
}

impl Iterator for Cube {
    type Item = Quad<Vertex>;

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }

    fn next(&mut self) -> Option<Quad<Vertex>> {
        self.range.next().map(|idx| self.face(idx))
    }
}

impl SharedVertex<Vertex> for Cube {
    fn shared_vertex(&self, idx: usize) -> Vertex {
        let (no, quad) = self.face_indexed(idx / 4);
        let vid = match idx % 4 {
            0 => quad.x,
            1 => quad.y,
            2 => quad.z,
            3 => quad.w,
            _ => unreachable!(),
        };
        Vertex {
            pos: self.vert(vid),
            normal: no,
            uv: self.uv(idx),
        }
    }

    fn shared_vertex_count(&self) -> usize {
        24
    }
}

impl IndexedPolygon<Quad<usize>> for Cube {
    fn indexed_polygon(&self, idx: usize) -> Quad<usize> {
        Quad::new(idx * 4 + 0, idx * 4 + 1, idx * 4 + 2, idx * 4 + 3)
    }

    fn indexed_polygon_count(&self) -> usize {
        6
    }
}
