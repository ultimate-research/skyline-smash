use skyline::libc::c_char;
use nnsdk::ui2d::{Pane, TextBox};

// TODO: Offset search
#[skyline::from_offset(0x59970)]
pub unsafe fn find_pane_by_name_recursive(
    pane: *const Pane,
    s: *const c_char,
) -> *mut Pane;

#[skyline::from_offset(0x583c0)]
pub unsafe fn find_pane_by_name(
    pane: *const Pane,
    s: *const c_char,
    recursive: bool,
) -> *mut Pane;

#[skyline::from_offset(0x37a1270)]
pub unsafe fn pane_set_text_string(pane: *mut TextBox, s: *const c_char);

#[skyline::from_offset(0x58290)]
pub unsafe fn pane_remove_child(pane: *mut Pane, child: *const Pane);

#[skyline::from_offset(0x58250)]
pub unsafe fn pane_append_child(pane: *mut Pane, child: *const Pane);

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr()
    };
}

pub trait SmashPane {
    unsafe fn find_pane_by_name_recursive(&self, s: &str) -> Option<&mut Pane>;
    unsafe fn find_pane_by_name(&self, s: &str, recursive: bool) -> Option<&mut Pane>;
    unsafe fn remove_child(&mut self, child: &Pane);
    unsafe fn append_child(&mut self, child: &Pane);
    unsafe fn detach(&mut self);
}

impl SmashPane for Pane {
    unsafe fn find_pane_by_name_recursive(&self, s: &str) -> Option<&mut Pane> {
        find_pane_by_name_recursive(self, c_str!(s)).as_mut()
    }

    unsafe fn find_pane_by_name(&self, s: &str, recursive: bool) -> Option<&mut Pane> {
        find_pane_by_name(self, c_str!(s), recursive).as_mut()
    }

    unsafe fn remove_child(&mut self, child: &Pane) {
        pane_remove_child(self, child as *const Pane);
    }

    unsafe fn append_child(&mut self, child: &Pane) {
        pane_append_child(self, child as *const Pane);
    }

    /// Detach from current parent pane
    unsafe fn detach(&mut self) {
        pane_remove_child(self.parent, self as *const Pane);
    }
}

pub trait SmashTextBox {
    unsafe fn set_text_string(&mut self, s: &str);
}


impl SmashTextBox for TextBox {
    unsafe fn set_text_string(&mut self, s: &str) {
        pane_set_text_string(self, c_str!(s));
    }
}
