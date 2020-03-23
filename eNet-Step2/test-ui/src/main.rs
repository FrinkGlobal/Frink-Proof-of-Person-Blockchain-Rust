//!
//! A demonstration of all non-primitive widgets available in Conrod.
//!
//!
//! Don't be put off by the number of method calls, they are only for demonstration and almost all
//! of them are optional. Conrod supports `Theme`s, so if you don't give it an argument, it will
//! check the current `Theme` within the `Ui` and retrieve defaults from there.
//!
#[macro_use] extern crate conrod;

#[allow(unused_imports)]
use conrod::backend::glium::glium::{self, Surface};

mod support;

#[allow(unused_imports)]
use support::EventLoop;

extern crate rand; // for making a random color.
use libc;
use std::mem;

use node_network;
use node_network::Node;
use node_network::RecvMsg;

/// This struct holds all of the variables used to demonstrate application data being passed
/// through the widgets. If some of these seem strange, that's because they are! Most of these
/// simply represent the aesthetic state of different parts of the GUI to offer visual feedback
/// during interaction with the widgets.
struct HostData {
    port: [u16; 3],
    public_key: [Vec<u8>; 3],
    private_key: [Vec<u8>; 3],
    send_length: [u64; 3],
    send_data: [Vec<u8>; 3],
    recv_data: [RecvMsg; 3],
    node: Node,
}

impl HostData {
    /// Constructor for the Demonstration Application model.
    fn new() -> HostData {
        HostData {
            port: [0u16, 0u16, 0u16],
            public_key: [Vec::new(), Vec::new(), Vec::new()],
            private_key: [Vec::new(), Vec::new(), Vec::new()],
            send_length: [10u64, 20u64, 30u64],
            send_data: [Vec::new(), Vec::new(), Vec::new()],
            recv_data: [RecvMsg::new(), RecvMsg::new(), RecvMsg::new()],
            node: Node::new(),
        }
    }

    fn init(&mut self) {
        // Initialize node
        self.node.init();

        self.node.start();        

        // Set information about hosts
        for i in 0..(self.node.hosts.len()) {
            let host = &self.node.hosts[i];
            self.port[i] = host.port;
            self.public_key[i] = host.secure.public_key.to_vec();
            self.private_key[i] = host.secure.private_key.to_vec();
        }
    }
}

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1000;

fn main() {
    // Build the window.
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Frink Task 2 - Test Application")
        .with_dimensions(WIDTH, HEIGHT);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let display = support::GliumDisplayWinitWrapper(display);
    display.0.gl_window().set_position(200, 5);

    // construct our `Ui`.
    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    // Identifiers used for instantiating our widgets.
    let mut ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let font_path = assets.join("fonts/NotoSans/NotoSans-Regular.ttf");
    ui.fonts.insert_from_file(font_path).unwrap();

    // A type used for converting `conrod::render::Primitives` into `Command`s that can be used
    // for drawing to the glium `Surface`.
    let mut renderer = conrod::backend::glium::Renderer::new(&display.0).unwrap();

    // The image map describing each of our widget->image mappings (in our case, none).
    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    // Our demonstration app that we'll control with our GUI.
    let mut app = HostData::new();
    app.init();

    // Poll events from the window.
    let mut event_loop = support::EventLoop::new();
    'main: loop {

        // Handle all events.
        for event in event_loop.next(&mut events_loop) {

            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), &display) {
                ui.handle_event(event);
                event_loop.needs_update();
            }

            match event {
                glium::glutin::Event::WindowEvent { event, .. } => match event {
                    // Break from the loop upon `Escape`.
                    glium::glutin::WindowEvent::Closed |
                    glium::glutin::WindowEvent::KeyboardInput {
                        input: glium::glutin::KeyboardInput {
                            virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                            ..
                        },
                        ..
                    } => break 'main,
                    _ => (),
                },
                _ => (),
            }
        }

        // We'll set all our widgets in a single function called `set_widgets`.
        {
            let mut ui = ui.set_widgets();
            set_widgets(&mut ui, &mut app, &mut ids);
        }

        // Render the `Ui` and then display it on the screen.
        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display.0, primitives, &image_map);            
            let mut target = display.0.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display.0, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}


