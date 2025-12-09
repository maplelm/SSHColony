/*
Copyright 2025 Luke Maple

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
you may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::engine::types::{Position, Position3D};

use super::{Canvas, Object};

pub struct Camera {
    x: i32,
    y: i32,
    z: i32,
    width: u32,
    height: u32,
    depth: u32,
}

impl Camera {
    pub fn new(w: u32, h: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            width: w,
            height: h,
            depth: 1,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn z(&self) -> i32 {
        self.z
    }

    pub fn startingz_x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    pub fn startingz_y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn startingz_z(mut self, z: i32) -> Self {
        self.z = z;
        self
    }

    pub fn starting_width(mut self, w: u32) -> Self {
        self.width = w;
        self
    }

    pub fn starting_height(mut self, h: u32) -> Self {
        self.height = h;
        self
    }

    pub fn starting_depth(mut self, d: u32) -> Self {
        self.depth = d;
        self
    }

    pub fn in_view(&self, o: &Object, canvas: &Canvas) -> bool {
        if o.is_text() {
            let sp: Position3D<i32> = o.pos();
            let size: Position3D<i32> = Position3D {
                x: o.width(canvas) as i32,
                y: o.height(canvas) as i32,
                z: 1,
            };
            let ep: Position3D<i32> = o.pos().join(size);
            let tx_lower = sp.x;
            let tx_upper = sp.x + size.x;
            let ty_lower = sp.y;
            let ty_upper = sp.y + size.y;
            let cx_lower = self.x();
            let cx_upper = self.x() + self.width() as i32;
            let cy_lower = self.y();
            let cy_upper = self.y() + self.height() as i32;
            let tx_range = (sp.x - size.x).abs();
            let ty_range = (sp.y - size.y).abs();
            let cx_range = (self.x - self.width as i32).abs();
            let cy_range = (self.y - self.height as i32).abs();
            let mut x_inbound: bool = false;
            let mut y_inbound: bool = false;

            if tx_range <= cx_range {
                x_inbound = (tx_lower <= cx_upper && tx_lower >= cx_lower)
                    || (tx_upper >= cx_lower && tx_upper <= cx_upper);
            } else {
                x_inbound = (cx_lower <= tx_upper && cx_lower >= tx_lower)
                    || (cx_upper >= tx_lower && cx_upper <= tx_upper);
            }

            if ty_range <= cy_range {
                y_inbound = (ty_lower <= cy_upper && ty_lower >= cy_lower)
                    || (ty_upper >= cy_lower && ty_upper <= cy_upper);
            } else {
                y_inbound = (cy_lower <= ty_upper && cy_lower >= ty_lower)
                    || (cy_upper >= ty_lower && cy_upper <= ty_upper);
            }

            x_inbound && y_inbound && self.z == sp.z
        } else if o.is_sprite() {
            let x_inbound = o.pos().x >= self.x && o.pos().x <= self.x + self.width as i32;
            let y_inbound = o.pos().y >= self.y && o.pos().y <= self.y + self.height as i32;
            x_inbound && y_inbound && self.z == o.pos().z
        } else {
            todo!();
        }
    }

    pub fn resize(&mut self, w: u32, h: u32, d: u32) {
        self.width = w;
        self.height = h;
        self.depth = d;
    }

    pub fn grow(&mut self, w: u32, h: u32, d: u32) {
        self.width += w;
        self.height += h;
        self.depth += d;
    }

    pub fn shrink(&mut self, w: u32, h: u32, d: u32) {
        self.width = if let Some(w) = self.width.checked_sub(w) {
            w
        } else {
            0
        };
        self.height = if let Some(h) = self.height.checked_sub(h) {
            h
        } else {
            0
        };
        self.depth = if let Some(d) = self.depth.checked_sub(d) {
            d
        } else {
            0
        };
    }

    pub fn shift(&mut self, x: i32, y: i32, z: i32) {
        self.x += x;
        self.y += y;
        self.z += z;
    }

    pub fn set_pos(&mut self, x: i32, y: i32, z: i32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    pub fn get_screen_pos(&self, obj_pos: Position3D<i32>) -> Position<i32> {
        Position {
            x: obj_pos.x - self.x,
            y: obj_pos.y - self.y,
        }
    }
}
