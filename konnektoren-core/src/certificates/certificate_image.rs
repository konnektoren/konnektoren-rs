use super::CertificateData;
use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use anyhow::Result;
use base64::engine::general_purpose;
use base64::Engine as _;
use image::{imageops, DynamicImage, ImageBuffer, ImageFormat, Luma, Rgba, RgbaImage, ImageReader};
use imageproc::drawing::draw_text_mut;
use imageproc::rect::Rect;
use plot_icon::generate_png;
use qrcode::{EcLevel, QrCode};
use std::cmp;
use std::io::Cursor;

pub fn create_certificate(
    certificate_data: &CertificateData,
    url: &str,
    issuer: &str,
) -> Result<DynamicImage> {
    let cert_width = 1000;
    let cert_height = 800;
    let border_color = Rgba([0, 123, 255, 255]);
    let border_thickness = 20u32;
    let highlight_color = Rgba([0, 123, 255, 255]);
    let text_color = Rgba([0, 0, 0, 255]);
    let background_color = Rgba([249, 249, 249, 255]);
    let qr_code_size = 150;

    let qr_code = QrCode::with_error_correction_level(url, EcLevel::H).unwrap();
    let qr_code_image = qr_code.render::<Luma<u8>>().quiet_zone(false).build();

    let qr_code_image_rgba =
        ImageBuffer::from_fn(qr_code_image.width(), qr_code_image.height(), |x, y| {
            let pixel = qr_code_image.get_pixel(x, y);
            if pixel[0] == 0 {
                Rgba([0, 0, 0, 255])
            } else {
                Rgba([255, 255, 255, 255])
            }
        });

    let resized_qr_code_image = imageops::resize(
        &qr_code_image_rgba,
        qr_code_size,
        qr_code_size,
        imageops::FilterType::Nearest,
    );

    let mut cert_image = RgbaImage::new(cert_width, cert_height);

    let logo_bytes = include_bytes!("../assets/favicon.png");
    let logo_image = ImageReader::new(Cursor::new(logo_bytes))
        .with_guessed_format()
        .expect("Failed to guess image format")
        .decode()
        .expect("Failed to decode image");

    let identicon_image = {
        let data: Vec<u8> = generate_png(certificate_data.to_base64().as_bytes(), 75)
            .expect("Failed to generate identicon");
        let mut image = ImageReader::new(Cursor::new(data));
        image.set_format(image::ImageFormat::Png);
        image.decode().expect("Failed to decode image")
    };

    let scaled_logo_image = imageops::resize(&logo_image, 100, 100, imageops::FilterType::Nearest);

    imageproc::drawing::draw_filled_rect_mut(
        &mut cert_image,
        Rect::at(0, 0).of_size(cert_width, cert_height),
        border_color,
    );

    imageproc::drawing::draw_filled_rect_mut(
        &mut cert_image,
        Rect::at(border_thickness as i32, border_thickness as i32).of_size(
            cert_width - 2 * border_thickness,
            cert_height - 2 * border_thickness,
        ),
        background_color,
    );

    let font_data: &[u8] = include_bytes!("../assets/Lora-Regular.ttf");
    let font = FontRef::try_from_slice(font_data).unwrap();

    image::imageops::overlay(&mut cert_image, &scaled_logo_image, 40, 40);
    let cert_width = cert_image.width();
    image::imageops::overlay(
        &mut cert_image,
        &identicon_image,
        cert_width as i64 - 40 - 75,
        40,
    );

    let title = "Certificate of Achievement";
    let scale_title = PxScale::from(40.0);
    let title_width = calculate_text_width(&font, scale_title, title);
    draw_text_mut(
        &mut cert_image,
        highlight_color,
        cmp::max(0, (cert_width as i32 - title_width as i32) / 2) as i32,
        80,
        scale_title,
        &font,
        title,
    );

    let name = &certificate_data.profile_name;
    let scale_name = PxScale::from(30.0);
    let name_width = calculate_text_width(&font, scale_name, name);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - name_width as i32) / 2) as i32,
        150,
        scale_name,
        &font,
        name,
    );

    let message = format!(
        "has completed the {} game path with {} challenges solved out of {} total challenges.",
        certificate_data.game_path_name,
        certificate_data.solved_challenges,
        certificate_data.total_challenges
    );
    let scale_message = PxScale::from(20.0);
    let message_width = calculate_text_width(&font, scale_message, &message);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - message_width as i32) / 2) as i32,
        200,
        scale_message,
        &font,
        &message,
    );

    let performance_message = format!("Performance: {}%", certificate_data.performance_percentage);
    let scale_performance = PxScale::from(20.0);
    let performance_width = calculate_text_width(&font, scale_performance, &performance_message);
    draw_text_mut(
        &mut cert_image,
        Rgba([0u8, 0u8, 0u8, 255u8]),
        cmp::max(0, (cert_width as i32 - performance_width as i32) / 2) as i32,
        250,
        scale_performance,
        &font,
        &performance_message,
    );

    let date_str = format!("{}", certificate_data.date.format("%d %B %Y"));
    let scale_date = PxScale::from(20.0);
    let date_width = calculate_text_width(&font, scale_date, &date_str);
    draw_text_mut(
        &mut cert_image,
        text_color,
        cmp::max(0, (cert_width as i32 - date_width as i32) / 2) as i32,
        300,
        scale_date,
        &font,
        &date_str,
    );

    let issued_by_message = format!("Issued by {}", issuer);
    let scale_issued_by = PxScale::from(20.0);
    let issued_by_width = calculate_text_width(&font, scale_issued_by, &issued_by_message);
    draw_text_mut(
        &mut cert_image,
        highlight_color,
        cmp::max(0, (cert_width as i32 - issued_by_width as i32) / 2) as i32,
        350,
        scale_issued_by,
        &font,
        &issued_by_message,
    );

    let qr_code_size = resized_qr_code_image.width() as u32;
    imageops::overlay(
        &mut cert_image,
        &resized_qr_code_image,
        cmp::max(0, (cert_width as i32 - qr_code_size as i32) / 2) as i64,
        (cert_height - qr_code_size - 50) as i64,
    );
    Ok(DynamicImage::ImageRgba8(cert_image))
}

fn calculate_text_width(font: &FontRef, scale: PxScale, text: &str) -> u32 {
    let scaled_font = font.as_scaled(scale);
    text.chars().fold(0.0, |acc, c| {
        acc + scaled_font.h_advance(scaled_font.scaled_glyph(c).id)
    }) as u32
}

pub fn create_certificate_data_url(
    certificate_data: &CertificateData,
    url: &str,
    issuer: &str,
) -> Result<String> {
    let image = create_certificate(certificate_data, url, issuer)?;

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    Ok(format!("data:image/png;base64,{}", res_base64))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_create_certificate() {
        let certificate_data = CertificateData::new(
            "Test Game Path".to_string(),
            10,
            5,
            "Test Player".to_string(),
            Utc::now(),
        );
        let url = "https://example.com";
        let issuer = "Test Issuer";

        let image = create_certificate(&certificate_data, url, issuer).unwrap();

        assert!(image.width() > 100);
        assert!(image.height() > 100);
    }

    #[test]
    fn test_create_certificate_data_url() {
        let certificate_data = CertificateData::new(
            "Test Game Path".to_string(),
            10,
            5,
            "Test Player".to_string(),
            Utc::now(),
        );
        let url = "https://example.com";
        let issuer = "Test Issuer";

        let data_url = create_certificate_data_url(&certificate_data, url, issuer).unwrap();

        assert_eq!(data_url.starts_with("data:image/png;base64,"), true);
    }
}
