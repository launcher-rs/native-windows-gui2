extern crate native_windows_derive2 as nwd;
extern crate native_windows_gui2 as nwg;

use nwd::NwgUi;
use nwg::{InsertListViewItem, ListView, ListViewColumnSortArrow, NativeUi, Window};

#[derive(Default, NwgUi)]
pub struct ListviewDemoApp {
    #[nwg_control(size:(855,450),position: (300,300),title: "ListView")]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()], OnInit: [ ListviewDemoApp::init_list_view ])]
    window: Window,

    #[nwg_resource(initial: 5)]
    view_icons: nwg::ImageList,

    #[nwg_resource(initial: 5, size: (16, 16))]
    view_icons_small: nwg::ImageList,

    //margin 上 右 下 左
    #[nwg_layout(parent:window,spacing:1,margin:[5, 15, 10, 15])]
    show_layout: nwg::GridLayout,

    #[nwg_control(ex_flags: ListViewExFlags::GRID | ListViewExFlags::FULL_ROW_SELECT,parent:window,list_style: ListViewStyle::Detailed)]
    #[nwg_events( OnListViewClick: [ListviewDemoApp::on_click], OnListViewRightClick: [ListviewDemoApp::on_right_cliek] )]
    #[nwg_layout_item(layout: show_layout,row: 0,row_span: 7, col:0,col_span:12)]
    list_view: ListView,

    #[nwg_control(parent: window,popup:true)]
    list_menu: nwg::Menu,

    #[nwg_control(parent: list_menu, text: "删除")]
    #[nwg_events(OnMenuItemSelected: [ListviewDemoApp::delete_item])]
    delete_item: nwg::MenuItem,

    #[nwg_control(text: "名称：",parent:window)]
    #[nwg_layout_item(layout: show_layout,row: 7,row_span: 1, col:0,col_span:1)]
    name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: show_layout,row: 7,row_span: 1, col:1,col_span:10)]
    name_input: nwg::TextInput,

    #[nwg_control(text: "价格：",parent:window)]
    #[nwg_layout_item(layout: show_layout,row: 8,row_span: 1, col:0,col_span:1)]
    price_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: show_layout,row: 8,row_span: 1, col:1,col_span:10)]
    price_input: nwg::TextInput,

    #[nwg_control(text: "数量：",parent:window)]
    #[nwg_layout_item(layout: show_layout,row: 9,row_span: 1, col:0,col_span:1)]
    number_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: show_layout,row: 9,row_span: 1, col:1,col_span:10)]
    number_input: nwg::TextInput,

    #[nwg_control(text: "取消选择",parent:window)]
    #[nwg_layout_item(layout: show_layout,row: 10,row_span: 1, col:6,col_span:2)]
    cancel_button: nwg::Button,

    #[nwg_control(text: "添加",parent:window)]
    #[nwg_layout_item(layout: show_layout,row: 10,row_span: 1, col:8,col_span:2)]
    #[nwg_events ( OnButtonClick: [ ListviewDemoApp::modify_item ])]
    add_button: nwg::Button,
}

impl ListviewDemoApp {
    fn init_list_view(&self) {
        let icons_small = &self.view_icons_small;

        icons_small.add_icon_from_filename("./res/cog.ico").unwrap();
        icons_small.add_icon_from_filename("./res/cog.ico").unwrap();

        let list = &self.list_view;
        for &column in &["姓名", "价格", "数量"] {
            list.insert_column(column);
        }
        list.set_column_width(2, 500);
        list.set_headers_enabled(true);

        list.set_column_sort_arrow(1, Some(ListViewColumnSortArrow::Up));
        list.set_image_list(Some(icons_small), nwg::ListViewImageListType::Small);

        let data: &[&[&str]] = &[
            &["香蕉", "10.0", "1000"],
            &["苹果", "2.0", "345"],
            &["猕猴桃", "10.0", "1200"],
            &["橘子", "5.0", "1000"],
        ];

        for d in data {
            list.insert_items_row(None, d);
        }
    }

    fn on_click(&self) {
        let list = &self.list_view;
        println!("click selected_count {}.........", list.selected_count()); // 选中条目数量
        println!("click column_len {}.........", list.column_len());
        println!("click column_width {}.........", list.column_width());
        println!("click selected_count {:?}.........", list.selected_item());
        println!("click selected_items {:?}.........", list.selected_items());
        println!("=====================");
        if let Some(selectd_item) = list.selected_item() {
            self.add_button.set_text("修改");
            if let Some(name) = list.item(selectd_item, 0, 200) {
                self.name_input.set_text(name.text.as_str());
            }
            if let Some(price) = list.item(selectd_item, 1, 200) {
                self.price_input.set_text(price.text.as_str());
            }
            if let Some(number) = list.item(selectd_item, 2, 200) {
                self.number_input.set_text(number.text.as_str());
            }
        } else {
            self.name_input.set_text("");
            self.price_input.set_text("");
            self.number_input.set_text("");
            self.add_button.set_text("添加");
        }
    }

    fn on_right_cliek(&self) {
        let list = &self.list_view;
        let selected_items = list.selected_items();
        if !selected_items.is_empty() {
            println!("selected_item: {:?}", selected_items);
            let (x, y) = nwg::GlobalCursor::position();
            self.list_menu.popup(x, y);
        }
    }

    fn delete_item(&self) {
        let list = &self.list_view;
        let selected_items = list.selected_items();
        if !selected_items.is_empty() {
            for item in selected_items {
                list.remove_item(item);
            }
        }
    }

    /// add_button 点击时间
    fn modify_item(&self) {
        let add_button_name = self.add_button.text();
        let list = &self.list_view;
        if add_button_name.eq("添加") {
            println!("添加........");
            // 添加数据
            let name = self.name_input.text();
            let price = self.price_input.text();
            let number = self.number_input.text();
            // 检测是否为空 name 为空
            if !name.is_empty() {
                let mut item = Vec::with_capacity(3);
                item.push(name);
                item.push(price);
                item.push(number);
                list.insert_items_row(None, &item);
            }
        }

        if add_button_name.eq("修改") {
            println!("修改........");
            // 添加数据
            let name = self.name_input.text();
            let price = self.price_input.text();
            let number = self.number_input.text();
            // 检测是否为空 name 为空
            if !name.is_empty()
                && let Some(index) = list.selected_item()
            {
                let aindex = index as i32;
                let item: InsertListViewItem = InsertListViewItem {
                    index: Some(aindex),
                    column_index: 0,
                    text: Some(name),
                    image: None,
                };
                list.update_item(index, item);

                let item: InsertListViewItem = InsertListViewItem {
                    index: Some(aindex),
                    column_index: 1,
                    text: Some(price),
                    image: None,
                };
                list.update_item(index, item);

                let item: InsertListViewItem = InsertListViewItem {
                    index: Some(aindex),
                    column_index: 2,
                    text: Some(number),
                    image: None,
                };
                list.update_item(index, item);
                // 清空下面框
                self.name_input.set_text("");
                self.price_input.set_text("");
                self.number_input.set_text("");
                self.add_button.set_text("添加");
            }
        }
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = ListviewDemoApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
