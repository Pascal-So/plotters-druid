use std::io::{Cursor, Read};

use image::io::Reader as ImageReader;
use image::RgbaImage;
use piet_common::{BitmapTarget, Device, ImageFormat};
use plotters::prelude::*;
use plotters_piet::PietBackend;

#[test]
fn snapshot_test() {
    let width = 1920;
    let height = 1080;

    let mut device = Device::new().unwrap();
    let mut bitmap = device.bitmap_target(width, height, 1.0).unwrap();

    {
        let mut render_ctx = bitmap.render_context();
        let piet_backend = PietBackend {
            size: (width as u32, height as u32),
            render_ctx: &mut render_ctx,
        };

        let root = piet_backend.into_drawing_area();

        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .caption("y=x^2", ("sans-serif", 50).into_font())
            .margin(5)
            .margin_right(15)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();

        chart.configure_mesh().draw().unwrap();

        chart
            .draw_series(LineSeries::new(
                (-50..=50).map(|x| x as f32 / 50.0).map(|x| (x, x * x)),
                &RED,
            ))
            .unwrap()
            .label("y = x^2")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE)
            .border_style(&BLACK)
            .draw()
            .unwrap();

        root.present().unwrap();
    }

    let expected = image::load_from_memory(include_bytes!("plot.png"))
        .expect("cannot decode snapshot")
        .into_rgba8();

    let actual = bitmap_to_image(&mut bitmap);

    if expected != actual {
        actual.save("asdf.png").unwrap();
        // assert_eq would spam the console with the entire list of bytes on fails
        assert!(false, "images differ");
    }
}

fn bitmap_to_image(bitmap: &mut BitmapTarget) -> RgbaImage {
    let buffer = bitmap.to_image_buf(ImageFormat::RgbaPremul).unwrap();

    RgbaImage::from_raw(
        buffer.width() as u32,
        buffer.height() as u32,
        buffer.raw_pixels().to_vec(),
    )
    .unwrap()
}
