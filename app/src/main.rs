use adw::{prelude::*, NavigationPage, NavigationSplitView};
use gtk::{glib, Align, Application, ApplicationWindow, Box, Button, CheckButton, Label, Notebook, Orientation, SpinButton, Stack, StackPage, StackSidebar, StackSwitcher, Switch};

const APP_ID: &str = "org.gtk_rs.GObjectProperties3";
fn main() -> glib::ExitCode {
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &adw::Application) {
    let wsbutton = CheckButton::with_label("   White moves with Stockfish -> Elo:");
    let bsbutton = CheckButton::with_label("   Black moves with Stockfish -> Elo:");
    let wmbutton = CheckButton::with_label("   White moves automatically");
    let bmbutton = CheckButton::with_label("   Black moves automatically");
    let welo = SpinButton::with_range(1320.0, 3190.0, 10.0);
    let belo = SpinButton::with_range(1320.0, 3190.0, 10.0);
    let sftime = SpinButton::with_range(100.0, 10000.0, 100.0);
    sftime.set_value(1000.0);
    welo.set_sensitive(false);
    belo.set_sensitive(false);
    sftime.set_sensitive(false);
    wsbutton.bind_property("active", &wmbutton, "active").build();
    wsbutton.bind_property("active", &welo, "sensitive").build();
    bsbutton.bind_property("active", &bmbutton, "active").build();
    bsbutton.bind_property("active", &belo, "sensitive").build();
    //wsbutton.bind_property("active", &sftime, "sensitive").build();
    //bsbutton.bind_property("active", &sftime, "sensitive").build();


    let welobox = Box::builder()
        .valign(Align::Center)
        .halign(Align::Start)
        .spacing(12)
        .orientation(Orientation::Horizontal)
        .build();
    let belobox = Box::builder()
        .valign(Align::Center)
        .halign(Align::Start)
        .spacing(12)
        .orientation(Orientation::Horizontal)
        .build();
    let sftimebox = Box::builder()
        .valign(Align::Center)
        .halign(Align::Start)
        .spacing(12)
        .orientation(Orientation::Horizontal)
        .build();
    sftimebox.append(&sftime);
    sftimebox.append(&Label::new(Some("  Time for Stockfish to think")));
    welobox.append(&wsbutton);
    welobox.append(&welo);
    belobox.append(&bsbutton);
    belobox.append(&belo);
    let mainbox = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::End)
        .halign(Align::Start)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    mainbox.append(&welobox);
    mainbox.append(&belobox);
    mainbox.append(&wmbutton);
    mainbox.append(&bmbutton);
    mainbox.append(&sftimebox);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("chess_firmware - New Game")
        .default_height(320)
        .default_width(480)
        .child(&mainbox)
        .build();
    // Present the window
    window.present();
    
}