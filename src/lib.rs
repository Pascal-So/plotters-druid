/*!

Use [Plotters](https://crates.io/crates/plotters) to draw plots in [Druid](https://crates.io/crates/druid).

Note that this is not directly a plotters backend in the sense described in
[plotters_backend](https://docs.rs/plotters-backend/latest/plotters_backend/), instead this uses
the plotters-piet backend and wraps it in a struct that implements [`druid::Widget`].

You'll mainly need [`Plot::new`] from this crate.

# Example

For more complete examples see [the GitHub repo](https://github.com/Pascal-So/plotters-druid#examples)

```rust
# use druid::{Widget, WindowDesc, AppLauncher};
# use plotters_druid::Plot;
# use plotters::prelude::*;
# #[derive(Clone, druid::Data)]
# struct AppState;
fn build_plot_widget() -> impl Widget<AppState> {
    Plot::new(|(width, height), data: &AppState, root| {
        root.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&root)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .unwrap();

        // see the plotters documentation on how to use `chart`
    })
}

# fn main() {
let main_window = WindowDesc::new(build_plot_widget);
# }
```
*/

use druid::{Data, Widget};
use plotters::{
    coord::Shift,
    prelude::{DrawingArea, IntoDrawingArea},
};
use plotters_piet::PietBackend;

/// The type of a plot widget.
///
/// See [`Plot::new`] for information on how to construct this.
///
/// This implements [`druid::Widget`] so it can be used like
/// any other widget type.
/// ```rust
/// # use druid::{Widget, WindowDesc, AppLauncher};
/// # use plotters_druid::Plot;
/// fn build_plot_widget() -> impl Widget<()> {
///     // ... construct and return widget using Plot::new()
///     # Plot::new(|_, _, _|{})
/// }
///
/// # fn main() {
/// let main_window = WindowDesc::new(build_plot_widget);
/// # }
/// ```
pub struct Plot<T: Data> {
    #[allow(clippy::type_complexity)]
    plot: Box<dyn Fn((u32, u32), &T, &DrawingArea<PietBackend, Shift>)>,
}

impl<T: Data> Plot<T> {
    /// Create a plot widget
    ///
    /// This takes a function that should draw the plot using the normal plotters API.
    /// The function has access to the width and height of the plotting area, to the
    /// [`Data`] of the rust widget, and to a plotters [`DrawingArea`].
    ///
    /// ```rust
    /// # use plotters_druid::Plot;
    /// # use plotters::prelude::*;
    /// # #[derive(Clone, druid::Data)]
    /// # struct AppState;
    /// Plot::new(|(width, height), data: &AppState, root| {
    ///     root.fill(&WHITE).unwrap();
    ///     let mut chart = ChartBuilder::on(&root)
    ///         .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
    ///         .unwrap();
    ///
    ///     // see the plotters documentation on how to use `chart`
    /// });
    /// ```
    pub fn new(f: impl Fn((u32, u32), &T, &DrawingArea<PietBackend, Shift>) + 'static) -> Plot<T> {
        Plot { plot: Box::new(f) }
    }
}

impl<T> Widget<T> for Plot<T>
where
    T: Data,
{
    fn event(&mut self, _: &mut druid::EventCtx, _: &druid::Event, _: &mut T, _: &druid::Env) {}

    fn lifecycle(
        &mut self,
        _: &mut druid::LifeCycleCtx,
        _: &druid::LifeCycle,
        _: &T,
        _: &druid::Env,
    ) {
    }

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, _env: &druid::Env) {
        if !old_data.same(data) {
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _: &T,
        _: &druid::Env,
    ) -> druid::Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &T, _: &druid::Env) {
        let druid::Size { width, height } = ctx.size();
        let size = (width as u32, height as u32);
        let backend = PietBackend {
            size,
            render_ctx: ctx.render_ctx,
        };

        (self.plot)(size, data, &backend.into_drawing_area());
    }
}
