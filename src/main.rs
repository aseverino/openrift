// SPDX-License-Identifier: MIT
//
// Copyright (c) 2025 Alexandre Severino
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

mod input;
mod ui;
mod game;
mod lua_interface;
mod items;
mod maps;
mod position;
mod tile;
mod tile_map;
mod creature;
mod monster_type;
mod monster;
mod spell_type;
mod player_spell;
mod spell_execution;
mod player;
mod combat;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Rogue".to_string(),
        window_width: 1920,
        window_height: 1100,
        fullscreen: false,
        ..Default::default()
    }
}

//use crate::ui::{Ui};

#[macroquad::main(window_conf)]
async fn main() {
    game::run().await;
}