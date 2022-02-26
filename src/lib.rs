use druid::{Data, Widget};
use plotters::{
    coord::Shift,
    prelude::{DrawingArea, IntoDrawingArea},
};
use plotters_piet::PietBackend;

pub struct Plot<T: Data> {
    plot: Box<dyn Fn((u32, u32), &T, &DrawingArea<PietBackend, Shift>)>,
}

impl<T: Data> Plot<T> {
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

    fn update(&mut self, ctx: &mut druid::UpdateCtx, old_data: &T, data: &T, env: &druid::Env) {
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
