#![allow(unused_imports)]
use mctrl::{motor::{diagonal, Field, FieldUsize, PosNow}, *};
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



    let mut magnet = motor::Magnet::new(26).unwrap();
    magnet.on();
    delay::delayms(7000);
    magnet.off();
    let mut a = position::Position::from_fen(
        "r3k2r/pp2ppbp/4b3/2p5/4q1P1/2P1B2P/PP2P3/RN1QK1NR w KQkq - 0 13",
    ).unwrap();
    for i in a.fields {
        println!("{:?}", i)
    }
    println!("h");
    let res = a.update(((5, 7), (3, 5)), "e3c5");
    for i in a.fields {
        println!("{:?}", i)
    };
    println!("{:?}", res);
    let res = a.update(((0, 7), (0, 9)), "e8g8").unwrap();
    for i in a.fields {
        println!("{:?}", i)
    };
    println!("{:?}", res);
    let res = a.update(((4, 9), (3, 9)), "g4g5").unwrap();
    for i in a.fields {
        println!("{:?}", i)
    };
    println!("{:?}", res);
    let res = a.update(((1, 10), (3, 10)), "h7h5").unwrap();
    for i in a.fields {
        println!("{:?}", i)
    };
    println!("{:?}", res);
    let res = a.update(((3, 9), (2, 10)), "g5h6").unwrap();
    for i in a.fields {
        println!("{:?}", i)
    };
    println!("{:?}", res);
    println!("{:?}", a);

    println!("{}", std::env::consts::ARCH);*/

    
    let a = motor::MotorInstructions::field_to_field(Field::ind_to_relative_ind((4, 4)), Field::ind_to_relative_ind((2, 2)), 4.0, true, &mut PosNow::new_from_field(Field::ind_to_relative_ind((5, 7))));
    let b = motor::MotorInstructions::diagonal(Field::from_tuple((-0.5, -1.5)), Field::from_tuple((3.5, -4.5)), 3.0, true, &mut PosNow::new_from_field(Field::ind_to_relative_ind((4, 5))));
    let os = motor::OffSet::new(Field::from_tuple((1, 1)), Some(false), Some(true));
    let c = os.offset(&mut PosNow::new());
    let d = os.resolve(&mut PosNow::new());

    let f1 = Field::ind_to_relative_ind((2, 2));
    let f2 = Field::ind_to_relative_ind((3, 3));
    let f3 = Field::ind_to_relative_ind((5, 5));
    let e = motor::MotorInstructions::field_to_field(f2, f3, 4.0, true, &mut PosNow::new_from_field(f1));
    let bl = position::BitList::from_pos(&position::Position::from_fen("r1b1kb1r/pppp1ppp/5q2/8/4P3/2NP4/PPP1QPPP/2KR1B1R w kq - 0 1").unwrap());



    a.print_out();
    b.print_out();
    c.print_out();
    d.print_out();
    e.print_out();
    bl.print_out();

    println!("bitlist: .count_area()  {}", bl.count_area(FieldUsize(0, 8), FieldUsize(5, 8)));
}
