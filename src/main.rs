use druid::{Data, WidgetPod, Vec2, Selector};
use druid::widget::prelude::*;

use druid::widget::TextBox;
struct Canvas {
    transform: WidgetPod<String, Transform<String, TextBox<String>>>,
    translation: Vec2,
    scale: f64,
    last_mouse_pos: Vec2,
    is_move_action_active: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            transform: WidgetPod::new(Transform::new(TextBox::multiline().with_placeholder("Write here..."))),
            translation: Vec2::ZERO,
            scale: 1.0,
            last_mouse_pos: Vec2::ZERO,
            is_move_action_active: false,
        }
    }
}

impl Widget<String> for Canvas {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut String, env: &Env) {
        match event {
            Event::MouseDown(m) if m.button == druid::MouseButton::Middle => {
                self.is_move_action_active = true;
                self.last_mouse_pos = m.pos.to_vec2();
            },
            Event::MouseUp(m) if m.button == druid::MouseButton::Middle => self.is_move_action_active = false,
            Event::MouseMove(m) if self.is_move_action_active => {
                self.translation += m.pos.to_vec2() - self.last_mouse_pos;
                self.last_mouse_pos = m.pos.to_vec2();
                ctx.submit_command(druid::Command::new(SET_TRANSLATION_ACTION, self.translation, druid::Target::Auto));
            },
            Event::Wheel(w) => {
                if w.wheel_delta.y > 0.0 {
                    self.scale /= 2.0;
                } else {
                    self.scale *= 2.0;
                }
                ctx.submit_command(druid::Command::new(SET_SCALE_ACTION, self.scale, druid::Target::Auto));
            }
            _ => {}
        }
        self.transform.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &String, env: &Env) {
        self.transform.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &String, data: &String, env: &Env) {
        self.transform.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &String, env: &Env) -> Size {
        let size = self.transform.layout(ctx, bc, data, env);
        self.transform.set_layout_rect(ctx, data, env, druid::Rect::from_origin_size((0.0, 0.0), size));
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.transform.paint(ctx, data, env);
    }
}

const SET_SCALE_ACTION: Selector<f64> = Selector::new("set-scale");
const SET_TRANSLATION_ACTION: Selector<Vec2> = Selector::new("set-translation");

struct Transform<T: Data, W: Widget<T>> {
    translation: Vec2,
    scale: f64,
    inner: WidgetPod<T, W>
}

impl<T, W> Transform<T, W>
where
    T: Data,
    W: Widget<T>
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
            Event::MouseDown(m) => {
                Some(Event::MouseDown(scale_pos(m)))
            },
            Event::MouseUp(m) => {
                Some(Event::MouseUp(scale_pos(m)))
            },
            Event::MouseMove(m) => {
                Some(Event::MouseMove(scale_pos(m)))
            },
            _ => None
        };

        maybe_event
    }
}

impl<T, W> Widget<T> for Transform<T, W>
where
    T: Data,
    W: Widget<T>
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

        let mouse_scrolled_event = event.transform_scroll(-self.translation, ctx.size().to_rect(), true);
        let mouse_transformed_event = mouse_scrolled_event.map(|e| self.transform_event_scale(&e)).flatten();
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
        self.inner.set_layout_rect(ctx, data, env, druid::Rect::from_origin_size((0.0, 0.0), size));
        self.inner.set_viewport_offset(self.translation);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        ctx.transform(druid::Affine::translate(self.translation));
        ctx.transform(druid::Affine::scale(self.scale));
        self.inner.paint(ctx, data, env);
    }
}

fn main() {
    let main_window = druid::WindowDesc::new(build_ui)
        .title("Transform widget")
        .window_size((400.0, 400.0));

    let data = String::new();

    druid::AppLauncher::with_window(main_window)
        .launch(data)
        .expect("Could not launch app");
}

fn build_ui() -> Canvas {
    Canvas::new()
}
