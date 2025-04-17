//! Useful function when working with bevy

use bevy::{app::Plugin, ecs::prelude::*, hierarchy::DespawnRecursiveExt};

/// Prelude module bringing entire public api of this module into scope
#[allow(unused_imports)]
pub mod prelude {
    pub use super::*;
}

/// Generic system that takes a component as a parameter, and will despawn all
/// entities with that component
///
/// # Example
/// ```rust
/// use bevy::prelude::*;
/// #[derive(Component)]
/// struct OnSplashScreen;
///
/// #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
/// enum GameState {
///     #[default]
///     Splash,
///     Menu,
///     Game,
/// }
///
/// App::new()
///     .add_systems(
///         OnExit(GameState::Splash),
///         despawn_entities_with_component::<OnSplashScreen>,
///     )
///     .run();
/// ```
#[allow(dead_code)]
pub fn despawn_entities_with_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

/// Extension trait for `bevy::app::App`
pub trait BevyAppExt {
    /// Attempt to add a [`Plugin`]
    /// If the plugin is already added, then do nothing.
    /// This is an alternative to [`.add_plugins()`] which will panic if a
    /// plugin has already been added.
    #[allow(dead_code)]
    fn try_add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self;

    /// Check if an `Event` has been added to the world.
    #[allow(dead_code)]
    fn event_exists<E: Event>(&self) -> bool;
}

impl BevyAppExt for bevy::app::App {
    #[allow(dead_code)]
    fn try_add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        if !self.is_plugin_added::<P>() {
            self.add_plugins(plugin);
        }
        self
    }

    #[allow(dead_code)]
    fn event_exists<E: Event>(&self) -> bool {
        self.world.contains_resource::<Events<E>>()
    }
}

pub mod run_conditions {
    use bevy::ecs::{
        event::{Event, Events},
        system::Res,
    };

    pub mod time {
        use bevy::{
            ecs::system::Res,
            time::{Time, Virtual},
        };

        /// run criteria if time is not paused
        #[inline]
        pub fn virtual_time_is_paused(time: Res<Time<Virtual>>) -> bool {
            time.is_paused()
        }
    }

    // pub fn on_any_event<E: Event, const N: usize>()

    /// Trait for checking if an event exists
    #[inline]
    pub const fn event_exists<T: Event>(res_event: Option<Res<Events<T>>>) -> bool {
        res_event.is_some()
    }

    //     pub fn any_input_just_pressed(
    //         // inputs: impl IntoIterator<Item = ButtonInput<KeyCode>>,
    //         // inputs: impl IntoIterator<Item = KeyCode>,
    //         // inputs: Vec<KeyCode>,
    //     ) -> impl Fn(Res<ButtonInput<KeyCode>>) -> bool
    // // where
    //     //     T: Copy + Eq + Send + Sync + 'static,
    //     {
    //         move |keyboard_input: Res<ButtonInput<KeyCode>>|
    // keyboard_input.any_pressed(inputs)

    //         // move |keyboard_input: Res<ButtonInput<T>>| {
    //         //     inputs.into_iter().any(|it|
    // keyboard_input.just_pressed(it))         // }
    //     }
}

// pub mod state {
//     use bevy::prelude::*;

//     pub fn echo_state<S: States>() -> impl Fn(Res<State<S>>) {
//         move |state: Res<State<S>>| {
//             info!("{} state is: {:?}", std::any::type_name::<S>(), state.get());
//         }
//     }

//     pub fn enter_state<S: States + Copy>(state: S) -> impl FnMut(ResMut<NextState<S>>) {
//         move |mut next_state: ResMut<NextState<S>>| {
//             next_state.set(state);
//         }
//     }
// }
