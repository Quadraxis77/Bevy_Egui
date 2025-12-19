mod scene;
mod drag;
mod widgets;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiPrimaryContextPass};
use egui_dock::{DockArea, DockState, NodeIndex, Style};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

use scene::ScenePlugin;
use drag::DragPlugin;

const DOCK_STATE_FILE: &str = "dock_state.ron";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Egui Dock".to_string(),
                resolution: WindowResolution::new(1280, 720),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(ScenePlugin)
        .add_plugins(DragPlugin)
        .add_systems(Startup, (setup_dock, maximize_window))
        .add_systems(EguiPrimaryContextPass, ui_system)
        .add_systems(Update, auto_save_dock_state)
        .run();
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
enum Panel {
    // Placeholder panels (permanent)
    LeftPanel,
    RightPanel,
    BottomPanel,
    Viewport,

    // Dynamic windows (can be opened/closed from menu)
    Inspector,
    Console,
    Hierarchy,
    Assets,
    CircleSliders,
}

impl Panel {
    fn is_placeholder(&self) -> bool {
        matches!(self, Panel::LeftPanel | Panel::RightPanel | Panel::BottomPanel | Panel::Viewport)
    }
}

impl std::fmt::Display for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Panel::LeftPanel => write!(f, "Left Panel"),
            Panel::RightPanel => write!(f, "Right Panel"),
            Panel::BottomPanel => write!(f, "Bottom Panel"),
            Panel::Viewport => write!(f, "Viewport"),
            Panel::Inspector => write!(f, "Inspector"),
            Panel::Console => write!(f, "Console"),
            Panel::Hierarchy => write!(f, "Hierarchy"),
            Panel::Assets => write!(f, "Assets"),
            Panel::CircleSliders => write!(f, "Circle Sliders"),
        }
    }
}

#[derive(Resource)]
struct DockResource {
    tree: DockState<Panel>,
    all_hidden: bool,
}

#[derive(Resource, Default)]
pub struct ViewportRect {
    pub rect: Option<egui::Rect>,
}

#[derive(Resource)]
pub struct WidgetDemoState {
    pub angle1: f32,
    pub angle2: f32,
    pub enable_snapping: bool,
}

impl Default for WidgetDemoState {
    fn default() -> Self {
        Self {
            angle1: 0.0,
            angle2: 45.0,
            enable_snapping: true,
        }
    }
}

fn load_dock_state() -> Option<DockState<Panel>> {
    if Path::new(DOCK_STATE_FILE).exists() {
        let data = fs::read_to_string(DOCK_STATE_FILE).ok()?;
        ron::from_str(&data).ok()
    } else {
        None
    }
}

fn save_dock_state(tree: &DockState<Panel>) {
    if let Ok(serialized) = ron::ser::to_string_pretty(tree, Default::default()) {
        let _ = fs::write(DOCK_STATE_FILE, serialized);
    }
}

fn create_default_layout() -> DockState<Panel> {
    // Create the initial layout with Viewport in the center
    let mut tree = DockState::new(vec![Panel::Viewport]);
    let surface = tree.main_surface_mut();

    // Build structure: Left (full height) | (Viewport / Bottom) | Right (full height)
    
    // First: Add left panel (20% width, full height)
    let [_left, rest] = surface.split_left(
        NodeIndex::root(),
        0.2,
        vec![Panel::LeftPanel]
    );

    // Second: Split bottom panel from viewport (70% top, 30% bottom)
    let [viewport, _bottom] = surface.split_below(
        rest,
        0.7,
        vec![Panel::BottomPanel]
    );

    // Third: Add right panel to the right of viewport only (25% of remaining width)
    let [_viewport, _right] = surface.split_right(
        viewport,
        0.25,
        vec![Panel::RightPanel]
    );

    tree
}

fn setup_dock(mut commands: Commands) {
    // Spawn a camera to enable egui rendering
    commands.spawn(Camera2d);

    let tree = load_dock_state().unwrap_or_else(|| {
        info!("Creating default dock layout");
        create_default_layout()
    });
    info!("Dock state initialized");
    commands.insert_resource(DockResource { 
        tree,
        all_hidden: false,
    });
    commands.init_resource::<ViewportRect>();
    commands.init_resource::<WidgetDemoState>();
}

fn maximize_window(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.set_maximized(true);
    }
}

