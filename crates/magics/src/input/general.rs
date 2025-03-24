use std::collections::HashMap;

use bevy::{app::AppExit, prelude::*, tasks::IoTaskPool};
use bevy_notify::prelude::*;
use chrono::Duration;
use gbp_config::{Config, DrawSetting};
use leafwing_input_manager::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{
    super::theme::CycleTheme,
    screenshot::{ScreenshotPlugin, TakeScreenshot},
    ChangingBinding,
};
use crate::{
    bevy_utils::run_conditions::event_exists,
    factorgraph::{
        graphviz::{ExportGraph, NodeKind},
        prelude::FactorGraph,
    },
    pause_play::PausePlay,
    planner::{robot::RadioAntenna, RobotConnections, RobotId},
    simulation_loader::SaveSettings,
    theme::CatppuccinTheme,
};

#[derive(Component)]
pub struct GeneralInputs;

pub struct GeneralInputPlugin;

impl Plugin for GeneralInputPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<ScreenshotPlugin>() {
            app.add_plugins(ScreenshotPlugin::default());
        }

        app.add_event::<EnvironmentEvent>()
            .add_event::<ExportFactorGraphAsGraphviz>()
            .add_event::<DrawSettingsEvent>()
            .add_event::<QuitApplication>()
            .add_event::<ExportFactorGraphAsGraphvizFinished>()
            .add_plugins(InputManagerPlugin::<GeneralAction>::default())
            .add_systems(PostStartup, bind_general_input)
            .add_systems(
                Update,
                (
                    general_actions_system,
                    pause_play_simulation.run_if(event_exists::<PausePlay>),
                    export_graph_on_event.run_if(on_event::<ExportFactorGraphAsGraphviz>()),
                    export_graph_finished_system.run_if(
                        event_exists::<ToastEvent>
                            .and_then(on_event::<ExportFactorGraphAsGraphvizFinished>()),
                    ),
                    screenshot,
                    quit_application_system,
                ),
            );
    }
}

/// Simple **Bevy** trigger `Event`
/// Write to this event whenever you want to toggle the environment
#[derive(Event, Debug, Copy, Clone)]
pub struct EnvironmentEvent;

/// Simple **Bevy** trigger `Event`
/// Write to this event whenever you want to export the graph to a `.dot` file
#[derive(Event, Debug, Copy, Clone)]
pub struct ExportFactorGraphAsGraphviz;

/// **Bevy** `Event` for the draw settings
/// This event is triggered when a draw setting is toggled
#[derive(Event, Debug, Clone)]
pub struct DrawSettingsEvent {
    /// The draw setting that was toggled
    pub setting: DrawSetting,
    /// The new value of the draw setting
    pub draw:    bool,
}

// TODO: refactor to this

// Toggle<DrawSetting<Uncertainty>>

// pub trait Toggleable {}

// #[derive(Clone, Copy, Event)]
// pub enum OnOff<T> {
//     On,
//     Off,
//     Toggle, // _phantom_data: PhantomData<T>,
// }

// pub struct Uncertainty;

// pub struct Setting<T> {
//     _phantom_data: PhantomData<T>,
// }

// pub struct Draw<T> {
//     _phantom_data: PhantomData<T>,
// }

// type Foo = OnOff<Setting<Draw<Uncertainty>>>;

// // OnOff<Setting<Draw<Uncertainty>>>

// // OnOff<DrawSetting<Uncertainty>>

// pub struct Toggle<T: Toggleable> {
//     _phantom_data: PhantomData<T>,
// }

// #[derive(Event, Debug, Clone)]
// pub struct DrawSettings<T>
// where
//     T: std::fmt::Debug,
// {
//     pub draw:      bool,
//     _phantom_data: PhantomData<T>,
// }

