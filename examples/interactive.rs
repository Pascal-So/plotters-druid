use druid::{
    widget::{Flex, Label, Slider},
    AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc,
};
use plotters::prelude::*;
use plotters_druid::Plot;

#[derive(Clone, Data, Lens)]
struct State {
    μ: f64,
}

fn build_plot_widget() -> impl Widget<State> {
    Plot::new(|_size, data: &State, root| {
        let μ = data.μ as f32;

        let res = 400;
        let font = FontDesc::new(FontFamily::SansSerif, 16., FontStyle::Normal);

        let mut chart = ChartBuilder::on(&root)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(10)
            .build_cartesian_2d(0.0..1_f32, 0.0..6_f32)
            .unwrap();

        chart
            .configure_mesh()
            .axis_style(&RGBColor(28, 28, 28))
            .x_label_style(font.clone().with_color(&WHITE))
            .y_label_style(font.clone().with_color(&WHITE))
            .draw()
            .unwrap();

        for (σ, idx) in [0.32_f32, 0.56, 1., 1.78, 3.16].into_iter().zip(0..) {
            let fac = 1. / (σ * std::f32::consts::TAU.sqrt());
            let color = Palette99::pick(idx);

            let data = (0..res).map(|x| x as f32 / res as f32).map(|x| {
                let y = fac * (-(logit(x) - μ).powi(2) / (2. * σ.powi(2))).exp() / (x * (1. - x));
                (x, y)
            });

            chart
                .draw_series(LineSeries::new(data, &color))
                .unwrap()
                .label(format!("σ = {σ}"))
                .legend(move |(x, y)| {
                    PathElement::new(
                        vec![(x, y), (x + 20, y)],
                        ShapeStyle::from(&color).stroke_width(2),
                    )
                });
        }
        chart
            .configure_series_labels()
            .position(SeriesLabelPosition::UpperRight)
            .background_style(&RGBColor(41, 41, 41))
            .border_style(&RGBColor(28, 28, 28))
            .label_font(font.with_color(&WHITE))
            .draw()
            .unwrap();
    })
}

fn build_slider_widget(name: String, min: f64, max: f64) -> impl Widget<f64> {
    Flex::row()
        .with_child(Label::new(name))
        .with_flex_child(
            Slider::new().with_range(min, max).env_scope(|env, _| {
                // remove the width limit in [`Slider`]
                env.set(druid::theme::WIDE_WIDGET_WIDTH, std::f64::INFINITY)
            }),
            1.,
        )
        .must_fill_main_axis(true)
        .fix_height(35.)
}

fn build_root_widget() -> impl Widget<State> {
    Flex::column()
        .with_flex_child(build_plot_widget(), 1.)
        .with_spacer(5.)
        .with_child(build_slider_widget("μ".to_string(), -3., 3.).lens(State::μ))
        .padding(10.)
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget)
        .title("Logit-Normal Distribution")
        .window_size((700.0, 450.0));

    AppLauncher::with_window(main_window)
        .launch(State { μ: 0.8 })
        .expect("Failed to launch application");
}

fn logit(p: f32) -> f32 {
    (p / (1. - p)).ln()
}
