/*!
A [Piet](https://crates.io/crates/piet) backend for [Plotters](https://crates.io/crates/plotters). This lets you draw plots on a Piet render context.
*/

use piet_common::{kurbo, Color, LineCap, Piet, RenderContext, StrokeStyle};
use plotters_backend::{BackendColor, BackendCoord, DrawingBackend, DrawingErrorKind};

#[derive(Debug, PartialEq, Eq)]
pub struct Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "plotters-piet error")
    }
}

impl std::error::Error for Error {}

/// The piet backend.
///
/// Note that the size of the piet context has to be specified here.
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
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
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
        let from = plotters_point_to_kurbo_mid(from);
        let to = plotters_point_to_kurbo_mid(to);

        self.render_ctx.stroke_styled(
            kurbo::Line::new(from, to),
            &plotters_color_to_piet(&style.color()),
            style.stroke_width() as f64,
            &STROKE_STYLE_SQUARE_CAP,
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
        let color = plotters_color_to_piet(&style.color());

        if fill {
            let upper_left = plotters_point_to_kurbo_corner(upper_left);
            let mut bottom_right = plotters_point_to_kurbo_corner(bottom_right);
            bottom_right.x += 1.;
            bottom_right.y += 1.;
            let rect = kurbo::Rect::new(upper_left.x, upper_left.y, bottom_right.x, bottom_right.y);

            self.render_ctx.fill(rect, &color);
        } else {
            let upper_left = plotters_point_to_kurbo_mid(upper_left);
            let bottom_right = plotters_point_to_kurbo_mid(bottom_right);
            let rect = kurbo::Rect::new(upper_left.x, upper_left.y, bottom_right.x, bottom_right.y);

            self.render_ctx
                .stroke(rect, &color, style.stroke_width() as f64);
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
            .map(|p| kurbo::PathEl::LineTo(plotters_point_to_kurbo_mid(p)))
            .collect();

        self.render_ctx.stroke_styled(
            &*path,
            &plotters_color_to_piet(&style.color()),
            style.stroke_width() as f64,
            &STROKE_STYLE_SQUARE_CAP,
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
        let center = plotters_point_to_kurbo_mid(center);
        let color = plotters_color_to_piet(&style.color());
        let circle = kurbo::Circle::new(center, radius as f64);

        if fill {
            self.render_ctx.fill(circle, &color);
        } else {
            self.render_ctx
                .stroke(circle, &color, style.stroke_width() as f64);
        }
        Ok(())
    }

    fn fill_polygon<S: plotters_backend::BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        vert: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let path: Vec<kurbo::PathEl> = vert
            .into_iter()
            .map(|p| kurbo::PathEl::LineTo(plotters_point_to_kurbo_mid(p)))
            .chain(std::iter::once(kurbo::PathEl::ClosePath))
            .collect();
        self.render_ctx
            .fill(&*path, &plotters_color_to_piet(&style.color()));
        Ok(())
    }

    // For now we use the default text drawing provided by plotters. This is definitely slower,
    // but at least we don't have to worry about matching the font size and offset which turns
    // out to be trickier than expected.
    // fn draw_text<TStyle: plotters_backend::BackendTextStyle>(
    //     &mut self,
    //     text: &str,
    //     style: &TStyle,
    //     pos: BackendCoord,
    // ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
    //     let pos = plotters_point_to_kurbo(pos);
    //     let color = plotters_color_to_piet(&style.color());

    //     let text_api = self.render_ctx.text();
    //     let font_family = match style.family() {
    //         plotters_backend::FontFamily::Serif => Ok(FontFamily::SERIF),
    //         plotters_backend::FontFamily::SansSerif => Ok(FontFamily::SANS_SERIF),
    //         plotters_backend::FontFamily::Monospace => Ok(FontFamily::MONOSPACE),
    //         plotters_backend::FontFamily::Name(name) => text_api
    //             .font_family(name)
    //             .ok_or(piet_common::Error::MissingFont),
    //     };

    //     let (font_style, weight) = match style.style() {
    //         plotters_backend::FontStyle::Normal => (FontStyle::Regular, FontWeight::REGULAR),
    //         plotters_backend::FontStyle::Oblique => (FontStyle::Italic, FontWeight::REGULAR),
    //         plotters_backend::FontStyle::Italic => (FontStyle::Italic, FontWeight::REGULAR),
    //         plotters_backend::FontStyle::Bold => (FontStyle::Regular, FontWeight::BOLD),
    //     };

    //     let alignment = match style.anchor().h_pos {
    //         plotters_backend::text_anchor::HPos::Left => TextAlignment::Start,
    //         plotters_backend::text_anchor::HPos::Right => TextAlignment::End,
    //         plotters_backend::text_anchor::HPos::Center => TextAlignment::Center,
    //     };

    //     let layout = text_api
    //         .new_text_layout(String::from(text))
    //         .font(font_family.unwrap(), style.size())
    //         .text_color(color)
    //         .alignment(alignment)
    //         .default_attribute(TextAttribute::Style(font_style))
    //         .default_attribute(TextAttribute::Weight(weight))
    //         .build()
    //         .unwrap();

    //     // todo: style.anchor().v_pos
    //     // todo: style.transform()

    //     self.render_ctx.draw_text(&layout, pos);
    //     Ok(())
    // }
}

fn plotters_color_to_piet(col: &BackendColor) -> piet_common::Color {
    Color::rgba8(col.rgb.0, col.rgb.1, col.rgb.2, (col.alpha * 256.) as u8)
}

fn plotters_point_to_kurbo_mid((x, y): BackendCoord) -> kurbo::Point {
    kurbo::Point {
        x: x as f64 + 0.5,
        y: y as f64 + 0.5,
    }
}

fn plotters_point_to_kurbo_corner((x, y): BackendCoord) -> kurbo::Point {
    kurbo::Point {
        x: x as f64,
        y: y as f64,
    }
}

const STROKE_STYLE_SQUARE_CAP: StrokeStyle = StrokeStyle {
    line_join: None,
    line_cap: Some(LineCap::Square),
    dash: None,
    miter_limit: None,
};
