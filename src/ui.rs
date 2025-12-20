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

#[derive(Resource)]
pub struct WidgetDemoState {
    pub angle1: f32,
    pub angle2: f32,
    pub enable_snapping: bool,
    pub orientation: Quat,
    pub qball_snapping: bool,
    pub qball_locked_axis: i32,
    pub qball_initial_distance: f32,
    // Direct lat/lon coordinates for each axis endpoint (ball 1)
    pub x_axis_lat: f32,
    pub x_axis_lon: f32,
    pub y_axis_lat: f32,
    pub y_axis_lon: f32,
    pub z_axis_lat: f32,
    pub z_axis_lon: f32,
    // Second ball state
    pub orientation2: Quat,
    pub qball2_locked_axis: i32,
    pub qball2_initial_distance: f32,
    pub x_axis_lat2: f32,
    pub x_axis_lon2: f32,
    pub y_axis_lat2: f32,
    pub y_axis_lon2: f32,
    pub z_axis_lat2: f32,
    pub z_axis_lon2: f32,
    // Modes widget state
    pub modes_data: Vec<(String, egui::Color32)>,
    pub selected_mode: usize,
    pub initial_mode: usize,
    pub renaming_mode: Option<usize>,
    pub rename_buffer: String,
    pub copy_into_dialog_open: bool,
    pub copy_into_source: usize,
    pub color_picker_state: Option<(usize, egui::ecolor::Hsva)>,
    // Chemical editor state
    pub chemical_type: usize,
    pub make_adhesion: bool,
    pub genome_name: String,
    // Adhesion settings sliders
    pub adhesion1: f32,
    pub adhesion2: f32,
    pub adhesion3: f32,
    pub adhesion4: f32,
    pub adhesion5: f32,
    pub adhesion6: f32,
    pub adhesion7: f32,
    pub adhesion8: f32,
    pub adhesion9: f32,
    // Adhesion settings checkboxes
    pub adhesion_can_break: bool,
    pub enable_twist_constraint: bool,
    // Quaternion ball mode selections
    pub qball1_mode: usize,
    pub qball2_mode: usize,
    pub qball1_keep_adhesion: bool,
    pub qball2_keep_adhesion: bool,
    // Parent settings sliders
    pub parent1: f32,
    pub parent2: f32,
    pub parent3: f32,
    pub parent4: f32,
    pub parent5: f32,
    pub parent6: f32,
    // Parent settings checkboxes
    pub prioritize_when_low: bool,
    // Time slider
    pub time_value: f32,
}

