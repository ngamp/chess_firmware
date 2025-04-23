use mctrl::{delay::delaymics, motor::{rps_to_del, Magnet, MotorInstructions, MotorMoveType, Mtr, MtrErrors, PosNow, Speeds}};
use position::position::{MoveError, Position};

#[derive(Debug)]
pub enum MachineErrors {
    Position(MoveError),
    Motor(MtrErrors)
}
#[derive(Debug)]
pub struct Machine {
    xmtr: Mtr,
    ymtr: Mtr,
    magnet: Magnet,
    position: Position,
    pos_mtr: PosNow,
}

impl Machine {

    pub fn new(xmtr: (bool, u8, u8, u8), ymtr: (bool, u8, u8, u8), magnet: u8) -> Result<Self, MachineErrors> {
        let xmtr = match Mtr::new(xmtr.0, xmtr.1, xmtr.2,  xmtr.3) {
            Ok(xm) => xm,
            Err(rr) => return Err(MachineErrors::Motor(rr))
        };
        let ymtr = match Mtr::new(ymtr.0, ymtr.1, ymtr.2,  ymtr.3) {
            Ok(ym) => ym,
            Err(rr) => return Err(MachineErrors::Motor(rr))
        };
        let mgnt = match Magnet::new(magnet) {
            Ok(m) => m,
            Err(rr) => return Err(MachineErrors::Motor(rr))
        };
        Ok(Self { xmtr, ymtr, magnet: mgnt, position: Position::new_reset(), pos_mtr: PosNow::new() })
    }

    pub fn set_position(&mut self, fen: &str) -> Result<(), MachineErrors>{
        self.position = match Position::from_fen(fen) {
            Ok(res) => res,
            Err(rr) => return Err(MachineErrors::Position(rr))
        };
        Ok(())
    }

    pub fn diagonal(&mut self, xdir: bool, ydir: bool, steps: u32, speed: Speeds) {
        if xdir {
            self.xmtr.dirpin.set_high();
        } else {
            self.xmtr.dirpin.set_low();
        };
        if ydir {
            self.ymtr.dirpin.set_high();
        } else {
            self.ymtr.dirpin.set_low();
        };
        let del = rps_to_del(speed.to_f32());
        for _ in 0..steps {
            self.xmtr.steppin.set_high();
            self.ymtr.steppin.set_high();
            delaymics(del);
            self.xmtr.steppin.set_low();
            self.ymtr.steppin.set_low();
            delaymics(del);
        };
        self.pos_mtr.update(true, steps, xdir);
        self.pos_mtr.update(false, steps, ydir);
    }

    pub fn print_status(&self) {
        println!("Machine:");
        println!("xmtr enabled: {}, ymtr enabled: {}", self.xmtr.is_enabled(), self.ymtr.is_enabled());
        println!("Motorposition: {:?}", self.pos_mtr);
    }

    pub fn do_mi(&mut self, mi: MotorInstructions) {
        self.xmtr.enable_motor();
        self.ymtr.enable_motor();
        for instruction in mi.instructions {
            match instruction {
                MotorMoveType::StraightX(mm) => {
                    if mm.magnet {
                        self.magnet.on();
                    } else {
                        self.magnet.off();
                    };
                    self.xmtr.move_steps(mm.len, mm.dir, mm.speed.to_f32(), &mut self.pos_mtr);
                },
                MotorMoveType::StraightY(mm) => {
                    if mm.magnet {
                        self.magnet.on();
                    } else {
                        self.magnet.off();
                    };
                    self.ymtr.move_steps(mm.len, mm.dir, mm.speed.to_f32(), &mut self.pos_mtr);
                },
                MotorMoveType::Diagonal(mm) => {
                    if mm.magnet {
                        self.magnet.on();
                    } else {
                        self.magnet.off();
                    };
                    self.diagonal(mm.dir, mm.dir2, mm.len, mm.speed);
                }
            };
            delaymics(100000);
        };
        self.xmtr.disable_motor();
        self.ymtr.disable_motor();
        self.magnet.off();
    }
}

#[derive(Debug)]
pub struct Game {
    pub machine: Machine,
    wm: bool,
    bm: bool,
    ws: bool,
    bs: bool,
    welo: u32,
    belo:  u32,
    sftime: u32
}
