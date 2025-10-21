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
#[deny(unused)]

#[derive(Clone, Hash, Copy, Debug, serde::Serialize, serde::Deserialize)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w,
            height: h,
        }
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }
}

