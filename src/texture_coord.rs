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

use mint::Vector2;

/// This is a UV rectangle
pub struct UVRect {
    offset: Vector2<f32>,
    scale: Vector2<f32>
}

impl UVRect {
    pub fn new(offset: Vector2<f32>, scale: Vector2<f32>) -> Self {
        UVRect{
            offset: offset,
            scale: scale
        }
    }

    pub fn coord(&self, uv: Vector2<f32>) -> Vector2<f32> {
        [self.offset[0] + self.scale[0] * uv[0],
         self.offset[1] + self.scale[1] * uv[1]]
    }
}

/// This is a UVCircle
pub struct UVCircle {
    offset: Vector2<f32>,
    radius: f32
}

impl UVCircle {
    pub fn new(offset: Vector2<f32>, radius: f32) -> Self {
        UVCircle{
            offset: offset,
            radius: radius
        }
    }

    pub fn coord(&self, u: f32) -> Vector2<f32> {
        [self.offset[0] + u.cos() * self.radius,
         self.offset[1] + u.sin() * self.radius]
    }
}