extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgPartial;
use nwg::stretch::{
    geometry::{Rect, Size},
    style::{AlignContent, Dimension as D, FlexDirection, FlexWrap},
};

const PT_5: D = D::Points(1.0);
const MARGIN: Rect<D> = Rect {
    start: PT_5,
    end: PT_5,
    top: PT_5,
    bottom: PT_5,
};

const WIDTH: D = D::Points(100.0);

#[derive(Default, NwgPartial)]
pub struct Tab1Ui {
    // #[nwg_control]
    // frame: nwg::Frame,
    #[nwg_layout(flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_content: AlignContent::FlexStart,
        padding: Rect { start: D::Undefined, end: D::Undefined, top: D::Points(5.0), bottom: D::Undefined }
    )]
    layout1: nwg::FlexboxLayout,

    #[nwg_control(focus: true,flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP")]
    #[nwg_layout_item(layout: layout1,size: Size { width: D::Percent(0.5), height: D::Percent(0.5) })]
    text_box: nwg::TextBox,

    #[nwg_control(focus: true,flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP")]
    #[nwg_layout_item(layout: layout1,size: Size { width: D::Percent(0.5), height: D::Percent(0.5) })]
    show_box: nwg::TextBox,

    //
    #[nwg_control(text: "Btn 1")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    // #[nwg_events( OnButtonClick: [Tab1Ui::say_hello] )]
    #[nwg_events( OnButtonClick:[ Tab1Ui::say_hello ] )]
    button1: nwg::Button,

    #[nwg_control(text: "Btn 2")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button2: nwg::Button,

    #[nwg_control(text: "Btn 3")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button3: nwg::Button,

    #[nwg_control(text: "Btn 2")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button21: nwg::Button,

    #[nwg_control(text: "Btn 3")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button31: nwg::Button,

    #[nwg_control(text: "Btn 2")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button211: nwg::Button,

    #[nwg_control(text: "Btn 3")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button311: nwg::Button,

    #[nwg_control(text: "Btn 4")]
    #[nwg_layout_item(layout: layout1, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button4: nwg::Button,
}

impl Tab1Ui {
    fn say_hello(&self) {
        let content = self.text_box.text();
        nwg::simple_message("Hello", content.as_str());
        nwg::modal_info_message(&self.button1, "hello", "world");

        println!("hello world")
    }
}
