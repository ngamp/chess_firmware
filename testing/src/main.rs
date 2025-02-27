use mctrl::motor;

fn main() {
    let mut m1 = motor::Mtr::new(true, 5, 6, 13).unwrap();
    m1.enable_motor();
    m1.move_steps(200, true, 600000);

}
