extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::{Button, Window};
use nwg::{ColorDialog, FontDialog, NativeUi, TextInput};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    text: String,
}

#[derive(Default, NwgUi)]
pub struct LoadConfigApp {
    #[nwg_control(size:(855,450),position: (300,300),title: "加载配置文件")]
    #[nwg_events( OnInit:[LoadConfigApp::init],OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: Window,

    // 颜色选择器
    color_dialog: ColorDialog,

    // 字体选择器
    font_dialog: FontDialog,

    #[nwg_control(text: "选择颜色",parent:window,size: (280, 70), position: (10, 120))]
    #[nwg_events( OnButtonClick: [LoadConfigApp::select_color] )]
    color_button: Button,

    #[nwg_control(text: "选择字体",parent:window,size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [LoadConfigApp::select_font] )]
    font_button: Button,

    #[nwg_control(parent:window,size: (280, 70), position: (10, 320))]
    text_input: TextInput,

    #[nwg_control(text: "保存",parent:window,size: (280, 70), position: (300, 320))]
    #[nwg_events( OnButtonClick: [LoadConfigApp::save] )]
    save_button: Button,
}

impl LoadConfigApp {
    fn init(&self) {
        if let Ok(data) = fs::read_to_string("config.json") {
            if let Ok(data) = serde_json::from_str::<Data>(data.as_str()) {
                self.text_input.set_text(data.text.as_str());
            }
        }
    }

    fn select_font(&self) {
        if self.font_dialog.run(Some(&self.window)) {
            println!("{:?}", self.font_dialog.font());
        }
    }

    fn select_color(&self) {
        if self.color_dialog.run(Some(&self.window)) {
            println!("{:?}", self.color_dialog.color());
        }
    }

    fn save(&self) {
        let data = self.text_input.text();
        let data = Data { text: data };

        let data = serde_json::to_string_pretty(&data).unwrap();
        if fs::write("config.json", data).is_ok() {
            nwg::modal_info_message(&self.window, "信息提示", "保存成功");
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = LoadConfigApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
