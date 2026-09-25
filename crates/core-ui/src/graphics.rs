use embedded_graphics::{
    draw_target::{DrawTarget, DrawTargetExt as _},
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
    Drawable as _,
};
use embedded_qr::{QrBuilder, Version7};

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub fn render_qr_code<D>(
    target: &mut D,
    payload_bytes: &[u8],
    offset: Point,
) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    const QR_CODE_SCALE: u32 = 3;
    const SCREEN_SIZE: Size = Size::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // Version 4 for PIX EMVCo payloads :-)
    let qr = QrBuilder::<Version7>::new()
        .with_ecc_level(embedded_qr::EccLevel::L)
        .build(payload_bytes)
        .unwrap();

    let qr_drawable = qr
        .into_drawable(BinaryColor::Off, BinaryColor::On)
        .with_module_size(QR_CODE_SCALE)
        .with_border(2);

    // Center the QR code on-screen.
    let pixel_width = qr.width() as u32 * QR_CODE_SCALE + 8;
    let final_size = Size::new(pixel_width, pixel_width);
    let top_left = (SCREEN_SIZE - final_size) / 2;
    let top_left = Point::new(top_left.width as i32, top_left.height as i32);

    qr_drawable.draw(&mut target.translated(top_left + offset))?;

    Ok(())
}
