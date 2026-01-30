use image::RgbaImage;
use piet_common::{BitmapTarget, Device, ImageFormat};
use plotters::prelude::*;
use plotters_piet::PietBackend;

// Copied from the plotters "Snowflake" example:
// https://github.com/plotters-rs/plotters/blob/0f195eadaac7d9a2390a3707fbe192f8e2645d34/plotters/examples/snowflake.rs

#[test]
fn snapshot_test() {
    let width = 600;
    let height = 450;

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
            .build_cartesian_2d(-2.0..2.0, -1.5..1.5)
            .unwrap();

        let mut snowflake_vertices = {
            let mut current: Vec<(f64, f64)> = vec![
                (0.0, 1.0),
                ((3.0f64).sqrt() / 2.0, -0.5),
                (-(3.0f64).sqrt() / 2.0, -0.5),
            ];
            for _ in 0..6 {
                current = snowflake_iter(&current[..]);
            }
            current
        };

        chart
            .draw_series(std::iter::once(Polygon::new(
                snowflake_vertices.clone(),
                RED.mix(0.2),
            )))
            .unwrap();
        snowflake_vertices.push(snowflake_vertices[0]);
        chart
            .draw_series(std::iter::once(PathElement::new(snowflake_vertices, RED)))
            .unwrap();

        root.present().unwrap();
    }

    let expected = image::load_from_memory(include_bytes!("snowflake.png"))
        .expect("cannot decode snapshot")
        .into_rgba8();

    let actual = bitmap_to_image(&mut bitmap);

    if expected != actual {
        actual.save("tests/snowflake-actual.png").unwrap();
        // assert_eq would spam the console with the entire list of bytes on fails
        assert!(false, "images differ");
    }
}

fn snowflake_iter(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut ret = vec![];
    for i in 0..points.len() {
        let (start, end) = (points[i], points[(i + 1) % points.len()]);
        let t = ((end.0 - start.0) / 3.0, (end.1 - start.1) / 3.0);
        let s = (
            t.0 * 0.5 - t.1 * (0.75f64).sqrt(),
            t.1 * 0.5 + (0.75f64).sqrt() * t.0,
        );
        ret.push(start);
        ret.push((start.0 + t.0, start.1 + t.1));
        ret.push((start.0 + t.0 + s.0, start.1 + t.1 + s.1));
        ret.push((start.0 + t.0 * 2.0, start.1 + t.1 * 2.0));
    }
    ret
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
