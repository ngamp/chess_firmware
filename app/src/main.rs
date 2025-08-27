use std::{cell::{Cell, RefCell}, rc::Rc};
use position::position::{DrawR, State};
use adw::prelude::*;
use gtk::{glib::{self, clone}, Align, ApplicationWindow, Box, Button, CheckButton, Entry, Label, Orientation, SpinButton, Stack, StackSwitcher, ToggleButton};
use mainp::{Game, Machine, MachineErrors};

const APP_ID: &str = "org.gtk_rs.GObjectProperties3";
const XDIRPIN: u8 = 23;
const XSTEPPIN: u8 = 24;
const XENBPIN: u8 =  25;
const YDIRPIN: u8 = 5;
const YSTEPPIN: u8 = 6;
const YENBPIN: u8 =  13;
const MAGNETPIN: u8 = 26;

#[cfg(target_arch = "aarch64")]
const ONRASPI: bool = false;
#[cfg(target_arch = "x86_64")]
const ONRASPI: bool = false;

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

    let typingbox = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(6)
        .orientation(Orientation::Vertical)
        .build();
    let erowbox1 = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(6)
        .orientation(Orientation::Horizontal)
        .build();
    let erowbox2 = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(6)
        .orientation(Orientation::Horizontal)
        .build();
    let erowbox3 = Box::builder()
        .valign(Align::Center)
        .halign(Align::Center)
        .spacing(6)
        .orientation(Orientation::Horizontal)
        .build();
    let entera = Button::with_label("A");
    let enterb = Button::with_label("B");
    let enterc = Button::with_label("C");
    let enterd = Button::with_label("D");
    let entere = Button::with_label("E");
    let enterf = Button::with_label("F");
    let enterg = Button::with_label("G");
    let enterh = Button::with_label("H");
    let enter1 = Button::with_label("1");
    let enter2 = Button::with_label("2");
    let enter3 = Button::with_label("3");
    let enter4 = Button::with_label("4");
    let enter5 = Button::with_label("5");
    let enter6 = Button::with_label("6");
    let enter7 = Button::with_label("7");
    let enter8 = Button::with_label("8");
    let backspace = Button::with_label("    <=    ");
    let _enterleft = Button::with_label("    <    ");
    let _enterright = Button::with_label("    >    ");
    //let makemove = Button::with_label("Make Move");
    //makemove.set_sensitive(false);
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
    erowbox1.append(&entera);
    erowbox1.append(&enterb);
    erowbox1.append(&enterc);
    erowbox1.append(&enterd);
    erowbox1.append(&entere);
    erowbox1.append(&enterf);
    erowbox1.append(&enterg);
    erowbox1.append(&enterh);
    erowbox2.append(&enter1);
    erowbox2.append(&enter2);
    erowbox2.append(&enter3);
    erowbox2.append(&enter4);
    erowbox2.append(&enter5);
    erowbox2.append(&enter6);
    erowbox2.append(&enter7);
    erowbox2.append(&enter8);
    //erowbox3.append(&enterleft);
    //erowbox3.append(&enterright);
    erowbox3.append(&backspace);
    //erowbox3.append(&makemove);
    typingbox.append(&erowbox1);
    typingbox.append(&erowbox2);
    typingbox.append(&erowbox3);
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
    

    let moveenterbox = Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .valign(Align::End)
        .halign(Align::Center)
        .spacing(12)
        .orientation(Orientation::Vertical)
        .build();

        let actionsbox = Box::builder()
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
    stack.add_titled(&moveenterbox, Some("move"), "move");
    stack.add_titled(&actionsbox, Some("actions"), "actions");
    stack.set_visible_child_name("setup");

    let stackswitcher = StackSwitcher::new();
    stackswitcher.set_stack(Some(&stack));

    setupbox.append(&welobox);
    setupbox.append(&belobox);
    setupbox.append(&savebox);
    setupbox.append(&sftimebox);
    //setupbox.append(&Frame::builder().child(&statuslabel).margin_top(12).build());

    moveenterbox.append(&typingbox);
    moveenterbox.append(&moveentry);

    actionsbox.append(&startbutton);

    mainbox.append(&stackswitcher);
    mainbox.append(&stack);
    mainbox.append(&statuslabel);
    
    let window = ApplicationWindow::builder()
        .application(app)
        .title("chess_firmware")
        //.fullscreened(true)
        .default_height(304)
        .default_width(480)
        .show_menubar(true)
        .child(&mainbox)
        .build();

    window.set_fullscreened(ONRASPI);
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
// region button inputs
    enter1.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('1');
        moveentry.set_text(&t);
        }));
    enter2.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('2');
        moveentry.set_text(&t);
        }));
    enter3.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('3');
        moveentry.set_text(&t);
        }));
    enter4.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('4');
        moveentry.set_text(&t);
        }));
    enter5.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('5');
        moveentry.set_text(&t);
        }));
    enter6.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('6');
        moveentry.set_text(&t);
        }));
    enter7.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('7');
        moveentry.set_text(&t);
        }));
    enter8.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('8');
        moveentry.set_text(&t);
        }));
    entera.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('a');
        moveentry.set_text(&t);
        }));
    enterb.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('b');
        moveentry.set_text(&t);
        }));
    enterc.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('c');
        moveentry.set_text(&t);
        }));
    enterd.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('d');
        moveentry.set_text(&t);
        }));
    entere.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('e');
        moveentry.set_text(&t);
        }));
    enterf.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('f');
        moveentry.set_text(&t);
        }));
    enterg.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('g');
        moveentry.set_text(&t);
        }));
    enterh.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.push('h');
        moveentry.set_text(&t);
        }));
    backspace.connect_clicked(clone!(#[strong]moveentry, move |_| {
        let mut t = moveentry.text().to_string();
        t.pop();
        moveentry.set_text(&t);
        }));
    moveentry.connect_icon_press(move |ent, _| {
        let text = ent.text().to_string().trim().to_string();
        if text.len() > 3 {
            let mut gm = game.borrow_mut();
            match gm.check_possible_move(&text) {
                Ok(ty) => {
                    statuslabel.set_text(&format!("Move valid: {:?}", ty));
                    match gm.update(Game::ctim(&text).unwrap(), &text, 3000, 1000) {
                        Ok((st, pfi)) => {
                            match st {
                                State::Mate(true) => statuslabel.set_text("Checkmate! White wins!"),
                                State::Mate(false) => statuslabel.set_text("Checkmate! Black wins!"),
                                State::Draw(DrawR::FiftyMove) => statuslabel.set_text("Draw by fifty-move rule"),
                                State::Draw(DrawR::Repetition) => statuslabel.set_text("Draw by repetition"),
                                State::Draw(DrawR::Stalemate) => statuslabel.set_text("Draw by stalemate"),
                                State::Normal => statuslabel.set_text("Move made successfully"),
                                _ => {}
                            };
                            ent.set_text("");
                            if gm.get_current_color() {
                                if gm.wm {
                                    //let currentpos = &gm.machine.pos_mtr;
                                    match gm.execute_move(pfi) {
                                        Ok(_) => statuslabel.set_text("White moved automatically and successfully"),
                                        Err(rr) => statuslabel.set_text(&format!("Failed to make automatic white move: {:?}", rr))
                                    }
                                }
                            } else {
                                if gm.bm {
                                    match gm.execute_move(pfi) {
                                        Ok(_) => statuslabel.set_text("Black moved automatically and successfully"),
                                        Err(rr) => statuslabel.set_text(&format!("Failed to make automatic bblack move: {:?}", rr))
                                    }
                                }
                            }
                        },
                        Err(rr) => statuslabel.set_text(&format!("Failed to update position: {:?}", rr).as_str())
                    }},
                Err(rr) => {//makemove.set_sensitive(false);
                    statuslabel.set_text(&format!("Invalid move: {:?}", rr))}
            }
            //let res = pos.position.update(, &text, if pos.position.colorw {welo.value_as_int() as u32} else {belo.value_as_int() as u32}, sftime.value_as_int() as u32)
        } else {
            statuslabel.set_text("Move too short");
        };
        });
// endregion
    
    
    
}

fn get_game() -> Result<Game, MachineErrors> {
    Game::new((true, XDIRPIN, XSTEPPIN, XENBPIN), (false, YDIRPIN, YSTEPPIN, YENBPIN), MAGNETPIN)
}

//(wmbutton.is_active(), bmbutton.is_active(), wsbutton.is_active(), bsbutton.is_active(), welo.value_as_int().abs() as u32, belo.value_as_int().abs() as u32, sftime.value_as_int().abs() as u32)