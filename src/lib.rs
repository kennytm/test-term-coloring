#![feature(libc)]

extern crate libc;

// Ref: http://hermanradtke.com/2015/01/12/terminal-window-size-with-rust-ffi.html

#[cfg(unix)]
use libc::{STDOUT_FILENO, TIOCGWINSZ, ioctl, winsize};

pub fn column_width() -> u16 {
    #[cfg(unix)]
    unsafe {
        let mut w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut w);
        w.ws_col
    }
}

#[test]
fn fetch_column_width() {
    assert_eq!(column_width(), 235);
}
