use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui_dock::{DockArea, Style};
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

use crate::dock::*;
use crate::widgets;

#[derive(Resource, Default)]
pub struct ViewportRect {
    pub rect: Option<egui::Rect>,
}

// Mode data structure matching the reference project
#[derive(Clone)]
pub struct ModeData {
    pub name: String,
    pub color: egui::Color32,
    // Parent settings
    pub split_angle_pitch: f32,
    pub split_angle_yaw: f32,
    pub split_mass: f32,
    pub split_interval: f32,
    pub nutrient_priority: f32,
    pub prioritize_when_low: bool,
    pub max_connections: f32,
    pub min_connections: f32,
    pub max_splits: f32,
    // Child A settings
    pub child_a_mode: usize,
    pub child_a_orientation: Quat,
    pub child_a_keep_adhesion: bool,
    pub child_a_x_lat: f32,
    pub child_a_x_lon: f32,
    pub child_a_y_lat: f32,
    pub child_a_y_lon: f32,
    pub child_a_z_lat: f32,
    pub child_a_z_lon: f32,
    // Child B settings
    pub child_b_mode: usize,
    pub child_b_orientation: Quat,
    pub child_b_keep_adhesion: bool,
    pub child_b_x_lat: f32,
    pub child_b_x_lon: f32,
    pub child_b_y_lat: f32,
    pub child_b_y_lon: f32,
    pub child_b_z_lat: f32,
    pub child_b_z_lon: f32,
    // Adhesion settings
    pub adhesion_can_break: bool,
    pub adhesion_break_force: f32,
    pub adhesion_rest_length: f32,
    pub linear_spring_stiffness: f32,
    pub linear_spring_damping: f32,
    pub angular_spring_stiffness: f32,
    pub angular_spring_damping: f32,
    pub max_angular_deviation: f32,
    pub enable_twist_constraint: bool,
    pub twist_constraint_stiffness: f32,
    pub twist_constraint_damping: f32,
    // Cell type
    pub cell_type: usize,
    pub make_adhesion: bool,
}

impl Default for ModeData {
    fn default() -> Self {
        Self {
            name: "M 0".to_string(),
            color: egui::Color32::from_rgb(255, 100, 100),
            split_angle_pitch: 0.0,
            split_angle_yaw: 0.0,
            split_mass: 1.5,
            split_interval: 5.0,
            nutrient_priority: 1.0,
            prioritize_when_low: true,
            max_connections: 20.0,
            min_connections: 0.0,
            max_splits: -1.0,
            child_a_mode: 0,
            child_a_orientation: Quat::IDENTITY,
            child_a_keep_adhesion: false,
            child_a_x_lat: 0.0,
            child_a_x_lon: 0.0,
            child_a_y_lat: 0.0,
            child_a_y_lon: 0.0,
            child_a_z_lat: 0.0,
            child_a_z_lon: 0.0,
            child_b_mode: 0,
            child_b_orientation: Quat::IDENTITY,
            child_b_keep_adhesion: false,
            child_b_x_lat: 0.0,
            child_b_x_lon: 0.0,
            child_b_y_lat: 0.0,
            child_b_y_lon: 0.0,
            child_b_z_lat: 0.0,
            child_b_z_lon: 0.0,
            adhesion_can_break: true,
            adhesion_break_force: 10.0,
            adhesion_rest_length: 2.0,
            linear_spring_stiffness: 50.0,
            linear_spring_damping: 1.0,
            angular_spring_stiffness: 10.0,
            angular_spring_damping: 1.0,
            max_angular_deviation: 0.0,
            enable_twist_constraint: false,
            twist_constraint_stiffness: 0.5,
            twist_constraint_damping: 1.0,
            cell_type: 0,
            make_adhesion: false,
        }
    }
}

