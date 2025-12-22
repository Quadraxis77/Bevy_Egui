use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// Plugin for genome management
pub struct GenomePlugin;

impl Plugin for GenomePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GenomeLibrary>()
            .init_resource::<CurrentGenome>();
    }
}

/// Storage for all genomes in the simulation
#[derive(Resource, Default)]
pub struct GenomeLibrary {
    pub genomes: Vec<GenomeData>,
}

impl GenomeLibrary {
    #[allow(dead_code)]
    pub fn add_genome(&mut self, genome: GenomeData) {
        self.genomes.push(genome);
    }
}

/// Current genome being edited/used
#[derive(Resource)]
pub struct CurrentGenome {
    pub genome: GenomeData,
    pub selected_mode_index: i32,
}

impl Default for CurrentGenome {
    fn default() -> Self {
        Self {
            genome: GenomeData::default(),
            selected_mode_index: 0,
        }
    }
}

/// Adhesion configuration
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct AdhesionSettings {
    pub can_break: bool,
    pub break_force: f32,
    pub rest_length: f32,
    pub linear_spring_stiffness: f32,
    pub linear_spring_damping: f32,
    pub orientation_spring_stiffness: f32,
    pub orientation_spring_damping: f32,
    pub max_angular_deviation: f32,
    pub twist_constraint_stiffness: f32,
    pub twist_constraint_damping: f32,
    pub enable_twist_constraint: bool,
}

impl Default for AdhesionSettings {
    fn default() -> Self {
        Self {
            can_break: true,
            break_force: 10.0,
            rest_length: 1.0,
            linear_spring_stiffness: 150.0,
            linear_spring_damping: 5.0,
            orientation_spring_stiffness: 50.0,
            orientation_spring_damping: 5.0,
            max_angular_deviation: 0.0,
            twist_constraint_stiffness: 2.0,
            twist_constraint_damping: 0.5,
            enable_twist_constraint: false,
        }
    }
}

/// Child settings for mode transitions
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildSettings {
    pub mode_number: i32,
    pub orientation: Quat,
    pub keep_adhesion: bool,
    pub enable_angle_snapping: bool,
    // Lat/lon tracking for quaternion ball widget (UI feedback only)
    #[serde(default)]
    pub x_axis_lat: f32,
    #[serde(default)]
    pub x_axis_lon: f32,
    #[serde(default)]
    pub y_axis_lat: f32,
    #[serde(default)]
    pub y_axis_lon: f32,
    #[serde(default)]
    pub z_axis_lat: f32,
    #[serde(default)]
    pub z_axis_lon: f32,
}

impl Default for ChildSettings {
    fn default() -> Self {
        Self {
            mode_number: 0,
            orientation: Quat::IDENTITY,
            keep_adhesion: true,
            enable_angle_snapping: true,
            x_axis_lat: 0.0,
            x_axis_lon: 0.0,
            y_axis_lat: 0.0,
            y_axis_lon: 0.0,
            z_axis_lat: 0.0,
            z_axis_lon: 0.0,
        }
    }
}

/// A single mode within a genome
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct ModeSettings {
    pub name: String,
    pub default_name: String,
    pub color: Vec3,
    pub opacity: f32,
    #[serde(default)]
    pub emissive: f32,

    // Cell type
    pub cell_type: i32,

    // Parent settings
    pub parent_make_adhesion: bool,
    pub split_mass: f32,
    #[serde(default)]
    pub split_mass_min: Option<f32>,
    pub split_interval: f32,
    #[serde(default)]
    pub split_interval_min: Option<f32>,
    pub nutrient_gain_rate: f32,
    pub max_cell_size: f32,
    pub split_ratio: f32,
    pub nutrient_priority: f32,
    pub prioritize_when_low: bool,
    pub parent_split_direction: Vec2,
    pub max_adhesions: i32,
    pub min_adhesions: i32,
    pub enable_parent_angle_snapping: bool,
    pub max_splits: i32,
    pub mode_a_after_splits: i32,
    pub mode_b_after_splits: i32,
    
    // Flagellocyte settings
    pub swim_force: f32,

    // Child settings
    pub child_a: ChildSettings,
    pub child_b: ChildSettings,

    // Adhesion settings
    pub adhesion_settings: AdhesionSettings,
}

