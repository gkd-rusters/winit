#![cfg(target_os = "macos")]

mod activation_hack;
mod app;
mod app_delegate;
mod app_state;
mod event;
mod event_loop;
mod ffi;
mod monitor;
mod observer;
mod util;
mod view;
mod window;
mod window_delegate;

use std::{fmt, ops::Deref, sync::Arc};

use window::get_window_cocoa_id;

use self::util::IdRef;
pub use self::{
    event_loop::{EventLoop, EventLoopWindowTarget, Proxy as EventLoopProxy},
    monitor::{available_monitors, MonitorHandle, VideoMode},
    window::{Id as WindowId, PlatformSpecificWindowBuilderAttributes, UnownedWindow},
};
use crate::{
    error::OsError as RootOsError, event::DeviceId as RootDeviceId, window::WindowAttributes,
};

pub(crate) use crate::icon::NoIcon as PlatformIcon;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceId;

impl DeviceId {
    pub unsafe fn dummy() -> Self {
        DeviceId
    }
}

// Constant device ID; to be removed when if backend is updated to report real device IDs.
pub(crate) const DEVICE_ID: RootDeviceId = RootDeviceId(DeviceId);

pub struct Window {
    window: Arc<UnownedWindow>,
    // We keep this around so that it doesn't get dropped until the window does.
    _delegate: util::IdRef,
}

#[derive(Debug)]
pub enum OsError {
    CGError(core_graphics::base::CGError),
    CreationError(&'static str),
}

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

impl Deref for Window {
    type Target = UnownedWindow;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.window
    }
}

impl Window {
    pub fn new<T: 'static>(
        _window_target: &EventLoopWindowTarget<T>,
        attributes: WindowAttributes,
        pl_attribs: PlatformSpecificWindowBuilderAttributes,
    ) -> Result<Self, RootOsError> {
        let parent_window_id = attributes.parent_window_id;
        let (window, _delegate) = UnownedWindow::new(attributes, pl_attribs)?;
        // set child to parent
        if let Some(parent_window_id) = parent_window_id {
            let parent_window_id: cocoa::base::id = get_window_cocoa_id(parent_window_id.0);
            window.add_child_to(IdRef::retain(parent_window_id));
        }
        Ok(Window { window, _delegate })
    }

    pub fn add_child_to(&self, parent_window_id: WindowId) {
        let parent_window_id: cocoa::base::id = get_window_cocoa_id(parent_window_id);
        self.window.add_child_to(IdRef::retain(parent_window_id));
    }

    pub fn child_windows(&self) {
        self.window.child_windows();
    }

    pub fn remove_self_as_child_from_parent(&self) {
        self.window.remove_self_as_child_from_parent();
    }

    pub fn close_window(&self) {
        self.window.close_window();
    }
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsError::CGError(e) => f.pad(&format!("CGError {}", e)),
            OsError::CreationError(e) => f.pad(e),
        }
    }
}
