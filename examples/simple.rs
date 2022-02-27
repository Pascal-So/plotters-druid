use druid::{AppLauncher, Widget, WindowDesc};
use plotters::prelude::*;
use plotters_druid::Plot;

fn build_plot_widget() -> impl Widget<()> {
    Plot::new(|_size, _data, root| {
        // Code taken from the plotters example: https://github.com/38/plotters#quick-start
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
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()
            .unwrap();
    })
}

fn main() {
    let main_window = WindowDesc::new(build_plot_widget)
        .title("Hello Plot!")
        .window_size((400.0, 400.0));

    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");
}
