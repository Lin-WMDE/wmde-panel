//! Config for cosmic-panel
#[cfg(feature = "wayland-rs")]
mod container_config;
mod panel_config;

#[cfg(feature = "wayland-rs")]
pub use container_config::*;
pub use panel_config::*;

/// Types of `CosmicPanelConfig` fields that live in the wrapper crate. Re-exported so that
/// writing a panel config does not mean depending on the wrapper as well.
#[cfg(feature = "wayland-rs")]
pub use xdg_shell_wrapper_config::{KeyboardInteractivity, Layer};
