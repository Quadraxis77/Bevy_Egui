use bevy::prelude::*;
use bevy_egui::egui::{self, Ui, Response, Sense, Stroke, Pos2, Vec2 as EguiVec2};
use std::f32::consts::PI;

/// Circular slider for float values with angle snapping
/// 
/// Returns true if the value changed
pub fn circular_slider_float(
    ui: &mut Ui,
    value: &mut f32,
    v_min: f32,
    v_max: f32,
    radius: f32,
    enable_snapping: bool,
) -> Response {
    // Calculate container size based on radius
    let container_width = radius * 2.0 + 20.0;
    let container_height = radius * 2.0 + 20.0;
    
    let (rect, mut response) = ui.allocate_exact_size(
        EguiVec2::new(container_width, container_height),
        Sense::click_and_drag(),
    );
    
    let center = Pos2::new(
        rect.left() + container_width / 2.0,
        rect.top() + container_height / 2.0,
    );
    
    // Get colors from theme
    let bg_color = ui.visuals().widgets.inactive.bg_fill;
    let slider_color = ui.visuals().selection.bg_fill;
    let slider_hovered_color = ui.visuals().widgets.hovered.bg_fill;
    
    // Check mouse position for grab zone
    let mouse_pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(Pos2::ZERO);
    let distance_from_center = (mouse_pos - center).length();
    
    // Define grab zones
    let inner_radius = 15.0;
    let outer_radius = radius + 25.0;
    let is_mouse_in_grab_zone = distance_from_center >= inner_radius
        && distance_from_center <= outer_radius
        && response.hovered();
    
    // Draw background circle
    let current_slider_color = if is_mouse_in_grab_zone {
        slider_hovered_color
    } else {
        bg_color
    };
    
    ui.painter().circle_stroke(
        center,
        radius,
        Stroke::new(3.0, current_slider_color),
    );
    
    // Draw directional arc
    if value.abs() > 0.001 {
        let arc_thickness = 8.0;
        let num_segments = (radius * 0.5).max(32.0) as usize;
        let current_arc_color = if is_mouse_in_grab_zone {
            slider_hovered_color
        } else {
            slider_color
        };
        
        let start_angle = -PI / 2.0;
        let end_angle = start_angle + (*value / 180.0) * PI;
        
        for i in 0..num_segments {
            let angle1 = start_angle + (end_angle - start_angle) * i as f32 / num_segments as f32;
            let angle2 = start_angle + (end_angle - start_angle) * (i + 1) as f32 / num_segments as f32;
            
            let point1 = Pos2::new(
                center.x + angle1.cos() * radius,
                center.y + angle1.sin() * radius,
            );
            let point2 = Pos2::new(
                center.x + angle2.cos() * radius,
                center.y + angle2.sin() * radius,
            );
            
            ui.painter().line_segment(
                [point1, point2],
                Stroke::new(arc_thickness, current_arc_color),
            );
        }
    }
    
    // Draw handle
    let handle_radius = 6.0;
    let handle_angle = -PI / 2.0 + (*value / 180.0) * PI;
    let handle_pos = Pos2::new(
        center.x + handle_angle.cos() * radius,
        center.y + handle_angle.sin() * radius,
    );
    let handle_color = if is_mouse_in_grab_zone {
        slider_hovered_color
    } else {
        slider_color
    };
    
    ui.painter().circle_filled(handle_pos, handle_radius, handle_color);
    
    // Handle mouse interaction
    if response.dragged() {
        let mouse_rel_x = mouse_pos.x - center.x;
        let mouse_rel_y = mouse_pos.y - center.y;
        let mouse_angle = mouse_rel_y.atan2(mouse_rel_x) + PI / 2.0;
        
        let mut degrees = mouse_angle * 180.0 / PI;
        if degrees > 180.0 {
            degrees -= 360.0;
        }
        if enable_snapping {
            degrees = (degrees / 11.25).round() * 11.25;
        }
        
        let new_value = degrees.clamp(v_min, v_max);
        if (new_value - *value).abs() > 0.001 {
            *value = new_value;
            response.mark_changed();
        }
    }
    
    // Draw text input in the center of the circle
    let text_input_width = 45.0;
    let text_input_height = 20.0;
    let text_input_pos = Pos2::new(
        center.x - text_input_width / 2.0,
        center.y - text_input_height / 2.0,
    );
    let text_input_rect = egui::Rect::from_min_size(
        text_input_pos,
        EguiVec2::new(text_input_width, text_input_height),
    );
    
    let child_ui = &mut ui.new_child(egui::UiBuilder::new().max_rect(text_input_rect));
    let mut text_value = format!("{:.2}", value);
    let text_response = child_ui.add(
        egui::TextEdit::singleline(&mut text_value)
            .desired_width(text_input_width)
            .horizontal_align(egui::Align::Center)
    );
    
    if text_response.lost_focus() {
        if let Ok(new_value) = text_value.parse::<f32>() {
            *value = new_value.clamp(v_min, v_max);
            response.mark_changed();
        }
    }
    
    response
}
