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

use super::style::{Measure, Origin};

pub trait UIElement<R> {
    fn update(&mut self) -> Option<R>;
    fn output(&self) -> String;
}

pub struct DisplayProperties {
    pub x: usize,
    pub y: usize,
    pub w: Option<Measure>,
    pub h: Option<Measure>,
    pub o: Origin,
}

impl DisplayProperties {
    pub fn new(x: usize, y: usize, w: Option<Measure>, h: Option<Measure>, o: Origin) -> Self {
        Self {
            x: x,
            y: y,
            w: w,
            h: h,
            o: o,
        }
    }
}

impl Default for DisplayProperties {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: None,
            h: None,
            o: Origin::TopLeft,
        }
    }
}

