#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use mirui::ecs::{Entity, World};
use mirui::prelude::*;
use mirui::ui::widgets::Text;

pub fn build_ui(world: &mut World, parent: Entity) {
    ui! {
        :(
            parent: parent
            world: world
        :)

        Column (grow: 1.0, padding: Padding::all(16)) {
            View (
                bg_color: ColorToken::Primary,
                text_color: ColorToken::OnPrimary,
                height: 40,
                border_radius: 8,
                padding: Padding::all(10)
            ) {
                Text ("{{project-name}}")
            }
            View (bg_color: ColorToken::SurfaceVariant, grow: 1.0)
            View (height: 30, padding: Padding::all(6)) {
                Text ("Built with mirui")
            }
        }
    };
}
