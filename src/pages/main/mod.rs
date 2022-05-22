mod chatroom;
mod sidebar;

use std::collections::VecDeque;

use relm4::actions::{RelmAction, RelmActionGroup};
use relm4::factory::FactoryVecDeque;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use adw::{prelude::*, HeaderBar, Leaflet, ViewStack, ViewSwitcherTitle};
use gtk::{Align, Box, Label, ListBox, MenuButton, Orientation, ScrolledWindow, Separator, Stack};

use self::chatroom::ChatroomInitParams;
use self::{chatroom::Chatroom, sidebar::UserItem};
use crate::app::AppMessage;

const MOCK_CHATS_LIST: [(&str, &str); 13] = [
    ("飞翔的企鹅", "Hello"),
    ("奔跑的野猪", "World"),
    ("摆烂的修勾", "喵喵"),
    ("躺平的猫咪", "汪汪"),
    ("想润的鼠鼠", "鼠鼠我啊"),
    ("咆哮的先辈", "哼哼"),
    ("叛逆的鲁路", "2333"),
    ("死神的笔记", "2333"),
    ("进击的巨人", "2333"),
    ("炼金的术士", "2333"),
    ("忧郁的凉宫", "2333"),
    ("灼眼的夏娜", "2333"),
    ("科学的磁炮", "2333"),
    // ("被填充过多并被用于测试文本对齐和溢出的字符串标签", "2333"),
];

pub struct MainPageModel {
    message: Option<MainMsg>,
    chats_list: FactoryVecDeque<ListBox, UserItem, MainMsg>,
    chatrooms: FactoryVecDeque<Stack, Chatroom, MainMsg>,
}

#[derive(Clone, Debug)]
pub struct Message {
    author: String,
    message: String,
}

#[derive(Debug)]
pub enum MainMsg {
    WindowFolded,
    SelectChatroom(i32),
}

relm4::new_action_group!(WindowActionGroup, "menu");
relm4::new_stateless_action!(ShortcutsAction, WindowActionGroup, "shortcuts");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for MainPageModel {
    type Input = MainMsg;
    type Output = AppMessage;
    type Widgets = MainPageWidgets;
    type InitParams = ();

    view! {
        #[root]
        main_page = &Leaflet {
            append: sidebar = &Box {
                set_vexpand: true,
                set_width_request: 360,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_show_start_title_buttons: false,
                    set_show_end_title_buttons: false,
                    set_title_widget = Some(&ViewSwitcherTitle) {
                        set_title: "Sidebar",
                        set_stack: Some(&stack)
                    }
                },
                append: stack = &ViewStack {
                    set_vexpand: true,
                }
            },
            append = &Separator::new(Orientation::Horizontal) {
            },
            append: chatroom = &Box {
                set_vexpand: true,
                set_hexpand: true,
                set_orientation: Orientation::Vertical,
                append = &HeaderBar {
                    set_title_widget = Some(&Label) {
                        set_label: "Chatroom"
                    },
                    pack_end = &MenuButton {
                        set_icon_name: "menu-symbolic",
                        set_menu_model: Some(&main_menu),
                    }
                },
                append: chatroom_stack = &Stack {},
            },
            connect_folded_notify[sender] => move |leaflet| {
                if leaflet.is_folded() {
                    sender.input(MainMsg::WindowFolded);
                }
            },
        },
        chats_stack = ScrolledWindow {
            set_child: sidebar_chats = Some(&ListBox) {
                set_css_classes: &["navigation-sidebar"],
                connect_row_activated[sender] => move |_, selected_row| {
                    let index = selected_row.index();
                    sender.input(MainMsg::SelectChatroom(index));
                },
            }
        },
        contact_stack = &Box {
            set_halign: Align::Center,
            append: &Label::new(Some("Contact"))
        }
    }

    menu! {
        main_menu: {
            "Keyboard Shortcuts" => ShortcutsAction,
            "About Gtk QQ" => AboutAction
        }
    }

    fn init(
        _init_params: Self::InitParams,
        root: &Self::Root,
        sender: &ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let stack: &ViewStack = &widgets.stack;
        let chats_stack = stack.add_titled(&widgets.chats_stack, None, "Chats");
        let contact_stack = stack.add_titled(&widgets.contact_stack, None, "Contact");
        chats_stack.set_icon_name(Some("chat-symbolic"));
        contact_stack.set_icon_name(Some("address-book-symbolic"));

        let shortcuts_action: RelmAction<ShortcutsAction> = RelmAction::new_stateless(move |_| {
            println!("Keyboard Shortcuts");
        });
        let about_action: RelmAction<AboutAction> = RelmAction::new_stateless(move |_| {
            println!("About Gtk QQ");
        });
        let group: RelmActionGroup<WindowActionGroup> = RelmActionGroup::new();
        group.add_action(shortcuts_action);
        group.add_action(about_action);

        let actions = group.into_action_group();
        widgets
            .main_page
            .insert_action_group("menu", Some(&actions));

        let mut chats_list: FactoryVecDeque<ListBox, UserItem, MainMsg> =
            FactoryVecDeque::new(widgets.sidebar_chats.clone(), &sender.input);
        let mut chatrooms: FactoryVecDeque<Stack, Chatroom, MainMsg> =
            FactoryVecDeque::new(widgets.chatroom_stack.clone(), &sender.input);

        MOCK_CHATS_LIST.iter().for_each(|(username, last_message)| {
            chats_list.push_back(UserItem {
                username: username.to_string(),
                last_message: last_message.to_string(),
            });
            chatrooms.push_back({
                let mut messages = VecDeque::new();
                for i in 0..18 {
                    let message = format!(
                        "{}\nThis is the No.{} message in this page.",
                        last_message,
                        i + 1
                    )
                    .to_string();
                    if i % 4 == 0 {
                        messages.push_back(Message {
                            author: "You".to_string(),
                            message,
                        });
                    } else {
                        messages.push_back(Message {
                            author: username.to_string(),
                            message,
                        });
                    }
                }
                ChatroomInitParams {
                    username: username.to_string(),
                    messages,
                }
            });
        });
        chats_list.render_changes();
        chatrooms.render_changes();
        ComponentParts {
            model: MainPageModel {
                message: None,
                chats_list,
                chatrooms,
            },
            widgets,
        }
    }

    fn update(&mut self, msg: MainMsg, _sender: &ComponentSender<Self>) {
        use MainMsg::*;
        match msg {
            WindowFolded => self.message = Some(MainMsg::WindowFolded),
            SelectChatroom(id) => self.message = Some(MainMsg::SelectChatroom(id)),
        }
        self.chats_list.render_changes();
        self.chatrooms.render_changes();
    }

    fn pre_view() {
        if let Some(message) = &model.message {
            use MainMsg::*;
            match message {
                WindowFolded => widgets.main_page.set_visible_child(&widgets.chatroom),
                SelectChatroom(id) => widgets
                    .chatroom_stack
                    .set_visible_child_name(id.to_string().as_str()),
            }
        }

        self.chatrooms.render_changes();
    }
}
