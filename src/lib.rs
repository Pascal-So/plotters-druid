use piet_common::{kurbo, Color, Piet, RenderContext};
use plotters_backend::{BackendColor, BackendCoord, DrawingBackend, DrawingErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "plotters-piet error")
    }
}

impl std::error::Error for Error {}

pub struct PietBackend<'a, 'b> {
    pub size: (u32, u32),
    pub render_ctx: &'a mut Piet<'b>,
}

impl<'a, 'b> DrawingBackend for PietBackend<'a, 'b> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("ensure_prepared");
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        println!("present");
        self.render_ctx
            .finish()
            .map_err(|_| DrawingErrorKind::DrawingError(Error {}))
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: BackendColor,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let x = point.0 as f64;
        let y = point.1 as f64;
        self.render_ctx.fill(
            kurbo::Rect::new(x, y, x + 1., y + 1.),
            &plotters_color_to_piet(&color),
        );
        Ok(())
    }

    fn draw_line<S: plotters_backend::BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let from = plotters_point_to_kurbo(from);
        let to = plotters_point_to_kurbo(to);

        self.render_ctx.stroke(
            kurbo::Line::new(from, to),
            &plotters_color_to_piet(&style.color()),
            style.stroke_width() as f64,
        );
        Ok(())
    }

    fn draw_rect<S: plotters_backend::BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        let upper_left = plotters_point_to_kurbo(upper_left);
        let bottom_right = plotters_point_to_kurbo(bottom_right);
        let color = plotters_color_to_piet(&style.color());

        if fill {
            self.render_ctx.fill(
                kurbo::Rect::new(upper_left.x, upper_left.y, bottom_right.x, bottom_right.y),
                &color,
            );
        } else {
            self.render_ctx.stroke(
                kurbo::Rect::new(upper_left.x, upper_left.y, bottom_right.x, bottom_right.y),
                &color,
                style.stroke_width() as f64,
            );
        }
        Ok(())
    }

    fn draw_path<S: plotters_backend::BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let path: Vec<kurbo::PathEl> = path
            .into_iter()
            .map(|p| kurbo::PathEl::LineTo(plotters_point_to_kurbo(p)))
            .collect();
        self.render_ctx.stroke(
            &*path,
            &plotters_color_to_piet(&style.color()),
            style.stroke_width() as f64,
        );
        Ok(())
    }

    fn draw_circle<S: plotters_backend::BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!();
    }

    fn fill_polygon<S: plotters_backend::BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!();
    }

    fn blit_bitmap<'c>(
        &mut self,
        pos: BackendCoord,
        (iw, ih): (u32, u32),
        src: &'c [u8],
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        todo!();
    }
}

fn plotters_color_to_piet(col: &BackendColor) -> piet_common::Color {
    Color::rgba8(col.rgb.0, col.rgb.1, col.rgb.2, (col.alpha * 256.) as u8)
}

fn plotters_point_to_kurbo((x, y): BackendCoord) -> kurbo::Point {
    kurbo::Point {
        x: x as f64 + 0.5,
        y: y as f64 + 0.5,
    }
}