fn ui_system(
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
            DockArea::new(&mut dock_resource.tree)
                .style(Style::from_egui(ctx.style().as_ref()))
                .show_leaf_collapse_buttons(false)
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

fn show_windows_menu(ui: &mut egui::Ui, dock_resource: &mut DockResource) {
    // List of dynamic windows that can be toggled
    let dynamic_windows = [
        Panel::Inspector,
        Panel::Console,
        Panel::Hierarchy,
        Panel::Assets,
        Panel::CircleSliders,
    ];

    for panel in &dynamic_windows {
        let is_open = is_panel_open(&dock_resource.tree, panel);

        if ui.selectable_label(is_open, format!("{}", panel)).clicked() {
            if is_open {
                close_panel(&mut dock_resource.tree, panel);
            } else {
                open_panel(&mut dock_resource.tree, panel);
            }
            ui.close();
        }
    }

    ui.separator();

    let hide_all_label = if dock_resource.all_hidden {
        "Show All"
    } else {
        "Hide All"
    };

    if ui.button(hide_all_label).clicked() {
        dock_resource.all_hidden = !dock_resource.all_hidden;
        ui.close();
    }
}

fn is_panel_open(tree: &DockState<Panel>, panel: &Panel) -> bool {
    // Use public API to check all tabs
    tree.iter_all_tabs().any(|(_, tab)| tab == panel)
}

fn close_panel(tree: &mut DockState<Panel>, panel: &Panel) {
    // Find the panel location
    if let Some((surface_index, node_index, tab_index)) = tree.find_tab(panel) {
        tree[surface_index].remove_tab((node_index, tab_index));
    }
}

fn open_panel(tree: &mut DockState<Panel>, panel: &Panel) {
    // Add the panel to the focused leaf
    tree.main_surface_mut().push_to_focused_leaf(panel.clone());
}

#[derive(Resource)]
struct SaveTimer {
    timer: Timer,
}

impl Default for SaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        }
    }
}

fn auto_save_dock_state(
    time: Res<Time>,
    mut save_timer: Local<SaveTimer>,
    dock_resource: Res<DockResource>,
) {
    save_timer.timer.tick(time.delta());

    if save_timer.timer.just_finished() {
        save_dock_state(&dock_resource.tree);
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
            // Other placeholder panels are completely empty
            Panel::LeftPanel | Panel::RightPanel | Panel::BottomPanel => {
                // Clear viewport rect for placeholder panels
                // (though they should be rendered after viewport in the tree)
                // No content - empty placeholder
            }
            Panel::Inspector => {
                ui.separator();
                ui.label("Object properties and settings");
                ui.add_space(10.0);
                ui.label("Selected: None");
            }
            Panel::Console => {
                ui.separator();
                ui.label("Application logs and messages");
                ui.add_space(10.0);
                ui.monospace("[INFO] Application started");
            }
            Panel::Hierarchy => {
                ui.separator();
                ui.label("Scene object tree");
                ui.add_space(10.0);
                ui.label("� Scetne");
                ui.label("  └─ � Ent;ity");
            }
            Panel::Assets => {
                ui.separator();
                ui.label("Project assets and resources");
                ui.add_space(10.0);
                ui.label("📂 Textures");
                ui.label("📂 Models");
                ui.label("📂 Scripts");
            }
            Panel::CircleSliders => {
                ui.checkbox(&mut self.widget_demo_state.enable_snapping, "Enable Snapping (11.25°)");
                ui.add_space(10.0);
                
                // Calculate available space and size for two sliders
                let available_width = ui.available_width();
                let spacing = 10.0;
                let right_margin = 10.0;
                let min_radius = 30.0;
                let max_radius = 80.0;
                
                // Calculate if we can fit two sliders side by side
                let min_width_for_two = (min_radius * 2.0 + 20.0) * 2.0 + spacing + right_margin;
                let side_by_side = available_width >= min_width_for_two;
                
                if side_by_side {
                    // Side by side layout
                    let slider_width = (available_width - spacing - right_margin) / 2.0;
                    let radius = (slider_width / 2.0 - 10.0).min(max_radius).max(min_radius);
                    
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Angle 1:");
                            widgets::circular_slider_float(
                                ui,
                                &mut self.widget_demo_state.angle1,
                                -180.0,
                                180.0,
                                radius,
                                self.widget_demo_state.enable_snapping,
                            );
                        });
                        
                        ui.add_space(spacing);
                        
                        ui.vertical(|ui| {
                            ui.label("Angle 2:");
                            widgets::circular_slider_float(
                                ui,
                                &mut self.widget_demo_state.angle2,
                                -180.0,
                                180.0,
                                radius,
                                self.widget_demo_state.enable_snapping,
                            );
                        });
                        
                        ui.add_space(right_margin);
                    });
                } else {
                    // Vertical stacked layout
                    let slider_width = available_width - right_margin;
                    let radius = (slider_width / 2.0 - 10.0).min(max_radius).max(min_radius);
                    
                    ui.vertical(|ui| {
                        ui.label("Angle 1:");
                        widgets::circular_slider_float(
                            ui,
                            &mut self.widget_demo_state.angle1,
                            -180.0,
                            180.0,
                            radius,
                            self.widget_demo_state.enable_snapping,
                        );
                        
                        ui.add_space(spacing);
                        
                        ui.label("Angle 2:");
                        widgets::circular_slider_float(
                            ui,
                            &mut self.widget_demo_state.angle2,
                            -180.0,
                            180.0,
                            radius,
                            self.widget_demo_state.enable_snapping,
                        );
                    });
                }
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
            _ => None,
        }
    }
}