#[derive(Resource)]
pub struct WidgetDemoState {
    // Genome-level state
    pub genome_name: String,
    pub modes_data: Vec<ModeData>,
    pub selected_mode: usize,
    pub initial_mode: usize,
    // UI state for modes panel
    pub renaming_mode: Option<usize>,
    pub rename_buffer: String,
    pub copy_into_dialog_open: bool,
    pub copy_into_source: usize,
    pub color_picker_state: Option<(usize, egui::ecolor::Hsva)>,
    // UI state for quaternion balls
    pub qball_snapping: bool,
    pub qball1_locked_axis: i32,
    pub qball1_initial_distance: f32,
    pub qball2_locked_axis: i32,
    pub qball2_initial_distance: f32,
    // UI state for circular sliders
    pub enable_snapping: bool,
    // Time slider
    pub time_value: f32,
}

impl Default for WidgetDemoState {
    fn default() -> Self {
        let mut modes = Vec::new();
        let colors = vec![
            egui::Color32::from_rgb(255, 100, 100),
            egui::Color32::from_rgb(100, 255, 100),
            egui::Color32::from_rgb(100, 100, 255),
            egui::Color32::from_rgb(255, 255, 100),
            egui::Color32::from_rgb(255, 100, 255),
            egui::Color32::from_rgb(100, 255, 255),
            egui::Color32::from_rgb(200, 150, 100),
            egui::Color32::from_rgb(150, 100, 200),
        ];
        
        for (i, color) in colors.iter().enumerate() {
            let mut mode = ModeData::default();
            mode.name = format!("M {}", i);
            mode.color = *color;
            mode.child_a_mode = i;
            mode.child_b_mode = i;
            modes.push(mode);
        }
        
        Self {
            genome_name: String::new(),
            modes_data: modes,
            selected_mode: 0,
            initial_mode: 0,
            renaming_mode: None,
            rename_buffer: String::new(),
            copy_into_dialog_open: false,
            copy_into_source: 0,
            color_picker_state: None,
            qball_snapping: true,
            qball1_locked_axis: -1,
            qball1_initial_distance: 0.0,
            qball2_locked_axis: -1,
            qball2_initial_distance: 0.0,
            enable_snapping: true,
            time_value: 0.0,
        }
    }
}

pub fn ui_system(
    mut contexts: Query<&mut EguiContext>,
    mut dock_resource: ResMut<DockResource>,
    mut viewport_rect: ResMut<ViewportRect>,
    mut widget_demo_state: ResMut<WidgetDemoState>,
) {
    for mut egui_context in contexts.iter_mut() {
        let ctx = egui_context.get_mut();

        // Configure scroll style to use solid scrollbars that don't overlap content
        ctx.style_mut(|style| {
            style.spacing.scroll = egui::style::ScrollStyle::solid();
            style.spacing.scroll.bar_outer_margin = 0.0;  // Remove dead space to right of scrollbar
            style.spacing.scroll.bar_inner_margin = 0.0;  // Content sits close to scrollbar
            style.spacing.scroll.floating_allocated_width = 0.0;  // No allocated space for floating bars
        });

        // Clear viewport rect at the start of each frame
        viewport_rect.rect = None;

        // Show menu bar at the top
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Windows", |ui| {
                    show_windows_menu(ui, &mut dock_resource);
                });
            });
        });

        // Show dock area in remaining space (only if not hidden)
        if !dock_resource.all_hidden {
            let mut style = Style::from_egui(ctx.style().as_ref());
            // Reduce separator minimum constraint to allow smaller panels
            style.separator.extra = 75.0; // Reduced from default 175.0
            
            DockArea::new(&mut dock_resource.tree)
                .style(style)
                .show_leaf_collapse_buttons(false)
                .show_leaf_close_all_buttons(false)
                .show(ctx, &mut TabViewer {
                    viewport_rect: &mut viewport_rect,
                    widget_demo_state: &mut widget_demo_state,
                });
        } else {
            // When hidden, set viewport to entire available screen area
            viewport_rect.rect = Some(ctx.available_rect());
        }
    }
}

struct TabViewer<'a> {
    viewport_rect: &'a mut ViewportRect,
    widget_demo_state: &'a mut WidgetDemoState,
}

