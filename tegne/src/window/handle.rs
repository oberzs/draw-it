// Oliver Berzs
// https://github.com/OllieBerzs/tegne-rs

// WindowHandle - OS window properties for surface creation

use std::os::raw::c_void;

#[cfg(target_os = "windows")]
#[derive(Copy, Clone)]
pub struct WindowHandle {
    pub hwnd: *const c_void,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "linux")]
#[derive(Copy, Clone)]
pub struct WindowHandle {
    pub xlib_window: std::os::raw::c_ulong,
    pub xlib_display: *mut c_void,
    pub width: u32,
    pub height: u32,
}

#[cfg(target_os = "macos")]
#[derive(Copy, Clone)]
pub struct WindowHandle {
    pub ns_window: *mut c_void,
    pub ns_view: *mut c_void,
    pub width: u32,
    pub height: u32,
}