impl Default for WidgetDemoState {
    fn default() -> Self {
        Self {
            angle1: 0.0,
            angle2: 45.0,
            enable_snapping: true,
            orientation: Quat::IDENTITY,
            qball_snapping: true,
            qball_locked_axis: -1,
            qball_initial_distance: 0.0,
            // All axes start at their own (0, 0) coordinates
            x_axis_lat: 0.0,
            x_axis_lon: 0.0,
            y_axis_lat: 0.0,
            y_axis_lon: 0.0,
            z_axis_lat: 0.0,
            z_axis_lon: 0.0,
            // Second ball
            orientation2: Quat::IDENTITY,
            qball2_locked_axis: -1,
            qball2_initial_distance: 0.0,
            x_axis_lat2: 0.0,
            x_axis_lon2: 0.0,
            y_axis_lat2: 0.0,
            y_axis_lon2: 0.0,
            z_axis_lat2: 0.0,
            z_axis_lon2: 0.0,
            // Modes
            modes_data: vec![
                ("M 0".to_string(), egui::Color32::from_rgb(255, 100, 100)),
                ("M 1".to_string(), egui::Color32::from_rgb(100, 255, 100)),
                ("M 2".to_string(), egui::Color32::from_rgb(100, 100, 255)),
                ("M 3".to_string(), egui::Color32::from_rgb(255, 255, 100)),
                ("M 4".to_string(), egui::Color32::from_rgb(255, 100, 255)),
                ("M 5".to_string(), egui::Color32::from_rgb(100, 255, 255)),
                ("M 6".to_string(), egui::Color32::from_rgb(200, 150, 100)),
                ("M 7".to_string(), egui::Color32::from_rgb(150, 100, 200)),
            ],
            selected_mode: 0,
            initial_mode: 0,
            renaming_mode: None,
            rename_buffer: String::new(),
            copy_into_dialog_open: false,
            copy_into_source: 0,
            color_picker_state: None,
            // Chemical editor
            chemical_type: 0,
            make_adhesion: false,
            genome_name: String::new(),
            // Adhesion settings
            adhesion1: 10.0,  // Break Force (0.1-100.0, default 10.0)
            adhesion2: 2.0,   // Rest Length (0.5-5.0, default 2.0)
            adhesion3: 50.0,  // Linear Spring Stiffness (0.1-500.0, default 50.0)
            adhesion4: 1.0,   // Linear Spring Damping (0.0-10.0, default 1.0)
            adhesion5: 10.0,  // Angular Spring Stiffness (0.1-100.0, default 10.0)
            adhesion6: 1.0,   // Angular Spring Damping (0.0-10.0, default 1.0)
            adhesion7: 0.0,   // Max Angular Deviation (0.0-180.0, default 0.0 = no limit)
            adhesion8: 0.5,   // Twist Constraint Stiffness (0.0-2.0, default 0.5)
            adhesion9: 1.0,   // Twist Constraint Damping (0.0-10.0, default 1.0)
            adhesion_can_break: true,
            enable_twist_constraint: false,
            // Quaternion ball modes
            qball1_mode: 0,
            qball2_mode: 0,
            qball1_keep_adhesion: false,
            qball2_keep_adhesion: false,
            // Parent settings
            parent1: 1.5,  // Split Mass (1.0-3.0, default 1.5)
            parent2: 5.0,  // Split Interval (1.0-60.0s, default 5.0s)
            parent3: 1.0,  // Nutrient Priority (0.1-10.0, default 1.0)
            parent4: 20.0, // Max Connections (0-20, default 20)
            parent5: 0.0,  // Min Connections (0-20, default 0)
            parent6: -1.0, // Max Splits (-1 to 20, default -1 = infinite)
            // Parent settings checkboxes
            prioritize_when_low: true,
            // Time slider
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
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.label("Object properties and settings");
                    ui.add_space(10.0);
                    ui.label("Selected: None");
                });
            }
            Panel::Console => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.label("Application logs and messages");
                    ui.add_space(10.0);
                    ui.monospace("[INFO] Application started");
                });
            }
            Panel::Hierarchy => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.label("Scene object tree");
                    ui.add_space(10.0);
                    ui.label("🌍 Scene");
                    ui.label("  └─ 📦 Entity");
                });
            }
            Panel::Assets => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.separator();
                    ui.label("Project assets and resources");
                    ui.add_space(10.0);
                    ui.label("📂 Textures");
                    ui.label("📂 Models");
                    ui.label("📂 Scripts");
                });
            }
            Panel::CircleSliders => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.checkbox(&mut self.widget_demo_state.enable_snapping, "Enable Snapping (11.25°)");
                    ui.add_space(10.0);
                    
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
                                &mut self.widget_demo_state.angle1,
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
                                &mut self.widget_demo_state.angle2,
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
                egui::ScrollArea::vertical().show(ui, |ui| {
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
                    
                    // Display balls horizontally with coordinates directly below each ball
                    ui.horizontal_top(|ui| {
                        ui.add_space(10.0);
                        
                        let ball_container_width = ball_radius * 2.0 + 20.0; // Fixed width for each ball container
                        
                        // Ball 1 with coordinates below
                        ui.allocate_ui_with_layout(
                            egui::vec2(ball_container_width, 0.0),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                widgets::quaternion_ball(
                                    ui,
                                    &mut self.widget_demo_state.orientation,
                                    &mut self.widget_demo_state.x_axis_lat,
                                    &mut self.widget_demo_state.x_axis_lon,
                                    &mut self.widget_demo_state.y_axis_lat,
                                    &mut self.widget_demo_state.y_axis_lon,
                                    &mut self.widget_demo_state.z_axis_lat,
                                    &mut self.widget_demo_state.z_axis_lon,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball_locked_axis,
                                    &mut self.widget_demo_state.qball_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 1
                                ui.checkbox(&mut self.widget_demo_state.qball1_keep_adhesion, "Keep Adhesion");
                                
                                // Mode dropdown for ball 1
                                let mode_color = self.widget_demo_state.modes_data[self.widget_demo_state.qball1_mode].1;
                                egui::ComboBox::from_id_salt("qball1_mode")
                                    .selected_text(
                                        egui::RichText::new(&self.widget_demo_state.modes_data[self.widget_demo_state.qball1_mode].0)
                                            .color(mode_color)
                                    )
                                    .width(80.0)
                                    .show_ui(ui, |ui| {
                                        for (i, (mode_name, color)) in self.widget_demo_state.modes_data.iter().enumerate() {
                                            ui.selectable_value(
                                                &mut self.widget_demo_state.qball1_mode, 
                                                i, 
                                                egui::RichText::new(mode_name).color(*color)
                                            );
                                        }
                                    });
                            }
                        );
                        
                        // Ball 2 with mode dropdown below
                        ui.allocate_ui_with_layout(
                            egui::vec2(ball_container_width, 0.0),
                            egui::Layout::top_down(egui::Align::Center),
                            |ui| {
                                widgets::quaternion_ball(
                                    ui,
                                    &mut self.widget_demo_state.orientation2,
                                    &mut self.widget_demo_state.x_axis_lat2,
                                    &mut self.widget_demo_state.x_axis_lon2,
                                    &mut self.widget_demo_state.y_axis_lat2,
                                    &mut self.widget_demo_state.y_axis_lon2,
                                    &mut self.widget_demo_state.z_axis_lat2,
                                    &mut self.widget_demo_state.z_axis_lon2,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball2_locked_axis,
                                    &mut self.widget_demo_state.qball2_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 2
                                ui.checkbox(&mut self.widget_demo_state.qball2_keep_adhesion, "Keep Adhesion");
                                
                                // Mode dropdown for ball 2
                                let mode_color = self.widget_demo_state.modes_data[self.widget_demo_state.qball2_mode].1;
                                egui::ComboBox::from_id_salt("qball2_mode")
                                    .selected_text(
                                        egui::RichText::new(&self.widget_demo_state.modes_data[self.widget_demo_state.qball2_mode].0)
                                            .color(mode_color)
                                    )
                                    .width(80.0)
                                    .show_ui(ui, |ui| {
                                        for (i, (mode_name, color)) in self.widget_demo_state.modes_data.iter().enumerate() {
                                            ui.selectable_value(
                                                &mut self.widget_demo_state.qball2_mode, 
                                                i, 
                                                egui::RichText::new(mode_name).color(*color)
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
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    
                    // Three buttons at the top
                    ui.horizontal(|ui| {
                        if ui.button("Save Genome").clicked() {
                            // TODO: Implement save genome
                        }
                        if ui.button("Load Genome").clicked() {
                            // TODO: Implement load genome
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
                    
                    // Type dropdown and checkbox on the same line
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        let cell_types = ["Photocyte", "Phagocyte", "Flagellocyte", "Devorocyte", "Lipocyte"];
                        egui::ComboBox::from_id_salt("cell_type")
                            .selected_text(cell_types[self.widget_demo_state.chemical_type])
                            .show_ui(ui, |ui| {
                                for (i, type_name) in cell_types.iter().enumerate() {
                                    ui.selectable_value(&mut self.widget_demo_state.chemical_type, i, *type_name);
                                }
                            });
                        
                        ui.checkbox(&mut self.widget_demo_state.make_adhesion, "Make Adhesion");
                    });
                });
            }
            Panel::AdhesionSettings => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(10.0);
                    
                    // Adhesion Can Break checkbox
                    ui.checkbox(&mut self.widget_demo_state.adhesion_can_break, "Adhesion Can Break");
                    
                    // Adhesion Break Force (0.1 to 100.0)
                    ui.label("Adhesion Break Force:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion1, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion1).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Adhesion Rest Length (0.5 to 5.0)
                    ui.label("Adhesion Rest Length:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion2, 0.5..=5.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion2).speed(0.01).range(0.5..=5.0));
                    });
                    
                    // Linear Spring Stiffness (0.1 to 500.0)
                    ui.label("Linear Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion3, 0.1..=500.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion3).speed(0.1).range(0.1..=500.0));
                    });
                    
                    // Linear Spring Damping (0.0 to 10.0)
                    ui.label("Linear Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion4, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion4).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Angular Spring Stiffness (0.1 to 100.0)
                    ui.label("Angular Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion5, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion5).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Angular Spring Damping (0.0 to 10.0)
                    ui.label("Angular Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion6, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion6).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Max Angular Deviation (0.0 to 180.0)
                    ui.label("Max Angular Deviation:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion7, 0.0..=180.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion7).speed(0.1).range(0.0..=180.0));
                    });
                    
                    ui.add_space(10.0);
                    
                    // Enable Twist Constraint checkbox
                    ui.checkbox(&mut self.widget_demo_state.enable_twist_constraint, "Enable Twist Constraint");
                    
                    // Twist Constraint Stiffness (0.0 to 2.0)
                    ui.label("Twist Constraint Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion8, 0.0..=2.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion8).speed(0.01).range(0.0..=2.0));
                    });
                    
                    // Twist Constraint Damping (0.0 to 10.0)
                    ui.label("Twist Constraint Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.adhesion9, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.adhesion9).speed(0.01).range(0.0..=10.0));
                    });
                });
            }
            Panel::ParentSettings => {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add_space(10.0);
                    
                    // Split Mass (1.0 to 3.0)
                    ui.label("Split Mass:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent1, 1.0..=3.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent1).speed(0.01).range(1.0..=3.0));
                    });
                    
                    // Split Interval (1.0 to 60.0 seconds)
                    ui.label("Split Interval:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent2, 1.0..=60.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent2).speed(0.1).range(1.0..=60.0).suffix("s"));
                    });
                    
                    // Nutrient Priority (0.1 to 10.0)
                    ui.label("Nutrient Priority:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent3, 0.1..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent3).speed(0.01).range(0.1..=10.0));
                    });
                    
                    // Prioritize When Low checkbox
                    ui.checkbox(&mut self.widget_demo_state.prioritize_when_low, "Prioritize When Low");
                    
                    ui.add_space(10.0);
                    
                    // Max Connections (0 to 20)
                    ui.label("Max Connections:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent4, 0.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent4).speed(0.1).range(0.0..=20.0));
                    });
                    
                    // Min Connections (0 to 20)
                    ui.label("Min Connections:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent5, 0.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent5).speed(0.1).range(0.0..=20.0));
                    });
                    
                    // Max Splits (-1 to 20, where -1 = infinite)
                    ui.label("Max Splits:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut self.widget_demo_state.parent6, -1.0..=20.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut self.widget_demo_state.parent6).speed(0.1).range(-1.0..=20.0));
                    });
                });
            }
            Panel::TimeSlider => {
                egui::ScrollArea::vertical().show(ui, |ui| {
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
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Handle rename dialog
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
                    widget_demo_state.modes_data[_rename_idx].0 = trimmed.to_string();
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
        
        let available_width = ui.available_width();
        
        let (selection_changed, initial_changed, add_clicked, remove_clicked, copy_clicked, copy_into_clicked, rename_idx, color_change) = widgets::modes_list(
            ui,
            &widget_demo_state.modes_data,
            &mut widget_demo_state.selected_mode,
            &mut widget_demo_state.initial_mode,
            available_width,
            widget_demo_state.copy_into_dialog_open,
            &mut widget_demo_state.color_picker_state,
        );
        
        if selection_changed {
            // If in copy into mode, this is the target selection
            if widget_demo_state.copy_into_dialog_open {
                let source_idx = widget_demo_state.copy_into_source;
                let target_idx = widget_demo_state.selected_mode;
                
                if source_idx != target_idx && source_idx < widget_demo_state.modes_data.len() 
                    && target_idx < widget_demo_state.modes_data.len() {
                    // Copy the color from source to target
                    let source_color = widget_demo_state.modes_data[source_idx].1;
                    widget_demo_state.modes_data[target_idx].1 = source_color;
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
            widget_demo_state.rename_buffer = widget_demo_state.modes_data[idx].0.clone();
        }
        
        // Handle color change from context menu color picker
        if let Some((idx, new_color)) = color_change {
            if idx < widget_demo_state.modes_data.len() {
                widget_demo_state.modes_data[idx].1 = new_color;
                info!("Changed color of mode {}", idx);
            }
        }
        
        // Handle copy mode
        if copy_clicked {
            let selected_idx = widget_demo_state.selected_mode;
            if selected_idx < widget_demo_state.modes_data.len() {
                let insert_idx = selected_idx + 1;
                
                // Copy the selected mode
                let (name, color) = widget_demo_state.modes_data[selected_idx].clone();
                
                // Generate new name
                let existing_names: Vec<String> = widget_demo_state.modes_data.iter()
                    .map(|(n, _)| n.clone())
                    .collect();
                let new_name = widgets::generate_next_mode_name(&name, &existing_names);
                
                widget_demo_state.modes_data.insert(insert_idx, (new_name, color));
                
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
                .map(|(name, _)| name.clone())
                .collect();
            let base_name = if selected_idx < widget_demo_state.modes_data.len() {
                &widget_demo_state.modes_data[selected_idx].0
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
            
            widget_demo_state.modes_data.insert(insert_idx, (new_name, new_color));
            
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
    });
}
