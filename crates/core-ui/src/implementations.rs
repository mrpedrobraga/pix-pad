use core::str::FromStr;

use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    mono_font::{
        ascii::{FONT_10X20, FONT_8X13},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    primitives::{Primitive, PrimitiveStyleBuilder, Rectangle},
    text::Text,
    Drawable as _,
};
use heapless::{format, String};

use crate::{
    graphics::render_qr_code, ButtonInput, MainScreen, PixClientInfo, PixPadApp,
    ReceiveScreenState, TransferScreenState, SCREEN_HEIGHT, SCREEN_WIDTH,
};

impl PixPadApp {
    /// Starts a new app.
    pub fn new() -> Self {
        Self {
            main_screen: MainScreen::InfoScreen,
            info: PixClientInfo {
                name: String::from_str("Pedro Braga").unwrap(),
            }, // read from some memory?
        }
    }

    /// Given the current screen, does something different with the input.
    pub fn handle_input(&mut self, button: ButtonInput) {
        use crate::MainScreen::*;

        match &mut self.main_screen {
            #[allow(clippy::single_match)]
            InfoScreen => match button {
                ButtonInput::Right => {
                    self.main_screen = ReceiveScreen(ReceiveScreenState::default())
                }
                _ => {}
            },
            ReceiveScreen(receive_state) => {
                let method_menu_item_count = 1;

                match receive_state {
                    ReceiveScreenState::SelectMethod { method } => match button {
                        ButtonInput::Left => self.main_screen = InfoScreen,
                        ButtonInput::Right => {
                            self.main_screen = TransferScreen(TransferScreenState::default())
                        }
                        ButtonInput::Up => {
                            *method = if *method == 0 {
                                method_menu_item_count - 1
                            } else {
                                method.saturating_sub(1)
                            }
                        }
                        ButtonInput::Down => *method = (*method + 1) % method_menu_item_count,
                        ButtonInput::Ok => *receive_state = ReceiveScreenState::ShowingQr,
                        _ => {}
                    },
                    #[allow(clippy::single_match)]
                    ReceiveScreenState::ShowingQr => match button {
                        ButtonInput::Back => *receive_state = ReceiveScreenState::default(),
                        _ => {}
                    },
                }
            }
            #[allow(clippy::single_match)]
            TransferScreen(state) => match state {
                TransferScreenState::SelectMethod { method } => {
                    let method_menu_item_count = 3;

                    match button {
                        ButtonInput::Left => {
                            self.main_screen = ReceiveScreen(ReceiveScreenState::default())
                        }
                        // ButtonInput::Right => {self.main_screen = ...},
                        ButtonInput::Up => {
                            *method = if *method == 0 {
                                method_menu_item_count - 1
                            } else {
                                method.saturating_sub(1)
                            }
                        }
                        ButtonInput::Down => *method = (*method + 1) % method_menu_item_count,
                        _ => {}
                    }
                }
                TransferScreenState::ScanningQr => {
                    // TODO: Allow cancel.
                }
                TransferScreenState::InsertingPhoneNumber => {
                    // TODO: Allow cancel and insertion.
                }
                TransferScreenState::InsertingCPF => {
                    // TODO: Allow cancel and insertion.
                }
                TransferScreenState::Sending => {
                    // Nothing!
                }
                TransferScreenState::Success => {
                    // TODO: OK and BACK do the same thing.
                }
            },
        }
    }

    pub fn render<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        use crate::MainScreen::*;
        let big_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        let small_style = MonoTextStyle::new(&FONT_8X13, BinaryColor::Off);
        let info_style = MonoTextStyle::new(&FONT_8X13, BinaryColor::On);

        let dark_style = PrimitiveStyleBuilder::new()
            .fill_color(BinaryColor::Off)
            .stroke_color(BinaryColor::Off)
            .build();

        /* Status Bar! */

        let status_bar_height = 16;
        Rectangle::new(Point::zero(), Size::new(SCREEN_WIDTH, status_bar_height))
            .into_styled(dark_style)
            .draw(target)?;

