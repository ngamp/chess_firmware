use mctrl::{delay::delaymics, motor::{rps_to_del, Magnet, MotorInstructions, MotorMoveType, Mtr, MtrErrors, PosNow, Speeds}};
use position::position::{ctim, MoveError, MoveType, PFError, PFIType, Position, State, UpdateError};
use stockfish::{SFErrors, SFResults};
pub use stockfish::SFResults as SFResEx;

#[derive(Debug)]
pub enum MachineErrors {
    Position(MoveError),
    Motor(MtrErrors)
}
#[derive(Debug)]
pub struct Machine {
    pub xmtr: Mtr,
    pub ymtr: Mtr,
    pub magnet: Magnet,
    pub position: Position,
    pub pos_mtr: PosNow,
}

impl Machine {

    pub fn dummy() -> Self {
        Self { xmtr: Mtr::dummy(), ymtr: Mtr::dummy(), magnet: Magnet::dummy(), position: Position::new_reset(), pos_mtr: PosNow::new() }
    }

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
            self.xmtr.dirpin.as_mut().unwrap().set_high();
        } else {
            self.xmtr.dirpin.as_mut().unwrap().set_low();
        };
        if ydir {
            self.ymtr.dirpin.as_mut().unwrap().set_high();
        } else {
            self.ymtr.dirpin.as_mut().unwrap().set_low();
        };
        let del = rps_to_del(speed.to_f32());
        let xsteppin = self.xmtr.steppin.as_mut().unwrap();
        let ysteppin = self.ymtr.steppin.as_mut().unwrap();
        for _ in 0..steps {
            xsteppin.set_high();
            ysteppin.set_high();
            delaymics(del);
            xsteppin.set_low();
            ysteppin.set_low();
            delaymics(del);
        };
        //self.pos_mtr.update(true, steps, xdir);
        //self.pos_mtr.update(false, steps, ydir);
    }

    pub fn print_status(&self) {
        println!("Machine:");
        println!("xmtr enabled: {}, ymtr enabled: {}, magnet enabled: {}", self.xmtr.is_enabled(), self.ymtr.is_enabled(), self.magnet.status());
        println!("Motorposition: {:?}", self.pos_mtr);
        //println!("Position:\n{:?}", self.position.;
    }



    pub fn do_mi(&mut self, mi: MotorInstructions) {
        println!("starting move: {:?}", self.pos_mtr);
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
                    self.xmtr.move_steps(mm.len, mm.dir, mm.speed.to_f32());
                },
                MotorMoveType::StraightY(mm) => {
                    if mm.magnet {
                        self.magnet.on();
                    } else {
                        self.magnet.off();
                    };
                    self.ymtr.move_steps(mm.len, mm.dir, mm.speed.to_f32());
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
        println!("finished move: {:?}", self.pos_mtr);
        self.xmtr.disable_motor();
        self.ymtr.disable_motor();
        self.magnet.off();
    }
}

#[derive(Debug)]
pub struct Game {
    pub machine: Machine,
    pub wm: bool,
    pub bm: bool,
    pub ws: bool,
    pub bs: bool,
    pub welo: u32,
    pub belo:  u32,
    pub sftime: u32,
    pub currentmove: Option<String>
}

#[derive(Debug)]
pub enum ExecError {
    Pathfinding(PFError),
    Executing
}

impl From<PFError> for ExecError {
    fn from(err: PFError) -> Self {
        ExecError::Pathfinding(err)
    }
}

impl Game {
    pub fn new(xmtr: (bool, u8, u8, u8), ymtr: (bool, u8, u8, u8), magnet: u8) -> Result<Self, MachineErrors> {
        let machine = Machine::new(xmtr, ymtr, magnet)?;
        Ok(Game { machine , wm: false, bm: false, ws: false, bs: false, welo: 1500, belo: 1500, sftime: 1000, currentmove: None })
    }

    pub fn set_settings(&mut self, set: (bool, bool, bool, bool, u32, u32, u32)) {
        self.wm = set.0;
        self.bm = set.1;
        self.ws = set.2;
        self.bs = set.3;
        self.welo = set.4;
        self.belo = set.5;
        self.sftime = set.6;
    }

    pub fn next() {

    }

    pub fn execute_move(&mut self, mov: Vec<PFIType>, mut oldpos: Position) -> Result<(), ExecError> {
        println!("Executing move: {:?}", mov);
        println!("Old position:");
        oldpos.print_out();
        println!("Current motor position: {:?}", self.machine.pos_mtr);
        let mi = oldpos.pathfinding(&mov, &mut self.machine.pos_mtr)?;
        println!("position now: {:?}", self.machine.pos_mtr);
        mi.print_out();
        self.machine.do_mi(mi);
        self.machine.print_status();
        Ok(())
    }

    pub fn get_current_color(&self) -> bool {
        self.machine.position.colorw
    }

    pub fn ctim(coordinate: &str) -> Result<((usize, usize), (usize, usize)), MoveError> {
        ctim(coordinate)
    }

    pub fn check_possible_move(&self, mov: &str) -> Result<MoveType, MoveError> {
        Ok(self.machine.position.validate_move_possibility(mov)?)
    }

    pub fn update(&mut self, ind_move: ((usize, usize), (usize, usize)), coord_move: &str, elo: u32, time: u32) -> Result<(State, Vec<PFIType>, Position), UpdateError> {
        self.machine.position.update(ind_move, coord_move, elo, time)
    }

    pub fn get_sf_move(&self) -> Result<SFResults, SFErrors> {
        stockfish::get_move(&self.machine.position.to_fen(), if self.machine.position.colorw {self.welo} else {self.belo}, self.sftime)
    }
}

pub fn mainf() {

}