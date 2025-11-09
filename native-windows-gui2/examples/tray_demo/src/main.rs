extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::NativeUi;

const TRAY_ICON: &[u8] = include_bytes!("../assets/cog.ico");
const TRAY_ICON2: &[u8] = include_bytes!("../assets/record.ico");

#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (300, 135), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_resource(source_bin: Some(TRAY_ICON))]
    icon: nwg::Icon,

    #[nwg_resource(source_bin: Some(TRAY_ICON2))]
    icon2: nwg::Icon,

    /// MousePressLeftUp: 左键点击时
    /// OnContextMenu: 右键点击时
    #[nwg_control(icon: Some(&data.icon), tip: Some("Hello"))]
    #[nwg_events(MousePressLeftUp: [BasicApp::show], OnContextMenu: [BasicApp::show_menu])]
    tray: nwg::TrayNotification,

    #[nwg_control(parent: window, popup: true)]
    tray_menu: nwg::Menu,

    #[nwg_control(parent: tray_menu, text: "显示主程序")]
    #[nwg_events(OnMenuItemSelected: [BasicApp::show])]
    show_item: nwg::MenuItem,

    #[nwg_control(parent: tray_menu, text: "退出程序")]
    #[nwg_events(OnMenuItemSelected: [BasicApp::exit])]
    exit_item: nwg::MenuItem,

    #[nwg_control(text: "Heisenberg", size: (280, 35), position: (10, 10), focus: true)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,
}

impl BasicApp {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn say_hello(&self) {
        // 修改图标
        self.tray.set_icon(&self.icon2);

        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    fn say_goodbye(&self) {
        println!("隐藏......");
        self.window.set_visible(false);
    }

    fn show(&self) {
        self.window.set_visible(true);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
