use std::thread::current;

use egui::{Color32, Sense, Stroke as EguiStroke, Ui, Vec2};
use crate::document::{Stroke, StrokePoint};

pub fn view_drawing(ui: &mut Ui, width: f32, height: f32, strokes: &mut Vec<Stroke>, current_stroke: &mut Option<Stroke>){
    let desired_size = Vec2::new(width, height);
    let (rect, res) = ui.allocate_exact_size(desired_size, Sense::click_and_drag());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 0.0, Color32::from_gray(30));

    if res.drag_started() {
        *current_stroke = Some(Stroke::new((255,255,255,255), 2.0));
    }

    if res.dragged() {
        if let Some(pos) = res.interact_pointer_pos(){
            if let Some(stroke) = current_stroke.as_mut(){
                let local = pos - rect.min;
                stroke.points.push(StrokePoint{x: local.x, y: local.y, pressure: 7.0});
            }
        }
    }

    if res.drag_stopped() {
        if let Some(stroke) = current_stroke.take(){
            strokes.push(stroke);
        }
    }

    for stroke in strokes.iter() {
        let color = Color32::from_rgba_unmultiplied(
            stroke.color.0, stroke.color.1, stroke.color.2, stroke.color.3,
        );
        let egui_stroke = EguiStroke::new(stroke.width, color);
        for pair in stroke.points.windows(2) {
            let p0 = rect.min + Vec2::new(pair[0].x, pair[0].y);
            let p1 = rect.min + Vec2::new(pair[1].x, pair[1].y);
            painter.line_segment([p0, p1], egui_stroke);
        }
    }

    for stroke in current_stroke.iter(){
        let color = Color32::from_rgba_unmultiplied(
            stroke.color.0, stroke.color.1, stroke.color.2, stroke.color.3,
        );
        let egui_stroke = EguiStroke::new(stroke.width, color);
        for pair in stroke.points.windows(2) {
            let p0 = rect.min + Vec2::new(pair[0].x, pair[0].y);
            let p1 = rect.min + Vec2::new(pair[1].x, pair[1].y);
            painter.line_segment([p0, p1], egui_stroke);
        }
    }
}