        let screen_bar_height = 32;
        Rectangle::new(
            Point::new(0, (SCREEN_HEIGHT - screen_bar_height) as i32),
            Size::new(SCREEN_WIDTH, screen_bar_height),
        )
        .into_styled(dark_style)
        .draw(target)?;

        Text::new(
            &self.info.name,
            Point::new(10, status_bar_height as i32 - 4),
            info_style,
        )
        .draw(target)?;

        /* Screen content! */
        match &self.main_screen {
            InfoScreen => {
                let saldo_centavos = 10000;

                Text::new("Bem vindo ao PIX!", Point::new(10, 34), small_style).draw(target)?;
                Text::new(
                    &format!(11; "R$ {},{:02}", saldo_centavos / 100, saldo_centavos % 100)
                        .unwrap(),
                    Point::new(10, 52),
                    big_style,
                )
                .draw(target)?;
            }
            ReceiveScreen(_state) => {
                let example_qrcode_payload = "00020101021126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Pedro Braga6008BRASILIA62070503***630448CD";

                match _state {
                    ReceiveScreenState::SelectMethod { method } => {
                        Text::new("Receber PIX", Point::new(10, 34), big_style).draw(target)?;
                        Text::new(
                            "Selecione um metodo\npara recebimento.",
                            Point::new(10, 48),
                            small_style,
                        )
                        .draw(target)?;

                        draw_menu_item(*method == 0, "1. Emitir Codigo QR", 80, target)?;
                    }
                    ReceiveScreenState::ShowingQr => {
                        render_qr_code(
                            target,
                            example_qrcode_payload.as_bytes(),
                            Point::new(0, -9),
                        )?;
                        Text::new("Press. VOLTA para sair.", Point::new(8, 190), info_style)
                            .draw(target)?;
                    }
                }
            }
            TransferScreen(state) => match state {
                crate::TransferScreenState::SelectMethod { method } => {
                    Text::new("Transferir PIX", Point::new(10, 34), big_style).draw(target)?;
                    Text::new(
                        "Selecione um metodo\npara transferir!",
                        Point::new(10, 48),
                        small_style,
                    )
                    .draw(target)?;

                    draw_menu_item(*method == 0, "1. Escanear Codigo QR", 80, target)?;
                    draw_menu_item(*method == 1, "2. Chave (celular)", 80 + 16, target)?;
                    draw_menu_item(*method == 2, "3. Chave (CPF)", 80 + 32, target)?;
                }
                crate::TransferScreenState::ScanningQr => {}
                crate::TransferScreenState::InsertingPhoneNumber => {}
                crate::TransferScreenState::InsertingCPF => {}
                crate::TransferScreenState::Sending => {
                    Text::new("Enviando PIX...", Point::new(10, 34), big_style).draw(target)?;
                    Text::new("Aguarde um momento...", Point::new(10, 48), small_style)
                        .draw(target)?;
                }
                crate::TransferScreenState::Success => {
                    Text::new("Enviado!", Point::new(10, 34), big_style).draw(target)?;
                    // TODO: Comprovante aqui!
                }
            },
        }

        Ok(())
    }
}

impl Default for PixPadApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Writes a single digit to a string buffer.
#[allow(unused)]
fn write_digit(s: &mut String<4>, digit: u8) -> core::fmt::Result {
    use core::fmt::Write;
    write!(s, "{}", digit)
}

#[allow(unused)]
fn draw_menu_item<D>(
    is_selected: bool,
    text: &str,
    position_y: i32,
    target: &mut D,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    if is_selected {
        draw_menu_item_selected(text, position_y, target)
    } else {
        draw_menu_item_unselected(text, position_y, target)
    }
}

fn draw_menu_item_unselected<D>(text: &str, position_y: i32, target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Text::new(
        text,
        Point::new(13, position_y + 10),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::Off),
    )
    .draw(target)?;

    Ok(())
}

fn draw_menu_item_selected<D>(text: &str, position_y: i32, target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    Rectangle::new(Point::new(10, position_y), Size::new(SCREEN_WIDTH - 20, 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::Off)
                .stroke_color(BinaryColor::Off)
                .build(),
        )
        .draw(target)?;

    Text::new(
        text,
        Point::new(13, position_y + 10),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
    )
    .draw(target)?;

    Ok(())
}
