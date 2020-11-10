use druid::widget::prelude::*;
use druid::{Data, Selector, Vec2, WidgetPod};

pub const SET_SCALE_ACTION: Selector<f64> = Selector::new("set-scale");
pub const SET_TRANSLATION_ACTION: Selector<Vec2> = Selector::new("set-translation");

pub struct Transform<T: Data, W: Widget<T>> {
    translation: Vec2,
    scale: f64,
    inner: WidgetPod<T, W>,
}

impl<T, W> Transform<T, W>
where
    T: Data,
    W: Widget<T>,
{
    pub fn new(widget: W) -> Self {
        Self {
            translation: Vec2::ZERO,
            scale: 1.0,
            inner: WidgetPod::new(widget),
        }
    }

    pub fn transform_event_scale(&self, event: &Event) -> Option<Event> {
        let scale_pos = |m: &druid::MouseEvent| {
            let mut m = m.clone();
            m.pos = (m.pos.to_vec2() / self.scale).to_point();
            m
        };

        let maybe_event = match event {
            Event::MouseDown(m) => Some(Event::MouseDown(scale_pos(m))),
            Event::MouseUp(m) => Some(Event::MouseUp(scale_pos(m))),
            Event::MouseMove(m) => Some(Event::MouseMove(scale_pos(m))),
            _ => None,
        };

        maybe_event
    }
}

impl<T, W> Widget<T> for Transform<T, W>
where
    T: Data,
    W: Widget<T>,
{
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Command(cmd) => {
                if cmd.is(SET_SCALE_ACTION) {
                    ctx.request_layout();
                    self.scale = *cmd.get_unchecked(SET_SCALE_ACTION);
                } else if cmd.is(SET_TRANSLATION_ACTION) {
                    ctx.request_layout();
                    self.translation = *cmd.get_unchecked(SET_TRANSLATION_ACTION);
                }
                ctx.set_handled();
                return;
            }
            _ => {}
        }

        let mouse_scrolled_event =
            event.transform_scroll(-self.translation, ctx.size().to_rect(), true);
        let mouse_transformed_event = mouse_scrolled_event
            .map(|e| self.transform_event_scale(&e))
            .flatten();
        let event = mouse_transformed_event.unwrap_or(event.clone());
        self.inner.event(ctx, &event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_layout_rect(
            ctx,
            data,
            env,
            druid::Rect::from_origin_size((0.0, 0.0), size),
        );
        self.inner.set_viewport_offset(-self.translation);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ctx.transform(druid::Affine::translate(self.translation));
        ctx.transform(druid::Affine::scale(self.scale));
        self.inner.paint(ctx, data, env);
    }
}
