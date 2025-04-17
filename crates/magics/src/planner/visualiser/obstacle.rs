//! Visualize how obstacle factors measure distance to a solid object in the
//! environment.

use bevy::prelude::*;
use gbp_config::Config;

use crate::factorgraph::prelude::FactorGraph;

#[derive(Default)]
pub struct ObstacleFactorVisualizerPlugin;

impl Plugin for ObstacleFactorVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visualize_obstacle_factors.run_if(enabled));
    }
}

// #[derive(Clone, Copy, Event, PartialEq, Eq)]
// pub enum ChangeBooleanSetting<T> {
//     On,
//     Off,
//     Toggle,
// }

// // fn system(ev: EventReader<ChangeBooleanSetting<Visual<ObstacleFactor>>>)
// {} fn system(ev: EventReader<events::ChangeBooleanSetting>) {}

// pub struct Visual<T>;
// pub struct ObstacleFactor;

// pub mod events {
//     use bevy::prelude::*;

//     pub type ChangeBooleanSetting =
//         super::ChangeBooleanSetting<super::Visual<super::ObstacleFactor>>;

//     fn handle_event() {}
// }

fn gradient(a: &Color, b: &Color) -> colorgrad::Gradient {
    // let [r, g, b] = a.
    // let a colorgrad::Color::from_linear_rgba(r, g, b, a)
    let a = colorgrad::Color::from_linear_rgba(a.r() as f64, a.g() as f64, a.b() as f64, 255.0);
    let b = colorgrad::Color::from_linear_rgba(b.r() as f64, b.g() as f64, b.b() as f64, 255.0);
    colorgrad::CustomGradient::new()
        .colors(&[a, b])
        .domain(&[0.0, 1.0])
        .mode(colorgrad::BlendMode::Hsv)
        .build()
        .unwrap()
}

// pub fn measurement_color(value: f32) -> bevy::render::color::Color {
//     let value = value.clamp(0.0, 1.0);
//     let r = value;
//     let g = 1.0 - r;
//     let b = 0.0;
//     bevy::render::color::Color::rgb(r, g, b)
// }

/// Draw a line between a variables estimated position and the sample point of
/// its connected obstacle factor. It uses the Gizmos API to draw a line between
/// the two points The color of the line is based on the measured value of the
/// obstacle factor. 1.0 -> rgb(255, 0, 0)
/// 0.0 -> rgb(0, 255, 0)
/// 0.5 -> rgb(128, 128, 0)
/// r = 255 * (1 - value)
/// g = 255 * value
/// b = 0
fn visualize_obstacle_factors(
    mut gizmos: Gizmos,
    factorgraphs: Query<&FactorGraph>,
    config: Res<Config>,
    // theme: Res<crate::theme::CatppuccinTheme>,
) {
    let height = -config.visualisation.height.objects;

    // let red: Color = Color::from_catppuccin_colour(theme.red());
    // let green: Color = Color::from_catppuccin_colour(theme.green());
    let red = Color::RED;
    let green = Color::GREEN;
    // let green = theme.green();
    let gradient = gradient(&green, &red);

    for factorgraph in &factorgraphs {
        for (variable, obstacle_factor) in factorgraph.variable_and_their_obstacle_factors() {
            // let estimated_position = variable.estimated_position_vec2();
            let last_measurement = obstacle_factor.last_measurement();

            // let Some(last_measurement) = obstacle_factor.last_measurement() else {
            //     continue;
            // };

            // let scale = 1e2;
            // let v: f32 = (last_measurement.value as f32 * 1e2).clamp(0.0, 1.0);
            let v: f32 = (last_measurement.value as f32).clamp(0.0, 1.0);
            // let red = 1.0 * v;
            // let green = 1.0 - red;
            // let color = Color::rgb(red, green, 0.0);

            // let color = measurement_color(last_measurement.value as f32);

            // let red = catppuccin::Colour(255u8, 0u8, 0u8);
            // let green = catppuccin::Colour(0u8, 255u8, 0u8);
            // let gradient = theme.gradient(red, green);

            let [r, g, b, _] = gradient.at(v as f64).to_array();
            let color = Color::rgb(r as f32, g as f32, b as f32);
            // let height = 0.5f32;
            // let scale: f32 = 1.1;
            // [x, y]
            // [x, y, 0]
            // [x, 0, y]
            // let start = estimated_position.extend(height).xzy();
            // let end = scale * last_measurement.pos.extend(height).xzy();
            // gizmos.line(start, end, color);
            let pos = last_measurement.pos.extend(height).xzy();
            gizmos.circle(
                pos,
                Direction3d::Y,
                // Vec3::NEG_Z.di,
                0.5,
                color,
            );

            gizmos.line(
                variable.estimated_position_vec2().extend(height).xzy(),
                pos,
                color,
            );
        }
    }
}

/// **Bevy** run condition for drawing obstacle factors
#[inline]
fn enabled(config: Res<Config>) -> bool {
    config.visualisation.draw.obstacle_factors && config.gbp.factors_enabled.obstacle
}
