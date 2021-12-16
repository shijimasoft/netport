use fltk::{app, prelude::*, window::Window, frame::Frame, enums::FrameType, input::Input, button::Button};

fn main() {
    let netport = app::App::default().with_scheme(app::AppScheme::Gtk);
    let mut window = Window::default()
        .with_size(300, 180)
        .with_label("NetPort")
        .center_screen();

    /* IP PANEL */
    let mut ip_panel = Frame::new(16,10,270,70,"");
    ip_panel.set_frame(FrameType::GtkDownFrame);

    let _ip_label = Frame::new(125, 20, 50, 8, "IP address:");

    // IP dots
    let _dot1_label = Frame::new(105, 54, 5, 5, ".");
    let _dot2_label = Frame::new(148, 54, 5, 5, ".");
    let _dot3_label = Frame::new(191, 54, 5, 5, ".");

    // Byte fields
    let mut first_txt = Input::new(69, 40, 34, 24, "");
    first_txt.set_maximum_size(3);
    let mut second_txt = Input::new(112, 40, 34, 24, "");
    second_txt.set_maximum_size(3);
    let mut third_txt = Input::new(155, 40, 34, 24, "");
    third_txt.set_maximum_size(3);
    let mut fourth_txt = Input::new(198, 40, 34, 24, "");
    fourth_txt.set_maximum_size(3);

    /* PORT PANEL */
    let mut port_panel = Frame::new(15,90,100,70,"");
    port_panel.set_frame(FrameType::GtkDownFrame);

    let _port_label = Frame::new(25, 107, 30, 8, "Port:");

    let mut port_txt = Input::new(60, 98, 42, 24, "");
    port_txt.set_maximum_size(4);

    let mut check_btn = Button::new(25, 128, 77, 21, "Check");
    
    /* STATUS PANEL */
    let mut status_panel = Frame::new(125,90,160,70,"");
    status_panel.set_frame(FrameType::GtkDownFrame);

    let mut status_label = Frame::new(168, 98, 70, 18, "Status");
    let mut address_label = Frame::new(140, 102, 130, 50, "");

    check_btn.set_callback(move |check_btn| {
        check_btn.deactivate();

        let ip_port: &str = &format!("{}.{}.{}.{}:{}", first_txt.value(), second_txt.value(), third_txt.value(), fourth_txt.value(), port_txt.value());
        
        if port_check::is_port_reachable(ip_port) 
            {status_label.set_label("Status: OPEN")} 
        else 
            {status_label.set_label("Status: CLOSED")}
        
        address_label.set_label(ip_port);

        check_btn.activate();
    });

    window.end();
    window.show();
    netport.run().unwrap();
}
