#![allow(missing_docs)]
use super::factor::ExternalVariableId;

/// Represents a factorgraph node in the graphviz output
pub struct Node {
    /// The index of the node
    pub index: usize,
    /// The kind of the node
    pub kind: NodeKind,
}

impl Node {
    /// Returns the color of the node
    pub const fn color(&self) -> &'static str {
        self.kind.color()
    }

    /// Returns the shape of the node
    pub const fn shape(&self) -> &'static str {
        self.kind.shape()
    }

    /// Returns the width of the node
    pub const fn width(&self) -> f64 {
        self.kind.width()
    }
}

pub enum NodeKind {
    Variable {
        // It is not dead `rustc` ...
        #[allow(dead_code)]
        x: f64,
        // It is not dead `rustc` ...
        #[allow(dead_code)]
        y: f64,
    },
    InterRobotFactor {
        // It is not dead `rustc` ...
        #[allow(dead_code)]
        active: bool,
        external_variable_id: ExternalVariableId,
    },
    // InterRobotFactor {
    //     /// The id of the robot the interrobot factor is connected to
    //     other_robot_id: RobotId,
    //     /// The index of the variable in the other robots factorgraph, that the interrobot
    // factor is connected with     variable_index_in_other_robot: usize,
    // },
    DynamicFactor,
    ObstacleFactor,
    TrackingFactor, // PoseFactor,
}

impl NodeKind {
    pub const fn color(&self) -> &'static str {
        match self {
            Self::Variable { .. } => "#eff1f5",         // latte base (white)
            Self::InterRobotFactor { .. } => "#a6da95", // green
            Self::DynamicFactor => "#8aadf4",           // blue
            Self::ObstacleFactor => "#ee99a0",          // mauve (purple)
            // Self::PoseFactor => "#c6aof6",     // maroon (red)
            Self::TrackingFactor => "#f4a15a", // orange
        }
    }

    pub const fn shape(&self) -> &'static str {
        match self {
            Self::Variable { .. } => "circle",
            _ => "square",
        }
    }

    pub const fn width(&self) -> f64 {
        match self {
            Self::Variable { .. } => 0.8,
            _ => 0.2,
        }
    }
}

pub struct Edge {
    pub from: usize,
    pub to: usize,
}

pub trait ExportGraph {
    fn export_graph(&self) -> (Vec<Node>, Vec<Edge>);
}
