use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, ReleaseEvent, RenderEvent,
                    Transformed};

use num;
use config::{color, font};
use menu::Settings;

#[derive(Copy, Clone)]
enum MenuSelection {
    Setting1,
    Setting2,
    Back
}

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    font: &mut GlyphCache,
    menu_selection: MenuSelection,
    settings: Settings
) {
    let mut _menu_color_setting1 = color::WHITE;
    let mut _menu_color_setting2 = color::WHITE;
    let mut _menu_color_back     = color::WHITE;

    match menu_selection {
        MenuSelection::Setting1 => _menu_color_setting1 = color::CYAN,
        MenuSelection::Setting2 => _menu_color_setting2 = color::CYAN,
        MenuSelection::Back     => _menu_color_back     = color::CYAN,
    }

    text(
        color::WHITE,
        font::SIZE + 12,
        "Settings",
        font,
        context.transform.trans(
            font::PADDING,
            48.0 + font::PADDING,
        ),
        graphics,
    ).unwrap();

    let mut new_line_offset = 40.0;
    
    // Setting 1
    text(
        _menu_color_setting1,
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
        _menu_color_setting1,
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
        _menu_color_setting2,
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
        _menu_color_setting2,
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
        _menu_color_back,
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

pub fn run(
    window: &mut PistonWindow,
    opengl: &mut GlGraphics,
    font: &mut GlyphCache,
    settings: &mut Settings
) {
    let mut menu_selection = MenuSelection::Setting1;
    let mut settings_step: f64 = 1.0;

    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(
                    context,
                    graphics,
                    font,
                    menu_selection,
                    *settings
                )
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W | Key::Up => match menu_selection {
                    MenuSelection::Setting1 => {}
                    MenuSelection::Setting2 => menu_selection = MenuSelection::Setting1,
                    MenuSelection::Back     => menu_selection = MenuSelection::Setting2,
                },
                Key::S | Key::Down => match menu_selection {
                    MenuSelection::Setting1 => menu_selection = MenuSelection::Setting2,
                    MenuSelection::Setting2 => menu_selection = MenuSelection::Back,
                    MenuSelection::Back     => {}
                },
                Key::A | Key::Left => {
                    match menu_selection {
                        MenuSelection::Setting1 => settings.setting1 = num::clamp(settings.setting1 - settings_step, 1.0, 1000.0),
                        MenuSelection::Setting2 => settings.setting2 = num::clamp(settings.setting2 - settings_step, 1.0, 1000.0),
                        MenuSelection::Back     => {}
                    }
                },
                Key::D | Key::Right => {
                    match menu_selection {
                        MenuSelection::Setting1 => settings.setting1 = num::clamp(settings.setting1 + settings_step, 1.0, 1000.0),
                        MenuSelection::Setting2 => settings.setting2 = num::clamp(settings.setting2 + settings_step, 1.0, 1000.0),
                        MenuSelection::Back     => {}
                    }
                },
                Key::Space | Key::Return => {
                    match menu_selection {
                        MenuSelection::Setting1 => {},
                        MenuSelection::Setting2 => {},
                        MenuSelection::Back     => break
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
                Key::LShift => {
                    settings_step = 1.0;
                },
                Key::LCtrl => {
                    settings_step = 1.0;
                },
                Key::LAlt => {
                    settings_step = 1.0;
                },
                _ => {}
            }
        }
    }
}
