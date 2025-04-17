use std::collections::BTreeMap;

use bevy::prelude::*;
use itertools::Itertools;
use ringbuf::{Rb, StaticRb};

const MAX_TRACE_LENGTH: usize = 10000;
// const SAMPLE_DELAY: f32 = 0.5;

use gbp_config::Config;

use crate::{
    planner::{
        robot::{RobotDespawned, RobotSpawned},
        RobotConnections, RobotId,
    },
    simulation_loader::{LoadSimulation, ReloadSimulation},
    theme::{CatppuccinTheme, ColorAssociation, ColorFromCatppuccinColourExt, DisplayColour},
};

pub struct TracerVisualiserPlugin;

impl Plugin for TracerVisualiserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Traces>()
            .add_systems(
                FixedUpdate,
                track_robots,
                // track_robots.run_if(on_timer(Duration::from_secs_f32(SAMPLE_DELAY))),
            )
            .add_systems(
                Update,
                (
                    create_tracer_when_a_robot_is_spawned,
                    // track_robots.run_if(on_timer(Duration::from_secs_f32(SAMPLE_DELAY))),
                    draw_traces.run_if(enabled),
                    reset.run_if(
                        on_event::<LoadSimulation>().or_else(on_event::<ReloadSimulation>()),
                    ),
                ),
            );
    }
}

pub struct Trace {
    // color:       Color,
    color: DisplayColour,
    ring_buffer: StaticRb<Vec3, MAX_TRACE_LENGTH>,
}

/// **Bevy** [`Resource`] to store all robot traces
// Uses a ring buffer to store the traces, to ensure a maximum fixed size.
#[derive(Default, Resource)]
pub struct Traces(pub BTreeMap<RobotId, Trace>);

#[allow(dead_code)]
fn remove_trace_of_despawned_robot(
    mut traces: ResMut<Traces>,
    mut despawn_robot_event: EventReader<RobotDespawned>,
) {
    for RobotDespawned(robot_id) in despawn_robot_event.read() {
        if traces.0.remove(robot_id).is_some() {
            info!("removed trace of robot: {:?}", robot_id);
        } else {
            error!(
                "attempted to remove trace of untracked robot: {:?}",
                robot_id
            );
        }
    }
}

fn create_tracer_when_a_robot_is_spawned(
    query: Query<(RobotId, &Transform, &ColorAssociation), With<RobotConnections>>,
    mut traces: ResMut<Traces>,
    mut spawn_robot_event: EventReader<RobotSpawned>,
    // theme: Res<CatppuccinTheme>,
) {
    spawn_robot_event.read().for_each(|RobotSpawned(robot_id)| {
        for (other_robot_id, transform, color_association) in query.iter() {
            // initialise the first position of the robot into the ring buffer
            let mut ring_buffer = StaticRb::default();
            let mut position = transform.translation;
            position.y = 0.05;
            let _ = ring_buffer.push_overwrite(position);

            if other_robot_id == *robot_id {
                let _ = traces.0.entry(*robot_id).or_insert(Trace {
                    color: color_association.name,
                    ring_buffer,
                });
            }
        }
    });
}

/// **Bevy** [`Update`] system
/// To update the [`Traces`] resource
#[allow(clippy::type_complexity)]
fn track_robots(
    query: Query<
        (RobotId, &Transform, &ColorAssociation),
        (With<RobotConnections>, Changed<Transform>),
    >,
    mut traces: ResMut<Traces>,
) {
    for (robot_id, transform, color_association) in &query {
        let mut position = transform.translation;
        position.y = 0.05;
        let _ = traces
            .0
            .entry(robot_id)
            .or_insert(Trace {
                color: color_association.name,
                ring_buffer: StaticRb::default(),
            })
            .ring_buffer
            .push_overwrite(position);
    }
}

#[inline]
fn enabled(config: Res<Config>) -> bool {
    config.visualisation.draw.paths
}

/// **Bevy** [`Update`] system
/// To draw the robot traces; using the [`Traces`] resource
fn draw_traces(mut gizmos: Gizmos, traces: Res<Traces>, theme: Res<CatppuccinTheme>) {
    for trace in traces.0.values() {
        // use a window of length 2 to iterate over the trace, and draw a line between
        // each pair of points
        for (start, end) in trace.ring_buffer.iter().tuple_windows() {
            let color = Color::from_catppuccin_colour(theme.get_display_colour(&trace.color));
            gizmos.line(*start, *end, color);
        }
    }
}

#[inline]
fn reset(mut traces: ResMut<Traces>) {
    traces.0.clear();
}
