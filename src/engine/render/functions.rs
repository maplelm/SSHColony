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

#![deny(unused)]

use super::super::enums::RenderSignal;
use std::sync::mpsc;

pub fn clear(tx: &mpsc::Sender<RenderSignal>) -> Result<(), mpsc::SendError<RenderSignal>> {
    tx.send(RenderSignal::Clear)
}
