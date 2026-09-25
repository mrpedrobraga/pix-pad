use std::time::Duration;

use core_ui::*;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
};
use embedded_graphics_simulator::{
    sdl2::Keycode, BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent,
    Window,
};

fn main() -> Result<(), std::io::Error> {
    const DISPLAY_SIZE: Size = Size::new(core_ui::SCREEN_WIDTH, core_ui::SCREEN_HEIGHT);
    const DISPLAY_SCALE: u32 = 2;

    let mut display = SimulatorDisplay::<BinaryColor>::new(DISPLAY_SIZE);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledWhite)
        .pixel_spacing(0)
        .scale(DISPLAY_SCALE)
        .build();
    let mut window = Window::new("Pix Pad (Simulator)", &output_settings);
    let mut app = PixPadApp::new();

    'running: loop {
        display.fill_solid(
            &Rectangle::new(Point::new(0, 0), DISPLAY_SIZE),
            BinaryColor::On,
        );

        app.render(&mut display);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => app.handle_input(ButtonInput::Up),
                    Keycode::Down => app.handle_input(ButtonInput::Down),
                    Keycode::Left => app.handle_input(ButtonInput::Left),
                    Keycode::Right => app.handle_input(ButtonInput::Right),
                    Keycode::Return | Keycode::SPACE => app.handle_input(ButtonInput::Ok),
                    Keycode::Escape | Keycode::BACKSPACE => app.handle_input(ButtonInput::Back),
                    Keycode::Kp1 | Keycode::Num1 => app.handle_input(ButtonInput::Num(1)),
                    Keycode::Kp2 | Keycode::Num2 => app.handle_input(ButtonInput::Num(2)),
                    Keycode::Kp3 | Keycode::Num3 => app.handle_input(ButtonInput::Num(3)),
                    Keycode::Kp4 | Keycode::Num4 => app.handle_input(ButtonInput::Num(4)),
                    Keycode::Kp5 | Keycode::Num5 => app.handle_input(ButtonInput::Num(5)),
                    Keycode::Kp6 | Keycode::Num6 => app.handle_input(ButtonInput::Num(6)),
                    Keycode::Kp7 | Keycode::Num7 => app.handle_input(ButtonInput::Num(7)),
                    Keycode::Kp8 | Keycode::Num8 => app.handle_input(ButtonInput::Num(8)),
                    Keycode::Kp9 | Keycode::Num9 => app.handle_input(ButtonInput::Num(9)),
                    Keycode::Kp0 | Keycode::Num0 => app.handle_input(ButtonInput::Num(0)),
                    _ => {}
                },
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(16));
    }
    Ok(())
}
