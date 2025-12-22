use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui_dock::{DockArea, Style};

use crate::dock::*;
use crate::widgets;
use crate::genome::{CurrentGenome, ModeSettings};

#[derive(Resource, Default)]
pub struct ViewportRect {
    pub rect: Option<egui::Rect>,
}

// Global UI state - matches ui::GlobalUiState
// These fields will be used when implementing window visibility toggles
#[allow(dead_code)]
#[derive(Resource)]
pub struct GlobalUiState {
    pub windows_locked: bool,
    pub ui_scale: f32,
    pub show_cell_inspector: bool,
    pub show_genome_editor: bool,
    pub show_scene_manager: bool,
    pub show_performance_monitor: bool,
    pub show_rendering_controls: bool,
    pub show_time_scrubber: bool,
    pub show_theme_editor: bool,
    pub show_camera_settings: bool,
    pub show_lighting_settings: bool,
}

impl Default for GlobalUiState {
    fn default() -> Self {
        Self {
            windows_locked: false,
            ui_scale: 1.0,
            show_cell_inspector: true,
            show_genome_editor: true,
            show_scene_manager: true,
            show_performance_monitor: false,
            show_rendering_controls: false,
            show_time_scrubber: true,
            show_theme_editor: false,
            show_camera_settings: false,
            show_lighting_settings: false,
        }
    }
}

