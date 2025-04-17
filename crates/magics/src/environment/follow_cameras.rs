use std::f32::consts::PI;

use ::bevy::prelude::*;

use crate::movement::{Local, OrbitMovementBundle, Velocity};

pub struct FollowCamerasPlugin;

impl Plugin for FollowCamerasPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_cameras, add_follow_cameras));
    }
}

/// TODO: should be named to something else as only the proportional part `p` is used
#[allow(clippy::upper_case_acronyms)]
#[derive(Component)]
pub struct PID {
    pub p: f32,
    // pub i: f32,
    // pub d: f32,
}

impl Default for PID {
    fn default() -> Self {
        Self {
            p: 1.0,
            // i: 0.0,
            // d: 0.0,
        }
    }
}

/// `Component` to tag an entity to be followed by a `FollowCamera`
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct FollowCameraMe {
    pub offset: Option<Vec3>,
    pub up_direction: Option<Direction3d>,
    pub attached: bool,
}

impl From<Vec3> for FollowCameraMe {
    fn from(v: Vec3) -> Self {
        Self {
            offset: Some(v),
            up_direction: None,
            attached: false,
        }
    }
}

impl FollowCameraMe {
    #[must_use]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            offset: Some(Vec3::new(x, y, z)),
            up_direction: None,
            attached: false,
        }
    }

    // pub const fn from_vec3(offset: Vec3) -> Self {
    //     Self {
    //         offset:       Some(offset),
    //         up_direction: None,
    //         attached:     false,
    //     }
    // }

    #[must_use]
    pub const fn with_up_direction(mut self, up_direction: Direction3d) -> Self {
        self.up_direction = Some(up_direction);
        self
    }

    #[must_use]
    pub const fn with_attached(mut self, attached: bool) -> Self {
        self.attached = attached;
        self
    }
}

/// `Component` to store the settings for a `FollowCamera`
#[derive(Component)]
pub struct FollowCameraSettings {
    pub target: Entity,
    pub offset: Vec3,
    pub pid: PID,
}

impl FollowCameraSettings {
    pub fn new(target: Entity) -> Self {
        Self {
            target,
            offset: Vec3::new(0.0, 5.0, -10.0).normalize() * 10.0,
            pid: PID {
                p: 1.0,
                // ..Default::default()
            },
        }
    }

    #[must_use]
    pub const fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }
}

// **Bevy** marker [`Component`] for follow cameras that are attached as
// children to other entities
#[derive(Component, PartialEq, Eq)]
pub enum CameraType {
    Attached,
    Free,
}

/// Bundle for a [`FollowCamera`] entity
#[derive(Bundle)]
pub struct FollowCameraBundle {
    pub settings: FollowCameraSettings,
    pub movement: OrbitMovementBundle,
    pub velocity: Velocity,
    pub camera: Camera3dBundle,
    pub camera_type: CameraType,
}

impl FollowCameraBundle {
    fn new(
        entity: Entity,
        target: Option<&Transform>,
        // offset: Option<Vec3>,
        // up_direction: Option<Direction3d>,
        // attached: bool,
        params: FollowCameraMe,
    ) -> Self {
        let target_transform =
            target.map_or_else(|| Transform::from_translation(Vec3::ZERO), |t| *t);
        // let offset = Vec3::new(0.0, 5.0, -10.0).normalize() * 10.0;
        let offset = params
            .offset
            .map_or_else(|| Vec3::new(0.0, 5.0, -10.0).normalize() * 10.0, |o| o);

        let up_direction = params.up_direction.map_or(Direction3d::Y, |u| u);

        let (camera_type, transform) = if params.attached {
            (
                CameraType::Attached,
                Transform::from_translation(offset).looking_at(Vec3::ZERO, up_direction.into()),
            )
        } else {
            (
                CameraType::Free,
                Transform::from_translation(target_transform.translation + offset)
                    .looking_at(target_transform.translation, up_direction.into()),
            )
        };

        // TODO: Maybe add this back in
        // transform offset to local space of target entity
        // let offset = target.rotation.inverse() * offset;
        // let offset = (target.compute_matrix() * offset.extend(1.0)).xyz();

        Self {
            settings: FollowCameraSettings::new(entity).with_offset(offset),
            movement: OrbitMovementBundle::default(),
            velocity: Velocity(Vec3::ZERO),
            camera: Camera3dBundle {
                // transform: Transform::from_translation(target_transform.translation + offset)
                //     .looking_at(target_transform.translation, up_direction.into()),
                transform,
                camera: Camera {
                    is_active: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            camera_type,
        }
    }
}

/// `Update` system to add a `FollowCamera` to any entity with a tagged to be
/// followed with a `FollowCameraMe` component
fn add_follow_cameras(
    mut commands: Commands,
    entities_to_attach_a_follow_cam_to: Query<(Entity, &Transform, &FollowCameraMe)>,
    cameras: Query<Entity, With<Camera3d>>,
    children: Query<&Children>,
) {
    for (entity, transform, follow_camera_flag) in &entities_to_attach_a_follow_cam_to {
        let camera_already_attached = children
            .iter_descendants(entity)
            .any(|e| cameras.get(e).is_ok());

        if camera_already_attached {
            // an entity can only have one follower camera attached to it
            continue;
        }

        let follower_camera = commands
            .spawn((
                FollowCameraBundle::new(entity, Some(transform), *follow_camera_flag),
                Local,
            ))
            .id();

        // Make the camera a child of the entity
        commands.entity(entity).push_children(&[follower_camera]);
    }
}

/// `Update` system to move all cameras tagged with the `FollowCamera` component
/// Queries for all targets with `Transforms` and their corresponding cameras
/// with `FollowCameraSettings` to move cameras correctly
#[allow(clippy::type_complexity)]
fn move_cameras(
    mut query_cameras: Query<(&mut Transform, &FollowCameraSettings, &CameraType), With<Camera>>,
    query_targets: Query<(Entity, &Transform), (With<FollowCameraMe>, Without<Camera>)>,
) {
    for (mut camera_transform, follow_settings, cam_type) in &mut query_cameras {
        if matches!(cam_type, CameraType::Attached) {
            continue;
        }
        for (target_entity, target_transform) in query_targets.iter() {
            if target_entity == follow_settings.target {
                let (target_yaw, ..) = target_transform.rotation.to_euler(EulerRot::YXZ);
                let (camera_yaw, ..) = camera_transform.rotation.to_euler(EulerRot::YXZ);
                let mut delta_yaw = (target_yaw + PI) - camera_yaw;

                if delta_yaw > PI {
                    delta_yaw -= PI * 2.0;
                } else if delta_yaw < -PI {
                    delta_yaw += PI * 2.0;
                }

                let rotate_by_yaw = Quat::from_axis_angle(Vec3::Y, target_yaw);
                let offset = rotate_by_yaw * follow_settings.offset;

                let target_position = target_transform.translation + offset;

                let delta = target_position - camera_transform.translation;
                let distance = delta.length();

                if distance < f32::EPSILON {
                    continue;
                }

                camera_transform.translation += delta * follow_settings.pid.p;
                // rotate by yaw
                camera_transform.rotate(Quat::from_axis_angle(Vec3::Y, delta_yaw));
            }
        }
    }
}