/// General actions that can be triggered either affecting the simulation or the
/// UI
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, EnumIter, Default)]
pub enum GeneralAction {
    #[default]
    /// Cycle between catppuccin themes
    CycleTheme,
    /// Export all factorgraphs as `graphviz` format
    ExportGraph,
    /// Take a screenshot of the primary window and save it to disk
    ScreenShot,
    /// Take a screenshot of the primary window and save it to disk
    SaveSettings,
    /// Quit the application, and end the program
    QuitApplication,
    /// Toggle the simulation time between paused and playing
    PausePlaySimulation,
}

impl std::fmt::Display for GeneralAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::CycleTheme => "Cycle Theme",
            Self::ExportGraph => "Export Graph",
            Self::ScreenShot => "Take ScreenShot",
            Self::SaveSettings => "Save Settings",
            Self::QuitApplication => "Quit Application",
            Self::PausePlaySimulation => "Pause/Play Simulation",
        })
    }
}

impl GeneralAction {
    fn default_keyboard_input(action: Self) -> UserInput {
        match action {
            Self::CycleTheme => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyT)),
            Self::ExportGraph => UserInput::Single(InputKind::PhysicalKey(KeyCode::KeyG)),
            Self::SaveSettings => {
                UserInput::modified(Modifier::Control, InputKind::PhysicalKey(KeyCode::KeyS))
            }
            Self::ScreenShot => {
                UserInput::modified(Modifier::Control, InputKind::PhysicalKey(KeyCode::KeyP))
            }
            Self::QuitApplication => {
                UserInput::modified(Modifier::Control, InputKind::PhysicalKey(KeyCode::KeyQ))
            }
            Self::PausePlaySimulation => UserInput::Single(InputKind::PhysicalKey(KeyCode::Space)),
        }
    }
}

fn bind_general_input(mut commands: Commands) {
    let mut input_map = InputMap::default();

    for action in GeneralAction::iter() {
        let input = GeneralAction::default_keyboard_input(action);
        input_map.insert(action, input);
    }

    commands.spawn((
        InputManagerBundle::<GeneralAction> {
            input_map,
            ..Default::default()
        },
        GeneralInputs,
    ));
}

