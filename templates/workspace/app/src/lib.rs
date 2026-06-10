#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use mirui::ecs::{Entity, World};
use mirui::prelude::*;

pub fn build_ui(world: &mut World, parent: Entity) {
    ui! {
        :(
            parent: parent
            world: world
        :)

        Column (grow: 1.0) {
            View (
                bg_color: ColorToken::Primary,
                text_color: ColorToken::OnPrimary,
                height: 40,
                text: "{{project-name}}",
                border_radius: 8
            )
            View (bg_color: ColorToken::SurfaceVariant, grow: 1.0)
            View (height: 30, text: "Built with mirui")
        }
    };
}
