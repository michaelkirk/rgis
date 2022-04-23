#![warn(
    clippy::unwrap_used,
    clippy::cast_lossless,
    clippy::unimplemented,
    clippy::expect_used
)]

use bevy::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::path;

const ZOOM_AMOUNT: f32 = 1.15; // Larger number will zoom more

#[derive(Debug)]
pub struct LayerLoadedEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct ToggleLayerVisibilityEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct CenterCameraEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct LayerBecameHiddenEvent(pub rgis_layer_id::LayerId);

#[derive(Debug)]
pub struct LayerBecameVisibleEvent(pub rgis_layer_id::LayerId);

/// Change the `Layer`'s color
pub struct UpdateLayerColorEvent(pub rgis_layer_id::LayerId, pub bevy::prelude::Color);
/// After a `Layer`'s color is changed
pub struct LayerColorUpdatedEvent(pub rgis_layer_id::LayerId);

pub struct DeleteLayerEvent(pub rgis_layer_id::LayerId);
pub struct LayerDeletedEvent(pub rgis_layer_id::LayerId);

pub enum MoveDirection {
    Up,
    Down,
}
pub struct MoveLayerEvent(pub rgis_layer_id::LayerId, pub MoveDirection);

pub struct LayerZIndexUpdatedEvent(pub rgis_layer_id::LayerId);

// TODO: this can be removed
#[derive(Debug)]
pub enum LoadGeoJsonFileEvent {
    #[cfg(not(target_arch = "wasm32"))]
    FromPath { path: path::PathBuf, crs: String },
    FromNetwork {
        name: String,
        url: String,
        crs: String,
    },
    FromBytes {
        file_name: String,
        bytes: Vec<u8>,
        crs: String,
    },
}

pub struct Plugin;

#[derive(Debug)]
pub struct PanCameraEvent {
    // X offset for camera position. Positive is right, negative is left.
    pub x: f32,
    // Y offset for camera position. Positive is up, negative is down.
    pub y: f32,
}

impl PanCameraEvent {
    pub fn up(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: amount }
    }

    pub fn right(amount: f32) -> Self {
        PanCameraEvent { x: amount, y: 0. }
    }

    pub fn down(amount: f32) -> Self {
        PanCameraEvent { x: 0., y: -amount }
    }

    pub fn left(amount: f32) -> Self {
        PanCameraEvent { x: -amount, y: 0. }
    }
}

#[derive(Debug)]
pub struct ZoomCameraEvent {
    // (amount ∈ (1, ∞)) → zoom in
    // (amount ∈ [1] → no change
    // (amount ∈ (0, 1)) → zoom out
    pub amount: f32,
}

impl ZoomCameraEvent {
    pub fn zoom_in() -> Self {
        ZoomCameraEvent {
            amount: ZOOM_AMOUNT,
        }
    }

    pub fn zoom_out() -> Self {
        ZoomCameraEvent {
            amount: 1. / ZOOM_AMOUNT,
        }
    }
}

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LoadGeoJsonFileEvent>()
            .add_event::<LayerLoadedEvent>()
            .add_event::<ToggleLayerVisibilityEvent>()
            .add_event::<LayerBecameHiddenEvent>()
            .add_event::<LayerBecameVisibleEvent>()
            .add_event::<PanCameraEvent>()
            .add_event::<ZoomCameraEvent>()
            .add_event::<CenterCameraEvent>()
            .add_event::<LayerColorUpdatedEvent>()
            .add_event::<UpdateLayerColorEvent>()
            .add_event::<MoveLayerEvent>()
            .add_event::<LayerZIndexUpdatedEvent>()
            .add_event::<DeleteLayerEvent>()
            .add_event::<LayerDeletedEvent>();
    }
}
