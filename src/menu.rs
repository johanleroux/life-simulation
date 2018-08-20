use opengl_graphics::{GlGraphics, GlyphCache};
use piston_window::{clear, text, Button, Context, Key, PistonWindow, PressEvent, RenderEvent,
                    Size, TextureSettings, Transformed};

use config::{color, font};
use settings;

#[derive(Copy, Clone)]
enum MenuSelection {
    Simulate,
    Load,
    Settings,
    Exit,
}

/// Volume for music and sound effects.
#[derive(Copy, Clone)]
pub struct Settings {
    pub setting1: f64,
    pub setting2: f64,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            setting1: 0.0,
            setting2: 0.0,
        }
    }
}

fn draw(
    context: Context,
    graphics: &mut GlGraphics,
    font: &mut GlyphCache,
    menu_selection: MenuSelection
) {
    let mut _menu_color_simulate = color::WHITE;
    let mut _menu_color_load     = color::WHITE;
    let mut _menu_color_settings = color::WHITE;
    let mut _menu_color_exit     = color::WHITE;

    match menu_selection {
        MenuSelection::Simulate => _menu_color_simulate = color::CYAN,
        MenuSelection::Load     => _menu_color_load     = color::CYAN,
        MenuSelection::Settings => _menu_color_settings = color::CYAN,
        MenuSelection::Exit     => _menu_color_exit     = color::CYAN,
    }

    let menu_lines = [
        color::ColoredText {
            color: _menu_color_simulate,
            text: "Simulate",
        },
        color::ColoredText {
            color: _menu_color_load,
            text: "Load",
        },
        color::ColoredText {
            color: _menu_color_settings,
            text: "Settings",
        },
        color::ColoredText {
            color: _menu_color_exit,
            text: "Exit",
        },
    ];

    text(
        color::WHITE,
        font::SIZE + 12,
        "Life Simulation",
        font,
        context.transform.trans(
            font::PADDING,
            48.0 + font::PADDING,
        ),
        graphics,
    ).unwrap();

    for (index, line) in menu_lines.iter().enumerate() {
        let new_line_offset = 40.0;
        text(
            line.color,
            font::SIZE,
            line.text,
            font,
            context.transform.trans(
                font::PADDING,
                (index as f64 + 1.0) * new_line_offset + 192.0,
            ),
            graphics,
        ).unwrap();
    }
}

pub fn run(mut window: &mut PistonWindow, mut opengl: &mut GlGraphics, _window_size: Size) {
    let mut font = GlyphCache::new(
        "./assets/fonts/FiraSans-Regular.ttf",
        (),
        TextureSettings::new(),
    ).unwrap();

    let mut game_settings = Settings::new();
    let mut menu_selection = MenuSelection::Simulate;
    while let Some(event) = window.next() {
        if let Some(args) = event.render_args() {
            opengl.draw(args.viewport(), |context, graphics| {
                clear(color::BLACK, graphics);
                draw(
                    context,
                    graphics,
                    &mut font,
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
                            settings::run(
                                &mut window,
                                &mut opengl,
                                &mut font,
                                &mut game_settings
                            );
                        }
                        MenuSelection::Exit => break,
                    }
                },
                Key::Escape => break,
                _ => {}
            }
        }
    }
}