impl<'a> egui_dock::TabViewer for TabViewer<'a> {
    type Tab = Panel;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.to_string().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab {
            Panel::Viewport => {
                // Capture the viewport rect for mouse interaction
                let rect = ui.available_rect_before_wrap();
                self.viewport_rect.rect = Some(rect);
                
                // Don't draw anything else - let the 3D scene show through
            }
            // Placeholder panels are empty - they just hold space for other tabs
            Panel::LeftPanel | Panel::RightPanel | Panel::BottomPanel => {
                // No content - empty placeholder
            }
            Panel::Inspector => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Object properties and settings");
                    ui.add_space(10.0);
                    ui.label("Selected: None");
                });
            }
            Panel::Console => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Application logs and messages");
                    ui.add_space(10.0);
                    ui.monospace("[INFO] Application started");
                });
            }
            Panel::Hierarchy => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Scene object tree");
                    ui.add_space(10.0);
                    ui.label("🌍 Scene");
                    ui.label("  └─ 📦 Entity");
                });
            }
            Panel::Assets => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Project assets and resources");
                    ui.add_space(10.0);
                    ui.label("📂 Textures");
                    ui.label("📂 Models");
                    ui.label("📂 Scripts");
                });
            }
            Panel::CircleSliders => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.checkbox(&mut self.widget_demo_state.enable_snapping, "Enable Snapping (11.25°)");
                    ui.add_space(10.0);
                    
                    // Get current mode
                    let mode = &mut self.widget_demo_state.modes_data[self.widget_demo_state.selected_mode];
                    
                    // Calculate available space and size for two sliders
                    let _available_width = ui.available_width();
                    let radius = 29.25;
                    
                    // Always side by side
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        
                        ui.vertical(|ui| {
                            ui.label("Pitch:");
                            widgets::circular_slider_float(
                                ui,
                                &mut mode.split_angle_pitch,
                                -180.0,
                                180.0,
                                radius,
                                self.widget_demo_state.enable_snapping,
                            );
                        });
                        
                        ui.vertical(|ui| {
                            ui.label("Yaw:");
                            widgets::circular_slider_float(
                                ui,
                                &mut mode.split_angle_yaw,
                                -180.0,
                                180.0,
                                radius,
                                self.widget_demo_state.enable_snapping,
                            );
                        });
                    });
                });
            }
            Panel::QuaternionBall => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.checkbox(&mut self.widget_demo_state.qball_snapping, "Enable Snapping (11.25°)");
                    ui.add_space(10.0);
                    
                    // Calculate responsive ball size
                    let available_width = ui.available_width();
                    let available_height = ui.available_height();
                    
                    // Reserve space for checkbox and coordinates
                    let reserved_height = 120.0;
                    let usable_height = available_height - reserved_height;
                    
                    // Always side by side
                    let ball_width = (available_width / 2.0) - 40.0;
                    let max_size = ball_width.min(usable_height);
                    let ball_radius = (max_size / 2.5).max(32.5).min(130.0);
                    
                    let ball_container_width = ball_radius * 2.0 + 20.0;
                    
                    // Get current mode index
                    let selected_mode_idx = self.widget_demo_state.selected_mode;
                    
                    // Collect mode display data before mutable borrows
                    let mode_display_data: Vec<(String, egui::Color32)> = self.widget_demo_state.modes_data.iter()
                        .map(|m| (m.name.clone(), m.color))
                        .collect();
                    
                    // Display balls horizontally with coordinates directly below each ball
                    ui.horizontal_top(|ui| {
                        ui.add_space(10.0);
                        
                        // Ball 1 (Child A) with mode dropdown below
                        ui.allocate_ui_with_layout(
                            egui::vec2(ball_container_width, 0.0),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.label("Child A");
                                
                                let mode = &mut self.widget_demo_state.modes_data[selected_mode_idx];
                                
                                widgets::quaternion_ball(
                                    ui,
                                    &mut mode.child_a_orientation,
                                    &mut mode.child_a_x_lat,
                                    &mut mode.child_a_x_lon,
                                    &mut mode.child_a_y_lat,
                                    &mut mode.child_a_y_lon,
                                    &mut mode.child_a_z_lat,
                                    &mut mode.child_a_z_lon,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball1_locked_axis,
                                    &mut self.widget_demo_state.qball1_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 1
                                ui.checkbox(&mut mode.child_a_keep_adhesion, "Keep Adhesion");
                                
                                // Mode label and dropdown for ball 1
                                ui.label("Mode:");
                                let child_a_mode_idx = mode.child_a_mode;
                                let mode_color = mode_display_data[child_a_mode_idx].1;
                                let brightness = mode_color.r() as f32 * 0.299 + mode_color.g() as f32 * 0.587 + mode_color.b() as f32 * 0.114;
                                let text_color = if brightness > 127.5 {
                                    egui::Color32::BLACK
                                } else {
                                    egui::Color32::WHITE
                                };
                                egui::ComboBox::from_id_salt("qball1_mode")
                                    .selected_text(
                                        egui::RichText::new(&mode_display_data[child_a_mode_idx].0)
                                            .color(text_color)
                                            .background_color(mode_color)
                                    )
                                    .width(ball_container_width - 20.0)
                                    .show_ui(ui, |ui| {
                                        for (i, (mode_name, mode_color)) in mode_display_data.iter().enumerate() {
                                            // Calculate brightness to determine text color
                                            let brightness = mode_color.r() as f32 * 0.299 + mode_color.g() as f32 * 0.587 + mode_color.b() as f32 * 0.114;
                                            let text_color = if brightness > 127.5 {
                                                egui::Color32::BLACK
                                            } else {
                                                egui::Color32::WHITE
                                            };

                                            let _response = ui.selectable_value(
                                                &mut mode.child_a_mode,
                                                i,
                                                egui::RichText::new(mode_name).color(text_color).background_color(*mode_color)
                                            );
                                        }
                                    });
                            }
                        );
                        
                        // Ball 2 (Child B) with mode dropdown below
                        ui.allocate_ui_with_layout(
                            egui::vec2(ball_container_width, 0.0),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                ui.label("Child B");
                                
                                let mode = &mut self.widget_demo_state.modes_data[selected_mode_idx];
                                
                                widgets::quaternion_ball(
                                    ui,
                                    &mut mode.child_b_orientation,
                                    &mut mode.child_b_x_lat,
                                    &mut mode.child_b_x_lon,
                                    &mut mode.child_b_y_lat,
                                    &mut mode.child_b_y_lon,
                                    &mut mode.child_b_z_lat,
                                    &mut mode.child_b_z_lon,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball2_locked_axis,
                                    &mut self.widget_demo_state.qball2_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 2
                                ui.checkbox(&mut mode.child_b_keep_adhesion, "Keep Adhesion");
                                
                                // Mode label and dropdown for ball 2
                                ui.label("Mode:");
                                let child_b_mode_idx = mode.child_b_mode;
                                let mode_color = mode_display_data[child_b_mode_idx].1;
                                let brightness = mode_color.r() as f32 * 0.299 + mode_color.g() as f32 * 0.587 + mode_color.b() as f32 * 0.114;
                                let text_color = if brightness > 127.5 {
                                    egui::Color32::BLACK
                                } else {
                                    egui::Color32::WHITE
                                };
                                egui::ComboBox::from_id_salt("qball2_mode")
                                    .selected_text(
                                        egui::RichText::new(&mode_display_data[child_b_mode_idx].0)
                                            .color(text_color)
                                            .background_color(mode_color)
                                    )
                                    .width(ball_container_width - 20.0)
                                    .show_ui(ui, |ui| {
                                        for (i, (mode_name, mode_color)) in mode_display_data.iter().enumerate() {
                                            // Calculate brightness to determine text color
                                            let brightness = mode_color.r() as f32 * 0.299 + mode_color.g() as f32 * 0.587 + mode_color.b() as f32 * 0.114;
                                            let text_color = if brightness > 127.5 {
                                                egui::Color32::BLACK
                                            } else {
                                                egui::Color32::WHITE
                                            };

                                            let _response = ui.selectable_value(
                                                &mut mode.child_b_mode,
                                                i,
                                                egui::RichText::new(mode_name).color(text_color).background_color(*mode_color)
                                            );
                                        }
                                    });
                            }
                        );
                    });
                });
            }
            Panel::Modes => {
                render_modes_panel(ui, self.widget_demo_state);
            }
            Panel::NameTypeEditor => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    
                    // Three buttons at the top
                    ui.horizontal(|ui| {
                        if ui.button("Save Genome").clicked() {
                            // Open save dialog
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON", &["json"])
                                .set_file_name(&format!("{}.json", self.widget_demo_state.genome_name))
                                .save_file()
                            {
                                info!("Would save genome to: {:?}", path);
                                // TODO: Implement actual save
                            }
                        }
                        if ui.button("Load Genome").clicked() {
                            // Open load dialog
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("JSON", &["json"])
                                .pick_file()
                            {
                                info!("Would load genome from: {:?}", path);
                                // TODO: Implement actual load
                            }
                        }
                        if ui.button("Genome Graph").clicked() {
                            // TODO: Implement genome graph
                        }
                    });
                    
                    ui.add_space(4.0);
                    
                    // Genome Name label and field on same line
                    ui.horizontal(|ui| {
                        ui.label("Genome Name:");
                        ui.text_edit_singleline(&mut self.widget_demo_state.genome_name);
                    });
                    
                    ui.add_space(4.0);
                    
                    // Get current mode
                    let mode = &mut self.widget_demo_state.modes_data[self.widget_demo_state.selected_mode];
                    
                    // Type dropdown and checkbox on the same line
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        let cell_types = ["Photocyte", "Phagocyte", "Flagellocyte", "Devorocyte", "Lipocyte"];
                        egui::ComboBox::from_id_salt("cell_type")
                            .selected_text(cell_types[mode.cell_type])
                            .show_ui(ui, |ui| {
                                for (i, type_name) in cell_types.iter().enumerate() {
                                    ui.selectable_value(&mut mode.cell_type, i, *type_name);
                                }
                            });
                        
                        ui.checkbox(&mut mode.make_adhesion, "Make Adhesion");
                    });
                });
            }
            Panel::AdhesionSettings => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    // Force content to fill available width
                    ui.set_width(ui.available_width());
                    ui.add_space(10.0);
                    
                    // Get current mode
                    let mode = &mut self.widget_demo_state.modes_data[self.widget_demo_state.selected_mode];
                    
                    // Adhesion Can Break checkbox
                    ui.checkbox(&mut mode.adhesion_can_break, "Adhesion Can Break");
                    
                    // Adhesion Break Force (0.1 to 100.0)
                    ui.label("Adhesion Break Force:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_break_force, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_break_force).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Adhesion Rest Length (0.5 to 5.0)
                    ui.label("Adhesion Rest Length:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_rest_length, 0.5..=5.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_rest_length).speed(0.01).range(0.5..=5.0));
                    });
                    
                    // Linear Spring Stiffness (0.1 to 500.0)
                    ui.label("Linear Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.linear_spring_stiffness, 0.1..=500.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.linear_spring_stiffness).speed(0.1).range(0.1..=500.0));
                    });
                    
                    // Linear Spring Damping (0.0 to 10.0)
                    ui.label("Linear Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.linear_spring_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.linear_spring_damping).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Angular Spring Stiffness (0.1 to 100.0)
                    ui.label("Angular Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.angular_spring_stiffness, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.angular_spring_stiffness).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Angular Spring Damping (0.0 to 10.0)
                    ui.label("Angular Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.angular_spring_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.angular_spring_damping).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Max Angular Deviation (0.0 to 180.0)
                    ui.label("Max Angular Deviation:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.max_angular_deviation, 0.0..=180.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.max_angular_deviation).speed(0.1).range(0.0..=180.0));
                    });
                    
                    ui.add_space(10.0);
                    
                    // Enable Twist Constraint checkbox
                    ui.checkbox(&mut mode.enable_twist_constraint, "Enable Twist Constraint");
                    
                    // Twist Constraint Stiffness (0.0 to 2.0)
                    ui.label("Twist Constraint Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.twist_constraint_stiffness, 0.0..=2.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.twist_constraint_stiffness).speed(0.01).range(0.0..=2.0));
                    });
                    
                    // Twist Constraint Damping (0.0 to 10.0)
                    ui.label("Twist Constraint Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.twist_constraint_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.twist_constraint_damping).speed(0.01).range(0.0..=10.0));
                    });
                });
            }
            Panel::ParentSettings => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    // Force content to fill available width
                    ui.set_width(ui.available_width());
                    ui.add_space(10.0);
                    
                    // Get current mode
                    let mode = &mut self.widget_demo_state.modes_data[self.widget_demo_state.selected_mode];
                    
                    // Split Mass (1.0 to 3.0)
                    ui.label("Split Mass:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.split_mass, 1.0..=3.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.split_mass).speed(0.01).range(1.0..=3.0));
                    });
                    
                    // Split Interval (1.0 to 60.0 seconds)
                    ui.label("Split Interval:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.split_interval, 1.0..=60.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.split_interval).speed(0.1).range(1.0..=60.0).suffix("s"));
                    });
                    
                    // Nutrient Priority (0.1 to 10.0)
                    ui.label("Nutrient Priority:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.nutrient_priority, 0.1..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.nutrient_priority).speed(0.01).range(0.1..=10.0));
                    });
                    
                    // Prioritize When Low checkbox
                    ui.checkbox(&mut mode.prioritize_when_low, "Prioritize When Low");
                    
                    ui.add_space(10.0);
                    
                    // Max Connections (0 to 20)
                    ui.label("Max Connections:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.max_connections, 0.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.max_connections).speed(0.1).range(0.0..=20.0));
                    });
                    
                    // Min Connections (0 to 20)
                    ui.label("Min Connections:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.min_connections, 0.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.min_connections).speed(0.1).range(0.0..=20.0));
                    });
                    
                    // Max Splits (-1 to 20, where -1 = infinite)
                    ui.label("Max Splits:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.max_splits, -1.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.max_splits).speed(0.1).range(-1.0..=20.0));
                    });
                });
            }
            Panel::TimeSlider => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Time:");
                        
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.time_value, 0.0..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.time_value).speed(0.1).range(0.0..=100.0));
                    });
                });
            }
        }
    }

    fn is_viewport(&self, _tab: &Self::Tab) -> bool {
        // Don't use built-in viewport mode
        false
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        // Return false for viewport to skip drawing background (make it transparent)
        // Return true for other panels to draw the background
        !matches!(tab, Panel::Viewport)
    }

    fn is_draggable(&self, _tab: &Self::Tab) -> bool {
        // Only placeholder panels cannot be dragged
        !_tab.is_placeholder()
    }
    
    fn hide_tab_button(&self, _tab: &Self::Tab) -> bool {
        // Hide tab buttons only for placeholder panels
        _tab.is_placeholder()
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        // Only non-placeholder panels can be closed
        !tab.is_placeholder()
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        // Only non-placeholder panels can be ejected to floating windows
        !tab.is_placeholder()
    }
    
    fn min_fraction(&self, tab: &Self::Tab) -> Option<f32> {
        // Bottom, left, and right panels should have the same minimum size (about 32px on a 720p window)
        match tab {
            Panel::BottomPanel | Panel::LeftPanel | Panel::RightPanel => Some(0.045),
            Panel::Modes => None, // Disable minimum to debug the actual issue
            _ => None,
        }
    }
}

