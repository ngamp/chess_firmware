use mctrl::*;

fn main() {
    let mut m1 = motor::Mtr::new(true, 5, 6, 13).unwrap();
    m1.enable_motor();
    m1.move_steps(1000, true, 2000000).unwrap();
    m1.move_steps(500, false, 600000).unwrap();

    let mut m2 = motor::Mtr::new(true, 23, 24, 25).unwrap();
    m2.enable_motor();
    m2.move_steps(1000, true, 2000000).unwrap();
    m2.move_steps(500, false, 600000).unwrap();

    let mut magnet = magnet::Magnet::new(26).unwrap();
    magnet.on();
    delay::delayms(5000);
    magnet.off();
}
