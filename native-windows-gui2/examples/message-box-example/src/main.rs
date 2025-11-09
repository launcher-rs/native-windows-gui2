/*!
A very simple application that show your name in a message box.
See `basic` for the version without the derive macro
 */

extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{AlignContent, Dimension as D, FlexDirection, FlexWrap},
};
use nwg::{FlexboxLayout, NativeUi};

const PT_5: D = D::Points(1.0);
// const FIFTY_PC: D = D::Percent(0.5);
const PT_10: D = D::Points(10.0);
// const PT_20: D = D::Points(20.0);
const PADDING: Rect<D> = Rect {
    start: PT_10,
    end: PT_10,
    top: PT_10,
    bottom: PT_10,
};
const MARGIN: Rect<D> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

const WIDTH: D = D::Points(200.0);

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (700, 450), position: (300, 300), title: "Flexbox example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    #[nwg_layout(parent: window, padding: PADDING,flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_content: AlignContent::FlexStart)]
    layout: FlexboxLayout,

    #[nwg_control(text: "error_message",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::error_message] )]
    error_message_button: nwg::Button,

    #[nwg_control(text: "simple_message",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::simple_message] )]
    simple_message_button: nwg::Button,

    #[nwg_control(text: "modal_fatal_message",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::modal_fatal_message] )]
    modal_fatal_message_button: nwg::Button,

    #[nwg_control(text: "modal_error_message",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::modal_error_message] )]
    modal_error_message_button: nwg::Button,

    #[nwg_control(text: "modal_info_message",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::modal_info_message] )]
    modal_info_message_button: nwg::Button,

    #[nwg_control(text: "自定义无声音",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::custom_message] )]
    custom_button: nwg::Button,

    #[nwg_control(text: "自定义提示框",parent:window)]
    #[nwg_layout_item(layout: layout, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    #[nwg_events( OnButtonClick: [BasicApp::custom_prompt_message] )]
    custom_prompt_button: nwg::Button,
}

impl BasicApp {
    // fn fatal_message(&self) {
    //     nwg::fatal_message("fatal_message", "fatal_message");
    // }

    fn error_message(&self) {
        nwg::error_message("error_message", "error_message");
    }

    fn simple_message(&self) {
        nwg::simple_message("simple_message", "simple_message");
    }

    fn modal_fatal_message(&self) {
        nwg::modal_fatal_message(&self.window, "modal_fatal_message", "modal_fatal_message");
    }

    fn modal_error_message(&self) {
        nwg::modal_error_message(&self.window, "modal_error_message", "modal_error_message");
    }

    fn modal_info_message(&self) {
        nwg::modal_info_message(&self.window, "modal_info_message", "modal_info_message");
    }

    fn custom_message(&self) {
        println!("好像默认不支持取消声音，声音是系统设置的")
    }

    fn custom_prompt_message(&self) {
        let p = nwg::MessageParams {
            title: "温馨提示",
            content: "确定要删除吗?",
            buttons: nwg::MessageButtons::OkCancel,
            icons: nwg::MessageIcons::Question,
        };

        if nwg::message(&p) == nwg::MessageChoice::Ok {
            println!("文件删除.........");
        } else {
            println!("删除操作取消");
        }
    }

    // fn exit(&self) {
    //     nwg::stop_thread_dispatch();
    // }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