fn render_modes_panel(ui: &mut egui::Ui, widget_demo_state: &mut WidgetDemoState) {
    // Handle rename dialog (outside scroll area)
    let mut rename_confirmed = false;
    let mut rename_cancelled = false;

    if let Some(_rename_idx) = widget_demo_state.renaming_mode {
        egui::Window::new("Rename Mode")
            .collapsible(false)
            .resizable(false)
            .show(ui.ctx(), |ui| {
                ui.label("Mode Name:");
                let response = ui.text_edit_singleline(&mut widget_demo_state.rename_buffer);

                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    rename_confirmed = true;
                }

                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        rename_confirmed = true;
                    }
                    if ui.button("Cancel").clicked() {
                        rename_cancelled = true;
                    }
                });

                // Auto-focus the text field
                if !response.has_focus() {
                    response.request_focus();
                }
            });
    }

    if rename_confirmed {
        if let Some(_rename_idx) = widget_demo_state.renaming_mode {
            let trimmed = widget_demo_state.rename_buffer.trim();
            if !trimmed.is_empty() && _rename_idx < widget_demo_state.modes_data.len() {
                widget_demo_state.modes_data[_rename_idx].name = trimmed.to_string();
                info!("Renamed mode {} to {}", _rename_idx, trimmed);
            }
        }
        widget_demo_state.renaming_mode = None;
        widget_demo_state.rename_buffer.clear();
    }

    if rename_cancelled {
        widget_demo_state.renaming_mode = None;
        widget_demo_state.rename_buffer.clear();
    }

    // Draw buttons outside scroll area
    let (add_clicked, remove_clicked, copy_clicked, copy_into_clicked, reset_clicked) = widgets::modes_buttons(
        ui,
        widget_demo_state.modes_data.len(),
        widget_demo_state.selected_mode,
        widget_demo_state.initial_mode,
    );

    ui.separator();

    // Show instruction text if in copy into mode (also outside scroll area)
    if widget_demo_state.copy_into_dialog_open {
        ui.colored_label(egui::Color32::YELLOW, "Select target mode to copy into:");
        ui.add_space(5.0);
    }

    // Convert modes_data to the format expected by modes_list_items
    let modes_display: Vec<(String, egui::Color32)> = widget_demo_state.modes_data.iter()
        .map(|m| (m.name.clone(), m.color))
        .collect();

    // Now create scroll area for the list
    let (selection_changed, initial_changed, rename_idx, color_change) = egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
        let available_width = ui.available_width();

        widgets::modes_list_items(
            ui,
            &modes_display,
            &mut widget_demo_state.selected_mode,
            &mut widget_demo_state.initial_mode,
            available_width,
            widget_demo_state.copy_into_dialog_open,
            &mut widget_demo_state.color_picker_state,
        )
    }).inner;

    if selection_changed {
        // If in copy into mode, this is the target selection
        if widget_demo_state.copy_into_dialog_open {
            let source_idx = widget_demo_state.copy_into_source;
            let target_idx = widget_demo_state.selected_mode;

            if source_idx != target_idx && source_idx < widget_demo_state.modes_data.len()
                && target_idx < widget_demo_state.modes_data.len() {
                // Copy all settings from source to target (except name)
                let source_mode = widget_demo_state.modes_data[source_idx].clone();
                let target_name = widget_demo_state.modes_data[target_idx].name.clone();
                widget_demo_state.modes_data[target_idx] = source_mode;
                widget_demo_state.modes_data[target_idx].name = target_name;
                info!("Copied mode {} into mode {}", source_idx, target_idx);
            }

            // Exit copy into mode
            widget_demo_state.copy_into_dialog_open = false;
        } else {
            info!("Selected mode changed to: {}", widget_demo_state.selected_mode);
        }
    }
    if initial_changed {
        info!("Initial mode changed to: {}", widget_demo_state.initial_mode);
    }

    // Handle rename request
    if let Some(idx) = rename_idx {
        widget_demo_state.renaming_mode = Some(idx);
        widget_demo_state.rename_buffer = widget_demo_state.modes_data[idx].name.clone();
    }

    // Handle color change from context menu color picker
    if let Some((idx, new_color)) = color_change {
        if idx < widget_demo_state.modes_data.len() {
            widget_demo_state.modes_data[idx].color = new_color;
            info!("Changed color of mode {}", idx);
        }
    }

    // Handle copy mode
    if copy_clicked {
        let selected_idx = widget_demo_state.selected_mode;
        if selected_idx < widget_demo_state.modes_data.len() {
            let insert_idx = selected_idx + 1;

            // Copy the selected mode
            let mut new_mode = widget_demo_state.modes_data[selected_idx].clone();

            // Generate new name
            let existing_names: Vec<String> = widget_demo_state.modes_data.iter()
                .map(|m| m.name.clone())
                .collect();
            new_mode.name = widgets::generate_next_mode_name(&new_mode.name, &existing_names);

            widget_demo_state.modes_data.insert(insert_idx, new_mode);

            // Adjust selection to the new copy
            widget_demo_state.selected_mode = insert_idx;

            // Adjust initial mode if needed
            if insert_idx <= widget_demo_state.initial_mode {
                widget_demo_state.initial_mode += 1;
            }

            info!("Copied mode at index {}", selected_idx);
        }
    }

    // Handle copy into mode
    if copy_into_clicked {
        let selected_idx = widget_demo_state.selected_mode;
        if selected_idx < widget_demo_state.modes_data.len() {
            // Enter copy into mode - user will click on target mode directly
            widget_demo_state.copy_into_dialog_open = true;
            widget_demo_state.copy_into_source = selected_idx;
        }
    }

    // Handle reset mode
    if reset_clicked {
        let selected_idx = widget_demo_state.selected_mode;
        if selected_idx < widget_demo_state.modes_data.len() {
            // Reset to default values
            let name = widget_demo_state.modes_data[selected_idx].name.clone();
            let color = widget_demo_state.modes_data[selected_idx].color;
            widget_demo_state.modes_data[selected_idx] = ModeData::default();
            widget_demo_state.modes_data[selected_idx].name = name;
            widget_demo_state.modes_data[selected_idx].color = color;
            widget_demo_state.modes_data[selected_idx].child_a_mode = selected_idx;
            widget_demo_state.modes_data[selected_idx].child_b_mode = selected_idx;
            info!("Reset mode {}", selected_idx);
        }
    }

    // Don't handle add/remove/copy when in copy into mode
    if widget_demo_state.copy_into_dialog_open {
        return;
    }

    // Handle add mode
    if add_clicked {
        let selected_idx = widget_demo_state.selected_mode;
        let insert_idx = if selected_idx < widget_demo_state.modes_data.len() {
            selected_idx + 1
        } else {
            widget_demo_state.modes_data.len()
        };

        // Generate new mode name
        let existing_names: Vec<String> = widget_demo_state.modes_data.iter()
            .map(|m| m.name.clone())
            .collect();
        let base_name = if selected_idx < widget_demo_state.modes_data.len() {
            &widget_demo_state.modes_data[selected_idx].name
        } else {
            "M 0"
        };
        let new_name = widgets::generate_next_mode_name(base_name, &existing_names);

        // Generate random color
        let mut hasher = RandomState::new().build_hasher();
        insert_idx.hash(&mut hasher);
        let hash = hasher.finish();
        let new_color = egui::Color32::from_rgb(
            ((hash % 156) + 100) as u8,
            (((hash >> 8) % 156) + 100) as u8,
            (((hash >> 16) % 156) + 100) as u8,
        );

        let mut new_mode = ModeData::default();
        new_mode.name = new_name;
        new_mode.color = new_color;
        new_mode.child_a_mode = insert_idx;
        new_mode.child_b_mode = insert_idx;

        widget_demo_state.modes_data.insert(insert_idx, new_mode);

        // Adjust selection if needed
        if insert_idx <= selected_idx {
            widget_demo_state.selected_mode = selected_idx + 1;
        }

        // Adjust initial mode if needed
        if insert_idx <= widget_demo_state.initial_mode {
            widget_demo_state.initial_mode += 1;
        }

        info!("Added new mode at index {}", insert_idx);
    }

    // Handle remove mode
    if remove_clicked {
        let selected = widget_demo_state.selected_mode;
        if widget_demo_state.modes_data.len() > 1
            && selected < widget_demo_state.modes_data.len()
            && selected != widget_demo_state.initial_mode {

            widget_demo_state.modes_data.remove(selected);

            // Adjust selected index
            if widget_demo_state.selected_mode >= widget_demo_state.modes_data.len() {
                widget_demo_state.selected_mode = widget_demo_state.modes_data.len() - 1;
            }

            // Adjust initial mode if needed
            if widget_demo_state.initial_mode > selected {
                widget_demo_state.initial_mode -= 1;
            }

            info!("Removed mode at index {}", selected);
        }
    }
}
