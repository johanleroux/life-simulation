use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent,
                    Size, TextureSettings, Transformed};

use config::color;

#[derive(Copy, Clone)]
enum MenuSelection {
    Simulate,
    Load,
    Settings,
    Exit,
}

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    font: &mut GlyphCache,
    menu_padding: f64,
    menu_selection: MenuSelection,
) {
    let mut _menu_simulate = color::WHITE;
    let mut _menu_load     = color::WHITE;
    let mut _menu_settings = color::WHITE;
    let mut _menu_exit     = color::WHITE;

    match menu_selection {
        MenuSelection::Simulate => _menu_simulate = color::CYAN,
        MenuSelection::Load     => _menu_load     = color::CYAN,
        MenuSelection::Settings => _menu_settings = color::CYAN,
        MenuSelection::Exit     => _menu_exit     = color::CYAN,
    }

    let menu_lines = [
        color::ColoredText {
            color: _menu_simulate,
            text: "Simulate",
        },
        color::ColoredText {
            color: _menu_load,
            text: "Load",
        },
        color::ColoredText {
            color: _menu_settings,
            text: "Settings",
        },
        color::ColoredText {
            color: _menu_exit,
            text: "Exit",
        },
    ];

    text(
        color::WHITE,
        48,
        "Life Simulation",
        font,
        context.transform.trans(
                menu_padding,
                48.0 + menu_padding,
            ),
            graphics,
        ).unwrap();

    for (index, line) in menu_lines.iter().enumerate() {
        let new_line_offset = 40.0;
        text(
            line.color,
            32,
            line.text,
            font,
            context.transform.trans(
                menu_padding,
                (index as f64 + 1.0) * new_line_offset + 192.0,
            ),
            graphics,
        ).unwrap();
    }
}

pub fn run(window: &mut PistonWindow, opengl: &mut GlGraphics, _window_size: Size) {
    let mut font = GlyphCache::new(
        "./assets/fonts/FiraSans-Regular.ttf",
        (),
        TextureSettings::new(),
    ).unwrap();

    let mut menu_selection = MenuSelection::Simulate;
    let menu_padding = 64 as f64;
    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(
                    context,
                    graphics,
                    &mut font,
                    menu_padding,
                    menu_selection
                );
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W | Key::Up => match menu_selection {
                    MenuSelection::Simulate => {}
                    MenuSelection::Load     => menu_selection = MenuSelection::Simulate,
                    MenuSelection::Settings => menu_selection = MenuSelection::Load,
                    MenuSelection::Exit     => menu_selection = MenuSelection::Settings,
                },
                Key::S | Key::Down => match menu_selection {
                    MenuSelection::Simulate => menu_selection = MenuSelection::Load,
                    MenuSelection::Load     => menu_selection = MenuSelection::Settings,
                    MenuSelection::Settings => menu_selection = MenuSelection::Exit,
                    MenuSelection::Exit     => {}
                },
                Key::Space | Key::Return => {
                    match menu_selection {
                        MenuSelection::Simulate => {
                            //
                        }
                        MenuSelection::Load => {
                            //
                        }
                        MenuSelection::Settings => {
                            //
                        }
                        MenuSelection::Exit => break,
                    }
                }
                _ => {}
            }
        }
    }
}
