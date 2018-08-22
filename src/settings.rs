use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, ReleaseEvent, RenderEvent, Transformed};

use num;
use config::{color, font};

#[derive(Copy, Clone)]
pub struct Settings {
    pub setting1: f64,
    pub setting2: f64,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            setting1: 1.0,
            setting2: 1.0,
        }
    }
}

fn draw(context: Context, graphics: &mut GlGraphics, font: &mut GlyphCache, menu_selection: i32, settings: Settings) {
    text(
        color::WHITE,
        font::TITLE_SIZE,
        "Settings",
        font,
        context.transform.trans(
            font::PADDING,
            font::TITLE_PADDING,
        ),
        graphics,
    ).unwrap();

    let mut new_line_offset = 40.0;
    
    // Setting 1
    text(
        font::color(menu_selection, 0),
        font::SIZE,
        "Setting 1",
        font,
        context.transform.trans(
            font::PADDING,
            new_line_offset + 192.0
        ),
        graphics,
    ).unwrap();
    text(
        font::color(menu_selection, 0),
        font::SIZE,
        &format!("{}", settings.setting1),
        font,
        context.transform.trans(
            font::PADDING + 300.0,
            new_line_offset + 192.0
        ),
        graphics,
    ).unwrap();

    new_line_offset += 40.0;

    // Setting 2
    text(
        font::color(menu_selection, 1),
        font::SIZE,
        "Setting 2",
        font,
        context.transform.trans(
            font::PADDING,
            new_line_offset + 192.0
        ),
        graphics,
    ).unwrap();
    text(
        font::color(menu_selection, 1),
        font::SIZE,
        &format!("{}", settings.setting2),
        font,
        context.transform.trans(
            font::PADDING + 300.0,
            new_line_offset + 192.0
        ),
        graphics,
    ).unwrap();

    new_line_offset += 80.0;

    // Back
    text(
        font::color(menu_selection, 2),
        font::SIZE,
        "Back",
        font,
        context.transform.trans(
            font::PADDING,
            new_line_offset + 192.0
        ),
        graphics,
    ).unwrap();
}

pub fn run(window: &mut PistonWindow, opengl: &mut GlGraphics, font: &mut GlyphCache, settings: &mut Settings) {
    let mut menu_selection: i32 = 0;
    let mut settings_step: f64 = 1.0;

    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(context, graphics, font, menu_selection, *settings);
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W | Key::Up => menu_selection = num::clamp(menu_selection - 1, 0, 2),
                Key::S | Key::Down => menu_selection = num::clamp(menu_selection + 1, 0, 2),
                Key::A | Key::Left => {
                    match menu_selection {
                        0 => settings.setting1 = num::clamp(settings.setting1 - settings_step, 1.0, 1000.0),
                        1 => settings.setting2 = num::clamp(settings.setting2 - settings_step, 1.0, 1000.0),
                        2 => {},
                        _ => {}
                    }
                },
                Key::D | Key::Right => {
                    match menu_selection {
                        0 => settings.setting1 = num::clamp(settings.setting1 + settings_step, 1.0, 1000.0),
                        1 => settings.setting2 = num::clamp(settings.setting2 + settings_step, 1.0, 1000.0),
                        2 => {},
                        _ => {}
                    }
                },
                Key::Space | Key::Return => {
                    match menu_selection {
                        0 => {},
                        1 => {},
                        2 => break,
                        _ => {}
                    }
                },
                Key::LShift => {
                    settings_step = 5.0;
                },
                Key::LCtrl => {
                    settings_step = 10.0;
                },
                Key::LAlt => {
                    settings_step = 100.0;
                },
                Key::Escape => break,
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::LShift | Key::LCtrl | Key::LAlt => {
                    settings_step = 1.0;
                },
                _ => {}
            }
        }
    }
}