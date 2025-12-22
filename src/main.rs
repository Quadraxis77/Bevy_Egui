mod scene;
mod drag;
mod widgets;
mod dock;
mod ui;
mod genome;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;

use scene::ScenePlugin;
use drag::DragPlugin;
use genome::GenomePlugin;
use dock::{setup_dock, auto_save_dock_state, save_on_exit};
use ui::ui_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "BioSpheres-Q Egui".to_string(),
                resolution: WindowResolution::new(1920, 1080),
                present_mode: bevy::window::PresentMode::AutoNoVsync,
                resize_constraints: bevy::window::WindowResizeConstraints {
                    min_width: 800.0,
                    min_height: 600.0,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(ScenePlugin)
        .add_plugins(DragPlugin)
        .add_plugins(GenomePlugin)
        .init_resource::<ui::GlobalUiState>()
        .init_resource::<ui::WidgetDemoState>()
        .add_systems(Startup, (setup_dock, maximize_window))
        .add_systems(bevy_egui::EguiPrimaryContextPass, ui_system)
        .add_systems(Update, (auto_save_dock_state, save_on_exit))
        .run();
}

fn maximize_window(mut windows: Query<&mut Window>) {
    for mut window in windows.iter_mut() {
        window.set_maximized(true);
    }
}