// In conrod, each widget must have its own unique identifier so that the `Ui` can keep track of
// its state between updates.
//
// To make this easier, conrod provides the `widget_ids` macro. This macro generates a new type
// with a unique `widget::Id` field for each identifier given in the list. See the `widget_ids!`
// documentation for more details.
widget_ids! {
    struct Ids {
        canvas,
        canvas_x_scrollbar,
        canvas_y_scrollbar,

        lbl_host_1,
        lbl_port_1,
        txt_port_1,
        btn_key_gen_1,
        btn_save_gen_1,
        lbl_public_key_label_1,
        lbl_public_key_bg_1,
        txt_public_key_1,
        scrollbar_public_key_x_1,
        scrollbar_public_key_y_1,
        lbl_private_key_label_1,
        lbl_private_key_bg_1,
        txt_private_key_1,
        scrollbar_private_key_x_1,
        scrollbar_private_key_y_1,
        lbl_send_1,
        btn_data_gen_1,
        btn_send_data_1,
        lbl_send_data_length_1,
        txt_send_data_length_1,
        txt_send_data_1,
        lbl_send_data_bg_1,
        lbl_recv_data_1,
        lbl_recv_data_bg_1,
        txt_lbl_recv_data_1,

        lbl_host_2,
        lbl_port_2,
        txt_port_2,
        btn_key_gen_2,
        btn_save_gen_2,
        lbl_public_key_label_2,
        lbl_public_key_bg_2,
        txt_public_key_2,
        scrollbar_public_key_x_2,
        scrollbar_public_key_y_2,
        lbl_private_key_label_2,
        lbl_private_key_bg_2,
        txt_private_key_2, 
        scrollbar_private_key_x_2,
        scrollbar_private_key_y_2,
        lbl_send_2,
        btn_data_gen_2,
        btn_send_data_2,
        lbl_send_data_length_2,
        txt_send_data_length_2,
        txt_send_data_2,
        lbl_send_data_bg_2,
        lbl_recv_data_2,
        lbl_recv_data_bg_2,
        txt_lbl_recv_data_2,
   
        lbl_host_3,
        lbl_port_3,
        txt_port_3,
        btn_key_gen_3,
        btn_save_gen_3,
        lbl_public_key_label_3,
        lbl_public_key_bg_3,
        txt_public_key_3,
        scrollbar_public_key_x_3,
        scrollbar_public_key_y_3,
        lbl_private_key_label_3,
        lbl_private_key_bg_3,
        txt_private_key_3,
        scrollbar_private_key_x_3,
        scrollbar_private_key_y_3,
        lbl_send_3,
        btn_data_gen_3,
        btn_send_data_3,
        lbl_send_data_length_3,
        txt_send_data_length_3,
        txt_send_data_3,
        lbl_send_data_bg_3,
        lbl_recv_data_3,
        lbl_recv_data_bg_3,
        txt_lbl_recv_data_3,

        line_inv_1,
        line_inv_2,

        title_pad_slider,
    }
}

