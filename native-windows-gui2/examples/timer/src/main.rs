extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::{AnimationTimer, Button, Window};
use std::cell::RefCell;
use std::time::Duration;

#[derive(Default, NwgUi)]
pub struct TimerApp {
    count: RefCell<i32>,

    #[nwg_control(size:(855,450),position: (300,300),title: "定时器")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()])]
    window: Window,

    #[nwg_control(text: "启动定时器",parent:window,size: (280, 70), position: (10, 50))]
    #[nwg_events( OnButtonClick: [TimerApp::start_timer] )]
    timer_start_btn: Button,

    #[nwg_control(text: "停止定时器",parent:window,size: (280, 70), position: (10, 120))]
    #[nwg_events( OnButtonClick: [TimerApp::stop_timer] )]
    timer_stop_btn: Button,

    #[nwg_control(text: "设置定时器为5秒",parent:window,size: (280, 70), position: (10, 200))]
    #[nwg_events( OnButtonClick: [TimerApp::set_timer5] )]
    set_interval_5btn: Button,

    #[nwg_control(text: "设置定时器为1秒",parent:window,size: (280, 70), position: (10, 270))]
    #[nwg_events( OnButtonClick: [TimerApp::set_timer1] )]
    set_interval_1btn: Button,

    #[nwg_control(parent:window,interval: Duration::from_secs(1) )]
    #[nwg_events( OnTimerTick: [TimerApp::timer_tick])]
    timer: AnimationTimer,
}

impl TimerApp {
    /// 启动定时器
    fn start_timer(&self) {
        self.timer.start();
    }

    /// 关闭定时器
    fn stop_timer(&self) {
        self.timer.stop();
    }

    /// 设置定时器
    fn set_timer5(&self) {
        self.timer.set_interval(Duration::from_secs(5))
    }

    fn set_timer1(&self) {
        self.timer.set_interval(Duration::from_secs(1))
    }

    /// 定时器触发函数
    fn timer_tick(&self) {
        // Refcell 内部可变性
        let mut count = self.count.borrow_mut();
        *count += 1;
        let title = format!("timer: {}", count);
        println!("{title}");
        self.window.set_text(title.as_str());
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = TimerApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
