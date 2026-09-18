#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use mirui::ecs::{Entity, World};
use mirui::prelude::*;
use mirui::ui::widgets::{Button, ParagraphStyle, Slider, Text, TextAlign, TextOverflow};

fn single_line(align: TextAlign) -> ParagraphStyle {
    ParagraphStyle {
        align,
        overflow: TextOverflow::Ellipsis,
        max_lines: Some(1),
        ..ParagraphStyle::label()
    }
}

pub fn build_ui(world: &mut World, parent: Entity) {
    let flowing = Signal::new(true);
    let energized = Signal::new(true);
    let (hero_mode, hero_energy, switch_mode, slider_energy) = (
        flowing.clone(),
        energized.clone(),
        flowing.clone(),
        energized.clone(),
    );

    ui! {
        :(
            parent: parent
            world: world
        :)

        Column (
            id: "template_shell",
            container: true,
            grow: 1.0,
            align: AlignItems::Center,
            justify: JustifyContent::Center,
            padding: @height {
                if height < Fixed::from_int(120) { Padding::all(4) } else { Padding::all(10) }
            },
            bg_color: ColorToken::Surface
        ) {
            Column (
                id: "template_card",
                width: Dimension::percent(100),
                max_width: 520,
                grow: 1.0,
                max_height: 260,
                min_height: 56,
                padding: @height {
                    if height < Fixed::from_int(120) { Padding::all(4) } else { Padding::all(8) }
                },
                row_gap: @height { if height < Fixed::from_int(120) { 2 } else { 5 } },
                bg_color: ColorToken::SurfaceVariant,
                border_color: ColorToken::Outline,
                border_width: 1,
                border_radius: 12
            ) {
                Row (
                    height: @height { if height < Fixed::from_int(120) { 12 } else { 18 } },
                    align: AlignItems::Center,
                    column_gap: @height { if height < Fixed::from_int(120) { 4 } else { 7 } }
                ) {
                    View (
                        width: @height { if height < Fixed::from_int(120) { 6 } else { 8 } },
                        height: @height { if height < Fixed::from_int(120) { 6 } else { 8 } },
                        bg_color: ColorToken::Success,
                        border_radius: 4
                    )
                    Text (
                        "{{project-name}}",
                        grow: 1.0,
                        height: @height { if height < Fixed::from_int(120) { 12 } else { 18 } },
                        font_size: @height {
                            if height < Fixed::from_int(120) { 7_u16 } else { 12_u16 }
                        },
                        text_color: ColorToken::OnSurface,
                        paragraph: single_line(TextAlign::Start)
                    )
                    Text (
                        text: ${ if hero_energy.get() { "READY" } else { "IDLE" } },
                        id: "template_status",
                        width: @height { if height < Fixed::from_int(120) { 30 } else { 44 } },
                        height: @height { if height < Fixed::from_int(120) { 12 } else { 18 } },
                        font_size: @height {
                            if height < Fixed::from_int(120) { 6_u16 } else { 8_u16 }
                        },
                        text_color: ColorToken::Primary,
                        paragraph: single_line(TextAlign::Center)
                    )
                }
                Text (
                    text: ${ if hero_mode.get() { "FLOW" } else { "FOCUS" } },
                    id: "template_mode",
                    grow: 1.0,
                    width: Dimension::percent(100),
                    font_size: @height {
                        if height < Fixed::from_int(120) { 12_u16 } else { 24_u16 }
                    },
                    text_color: ColorToken::OnSurface,
                    paragraph: ParagraphStyle::label()
                )
                Slider (
                    id: "template_energy",
                    width: Dimension::percent(100),
                    height: @height { if height < Fixed::from_int(120) { 8 } else { 12 } },
                    min: Fixed::ZERO,
                    max: Fixed::from_int(100),
                    value: Fixed::from_int(68),
                    track_color: ColorToken::Surface,
                    fill_color: ColorToken::Primary,
                    thumb_color: ColorToken::OnPrimary
                ) on ValueChanged {
                    let _ = old;
                    slider_energy.set(*new >= Fixed::from_int(35));
                }
                Row (
                    height: @height { if height < Fixed::from_int(120) { 18 } else { 24 } },
                    column_gap: @height { if height < Fixed::from_int(120) { 3 } else { 6 } }
                ) {
                    Button (
                        id: "template_switch_mode",
                        grow: 1.0,
                        height: Dimension::percent(100),
                        padding: @height {
                            if height < Fixed::from_int(120) { Padding::all(2) } else { Padding::all(4) }
                        },
                        normal_color: ColorToken::Primary,
                        pressed_color: ColorToken::Secondary,
                        text_color: ColorToken::OnPrimary,
                        border_radius: 8
                    ) [
                        Text::label("SWITCH MODE"),
                    ] on Tap { switch_mode.update(|mode| *mode = !*mode); }
                    Button (
                        width: @height { if height < Fixed::from_int(120) { 44 } else { 64 } },
                        height: Dimension::percent(100),
                        padding: @height {
                            if height < Fixed::from_int(120) { Padding::all(2) } else { Padding::all(4) }
                        },
                        normal_color: ColorToken::Surface,
                        pressed_color: ColorToken::Outline,
                        text_color: ColorToken::OnSurface,
                        border_color: ColorToken::Outline,
                        border_width: 1,
                        border_radius: 8
                    ) [
                        Text::label("PULSE"),
                    ] on Tap { energized.update(|active| *active = !*active); }
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use mirui::core::reactive::flush_signal_dirty;
    use mirui::input::event::gesture::GestureEvent;
    use mirui::input::event::GestureHandler;
    use mirui::ui::builder::WidgetBuilder;
    use mirui::ui::widgets::slider::{SliderEvent, SliderHandler};
    use mirui::ui::IdMap;

    fn text<'a>(world: &'a World, id: &'static str) -> alloc::borrow::Cow<'a, str> {
        let entity = world.find_by_id(id).expect("template id");
        world
            .get::<Text>(entity)
            .expect("text widget")
            .resolve(world)
    }

    #[test]
    fn controls_update_the_visible_state() {
        let mut world = World::new();
        world.insert_resource(IdMap::new());
        let parent = WidgetBuilder::new(&mut world).id();
        build_ui(&mut world, parent);

        assert_eq!(text(&world, "template_mode"), "FLOW");
        let button = world
            .find_by_id("template_switch_mode")
            .expect("mode button");
        GestureHandler::trigger(
            &mut world,
            button,
            &GestureEvent::Tap {
                x: Fixed::ZERO,
                y: Fixed::ZERO,
                target: button,
            },
        );
        flush_signal_dirty(&mut world);
        assert_eq!(text(&world, "template_mode"), "FOCUS");

        let slider = world.find_by_id("template_energy").expect("energy slider");
        let callback = world
            .get::<SliderHandler>(slider)
            .expect("slider handler")
            .on_event
            .clone_out();
        callback.call(
            &mut world,
            slider,
            &SliderEvent::ValueChanged {
                new: Fixed::from_int(20),
                old: Fixed::from_int(68),
            },
        );
        flush_signal_dirty(&mut world);
        assert_eq!(text(&world, "template_status"), "IDLE");
    }

    #[test]
    fn compact_layout_stays_inside_a_160_by_80_surface() {
        let mut app = App::headless(160, 80);
        app.with_default_widgets().with_default_systems();
        let root = app.spawn_root().id();
        build_ui(&mut app.world, root);
        app.render().unwrap();

        for id in [
            "template_card",
            "template_status",
            "template_mode",
            "template_energy",
            "template_switch_mode",
        ] {
            let entity = app.world.find_by_id(id).expect("template id");
            let rect = app
                .world
                .get::<mirui::ui::ComputedRect>(entity)
                .expect("computed layout")
                .0;
            assert!(
                rect.x >= Fixed::ZERO && rect.y >= Fixed::ZERO,
                "{id}: {rect:?}"
            );
            assert!(
                rect.x + rect.w <= Fixed::from_int(160) && rect.y + rect.h <= Fixed::from_int(80),
                "{id}: {rect:?}",
            );
        }
    }
}
