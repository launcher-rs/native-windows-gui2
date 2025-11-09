// #![windows_subsystem = "windows"]
/*!
A very simple application that shows your name in a message box.
Unlike `basic_d`, this example uses layout to position the controls in the window
 */

extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::{ComboBox, ListBox, NativeUi};

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (900, 450), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(collection: vec!["a","b","c","d"], selected_index: Some(1))]
    #[nwg_layout_item(layout: grid, col: 0, row: 0)]
    #[nwg_events( OnComboxBoxSelection: [BasicApp::update_view] )]
    view_style: ComboBox<&'static str>,

    #[nwg_control(collection: vec!["a","b","c","d"], selected_index: Some(1))]
    // 有问题
    // #[nwg_control(ex_flags: ListBoxFlags::VISIBLE | ListBoxFlags::MULTI_SELECT,collection: vec!["a","b","c","d"], multi_selection: vec![0,1])]
    #[nwg_layout_item(layout: grid, col: 0, row: 1,row_span: 3)]
    #[nwg_events( OnListBoxSelect: [BasicApp::on_box_select] )]
    multi_select: ListBox<&'static str>,
}

impl BasicApp {
    /// OnComboxBoxSelection：当选择组合框中的新值时
    fn update_view(&self) {
        let value = self.view_style.selection_string().unwrap_or_default();
        println!("select value: {}", value);
    }

    fn on_box_select(&self) {
        println!("on box select...")
    }

    fn say_goodbye(&self) {
        // nwg::modal_info_message(&self.window, "Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