fn export_factorgraphs_as_graphviz(
    query: Query<(Entity, &FactorGraph, &RadioAntenna), With<RobotConnections>>,
    config: &Config,
) -> Option<String> {
    if query.is_empty() {
        // There are no factorgraph in the scene/world
        warn!("There are no factorgraphs in the scene/world");
        return None;
    }

    // let _external_edge_length = 8.0;
    // let _internal_edge_length = 1.0;
    let cluster_margin = 16;

    let mut buf = String::with_capacity(4 * 1024); // 4 kB
    let mut append_line_to_output = |line: &str| {
        buf.push_str(line);
        buf.push('\n');
    };
    append_line_to_output("graph {");
    append_line_to_output("  dpi=96;");
    append_line_to_output(r#"  label="factorgraph""#);
    append_line_to_output("  node [style=filled];");
    append_line_to_output("  layout=neato;");

    // A hashmap used to keep track of which variable in another robots factorgraph,
    // is connected to a interrobot factor in the current robots factorgraph.
    let mut all_external_connections =
        HashMap::<RobotId, HashMap<usize, (RobotId, usize, bool)>>::with_capacity(
            query.iter().len(),
        );

    for (robot_id, factorgraph, antenna) in query.iter() {
        let (nodes, edges) = factorgraph.export_graph();

        // append_line_to_output(&format!(r#"  subgraph "cluster_{:?}" {{"#, robot_id));
        append_line_to_output(&format!(r#"  subgraph "{:?}" {{"#, robot_id));
        append_line_to_output(&format!("  margin={}", cluster_margin));
        append_line_to_output(&format!(r#"  label="{:?}""#, robot_id));
        // Add all nodes
        for node in &nodes {
            let pos = match node.kind {
                NodeKind::Variable { x, y } => Some((x, y)),
                _ => None,
            };

            let label = match node.kind {
                NodeKind::Variable { .. } => format!("v{}", node.index),
                NodeKind::InterRobotFactor { .. } => "fr".to_string(),
                NodeKind::DynamicFactor => "fd".to_string(),
                NodeKind::ObstacleFactor => "fo".to_string(),
                NodeKind::TrackingFactor => "ft".to_string(),
            };

            let line = {
                let mut line = String::with_capacity(32);
                line.push_str(&format!(
                    r#""{:?}_{:?}" [label="{}", fillcolor="{}", shape={}, width="{}""#,
                    robot_id,
                    node.index,
                    label,
                    // node.index,
                    node.color(),
                    node.shape(),
                    node.width()
                ));
                if let Some((x, y)) = pos {
                    // line.push_str(&format!(r#", pos="{x},{y}!""#));
                    // line.push_str(&format!(r#", pos="{x}, {y}""#));
                }
                line.push(']');
                line
            };

            append_line_to_output(&line);

            // let mut line = String::with_capacity(32);
            // line.push_str(&format!(
            //    r#""{:?}_{:?}" [label="{}", fillcolor="{}", shape={},
            // width="{}""#,    robot_id,
            //    node.index * 10,
            //    "fp",
            //    // node.index,
            //    "purple",
            //    // node.color(),
            //    node.shape(),
            //    node.width()
            //));
        }
        append_line_to_output("}");

        append_line_to_output("");
        // Add all internal edges
        for edge in &edges {
            let line = format!(
                r#""{:?}_{:?}" -- "{:?}_{:?}""#,
                robot_id, edge.from, robot_id, edge.to
            );
            append_line_to_output(&line);
        }

        let external_connections: HashMap<usize, (RobotId, usize, bool)> = nodes
            .into_iter()
            .filter_map(|node| match node.kind {
                NodeKind::InterRobotFactor {
                    active,
                    external_variable_id,
                } => Some((
                    node.index,
                    (
                        external_variable_id.factorgraph_id,
                        external_variable_id.variable_index.index(),
                        antenna.active,
                        // connection.id_of_robot_connected_with,
                        // connection
                        //     .index_of_connected_variable_in_other_robots_factorgraph
                        //     .index(),
                    ),
                )),
                _ => None,
            })
            .collect();

        all_external_connections.insert(robot_id, external_connections);
    }

    // Add edges between interrobot factors and the variable they are connected to
    // in another robots graph
    for (from_robot_id, from_connections) in all_external_connections {
        for (from_factor, (to_robot_id, to_variable_index, active)) in from_connections {
            append_line_to_output(&format!(
                r#" "{:?}_{:?}" -- "{:?}_{:?}" [len={}, style={}, color="{}", penwidth=3.0]"#,
                from_robot_id,
                from_factor,
                to_robot_id,
                to_variable_index,
                if active {
                    config.graphviz.interrobot.active.len
                } else {
                    config.graphviz.interrobot.inactive.len
                },
                if active {
                    &config.graphviz.interrobot.active.style
                } else {
                    &config.graphviz.interrobot.inactive.style
                },
                if active {
                    &config.graphviz.interrobot.active.color
                } else {
                    &config.graphviz.interrobot.inactive.color
                }
            ));
        }
    }

    append_line_to_output("}"); // closing '}' for starting "graph {"
    Some(buf)
}

fn cycle_theme(
    theme_event_writer: &mut EventWriter<CycleTheme>,
    catppuccin_theme: Res<CatppuccinTheme>,
) {
    info!("toggling application theme");

    let next_theme = match catppuccin_theme.flavour {
        catppuccin::Flavour::Latte => catppuccin::Flavour::Frappe,
        catppuccin::Flavour::Frappe => catppuccin::Flavour::Macchiato,
        catppuccin::Flavour::Macchiato => catppuccin::Flavour::Mocha,
        catppuccin::Flavour::Mocha => catppuccin::Flavour::Latte,
    };

    theme_event_writer.send(CycleTheme(next_theme));
}

fn export_graph_on_event(
    mut evr_export_factorgraph_as_graphviz: EventReader<ExportFactorGraphAsGraphviz>,
    query: Query<(Entity, &FactorGraph, &RadioAntenna), With<RobotConnections>>,
    config: Res<Config>,
    evw_export_graph_finished: EventWriter<ExportFactorGraphAsGraphvizFinished>,
) {
    if evr_export_factorgraph_as_graphviz.read().next().is_some() {
        if let Err(e) = handle_export_graph(
            query,
            config.as_ref(),
            evw_export_graph_finished,
            // toast_event,
        ) {
            error!("failed to export factorgraphs with error: {:?}", e);
        }
    }
}

/// **Bevy** [`Event`] for when the export graph is finished
/// Can either succeed or fail with a message
#[derive(Event, Debug)]
pub enum ExportFactorGraphAsGraphvizFinished {
    /// The export was successful with a message
    Success(String),
    /// The export failed with a message
    Failure(String),
}

fn handle_export_graph(
    q: Query<(Entity, &FactorGraph, &RadioAntenna), With<RobotConnections>>,
    config: &Config,
    mut export_graph_finished_event: EventWriter<ExportFactorGraphAsGraphvizFinished>,
    // mut toast_event: EventWriter<ToastEvent>,
) -> std::io::Result<()> {

    let Some(output) = export_factorgraphs_as_graphviz(q, config) else {
        warn!("There are no factorgraphs in the world");
        // toast_event.send(ToastEvent::warning(
        //     "There are no factorgraphs in the world".to_string(),
        // ));
        export_graph_finished_event.send(ExportFactorGraphAsGraphvizFinished::Failure(
            "There are no factorgraphs in the world".to_string(),
        ));

        return Ok(());
    };

    let dot_output_path = std::path::PathBuf::from("factorgraphs.dot");
    if dot_output_path.exists() {
        warn!(
            "output destination: ./{:#?} already exists!",
            dot_output_path
        );
        warn!("overwriting ./{:#?}", dot_output_path);
    }
    info!("exporting all factorgraphs to ./{:#?}", dot_output_path);
    // toast_event.send(ToastEvent::info(format!(
    //     "exporting all factorgraphs to ./{:#?}",
    //     dot_output_path
    // )));
    export_graph_finished_event.send(ExportFactorGraphAsGraphvizFinished::Success(
        dot_output_path.to_string_lossy().to_string(),
    ));

    std::fs::write(&dot_output_path, output.as_bytes())?;

    IoTaskPool::get()
        .spawn(async move {
            let extension = "png";
            let image_output_path = dot_output_path.with_extension(extension);
            let args = [
                "-T",
                extension,
                //"png",
                "-o",
                image_output_path.to_str().expect("is valid UTF8"),
                dot_output_path.to_str().expect("is valid UTF8"),
            ];
            let Ok(output) = std::process::Command::new("dot").args(args).output() else {
                // let error_msg = format!(
                //     "failed to compile ./{:?} with dot. reason: dot was not found in $PATH",
                //     dot_output_path
                // );
                error!(
                    "failed to compile ./{:?} with dot. reason: dot was not found in $PATH",
                    dot_output_path
                );

                // toast_event.send(ToastEvent::error(error_msg));

                return;
            };

            if output.status.success() {
                info!(
                    "compiled {:?} to {:?} with dot",
                    dot_output_path, image_output_path
                );
            } else {
                error!(
                    "attempting to compile graph with dot, returned a non-zero exit status: {:?}",
                    output
                );
            }
        })
        .detach();

    Ok(())
}

/// **Bevy** [`Update`] system, to send a Toast when factorgraph export is
/// finished
fn export_graph_finished_system(
    mut export_graph_finished_reader: EventReader<ExportFactorGraphAsGraphvizFinished>,
    mut toast_event: EventWriter<ToastEvent>,
) {
    for event in export_graph_finished_reader.read() {
        match event {
            ExportFactorGraphAsGraphvizFinished::Success(path) => {
                toast_event.send(ToastEvent::info(format!(
                    "successfully exported factorgraphs to ./{:#?}",
                    path
                )));
            }
            ExportFactorGraphAsGraphvizFinished::Failure(path) => {
                toast_event.send(ToastEvent::error(format!(
                    "failed to export factorgraphs to ./{:#?}",
                    path
                )));
            }
        }
    }
}

/// **Bevy** [`Event`] for quitting the application
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct QuitApplication;

fn quit_application_system(
    mut quit_application_reader: EventReader<QuitApplication>,
    mut app_exit_event: EventWriter<AppExit>,
) {
    for _ in quit_application_reader.read() {
        info!("quitting application");
        app_exit_event.send(AppExit);
    }
}

#[allow(clippy::too_many_arguments)]
fn general_actions_system(
    mut theme_event: EventWriter<CycleTheme>,
    query: Query<&ActionState<GeneralAction>, With<GeneralInputs>>,
    query_graphs: Query<(Entity, &FactorGraph, &RadioAntenna), With<RobotConnections>>,
    config: Res<Config>,
    currently_changing: Res<ChangingBinding>,
    catppuccin_theme: Res<CatppuccinTheme>,
    // mut app_exit_event: EventWriter<AppExit>,
    mut quit_application_event: EventWriter<QuitApplication>,
    export_graph_finished_event: EventWriter<ExportFactorGraphAsGraphvizFinished>,
    mut evw_save_settings: EventWriter<SaveSettings>,
    mut evw_toast: EventWriter<ToastEvent>,
    // mut pause_play_event: EventWriter<PausePlay>,
    // toast_event: EventWriter<ToastEvent>,
) {
    if currently_changing.on_cooldown() || currently_changing.is_changing() {
        return;
    }
    let Ok(action_state) = query.get_single() else {
        warn!("general_actions_system was called without an action state!");
        return;
    };

    if action_state.just_pressed(&GeneralAction::CycleTheme) {
        cycle_theme(&mut theme_event, catppuccin_theme);
    } else if action_state.just_pressed(&GeneralAction::ExportGraph) {
        if let Err(e) = handle_export_graph(
            query_graphs,
            config.as_ref(),
            export_graph_finished_event,
            // toast_event,
        ) {
            error!("failed to export factorgraphs with error: {:?}", e);
        }
    }

    if action_state.just_pressed(&GeneralAction::QuitApplication) {
        quit_application_event.send(QuitApplication);
        // info!("quitting application");
        // app_exit_event.send(AppExit);
    }

    if action_state.just_pressed(&GeneralAction::SaveSettings) {
        evw_save_settings.send(SaveSettings);
        let toast = ToastEvent {
            caption: "saved settings to config.toml".to_string(),
            options: ToastOptions {
                duration: Some(std::time::Duration::from_millis(500)),
                level: ToastLevel::Success,
                show_progress_bar: false,
                closable: false,
            },
        };
        evw_toast.send(toast);
    }
}

fn pause_play_simulation(
    query: Query<&ActionState<GeneralAction>, With<GeneralInputs>>,
    currently_changing: Res<ChangingBinding>,
    mut pause_play_event: EventWriter<PausePlay>,
) {
    if currently_changing.on_cooldown() || currently_changing.is_changing() {
        return;
    }

    let Ok(action_state) = query.get_single() else {
        warn!("pause_play_simulation was called without an action state!");
        return;
    };

    if action_state.just_pressed(&GeneralAction::PausePlaySimulation) {
        pause_play_event.send(PausePlay::Toggle);
    }
}

fn screenshot(
    query: Query<&ActionState<GeneralAction>, With<GeneralInputs>>,
    currently_changing: Res<ChangingBinding>,
    mut screen_shot_event: EventWriter<TakeScreenshot>,
) {
    if currently_changing.on_cooldown() || currently_changing.is_changing() {
        return;
    }

    let Ok(action_state) = query.get_single() else {
        warn!("screenshot was called without an action state!");
        return;
    };

    if action_state.just_pressed(&GeneralAction::ScreenShot) {
        info!("Sending TakeScreenshot::default() event");
        screen_shot_event.send(TakeScreenshot::default());
    }
}
