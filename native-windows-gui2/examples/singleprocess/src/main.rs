/*!
A very simple application that show your name in a message box.
See `basic` for the version without the derive macro
 */

extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::ptr;
use winapi::shared::minwindef::BOOL;
use winapi::um::winuser::{FindWindowW, IsWindow, SW_NORMAL, SetForegroundWindow, ShowWindow};

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

pub fn is_process_single() -> bool {
    use winapi::shared::ntdef::HANDLE;
    use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
    use winapi::um::errhandlingapi::GetLastError;
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::synchapi::CreateMutexW;

    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let flags = "sdfhrh57b56dste6hjghjgh";
    let flags: Vec<u16> = OsStr::new(flags).encode_wide().chain(Some(0u16)).collect();

    unsafe {
        let h_mutex: HANDLE = CreateMutexW(ptr::null_mut(), 0, flags.as_ptr());

        if ERROR_ALREADY_EXISTS == GetLastError() {
            if !h_mutex.is_null() {
                CloseHandle(h_mutex);
            }
            return true;
        }

        false
    }
}

pub fn to_utf16(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().chain(Some(0u16)).collect()
}

fn main() {
    // 方式一
    // if is_process_single() {return; }

    // 方式二： 发现并激活当前窗口,不好用
    unsafe {
        let class_name = to_utf16("ClassName");
        let window_name = to_utf16("Caption11");
        let h_wnd = FindWindowW(class_name.as_ptr(), window_name.as_ptr());
        let result: BOOL = IsWindow(h_wnd);
        if result != 0 {
            ShowWindow(h_wnd, SW_NORMAL);
            SetForegroundWindow(h_wnd);
            return;
        }
    }

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
