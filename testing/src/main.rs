use mctrl::*;

fn main() {
    let mut m1 = motor::Mtr::new(true, 5, 6, 13).unwrap();
    m1.enable_motor();
    m1.move_steps(600, true, 3.0).unwrap();
    //m1.move_steps(500, false, 2.0).unwrap();
    m1.disable_motor();

    let mut m2 = motor::Mtr::new(true, 23, 24, 25).unwrap();
    m2.enable_motor();
    m2.move_steps(2000, true, 2.0).unwrap();
    //m2.move_steps(500, false, 2.0).unwrap();
    m2.disable_motor();

    let mut magnet = magnet::Magnet::new(26).unwrap();
    magnet.on();
    delay::delayms(3000);
    magnet.off();
}
