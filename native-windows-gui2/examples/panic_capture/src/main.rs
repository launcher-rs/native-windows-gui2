/*!
A very simple application that show your name in a message box.
See `basic` for the version without the derive macro
 */

extern crate core;
extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use std::{fs, panic};

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 135), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [ nwg::stop_thread_dispatch() ] )]
    window: nwg::Window,

    #[nwg_control(text: "panic", size: (100, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    panic_button: nwg::Button,

    #[nwg_control(text: "ok", size: (100, 70), position: (140, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_ok] )]
    ok_button: nwg::Button,
}

impl BasicApp {
    fn say_hello(&self) {
        let _ = panic::catch_unwind(|| {
            println!("hello world");
            fs::read_to_string("seb.txt").unwrap();
        });
    }
    fn say_ok(&self) {
        let result = panic::catch_unwind(|| {
            println!("hello world");
            "hello 你好"
        });
        match result {
            Ok(res) => {
                println!("ok {:?}", res);
            }
            Err(err) => {
                println!("err {:?}", err);
            }
        }
    }
}

fn main() {
    // panic::catch_unwind +  panic::set_hook yyds
    panic::set_hook(Box::new(|err| {
        // 可以写日志
        println!("发送错误: {}", err)
    }));

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