/// Set all `Widget`s within the User Interface.
///
/// The first time this gets called, each `Widget`'s `State` will be initialised and cached within
/// the `Ui` at their given indices. Every other time this get called, the `Widget`s will avoid any
/// allocations by updating the pre-existing cached state. A new graphical `Element` is only
/// retrieved from a `Widget` in the case that it's `State` has changed in some way.
fn set_widgets(ui: &mut conrod::UiCell, app: &mut HostData, ids: &mut Ids) {
    use conrod::{color, widget, Borderable, Labelable, Positionable, Sizeable, Colorable, Widget};

    let bg_color = conrod::color::rgb(0.2, 0.35, 0.45);
    let label_color = conrod::color::rgb(0.08, 0.14, 0.18);
    let label_font = 16;
    let label_key_font = 12;
    let label_text_color = label_color.plain_contrast().alpha(0.5);
    let btn_height = 50.0;
    let btn_color = conrod::color::rgba(1.0, 1.0, 1.0, 0.3);
    let btn_bg_color = conrod::color::rgba(0.0, 0.0, 0.0, 0.6);
    let border_width = 1.0;
    let txt_key_color = conrod::color::rgba(0.0, 0.0, 0.0, 1.0);
    let txt_lbl_bg_color = conrod::color::rgba(0.0, 0.0, 0.0, 0.01);
    let txt_lbl_border_color = conrod::color::rgba(0.0, 0.0, 0.0, 0.7);

    // We can use this `Canvas` as a parent Widget upon which we can place other widgets.
    widget::Canvas::new()
        // .border(border_width)
        .pad(30.0)
        .color(bg_color)
        .scroll_kids()
        .set(ids.canvas, ui);
    widget::Scrollbar::x_axis(ids.canvas).auto_hide(true).set(ids.canvas_y_scrollbar, ui);
    widget::Scrollbar::y_axis(ids.canvas).auto_hide(true).set(ids.canvas_x_scrollbar, ui);

    // Interval Lines
    let x_inv = (WIDTH / 3 / 2) as f64;
    let y_inv = (HEIGHT / 2) as f64;
    widget::Line::abs([x_inv, (-y_inv)], [x_inv, y_inv])
        .thickness(1.0)
        .color(conrod::color::rgba(0.0, 0.0, 0.0, 0.5))
        .set(ids.line_inv_1, ui);
    widget::Line::abs([(-x_inv), (-y_inv)], [(-x_inv), y_inv])
        .thickness(1.0)
        .color(conrod::color::rgba(0.0, 0.0, 0.0, 0.5))
        .set(ids.line_inv_2, ui);

    for id in 0..3usize {
        let (
            lbl_host,
            lbl_port,
            txt_port,
            btn_key_gen,
            btn_save_gen,
            lbl_public_key_label,
            lbl_public_key_bg,
            txt_public_key,
            scrollbar_public_key_x,
            scrollbar_public_key_y,
            lbl_private_key_label,
            lbl_private_key_bg,
            txt_private_key,
            scrollbar_private_key_x,
            scrollbar_private_key_y,
            lbl_send,
            btn_data_gen,
            btn_send_data,
            lbl_send_data_length,
            txt_send_data_length,
            txt_send_data,
            lbl_send_data_bg,
            lbl_recv_data,
            lbl_recv_data_bg,
            txt_lbl_recv_data,
        ) = match id {
            0 => (
                ids.lbl_host_1,
                ids.lbl_port_1,
                ids.txt_port_1,
                ids.btn_key_gen_1,
                ids.btn_save_gen_1,
                ids.lbl_public_key_label_1,
                ids.lbl_public_key_bg_1,
                ids.txt_public_key_1,
                ids.scrollbar_public_key_x_1,
                ids.scrollbar_public_key_y_1,
                ids.lbl_private_key_label_1,
                ids.lbl_private_key_bg_1,
                ids.txt_private_key_1,
                ids.scrollbar_private_key_x_1,
                ids.scrollbar_private_key_y_1,
                ids.lbl_send_1,
                ids.btn_data_gen_1,
                ids.btn_send_data_1,
                ids.lbl_send_data_length_1,
                ids.txt_send_data_length_1,
                ids.txt_send_data_1,
                ids.lbl_send_data_bg_1,
                ids.lbl_recv_data_1,
                ids.lbl_recv_data_bg_1,
                ids.txt_lbl_recv_data_1,
            ),
            1 => (
                ids.lbl_host_2,
                ids.lbl_port_2,
                ids.txt_port_2,
                ids.btn_key_gen_2,
                ids.btn_save_gen_2,
                ids.lbl_public_key_label_2,
                ids.lbl_public_key_bg_2,
                ids.txt_public_key_2,
                ids.scrollbar_public_key_x_2,
                ids.scrollbar_public_key_y_2,
                ids.lbl_private_key_label_2,
                ids.lbl_private_key_bg_2,
                ids.txt_private_key_2,
                ids.scrollbar_private_key_x_2,
                ids.scrollbar_private_key_y_2,
                ids.lbl_send_2,
                ids.btn_data_gen_2,
                ids.btn_send_data_2,
                ids.lbl_send_data_length_2,
                ids.txt_send_data_length_2,
                ids.txt_send_data_2,
                ids.lbl_send_data_bg_2,
                ids.lbl_recv_data_2,
                ids.lbl_recv_data_bg_2,
                ids.txt_lbl_recv_data_2,
            ),
            2 => (
                ids.lbl_host_3,
                ids.lbl_port_3,
                ids.txt_port_3,
                ids.btn_key_gen_3,
                ids.btn_save_gen_3,
                ids.lbl_public_key_label_3,
                ids.lbl_public_key_bg_3,
                ids.txt_public_key_3,
                ids.scrollbar_public_key_x_3,
                ids.scrollbar_public_key_y_3,
                ids.lbl_private_key_label_3,
                ids.lbl_private_key_bg_3,
                ids.txt_private_key_3,
                ids.scrollbar_private_key_x_3,
                ids.scrollbar_private_key_y_3,
                ids.lbl_send_3,
                ids.btn_data_gen_3,
                ids.btn_send_data_3,
                ids.lbl_send_data_length_3,
                ids.txt_send_data_length_3,
                ids.txt_send_data_3,
                ids.lbl_send_data_bg_3,
                ids.lbl_recv_data_3,
                ids.lbl_recv_data_bg_3,
                ids.txt_lbl_recv_data_3,
            ),
            _ => unreachable!(),
        };

        let offeset: f64 = (WIDTH / 3 * (id as u32)) as f64;

        // Host Name.
        widget::Text::new(&format!("Host {}", id + 1))
            .top_left_with_margins_on(ids.canvas, 0.0, 3.0 + offeset)
            .font_size(22)
            .color(bg_color.plain_contrast())
            .set(lbl_host, ui);

        // Port Label.
        widget::Text::new("Port = ")
            .top_left_with_margins_on(lbl_host, 5.0, 292.0)
            .font_size(label_font)
            .color(label_text_color)
            .set(lbl_port, ui);
        
        // Port Number Input
        for event in widget::TextBox::new(&format!("{}", app.port[id]))
            .top_left_with_margins_on(lbl_host, 0.0, 345.0)
            .font_size(label_font)
            .w_h(87.0, 30.0)
            .border(0.0)
            .border_color(bg_color.invert().plain_contrast())
            .color(color::WHITE)
            .set(txt_port, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("Port Number {}: {:?}", id, &mut app.port[id]),
                widget::text_box::Event::Update(string) => {
                    if string.parse::<u16>().is_ok() {
                        app.port[id] = string.parse::<u16>().unwrap();
                        app.node.hosts[id].port = app.port[id];
                    }
                },
            }
        }

        // Key Generate Button
        if widget::Button::new()
            .w_h(200.0, btn_height)
            .down_from(lbl_host, 30.0)
            .color(btn_bg_color)
            .border(0.0)
            .label("Generate Key Pair")
            .label_font_size(label_font)
            .label_color(btn_color)
            .set(btn_key_gen, ui)
            .was_clicked()
        {
            app.node.hosts[id].secure.generate_keypair();
            app.public_key[id] = app.node.hosts[id].secure.public_key.to_vec();
            app.private_key[id] = app.node.hosts[id].secure.private_key.to_vec();
        }

        // Save Generate Button
        if widget::Button::new()
            .w_h(80.0, btn_height)
            .top_left_with_margins_on(btn_key_gen, 0.0, 355.0)
            .color(btn_bg_color)
            .border(0.0)
            .label("Save")
            .label_font_size(label_font)
            .label_color(btn_color)
            .label_x(conrod::position::Relative::Align(conrod::position::Align::Middle))
            .set(btn_save_gen, ui)
            .was_clicked()
        {
            app.node.save_hosts();

            let mut peer = &mut app.node.hosts[0].peers[id];
            peer.port = app.port[id];
            peer.key.clear();
            for i in 0..app.public_key[id].len() {
                peer.key.push(app.public_key[id][i]);
            }
            app.node.hosts[0].save_peerlist();

            app.node.restart();
        }

        // Public Key Label.
        widget::Text::new(&format!("Public Key (length = {})", app.public_key[id].len()))
            .top_left_with_margins_on(btn_key_gen, 70.0, 20.0)
            .font_size(label_font)
            .color(label_text_color)
            .set(lbl_public_key_label, ui);
            
        // Public Key TextBox.
        let mut pk_str = String::new();
        for k in 0..app.public_key[id].len() {
            pk_str.push_str(&format!("{:02X}", app.public_key[id][k]));
        }
        widget::TextBox::new("")
            .w_h(412.0, 100.0)
            .top_left_with_margins_on(lbl_public_key_label, 25.0, 0.0)
            .color(txt_lbl_bg_color)
            .border(border_width)
            .border_color(txt_lbl_border_color)
            .set(lbl_public_key_bg, ui);
        widget::TextEdit::new(&pk_str)
            .w_h(412.0, 100.0)
            .top_left_with_margins_on(lbl_public_key_label, 25.0, 0.0)
            .font_size(label_key_font)
            .color(txt_key_color)
            // .border(border_width)
            // .border_color(txt_lbl_border_color)
            .place_on_kid_area(true)
            .scroll_kids()
            .set(txt_public_key, ui);
        widget::Scrollbar::x_axis(txt_public_key).auto_hide(true).set(scrollbar_public_key_x, ui);
        widget::Scrollbar::y_axis(txt_public_key).auto_hide(true).set(scrollbar_public_key_y, ui);

        // Private Key Label.
        widget::Text::new(&format!("Private Key (length = {})", app.private_key[id].len()))
            .down_from(txt_public_key, 15.0)
            .font_size(label_font)
            .color(label_text_color)
            .set(lbl_private_key_label, ui);
            
        // Private Key TextBox.
        let mut sk_str = String::new();
        for k in 0..app.public_key[id].len() {
            sk_str.push_str(&format!("{:02X}", app.private_key[id][k]));
        }
        widget::TextBox::new("")
            .top_left_with_margins_on(lbl_private_key_label, 25.0, 0.0)
            .font_size(label_key_font)
            .w_h(412.0, 150.0)
            .color(txt_lbl_bg_color)
            .border(border_width)
            .border_color(txt_lbl_border_color)
            .set(lbl_private_key_bg, ui);
        widget::TextEdit::new(&sk_str)
            .top_left_with_margins_on(lbl_private_key_label, 25.0, 0.0)
            .font_size(label_key_font)
            .w_h(412.0, 150.0)
            .color(txt_key_color)
            // .border(border_width)
            // .border_color(txt_lbl_border_color)
            .scroll_kids()
            .set(txt_private_key, ui);
        widget::Scrollbar::x_axis(txt_private_key).auto_hide(true).set(scrollbar_private_key_x, ui);
        widget::Scrollbar::y_axis(txt_private_key).auto_hide(true).set(scrollbar_private_key_y, ui);

        // Send Label.
        widget::Text::new("Send")
            .down_from(btn_key_gen, 370.0)
            .font_size(20)
            .color(label_text_color)
            .set(lbl_send, ui);

        // Send Length Label.
        widget::Text::new("Length = ")
            .top_left_with_margins_on(lbl_send, 5.0, 270.0)
            .font_size(label_font)
            .color(label_text_color)
            .set(lbl_send_data_length, ui);
        
        // Send Length TextEdit.
        for event in widget::TextBox::new(&format!("{}", app.send_length[id]))
            .top_left_with_margins_on(lbl_send_data_length, -5.0, 75.0)
            .font_size(label_font)
            .w_h(87.0, 30.0)
            .border(0.0)
            .border_color(bg_color.invert().plain_contrast())
            .color(color::WHITE)
            .set(txt_send_data_length, ui)
        {
            match event {
                widget::text_box::Event::Enter => println!("Send Length {}: {:?}", id, &mut app.send_length[id]),
                widget::text_box::Event::Update(string) => {
                    match string.parse::<u64>() {
                        Ok(num) => app.send_length[id] = num,
                        Err(_) => println!("Couldn't read send length", ),
                    }
                },
            }
        }

        // Generate Random Data Button
        if widget::Button::new()
            .w_h(200.0, btn_height)
            .down_from(lbl_send, 30.0)
            .color(btn_bg_color)
            .border(0.0)
            .label("Generate Random Data")
            .label_font_size(label_font)
            .label_color(btn_color)
            .label_x(conrod::position::Relative::Align(conrod::position::Align::Middle))
            .set(btn_data_gen, ui)
            .was_clicked()
        {
            // Generate random data            
            if true {
                if app.send_length[id] > 0 {
                    let mlen: u64 = app.send_length[id] as u64;
                    #[allow(unused_assignments)]
                    let mut msg = std::ptr::null_mut();
                    
                    app.send_data[id].clear();
                    unsafe {
                        msg = libc::calloc(mlen as usize, mem::size_of::<u8>()) as *mut u8;        
                        node_network::randombytes(msg, mlen);

                        // Display message
                        for i in 0..mlen {
                            app.send_data[id].push(*msg.offset(i as isize));
                        }
                    }
                    unsafe { libc::free(msg as *mut libc::c_void) };
                }
            } else {
                let tmp_str = "Hello World".as_bytes();
                app.send_data[id].clear();
                for i in 0..tmp_str.len() {
                    app.send_data[id].push(tmp_str[i]);
                }
                app.send_length[id] = tmp_str.len() as u64;
            }
        }

        // Send Data Button
        if widget::Button::new()
            .w_h(80.0, btn_height)
            .right_from(btn_data_gen, 153.0)
            .color(btn_bg_color)
            .border(0.0)
            .label("Send")
            .label_font_size(label_font)
            .label_color(btn_color)
            .label_x(conrod::position::Relative::Align(conrod::position::Align::Middle))
            .set(btn_send_data, ui)
            .was_clicked()
        {
            app.node.hosts[id].broadcast_message(&app.send_data[id]);
            for _i in 0..10 {
                app.node.execute();
            }
        }

        // Send Data TextBox.
        let mut send_str = String::new();
        let _snd_dt = &app.send_data[id];
        for i in 0..(_snd_dt.len()) {
            send_str.push_str(&format!("{:02X}", _snd_dt[i as usize]));
        }
        widget::TextBox::new("")
            .down_from(btn_data_gen, 25.0)
            .font_size(label_font)
            .w_h(433.0, 150.0)
            .color(color::WHITE.alpha(0.5))
            .text_color(color::WHITE.alpha(0.0))
            .border(border_width)
            .border_color(txt_lbl_border_color.alpha(1.0))
            .pad_text(10.0)
            .scroll_kids()
            .set(lbl_send_data_bg, ui);

        widget::TextEdit::new(&send_str)
            .down_from(btn_data_gen, 25.0)
            .w_h(433.0, 150.0)
            .font_size(label_font)
            .rgba(0.0, 0.0, 0.0, 0.5)
            .color(color::BLACK.alpha(0.8))
            .scroll_kids()
            .set(txt_send_data, ui);

        // Receive Data Label.
        widget::Text::new("Received Data")
            .down_from(txt_send_data, 20.0)
            .font_size(20)
            .color(label_text_color)
            .set(lbl_recv_data, ui);
        
        // Received Data TextEdit.
        if app.node.hosts[id].received {
            let msg = app.node.hosts[id].recv_messages.last().unwrap();
            app.recv_data[id].sender = msg.sender;
            app.recv_data[id].timestamp = String::new();
            app.recv_data[id].timestamp.push_str(&msg.timestamp);
            app.recv_data[id].msg.clear();
            for i in 0..msg.msg.len() {
                app.recv_data[id].msg.push(msg.msg[i]);
            }
            app.node.hosts[id].received = false;
        }
        let mut recv_str = String::new();
        if app.recv_data[id].sender != 0 {
            recv_str.push_str(&format!("From port {} in {} : ", app.recv_data[id].sender, app.recv_data[id].timestamp));
            for i in 0..app.recv_data[id].msg.len() {
                recv_str.push_str(&format!("{:02X}", app.recv_data[id].msg[i]));
            }
        }

        widget::TextBox::new("")
            .down_from(lbl_recv_data, 15.0)
            .w_h(433.0, 130.0)
            .font_size(label_font)
            .color(txt_lbl_bg_color)
            .border(border_width)
            .border_color(txt_lbl_border_color)
            .set(lbl_recv_data_bg, ui);
        widget::TextEdit::new(&recv_str)
            .down_from(lbl_recv_data, 15.0)
            .w_h(433.0, 130.0)
            .font_size(label_font)
            .rgba(0.0, 0.0, 0.0, 0.5)
            .color(color::BLACK.alpha(0.8))
            .scroll_kids()
            .set(txt_lbl_recv_data, ui);
    }
}
