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

mod camera;
mod canvas;
mod drawable;
mod enums;
mod functions;
mod render_thread;
mod render_unit;
mod sprite;
mod text;

pub use camera::Camera;
pub use canvas::Canvas;
pub use drawable::{Character as Char, Glyph, PushChar, PushText, Text};
pub use enums::*;
pub use functions::*;
pub use render_thread::{RenderQueue, render_thread};
pub use render_unit::RenderUnitId;
pub use sprite::Sprite;
pub use text::{Textbox, TextboxSlice};

use render_unit::RenderUnit;
