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

pub const CLEAR_COLORS: &str = "\x1b[0m";
pub const CURSOR_HOME: &str = "\x1b[H";

pub const FULL_BLOCK: char = '\u{2588}'; // █
pub const DARK_BLOCK: char = '\u{2593}'; // ▓
pub const MEDIUM_BLOCK: char = '\u{2592}'; // ▓
pub const LIGHT_BLOCK: char = '\u{2591}'; // ░
pub const TOP_BLOCK: char = '\u{2580}'; // ▀
pub const BOTTOM_BLOCK: char = '\u{2584}'; // ▄

pub const HEAVY_H_LINE: char = '\u{2501}'; // ━
pub const HEAVY_V_LINE: char = '\u{2503}'; // ┃
pub const HEAVY_TRC: char = '\u{2513}'; // ┓
pub const HEAVY_TLC: char = '\u{250F}'; // ┏
pub const HEAVY_BRC: char = '\u{251B}'; // ┛
pub const HEAVY_BLC: char = '\u{2517}'; //┗
pub const HEAVY_TOP_T: char = '\u{2533}'; // ┳
pub const HEAVY_BOTTOM_T: char = '\u{253B}'; // ┻
pub const HEAVY_RIGHT_T: char = '\u{252B}'; // ┫
pub const HEAVY_LEFT_T: char = '\u{2523}'; // ┣
pub const HEAVY_CROSS: char = '\u{254B}'; // ╋

pub const DOUBLE_H_LINE: char = '\u{2550}'; // ═
pub const DOUBLE_V_LINE: char = '\u{2551}'; // ║
pub const DOUBLE_TRC: char = '\u{2557}'; // ╗
pub const DOUBLE_TLC: char = '\u{2554}'; // ╔
pub const DOUBLE_BRC: char = '\u{255D}'; // ╝
pub const DOUBLE_BLC: char = '\u{255A}'; // ╚
pub const DOUBLE_TOP_T: char = '\u{2566}'; // ╦
pub const DOUBLE_BOTTOM_T: char = '\u{2569}'; // ╩
pub const DOUBLE_RIGHT_T: char = '\u{2563}'; // ╣
pub const DOUBLE_LEFT_T: char = '\u{2560}'; // ╠
pub const DOUBLE_CROSS: char = '\u{256C}'; // ╬
