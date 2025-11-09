extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::{GridLayout, NativeUi, Tab, TabsContainer};

#[derive(Default, NwgUi)]
pub struct FlexBoxApp {
    #[nwg_control(size: (855, 450), position: (300, 300), title: "Flexbox example")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: nwg::Window,

    #[nwg_layout(parent: window)]
    tab_container_layout: GridLayout,

    // Tab
    #[nwg_control(parent:window)]
    #[nwg_layout_item(layout:tab_container_layout)]
    controls_holder: TabsContainer,

    #[nwg_control(text: "主页",parent: controls_holder)]
    tab_demo1: Tab,

    #[nwg_control(text: "配置",parent: controls_holder)]
    tab_demo2: Tab,

    #[nwg_layout(parent: tab_demo1)]
    layout1: nwg::GridLayout,

    #[nwg_layout(parent: tab_demo2)]
    layout2: nwg::GridLayout,

    #[nwg_control(focus: true,flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP",parent:tab_demo1)]
    #[nwg_layout_item(layout: layout1, row: 0, row_span: 6, col: 0, col_span: 6 )]
    text_box: nwg::TextBox,

    #[nwg_control(focus: true,flags: "VSCROLL|AUTOVSCROLL|VISIBLE|TAB_STOP",parent:tab_demo1)]
    #[nwg_layout_item(layout: layout1, row: 0, row_span: 6, col: 6, col_span: 6 )]
    show_box: nwg::TextBox,

    //
    #[nwg_control(text: "Btn 1",parent:tab_demo1)]
    #[nwg_layout_item(layout: layout1, row: 6, row_span: 1, col: 0, col_span: 2 )]
    button1: nwg::Button,

    #[nwg_control(text: "Btn 5",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, row: 1, row_span: 1, col: 1, col_span: 2 )]
    button5: nwg::Button,

    #[nwg_control(text: "Btn 6",parent:tab_demo2)]
    #[nwg_layout_item(layout: layout2, row: 1, row_span: 2, col: 6, col_span: 2 )]
    button6: nwg::Button,
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _ui = FlexBoxApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
