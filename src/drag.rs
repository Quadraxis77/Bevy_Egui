use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};
use crate::scene::DraggableSphere;
use crate::ViewportRect;

#[derive(Resource, Default)]
struct DragState {
    dragging: Option<Entity>,
    drag_offset: Vec3,
    drag_plane_distance: f32,
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>()
            .add_systems(Update, (handle_mouse_input, update_drag_position).chain());
    }
}

fn handle_mouse_input(
    mut drag_state: ResMut<DragState>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    sphere_query: Query<(Entity, &GlobalTransform), With<DraggableSphere>>,
    viewport_rect: Res<ViewportRect>,
    mut egui_context: Query<&mut EguiContext>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    // Get egui context
    let Ok(mut egui_ctx) = egui_context.single_mut() else {
        return;
    };
    let ctx = egui_ctx.get_mut();

    // Handle mouse press - start dragging
    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            // Get the pointer position directly from egui context
            // This is already in the correct egui coordinate space
            let egui_pos = ctx.pointer_latest_pos();
            
            if let Some(egui_pos) = egui_pos {
                // Check if we're in the viewport rect
                let in_viewport = viewport_rect.rect.map_or(false, |rect| rect.contains(egui_pos));
                
                // Only allow interaction if we're in the viewport (not over other UI panels)
                if !in_viewport {
                    return;
                }

                // Raycast to check if we hit the sphere
                if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    // Check if ray hits any sphere
                    for (entity, sphere_transform) in sphere_query.iter() {
                        let sphere_pos = sphere_transform.translation();
                        let sphere_radius = 0.5;

                        if let Some(distance) = ray_sphere_intersection(ray.origin, *ray.direction, sphere_pos, sphere_radius) {
                            let hit_point = ray.origin + *ray.direction * distance;
                            drag_state.dragging = Some(entity);
                            drag_state.drag_offset = sphere_pos - hit_point;
                            drag_state.drag_plane_distance = distance;
                            break;
                        }
                    }
                }
            }
        }
    }

    // Handle mouse release - stop dragging
    if mouse_button.just_released(MouseButton::Left) {
        drag_state.dragging = None;
    }
}

fn update_drag_position(
    drag_state: Res<DragState>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut sphere_query: Query<&mut Transform, With<DraggableSphere>>,
) {
    if let Some(dragged_entity) = drag_state.dragging {
        let Ok(window) = windows.single() else {
            return;
        };

        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        if let Some(cursor_position) = window.cursor_position() {
            if let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                // Project cursor onto a plane at the drag distance
                let new_position = ray.origin + *ray.direction * drag_state.drag_plane_distance;

                if let Ok(mut transform) = sphere_query.get_mut(dragged_entity) {
                    transform.translation = new_position + drag_state.drag_offset;
                    // Clamp Y position to stay above ground
                    transform.translation.y = transform.translation.y.max(0.5);
                }
            }
        }
    }
}

fn ray_sphere_intersection(
    ray_origin: Vec3,
    ray_direction: Vec3,
    sphere_center: Vec3,
    sphere_radius: f32,
) -> Option<f32> {
    let oc = ray_origin - sphere_center;
    let a = ray_direction.dot(ray_direction);
    let b = 2.0 * oc.dot(ray_direction);
    let c = oc.dot(oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None
    } else {
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t > 0.0 {
            Some(t)
        } else {
            None
        }
    }
}
