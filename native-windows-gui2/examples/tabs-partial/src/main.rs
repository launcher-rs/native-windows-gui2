mod tab1;
mod tab2;
mod tab3;

extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::{FlexboxLayout, NativeUi, Tab, TabsContainer};

// Stretch style
use crate::tab1::Tab1Ui;
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
// const WIDTH :D = D::Percent(20.0);

#[derive(Default, NwgUi)]
pub struct FlexBoxApp {
    #[nwg_control(size: (855, 450), position: (300, 300), title: "Flexbox example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    tab_container_layout: FlexboxLayout,

    // Tab
    #[nwg_control(parent:window)]
    #[nwg_layout_item(layout:tab_container_layout)]
    controls_holder: TabsContainer,

    #[nwg_control(text: "Tab1",parent: controls_holder)]
    tab_demo1: Tab,

    #[nwg_control(text: "Tab2",parent: controls_holder)]
    tab_demo2: Tab,

    #[nwg_control(text: "Tab3",parent: controls_holder)]
    tab_demo3: Tab,

    #[nwg_layout(parent: tab_demo2, flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_content: AlignContent::FlexStart)]
    layout2: nwg::FlexboxLayout,
    #[nwg_layout(parent: tab_demo3, flex_direction: FlexDirection::Row, flex_wrap: FlexWrap::Wrap, align_content: AlignContent::FlexStart)]
    layout3: nwg::FlexboxLayout,

    #[nwg_partial(parent: tab_demo1)]
    // #[nwg_events( (button21, OnButtonClick):[FlexBoxApp:say_hello] )]
    tab1_ui: Tab1Ui,

    #[nwg_control(text: "Btn 5",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button5: nwg::Button,

    #[nwg_control(text: "Btn 6",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button6: nwg::Button,

    #[nwg_control(text: "Btn 7",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button7: nwg::Button,

    #[nwg_control(text: "Btn 8",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button8: nwg::Button,

    #[nwg_control(text: "Btn 9",parent:tab_demo3)]
    #[nwg_layout_item(layout: layout3, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button9: nwg::Button,

    #[nwg_control(text: "Btn 10",parent:tab_demo3)]
    #[nwg_layout_item(layout: layout3, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button10: nwg::Button,

    #[nwg_control(text: "Btn 11",parent:tab_demo3)]
    #[nwg_layout_item(layout: layout3, margin: MARGIN, size: Size { width: WIDTH, height: D::Points(50.0) })]
    button11: nwg::Button,
}

impl FlexBoxApp {
    // fn say_hello(&self) {
    //     nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    // }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