impl ModeSettings {
    /// Create a new mode that splits back to itself
    pub fn new_self_splitting(mode_index: i32, name: String) -> Self {
        Self {
            default_name: name.clone(),
            name,
            color: Vec3::new(1.0, 1.0, 1.0),
            opacity: 1.0,
            emissive: 0.0,
            cell_type: 0,
            parent_make_adhesion: false,
            split_mass: 1.5,
            split_mass_min: None,
            split_interval: 5.0,
            split_interval_min: None,
            nutrient_gain_rate: 0.2,
            max_cell_size: 2.0,
            split_ratio: 0.5,
            nutrient_priority: 1.0,
            prioritize_when_low: true,
            parent_split_direction: Vec2::ZERO,
            max_adhesions: 20,
            min_adhesions: 0,
            enable_parent_angle_snapping: true,
            max_splits: -1,
            mode_a_after_splits: -1,
            mode_b_after_splits: -1,
            swim_force: 0.5,
            child_a: ChildSettings {
                mode_number: mode_index,
                ..Default::default()
            },
            child_b: ChildSettings {
                mode_number: mode_index,
                ..Default::default()
            },
            adhesion_settings: AdhesionSettings::default(),
        }
    }
}

impl Default for ModeSettings {
    fn default() -> Self {
        Self {
            name: "Untitled Mode".to_string(),
            default_name: "Untitled Mode".to_string(),
            color: Vec3::new(1.0, 1.0, 1.0),
            opacity: 1.0,
            emissive: 0.0,
            cell_type: 0,
            parent_make_adhesion: false,
            split_mass: 1.5,
            split_mass_min: None,
            split_interval: 5.0,
            split_interval_min: None,
            nutrient_gain_rate: 0.2,
            max_cell_size: 2.0,
            split_ratio: 0.5,
            nutrient_priority: 1.0,
            prioritize_when_low: true,
            parent_split_direction: Vec2::ZERO,
            max_adhesions: 20,
            min_adhesions: 0,
            enable_parent_angle_snapping: true,
            max_splits: -1,
            mode_a_after_splits: -1,
            mode_b_after_splits: -1,
            swim_force: 0.5,
            child_a: ChildSettings::default(),
            child_b: ChildSettings::default(),
            adhesion_settings: AdhesionSettings::default(),
        }
    }
}

/// A complete genome definition
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct GenomeData {
    pub name: String,
    pub initial_mode: i32,
    pub initial_orientation: Quat,
    pub modes: Vec<ModeSettings>,
}

impl Default for GenomeData {
    fn default() -> Self {
        let mut genome = Self {
            name: "Untitled Genome".to_string(),
            initial_mode: 0,
            initial_orientation: Quat::IDENTITY,
            modes: Vec::new(),
        };
        
        // Create all 120 modes
        for i in 0..120 {
            let mode_name = format!("M {}", i);
            let mut mode = ModeSettings::new_self_splitting(i as i32, mode_name);
            
            // Generate a color based on the mode number using HSV
            let hue = (i as f32 / 120.0) * 360.0;
            let (r, g, b) = hue_to_rgb(hue);
            mode.color = Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            
            genome.modes.push(mode);
        }
        
        genome
    }
}

// Helper function to convert HSV hue to RGB
fn hue_to_rgb(hue: f32) -> (u8, u8, u8) {
    let h = hue / 60.0;
    let c = 1.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    
    let (r, g, b) = if h < 1.0 {
        (c, x, 0.0)
    } else if h < 2.0 {
        (x, c, 0.0)
    } else if h < 3.0 {
        (0.0, c, x)
    } else if h < 4.0 {
        (0.0, x, c)
    } else if h < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    // Scale to 100-255 range for better visibility
    let scale = |v: f32| ((v * 155.0) + 100.0) as u8;
    (scale(r), scale(g), scale(b))
}

impl GenomeData {
    /// Save genome to a JSON file
    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load genome from a JSON file
    #[allow(dead_code)]
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        let genome = serde_json::from_str(&json)?;
        Ok(genome)
    }
}
