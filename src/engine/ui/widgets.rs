use super::Menu;
use super::border::Border;
use super::menu::Item;
use super::style::{Justify, Measure, Origin};
use crate::engine::render::Object;

const MENU_CURSOR_OFFSET: usize = 2;
const MENU_CURSOR_TOTAL_OFFSET: usize = MENU_CURSOR_OFFSET * 2;

pub enum Widget<I, O> {
    Menu(Menu<I, O>),
}

/*
pub enum Widget<I, O> {
    Menu {
        x: usize,
        y: usize,
        width: Option<Measure>,
        height: Option<Measure>
        border: Option<Border>,
        cursor_sprite: Object,
        justify: Justify,
        origin: Origin,
        items: Item<I, O>,
        cursor_pos: usize,
        per_page: u16,
        page: u16,
    },
    Textbox {
        border: Option<Border>,
        width: Option<Measure>,
        height: Option<Measure>,
        text: String,
    },
    Checkbox {
        border: Option<Border>,
        width: Option<Measure>,
        height: Option<Measure>,
        text: String,
        checked: bool
    },
}
*/
