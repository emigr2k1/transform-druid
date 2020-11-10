use druid::widget::prelude::*;
use druid::widget::TextBox;
use druid::{Vec2, WidgetPod};

use crate::transform::{Transform, SET_SCALE_ACTION, SET_TRANSLATION_ACTION};

/// Widget to control `Transform` properties with the mouse.
pub struct Canvas {
    transform: WidgetPod<String, Transform<String, TextBox<String>>>,
    translation: Vec2,
    scale: f64,
    last_mouse_pos: Vec2,
    is_move_action_active: bool,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            transform: WidgetPod::new(Transform::new(
                TextBox::multiline().with_placeholder("Write here..."),
            )),
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
            }
            Event::MouseUp(m) if m.button == druid::MouseButton::Middle => {
                self.is_move_action_active = false
            }
            Event::MouseMove(m) if self.is_move_action_active => {
                self.translation += m.pos.to_vec2() - self.last_mouse_pos;
                self.last_mouse_pos = m.pos.to_vec2();
                ctx.submit_command(druid::Command::new(
                    SET_TRANSLATION_ACTION,
                    self.translation,
                    druid::Target::Auto,
                ));
            }
            Event::Wheel(w) => {
                if w.wheel_delta.y > 0.0 {
                    self.scale /= 2.0;
                } else {
                    self.scale *= 2.0;
                }
                ctx.submit_command(druid::Command::new(
                    SET_SCALE_ACTION,
                    self.scale,
                    druid::Target::Auto,
                ));
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

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &String,
        env: &Env,
    ) -> Size {
        let size = self.transform.layout(ctx, bc, data, env);
        self.transform.set_layout_rect(
            ctx,
            data,
            env,
            druid::Rect::from_origin_size((0.0, 0.0), size),
        );
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
        self.transform.paint(ctx, data, env);
    }
}
