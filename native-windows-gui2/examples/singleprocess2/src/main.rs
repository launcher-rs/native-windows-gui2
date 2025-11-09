/*!
A very simple application that show your name in a message box.
See `basic` for the version without the derive macro
 */

extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::ptr;
use winapi::shared::minwindef::{BOOL, FALSE, LPARAM, TRUE};
use winapi::shared::ntdef::HANDLE;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    EnumWindows, GetPropW, IsWindow, SetForegroundWindow, SetPropW, ShowWindow, SW_NORMAL,
};

#[no_mangle]
#[used]
pub static mut G_H_VALUE: HANDLE = 1 as HANDLE;

pub static G_SZ_PROP_NAME: &str = "prop name111111111";

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 135), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    fn say_goodbye(&self) {
        // nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

pub fn to_utf16<'a>(s: &'a str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s)
        .encode_wide()
        .chain(Some(0u16).into_iter())
        .collect()
}

unsafe extern "system" fn enum_wnd_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let propname = to_utf16(G_SZ_PROP_NAME);
    let h = GetPropW(hwnd, propname.as_ptr());

    if h == G_H_VALUE {
        *(lparam as *mut HWND) = hwnd;
        return FALSE;
    }
    return TRUE;
}

fn main() {
    unsafe {
        let mut hwnd: HWND = ptr::null_mut();
        EnumWindows(Some(enum_wnd_proc), &mut hwnd as *mut HWND as LPARAM);
        if IsWindow(hwnd) == TRUE {
            ShowWindow(hwnd, SW_NORMAL);
            SetForegroundWindow(hwnd);
            println!("进程已经存在..........");
            return;
        }
    }

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    let handle = app.window.handle.hwnd().unwrap();
    unsafe {
        let propname = to_utf16(G_SZ_PROP_NAME);
        SetPropW(handle, propname.as_ptr(), G_H_VALUE);
    }

    nwg::dispatch_thread_events();
}
