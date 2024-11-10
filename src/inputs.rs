use raylib::prelude::*;

use crate::Map;

pub fn inputs(rl: &mut RaylibHandle, map: &mut Map, stepping: bool) -> bool {
    let mut stepping = stepping;
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        stepping = !stepping;
    }
    stepping
}