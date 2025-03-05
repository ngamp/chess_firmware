#![allow(unused_imports)]
use mctrl::{motor::diagonal, *};
use position::position;

fn main() {
    /*let mut m1 = motor::Mtr::new(true, 5, 6, 13).unwrap();
    m1.enable_motor();
    m1.move_steps(1600, true, 2.5).unwrap();
    delay::delayms(200);
    m1.move_steps(400, false, 4.0).unwrap();
    delay::delayms(200);
    m1.move_steps(800, true, 6.0).unwrap();
    delay::delayms(200);
    m1.move_steps(1200, false, 5.0).unwrap();
    m1.disable_motor();

    let mut m2 = motor::Mtr::new(true, 23, 24, 25).unwrap();
    m2.enable_motor();
    m2.move_steps(120, true, 4.0).unwrap();
    delay::delayms(200);
    m2.move_steps(400, false, 4.0).unwrap();
    delay::delayms(200);
    m2.move_steps(800, true, 6.0).unwrap();
    delay::delayms(200);
    m2.move_steps(1200, false, 5.0).unwrap();

    m1.enable_motor();
    m2.enable_motor();

    diagonal(&mut m1, &mut m2, false, true, 1600, 3.0).unwrap();
    diagonal(&mut m1, &mut m2, false, true, 3200, 5.5).unwrap();

    m1.disable_motor();
    m2.disable_motor();



    let mut magnet = magnet::Magnet::new(26).unwrap();
    magnet.on();
    delay::delayms(7000);
    magnet.off();*/
    std::env::set_var("RUST_BACKTRACE", "1");
    let a = position::Position::from_fen(
        "8/3R4/3kp3/1Q6/1P6/8/1PK1P3/8 b - - 8 44",
    );
    println!("{:?}", a);
}
