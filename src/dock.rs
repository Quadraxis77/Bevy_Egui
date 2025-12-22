use bevy::prelude::*;
use egui_dock::DockState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::Duration;

const DOCK_STATE_FILE: &str = "dock_state.ron";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Panel {
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
    QuaternionBall,
    Modes,
    NameTypeEditor,
    AdhesionSettings,
    ParentSettings,
    TimeSlider,
}

impl Panel {
    pub fn is_placeholder(&self) -> bool {
        matches!(self, Panel::LeftPanel | Panel::RightPanel | Panel::BottomPanel)
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
            Panel::CircleSliders => write!(f, "Parent Split Angle"),
            Panel::QuaternionBall => write!(f, "Child Settings"),
            Panel::Modes => write!(f, "Modes"),
            Panel::NameTypeEditor => write!(f, "Genome Editor"),
            Panel::AdhesionSettings => write!(f, "Adhesion Settings"),
            Panel::ParentSettings => write!(f, "Parent Settings"),
            Panel::TimeSlider => write!(f, "Time Slider"),
        }
    }
}

#[derive(Resource)]
pub struct DockResource {
    pub tree: DockState<Panel>,
    pub all_hidden: bool,
}

pub fn load_dock_state() -> Option<DockState<Panel>> {
    if Path::new(DOCK_STATE_FILE).exists() {
        let data = fs::read_to_string(DOCK_STATE_FILE).ok()?;
        ron::from_str(&data).ok()
    } else {
        None
    }
}

pub fn save_dock_state(tree: &DockState<Panel>) {
    if let Ok(serialized) = ron::ser::to_string_pretty(tree, Default::default()) {
        let _ = fs::write(DOCK_STATE_FILE, serialized);
    }
}

pub fn create_default_layout() -> DockState<Panel> {
    // Create the initial layout with Viewport in the center
    let mut tree = DockState::new(vec![Panel::Viewport]);
    let surface = tree.main_surface_mut();

    // Build structure: Modes | Left (full height) | (Viewport / Bottom) | Right (full height)
    
    // First: Add Modes panel on the far left (6% width - reduced from 10%)
    let [_modes, rest] = surface.split_left(
        egui_dock::NodeIndex::root(),
        0.06,
        vec![Panel::Modes]
    );
    
    // Second: Add left panel (20% of remaining width, full height)
    let [_left_node, rest] = surface.split_left(
        rest,
        0.22,
        vec![Panel::LeftPanel]
    );

    // Third: Split bottom panel from viewport (70% top, 30% bottom)
    let [viewport, _bottom] = surface.split_below(
        rest,
        0.7,
        vec![Panel::BottomPanel]
    );

    // Fourth: Add right panel to the right of viewport only (25% of remaining width)
    let [_viewport, _right] = surface.split_right(
        viewport,
        0.25,
        vec![Panel::RightPanel]
    );

    // Set absolute minimum size for left panel (300 pixels)
    // The parent of left_node is the horizontal split that contains it
    if let egui_dock::Node::Horizontal(split) = &mut surface[egui_dock::NodeIndex::root()] {
        split.absolute_size_left = Some(0.0); // Will be initialized to current size on first render
    }

    tree
}

pub fn setup_dock(mut commands: Commands) {
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
    commands.init_resource::<crate::ui::ViewportRect>();
    commands.init_resource::<crate::ui::WidgetDemoState>();
}

pub fn is_panel_open(tree: &DockState<Panel>, panel: &Panel) -> bool {
    // Use public API to check all tabs
    tree.iter_all_tabs().any(|(_, tab)| tab == panel)
}

pub fn close_panel(tree: &mut DockState<Panel>, panel: &Panel) {
    // Find the panel location
    if let Some((surface_index, node_index, tab_index)) = tree.find_tab(panel) {
        tree[surface_index].remove_tab((node_index, tab_index));
    }
}

pub fn open_panel(tree: &mut DockState<Panel>, panel: &Panel) {
    // Add the panel to the focused leaf
    tree.main_surface_mut().push_to_focused_leaf(panel.clone());
}

#[derive(Resource)]
pub struct SaveTimer {
    timer: Timer,
}

impl Default for SaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        }
    }
}

pub fn auto_save_dock_state(
    time: Res<Time>,
    mut save_timer: Local<SaveTimer>,
    dock_resource: Res<DockResource>,
) {
    save_timer.timer.tick(time.delta());

    if save_timer.timer.just_finished() {
        save_dock_state(&dock_resource.tree);
    }
}

pub fn save_on_exit(
    dock_resource: Res<DockResource>,
    mut exit_events: MessageReader<bevy::app::AppExit>,
) {
    for _ in exit_events.read() {
        save_dock_state(&dock_resource.tree);
        info!("Saved dock state on exit");
    }
}

pub fn show_windows_menu(ui: &mut bevy_egui::egui::Ui, dock_resource: &mut DockResource) {
    // List of dynamic windows that can be toggled
    let dynamic_windows = [
        Panel::Inspector,
        Panel::Console,
        Panel::Hierarchy,
        Panel::Assets,
        Panel::CircleSliders,
        Panel::QuaternionBall,
        Panel::Modes,
        Panel::NameTypeEditor,
        Panel::AdhesionSettings,
        Panel::ParentSettings,
        Panel::TimeSlider,
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
