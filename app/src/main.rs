use std::{cell::{Cell, RefCell}, rc::Rc};

use adw::prelude::*;
use gtk::{gio::Icon, glib::{self, clone}, Align, ApplicationWindow, Box, Button, CheckButton, Entry, Frame, Label, Orientation, SpinButton, Stack, StackSwitcher, ToggleButton};
use mainp::{Game, Machine, MachineErrors};

const APP_ID: &str = "org.gtk_rs.GObjectProperties3";
const XDIRPIN: u8 = 23;
const XSTEPPIN: u8 = 24;
const XENBPIN: u8 =  25;
const YDIRPIN: u8 = 5;
const YSTEPPIN: u8 = 6;
const YENBPIN: u8 =  13;
const MAGNETPIN: u8 = 26;
fn main() -> glib::ExitCode {
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}


fn build_ui(app: &adw::Application) {
    let mut game = Rc::new(RefCell::new(Game {machine: Machine::dummy(), wm: false, bm: false, ws: false, bs: false, welo: 2000, belo: 2000, sftime: 1000, currentmove: None }));
    let wsbutton = CheckButton::with_label("   White moves with Stockfish -> Elo:");
    let bsbutton = CheckButton::with_label("   Black moves with Stockfish -> Elo:");
    let wmbutton = CheckButton::with_label("   White moves automatically");
    let bmbutton = CheckButton::with_label("   Black moves automatically");
    let welo = SpinButton::with_range(1320.0, 3190.0, 10.0);
    let belo = SpinButton::with_range(1320.0, 3190.0, 10.0);
    let sftime = SpinButton::with_range(100.0, 10000.0, 100.0);
    let startbutton = ToggleButton::builder()
        .label("Start Game")
        .build();
    let savebutton = Button::with_label("Save Settings");
    let moveentry = Entry::builder()
        .placeholder_text("Enter your move:")
        .secondary_icon_name("object-select-symbolic")
        //.secondary_icon_gicon(secondary_icon_gicon)
        .secondary_icon_tooltip_text("Check move")
        .build();
    sftime.set_value(1000.0);
    welo.set_sensitive(false);
    belo.set_sensitive(false);
    wsbutton.bind_property("active", &wmbutton, "active").build();
    wsbutton.bind_property("active", &welo, "sensitive").build();
    bsbutton.bind_property("active", &bmbutton, "active").build();
    bsbutton.bind_property("active", &belo, "sensitive").build();
    //wsbutton.bind_property("active", &sftime, "sensitive").build();
    //bsbutton.bind_property("active", &sftime, "sensitive").build();

    let sfbox = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(6)
        .build();
    let savebox = Box::builder()
    .valign(Align::Center)
    .halign(Align::Start)
    .spacing(24)
    .orientation(Orientation::Horizontal)
    .build();
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
    let statuslabel = Label::new(Some("OK"));
    sftimebox.append(&sftime);
    sftimebox.append(&Label::new(Some("  Time for Stockfish to think")));
    welobox.append(&wsbutton);
    welobox.append(&welo);
    belobox.append(&bsbutton);
    belobox.append(&belo);
    sfbox.append(&wmbutton);
    sfbox.append(&bmbutton);
    savebox.append(&sfbox);
    savebox.append(&savebutton);
    let setupbox = Box::builder()
        //.margin_top(12)
        //.margin_bottom(12)
        .margin_start(12)
        //.margin_end(12)
        .valign(Align::End)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    

    let statusbox = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::End)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

    let mainbox = Box::builder()
        //.margin_top(6)
        //.margin_bottom()
        //.margin_start(12)
        //.margin_end(12)
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();
    

    let stack = Stack::new();
    stack.add_titled(&setupbox, Some("setup"), "setup");
    stack.add_titled(&statusbox, Some("status"), "status");
    stack.set_visible_child_name("setup");

    let stackswitcher = StackSwitcher::new();
    stackswitcher.set_stack(Some(&stack));

    setupbox.append(&welobox);
    setupbox.append(&belobox);
    setupbox.append(&savebox);
    setupbox.append(&sftimebox);
    setupbox.append(&Frame::builder().child(&statuslabel).margin_top(12).build());

    statusbox.append(&moveentry);
    statusbox.append(&startbutton);

    mainbox.append(&stackswitcher);
    mainbox.append(&stack);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("chess_firmware")
        .default_height(320)
        .default_width(480)
        .show_menubar(false)
        .child(&mainbox)
        .build();


    // Present the window
    window.present();

    let sd = match get_game() {
        Ok(g) => {
            statuslabel.set_text("Game initialized successfully");
            game = Rc::new(RefCell::new(g));
            false
        },
        Err(rr) => {
            statuslabel.set_text(format!("Failed to initialize Game: {:?}", rr).as_str());
            savebutton.set_label("Shutdown now");
            true
        }
    };

    savebutton.connect_clicked(clone!(#[strong]game, move |_| {
        if sd {
            //println!("{}, {}", window.height() , window.width());
            window.close()
        } else {
            game.borrow_mut().set_settings((wmbutton.is_active(), bmbutton.is_active(), wsbutton.is_active(), 
            bsbutton.is_active(), welo.value_as_int().abs() as u32, belo.value_as_int().abs() as u32, sftime.value_as_int().abs() as u32));
        };
    }));

    let running = Cell::new(false);
    startbutton.connect_clicked(move |but| {
        running.set(!running.get());
        if running.get() {
            but.set_label("Pause");
        } else {
            but.set_label("Resume");
        }
        println!("{:?}", running)
        });

    
    
    
}

fn get_game() -> Result<Game, MachineErrors> {
    Game::new((true, XDIRPIN, XSTEPPIN, XENBPIN), (false, YDIRPIN, YSTEPPIN, YENBPIN), MAGNETPIN)
}

//(wmbutton.is_active(), bmbutton.is_active(), wsbutton.is_active(), bsbutton.is_active(), welo.value_as_int().abs() as u32, belo.value_as_int().abs() as u32, sftime.value_as_int().abs() as u32)