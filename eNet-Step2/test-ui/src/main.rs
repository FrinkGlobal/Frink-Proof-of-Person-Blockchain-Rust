use std::{
    cell::{Cell, RefCell},
    collections::HashSet,
};

use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    StartEnet,
    KeyGenerate,
    DataGenerate,
    SendData,
}

pub struct MainViewState {
    send_data: RefCell<Vec<String>>,
    recv_data: RefCell<Vec<String>>,
    action: Cell<Option<Action>>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState {
            send_data: RefCell::new(vec![
                "Send 1".to_string(),
            ]),
            recv_data: RefCell::new(vec![
                "Receive 1".to_string(),
            ]),
            action: Cell::new(None),
        }
    }
}

impl MainViewState {
    fn action(&self, action: impl Into<Option<Action>>) {
        self.action.set(action.into());
    }
}

impl State for MainViewState {
    fn update(&self, _ctx: &mut Context<'_>) {
        if let Some(action) = self.action.get() {
            match action {
                Action::StartEnet => {
                    println!("Start Button Pressed");
                }
                Action::KeyGenerate => {
                    println!("Key Generate Button Pressed");
                }
                Action::DataGenerate => {
                    println!("Data Generate Button Pressed");
                }
                Action::SendData => {
                    println!("Send Button Pressed");
                }
            }

            self.action.set(None);
        }
    }

    fn update_post_layout(&self, _ctx: &mut Context<'_>) {
    }
}

fn create_header(_ctx: &mut BuildContext, text: &str) -> Entity {
    TextBlock::create()
        .text(text)
        .selector(Selector::new().with("text-block").class("h1"))
        .build(_ctx)
}

widget!(
    MainView<MainViewState> {
        selected_indices: SelectedIndices,
        result: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, _ctx: &mut BuildContext) -> Self {
        let start_enet = self.clone_state();
        let generate_keys = self.clone_state();
        let generate_data = self.clone_state();
        let send_data = self.clone_state();
        let list_state = self.clone_state();
        let send_view_state = self.clone_state();
        let recv_view_state = self.clone_state();
        let send_list_count = list_state.send_data.borrow().len();
        let recv_list_count = list_state.recv_data.borrow().len();

        self.name("MainView")
            .result("Enet Test")
            .selected_indices(HashSet::new())
            .child(
                Grid::create()
                    .margin((30.0, 30.0, 0.0, 30.0))
                    .columns(
                        Columns::create()
                            .column(500.0)
                            .column(500.0)
                            .build(),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(0))
                            // Column 0
                            .child(                                
                                Button::create()
                                    .text("Start")
                                    .selector(Selector::new().with("button").class("primary"))
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(0))
                                    .width(150.0)
                                    .horizontal_alignment(Alignment::Start)
                                    .on_click(move |_| {
                                        start_enet.action(Action::StartEnet);
                                        true
                                    })
                                    .build(_ctx),
                            )
                            .child(                                
                                Button::create()
                                    .text("Key Generate")
                                    .selector(Selector::new().with("button").class("primary"))
                                    .margin((0.0, 15.0, 0.0, 15.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(1))
                                    .width(150.0)
                                    .horizontal_alignment(Alignment::Start)
                                    .on_click(move |_| {
                                        generate_keys.action(Action::KeyGenerate);
                                        true
                                    })
                                    .build(_ctx),
                            )
                            .child(create_header(_ctx, "Private Key"))
                            .child(
                                TextBox::create()
                                    .water_mark("Private Key...")
                                    .text("")
                                    .clip(true)
                                    .margin((0.0, 8.0, 0.0, 15.0))
                                    .attach(Grid::row(3))
                                    .height(100.0)
                                    .width(490.0)
                                    .enabled(false)
                                    .build(_ctx),
                            )
                            .child(create_header(_ctx, "Public Key"))
                            .child(
                                TextBox::create()
                                    .water_mark("Public Key...")
                                    .text("")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(5))
                                    .height(100.0)
                                    .width(490.0)
                                    .enabled(false)
                                    .build(_ctx),
                            )
                            .child(                                
                                Button::create()
                                    .text("Data Generate")
                                    .selector(Selector::new().with("button").class("primary"))
                                    .margin((0.0, 15.0, 0.0, 0.0))
                                    .attach(Grid::column(0))
                                    .attach(Grid::row(6))
                                    .width(150.0)
                                    .horizontal_alignment(Alignment::Start)
                                    .on_click(move |_| {
                                        generate_data.action(Action::DataGenerate);
                                        true
                                    })
                                    .build(_ctx),
                            )
                            .child(
                                TextBox::create()
                                    .water_mark("Random Data...")
                                    .text("")
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .attach(Grid::row(7))
                                    .height(100.0)
                                    .width(490.0)
                                    .enabled(false)
                                    .build(_ctx),
                            )
                            .child(                                
                                Button::create()
                                    .text("Send")
                                    .selector(Selector::new().with("button").class("primary"))
                                    .margin((0.0, 15.0, 10.0, 0.0))
                                    .attach(Grid::row(8))
                                    .width(150.0)
                                    .horizontal_alignment(Alignment::End)
                                    .on_click(move |_| {
                                        send_data.action(Action::SendData);
                                        true
                                    })
                                    .build(_ctx),
                            )
                            .build(_ctx),
                    )
                    .child(
                        Stack::create()
                            .attach(Grid::column(1))
                            .margin((30.0, 0.0, 0.0, 0.0))
                            .child(create_header(_ctx, "Sent Data"))
                            .child(
                                ListView::create()
                                    .attach(Grid::column(2))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .selected_indices(id)
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .height(250.0)
                                    .width(490.0)
                                    .items_builder(move |bc, index| {
                                        TextBlock::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .vertical_alignment("center")
                                            .text(
                                                send_view_state.send_data.borrow()[index]
                                                    .as_str(),
                                            )
                                            .build(bc)
                                    })
                                    .count(send_list_count)
                                    .build(_ctx),
                            )
                            .child(create_header(_ctx, "Received Data"))
                            .child(
                                ListView::create()
                                    .attach(Grid::column(2))
                                    .attach(Grid::column_span(3))
                                    .attach(Grid::row(3))
                                    .selected_indices(id)
                                    .margin((0.0, 8.0, 0.0, 0.0))
                                    .height(250.0)
                                    .width(490.0)
                                    .items_builder(move |bc, index| {
                                        TextBlock::create()
                                            .margin((0.0, 0.0, 0.0, 2.0))
                                            .vertical_alignment("center")
                                            .text(
                                                recv_view_state.recv_data.borrow()[index]
                                                    .as_str(),
                                            )
                                            .build(bc)
                                    })
                                    .count(recv_list_count)
                                    .build(_ctx),
                            )
                            .build(_ctx),
                    )
                    .build(_ctx),
            )
    }
}

fn main() {
    // use this only if you want to run it as web application.
    orbtk::initialize();

    Application::new()
        .window(|_ctx| {
            Window::create()
                .title("ENet Sign Test...")
                .position((400.0, 150.0))
                .size(1080.0, 650.0)
                .resizeable(true)
                .child(MainView::create().build(_ctx))
                .build(_ctx)
        })
        .run();    
}