// UI state for internal widget management
#[derive(Resource)]
pub struct WidgetDemoState {
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
        Self {
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
    mut current_genome: ResMut<CurrentGenome>,
    mut widget_demo_state: ResMut<WidgetDemoState>,
    global_ui_state: Res<GlobalUiState>,
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
                    show_windows_menu(ui, &mut dock_resource, &global_ui_state);
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
                    current_genome: &mut current_genome,
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
    current_genome: &'a mut CurrentGenome,
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
            // BioSpheres-Q standard panels (placeholders for now)
            Panel::CellInspector => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Cell Inspector");
                    ui.label("Click on a cell to inspect it");
                });
            }
            Panel::GenomeEditor => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Genome Editor");
                    ui.label("Genome editing interface");
                });
            }
            Panel::SceneManager => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Scene Manager");
                    ui.label("Scene management controls");
                });
            }
            Panel::PerformanceMonitor => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Performance Monitor");
                    ui.label("FPS and performance metrics");
                });
            }
            Panel::RenderingControls => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Rendering Controls");
                    ui.label("Graphics settings");
                });
            }
            Panel::TimeScrubber => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Time Scrubber");
                    ui.label("Timeline control");
                });
            }
            Panel::ThemeEditor => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Theme Editor");
                    ui.label("UI theme customization");
                });
            }
            Panel::CameraSettings => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Camera Settings");
                    ui.label("Camera configuration");
                });
            }
            Panel::LightingSettings => {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                    ui.separator();
                    ui.label("Lighting Settings");
                    ui.label("Lighting configuration");
                });
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
                    let selected_idx = self.current_genome.selected_mode_index as usize;
                    if selected_idx < self.current_genome.genome.modes.len() {
                        let mode = &mut self.current_genome.genome.modes[selected_idx];
                        
                        // Calculate available space and size for two sliders
                        let _available_width = ui.available_width();
                        let radius = 29.25;
                        
                        // Always side by side
                        ui.horizontal(|ui| {
                            ui.add_space(10.0);
                            
                            ui.vertical(|ui| {
                                ui.label("Pitch:");
                                let mut pitch = mode.parent_split_direction.x;
                                widgets::circular_slider_float(
                                    ui,
                                    &mut pitch,
                                    -180.0,
                                    180.0,
                                    radius,
                                    self.widget_demo_state.enable_snapping,
                                );
                                mode.parent_split_direction.x = pitch;
                            });
                            
                            ui.vertical(|ui| {
                                ui.label("Yaw:");
                                let mut yaw = mode.parent_split_direction.y;
                                widgets::circular_slider_float(
                                    ui,
                                    &mut yaw,
                                    -180.0,
                                    180.0,
                                    radius,
                                    self.widget_demo_state.enable_snapping,
                                );
                                mode.parent_split_direction.y = yaw;
                            });
                        });
                    }
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
                    let selected_mode_idx = self.current_genome.selected_mode_index as usize;
                    
                    // Collect mode display data before mutable borrows
                    let mode_display_data: Vec<(String, egui::Color32)> = self.current_genome.genome.modes.iter()
                        .map(|m| {
                            let color = m.color;
                            let r = (color.x * 255.0) as u8;
                            let g = (color.y * 255.0) as u8;
                            let b = (color.z * 255.0) as u8;
                            (m.name.clone(), egui::Color32::from_rgb(r, g, b))
                        })
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
                                
                                if selected_mode_idx >= self.current_genome.genome.modes.len() {
                                    return;
                                }
                                let mode = &mut self.current_genome.genome.modes[selected_mode_idx];
                                
                                widgets::quaternion_ball(
                                    ui,
                                    &mut mode.child_a.orientation,
                                    &mut mode.child_a.x_axis_lat,
                                    &mut mode.child_a.x_axis_lon,
                                    &mut mode.child_a.y_axis_lat,
                                    &mut mode.child_a.y_axis_lon,
                                    &mut mode.child_a.z_axis_lat,
                                    &mut mode.child_a.z_axis_lon,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball1_locked_axis,
                                    &mut self.widget_demo_state.qball1_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 1
                                ui.checkbox(&mut mode.child_a.keep_adhesion, "Keep Adhesion");
                                
                                // Mode label and dropdown for ball 1
                                ui.label("Mode:");
                                let child_a_mode_idx = mode.child_a.mode_number as usize;
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

                                            let mut current_mode = mode.child_a.mode_number as usize;
                                            let _response = ui.selectable_value(
                                                &mut current_mode,
                                                i,
                                                egui::RichText::new(mode_name).color(text_color).background_color(*mode_color)
                                            );
                                            if current_mode != mode.child_a.mode_number as usize {
                                                mode.child_a.mode_number = current_mode as i32;
                                            }
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
                                
                                if selected_mode_idx >= self.current_genome.genome.modes.len() {
                                    return;
                                }
                                let mode = &mut self.current_genome.genome.modes[selected_mode_idx];
                                
                                widgets::quaternion_ball(
                                    ui,
                                    &mut mode.child_b.orientation,
                                    &mut mode.child_b.x_axis_lat,
                                    &mut mode.child_b.x_axis_lon,
                                    &mut mode.child_b.y_axis_lat,
                                    &mut mode.child_b.y_axis_lon,
                                    &mut mode.child_b.z_axis_lat,
                                    &mut mode.child_b.z_axis_lon,
                                    ball_radius,
                                    self.widget_demo_state.qball_snapping,
                                    &mut self.widget_demo_state.qball2_locked_axis,
                                    &mut self.widget_demo_state.qball2_initial_distance,
                                );
                                
                                ui.add_space(5.0);
                                
                                // Keep Adhesion checkbox for ball 2
                                ui.checkbox(&mut mode.child_b.keep_adhesion, "Keep Adhesion");
                                
                                // Mode label and dropdown for ball 2
                                ui.label("Mode:");
                                let child_b_mode_idx = mode.child_b.mode_number as usize;
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

                                            let mut current_mode = mode.child_b.mode_number as usize;
                                            let _response = ui.selectable_value(
                                                &mut current_mode,
                                                i,
                                                egui::RichText::new(mode_name).color(text_color).background_color(*mode_color)
                                            );
                                            if current_mode != mode.child_b.mode_number as usize {
                                                mode.child_b.mode_number = current_mode as i32;
                                            }
                                        }
                                    });
                            }
                        );
                    });
                });
            }
            Panel::Modes => {
                render_modes_panel(ui, self.current_genome, self.widget_demo_state);
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
                                .set_file_name(&format!("{}.json", self.current_genome.genome.name))
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
                        ui.text_edit_singleline(&mut self.current_genome.genome.name);
                    });
                    
                    ui.add_space(4.0);
                    
                    // Get current mode
                    let selected_idx = self.current_genome.selected_mode_index as usize;
                    if selected_idx >= self.current_genome.genome.modes.len() {
                        ui.label("No mode selected");
                        return;
                    }
                    let mode = &mut self.current_genome.genome.modes[selected_idx];
                    
                    // Type dropdown and checkbox on the same line
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        let cell_types = ["Photocyte", "Phagocyte", "Flagellocyte", "Devorocyte", "Lipocyte"];
                        egui::ComboBox::from_id_salt("cell_type")
                            .selected_text(cell_types[mode.cell_type as usize])
                            .show_ui(ui, |ui| {
                                for (i, type_name) in cell_types.iter().enumerate() {
                                    ui.selectable_value(&mut mode.cell_type, i as i32, *type_name);
                                }
                            });
                        
                        ui.checkbox(&mut mode.parent_make_adhesion, "Make Adhesion");
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
                    let selected_idx = self.current_genome.selected_mode_index as usize;
                    if selected_idx >= self.current_genome.genome.modes.len() {
                        ui.label("No mode selected");
                        return;
                    }
                    let mode = &mut self.current_genome.genome.modes[selected_idx];
                    
                    // Adhesion Can Break checkbox
                    ui.checkbox(&mut mode.adhesion_settings.can_break, "Adhesion Can Break");
                    
                    // Adhesion Break Force (0.1 to 100.0)
                    ui.label("Adhesion Break Force:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.break_force, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.break_force).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Adhesion Rest Length (0.5 to 5.0)
                    ui.label("Adhesion Rest Length:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.rest_length, 0.5..=5.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.rest_length).speed(0.01).range(0.5..=5.0));
                    });
                    
                    // Linear Spring Stiffness (0.1 to 500.0)
                    ui.label("Linear Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.linear_spring_stiffness, 0.1..=500.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.linear_spring_stiffness).speed(0.1).range(0.1..=500.0));
                    });
                    
                    // Linear Spring Damping (0.0 to 10.0)
                    ui.label("Linear Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.linear_spring_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.linear_spring_damping).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Orientation Spring Stiffness (0.1 to 100.0)
                    ui.label("Orientation Spring Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.orientation_spring_stiffness, 0.1..=100.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.orientation_spring_stiffness).speed(0.1).range(0.1..=100.0));
                    });
                    
                    // Orientation Spring Damping (0.0 to 10.0)
                    ui.label("Orientation Spring Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.orientation_spring_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.orientation_spring_damping).speed(0.01).range(0.0..=10.0));
                    });
                    
                    // Max Angular Deviation (0.0 to 180.0)
                    ui.label("Max Angular Deviation:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.max_angular_deviation, 0.0..=180.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.max_angular_deviation).speed(0.1).range(0.0..=180.0));
                    });
                    
                    ui.add_space(10.0);
                    
                    // Enable Twist Constraint checkbox
                    ui.checkbox(&mut mode.adhesion_settings.enable_twist_constraint, "Enable Twist Constraint");
                    
                    // Twist Constraint Stiffness (0.0 to 2.0)
                    ui.label("Twist Constraint Stiffness:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.twist_constraint_stiffness, 0.0..=2.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.twist_constraint_stiffness).speed(0.01).range(0.0..=2.0));
                    });
                    
                    // Twist Constraint Damping (0.0 to 10.0)
                    ui.label("Twist Constraint Damping:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.adhesion_settings.twist_constraint_damping, 0.0..=10.0).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.adhesion_settings.twist_constraint_damping).speed(0.01).range(0.0..=10.0));
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
                    let selected_idx = self.current_genome.selected_mode_index as usize;
                    if selected_idx >= self.current_genome.genome.modes.len() {
                        ui.label("No mode selected");
                        return;
                    }
                    let mode = &mut self.current_genome.genome.modes[selected_idx];
                    
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
                        ui.add(egui::Slider::new(&mut mode.max_adhesions, 0..=20).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.max_adhesions).speed(1).range(0..=20));
                    });
                    
                    // Min Connections (0 to 20)
                    ui.label("Min Connections:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.min_adhesions, 0..=20).show_value(false));
                        ui.add(egui::DragValue::new(&mut mode.min_adhesions).speed(1).range(0..=20));
                    });
                    
                    // Max Splits (-1 to 20, where -1 = infinite)
                    ui.label("Max Splits:");
                    ui.horizontal(|ui| {
                        let available = ui.available_width();
                        let slider_width = if available > 80.0 { available - 70.0 } else { 50.0 };
                        ui.style_mut().spacing.slider_width = slider_width;
                        ui.add(egui::Slider::new(&mut mode.max_splits, -1..=20).show_value(false));
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

    fn is_placeholder(&self, tab: &Self::Tab) -> bool {
        // Placeholder panels hold space but don't show tabs or allow dragging
        tab.is_placeholder()
    }

    fn is_viewport(&self, tab: &Self::Tab) -> bool {
        // Viewport is a special panel for 3D rendering
        matches!(tab, Panel::Viewport)
    }

    fn clear_background(&self, tab: &Self::Tab) -> bool {
        // Return false for viewport to skip drawing background (make it transparent)
        // Return true for other panels to draw the background
        !matches!(tab, Panel::Viewport)
    }

    fn is_closeable(&self, tab: &Self::Tab) -> bool {
        // Only non-placeholder, non-viewport panels can be closed
        !tab.is_placeholder() && !self.is_viewport(tab)
    }

    fn allowed_in_windows(&self, tab: &mut Self::Tab) -> bool {
        // Only non-placeholder, non-viewport panels can be ejected to floating windows
        !tab.is_placeholder() && !self.is_viewport(tab)
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

fn render_modes_panel(ui: &mut egui::Ui, current_genome: &mut CurrentGenome, widget_demo_state: &mut WidgetDemoState) {
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
            if !trimmed.is_empty() && _rename_idx < current_genome.genome.modes.len() {
                current_genome.genome.modes[_rename_idx].name = trimmed.to_string();
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
    let (copy_into_clicked, reset_clicked) = widgets::modes_buttons(
        ui,
        current_genome.genome.modes.len(),
        current_genome.selected_mode_index as usize,
        current_genome.genome.initial_mode as usize,
    );

    ui.separator();

    // Show instruction text if in copy into mode (also outside scroll area)
    if widget_demo_state.copy_into_dialog_open {
        ui.colored_label(egui::Color32::YELLOW, "Select target mode to copy into:");
        ui.add_space(5.0);
    }

    // Convert modes to display format
    let modes_display: Vec<(String, egui::Color32)> = current_genome.genome.modes.iter()
        .map(|m| {
            let color = m.color;
            let r = (color.x * 255.0) as u8;
            let g = (color.y * 255.0) as u8;
            let b = (color.z * 255.0) as u8;
            (m.name.clone(), egui::Color32::from_rgb(r, g, b))
        })
        .collect();

    // Now create scroll area for the list
    let (selection_changed, initial_changed, rename_idx, color_change) = egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
        let available_width = ui.available_width();

        let mut selected_mode = current_genome.selected_mode_index as usize;
        let mut initial_mode = current_genome.genome.initial_mode as usize;
        
        let result = widgets::modes_list_items(
            ui,
            &modes_display,
            &mut selected_mode,
            &mut initial_mode,
            available_width,
            widget_demo_state.copy_into_dialog_open,
            &mut widget_demo_state.color_picker_state,
        );
        
        current_genome.selected_mode_index = selected_mode as i32;
        current_genome.genome.initial_mode = initial_mode as i32;
        
        result
    }).inner;

    if selection_changed {
        // If in copy into mode, this is the target selection
        if widget_demo_state.copy_into_dialog_open {
            let source_idx = widget_demo_state.copy_into_source;
            let target_idx = current_genome.selected_mode_index as usize;

            if source_idx != target_idx && source_idx < current_genome.genome.modes.len()
                && target_idx < current_genome.genome.modes.len() {
                // Copy all settings from source to target (including color, except name)
                let source_mode = current_genome.genome.modes[source_idx].clone();
                let target_name = current_genome.genome.modes[target_idx].name.clone();
                current_genome.genome.modes[target_idx] = source_mode;
                current_genome.genome.modes[target_idx].name = target_name;
                info!("Copied mode {} into mode {}", source_idx, target_idx);
            }

            // Exit copy into mode
            widget_demo_state.copy_into_dialog_open = false;
        } else {
            info!("Selected mode changed to: {}", current_genome.selected_mode_index);
        }
    }
    if initial_changed {
        info!("Initial mode changed to: {}", current_genome.genome.initial_mode);
    }

    // Handle rename request
    if let Some(idx) = rename_idx {
        widget_demo_state.renaming_mode = Some(idx);
        widget_demo_state.rename_buffer = current_genome.genome.modes[idx].name.clone();
    }

    // Handle color change from context menu color picker
    if let Some((idx, new_color)) = color_change {
        if idx < current_genome.genome.modes.len() {
            let r = new_color.r() as f32 / 255.0;
            let g = new_color.g() as f32 / 255.0;
            let b = new_color.b() as f32 / 255.0;
            current_genome.genome.modes[idx].color = Vec3::new(r, g, b);
            info!("Changed color of mode {}", idx);
        }
    }

    // Handle copy into mode
    if copy_into_clicked {
        let selected_idx = current_genome.selected_mode_index as usize;
        if selected_idx < current_genome.genome.modes.len() {
            // Enter copy into mode - user will click on target mode directly
            widget_demo_state.copy_into_dialog_open = true;
            widget_demo_state.copy_into_source = selected_idx;
        }
    }

    // Handle reset mode
    if reset_clicked {
        let selected_idx = current_genome.selected_mode_index as usize;
        if selected_idx < current_genome.genome.modes.len() {
            // Reset to default values
            let name = current_genome.genome.modes[selected_idx].name.clone();
            let color = current_genome.genome.modes[selected_idx].color;
            current_genome.genome.modes[selected_idx] = ModeSettings::default();
            current_genome.genome.modes[selected_idx].name = name;
            current_genome.genome.modes[selected_idx].color = color;
            current_genome.genome.modes[selected_idx].child_a.mode_number = selected_idx as i32;
            current_genome.genome.modes[selected_idx].child_b.mode_number = selected_idx as i32;
            info!("Reset mode {}", selected_idx);
        }
    }
}
