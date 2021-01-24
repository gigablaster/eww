use crate::geometry::*;
use anyhow::*;

#[cfg(feature = "x11")]
pub mod x11;

#[cfg(feature = "wayland")]
pub mod wayland;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StackingStrategy {
    AlwaysOnTop,
    AlwaysOnBottom,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MonitorData {
    pub port_name: String,
    pub primary: bool,
    pub rect: Rect,
}

impl Rectangular for MonitorData {
    fn get_rect(&self) -> Rect {
        self.rect
    }
}

pub trait DisplayBackend {
    type WinId: Copy + std::fmt::Debug;

    fn get_monitors(&self) -> Result<Vec<MonitorData>>;
    fn get_primary_monitor(&self) -> Result<MonitorData>;

    fn map_window(&self, win: Self::WinId) -> Result<()>;

    fn place_window_at(&self, win: Self::WinId, x: i32, y: i32) -> Result<()>;
    fn resize_window(&self, win: Self::WinId, width: u32, height: u32) -> Result<()>;
    fn set_stacking_strategy(&self, win: Self::WinId, strategy: StackingStrategy) -> Result<()>;
    fn set_as_dock(&self, win: Self::WinId) -> Result<()>;
    fn set_unmanaged(&self, win: Self::WinId) -> Result<()>;
    fn set_application_id<S: AsRef<str>>(&self, win: Self::WinId, id: S) -> Result<()>;
    fn get_window_id_of(&self, window: &gtk4::Window) -> Self::WinId;

    fn get_monitor(&self, name: &str) -> Result<MonitorData> {
        self.get_monitors()?
            .into_iter()
            .find(|m| &m.port_name == name)
            .context(format!("No monitor named {} found", name))
    }
}

#[cfg(target_os = "macos")]
pub fn get_backend() -> Result<impl DisplayBackend> {
    unimplemented!()
}
#[cfg(feature = "x11")]
pub fn get_backend() -> Result<impl DisplayBackend> {
    x11::X11Backend::new()
}
#[cfg(feature = "wayland")]
pub fn get_backend() -> Result<impl DisplayBackend> {
    wayland::WaylandBackend::new()
}