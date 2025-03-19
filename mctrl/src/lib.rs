pub const MMR: f32 = 14.135;
pub const MMF: f32 = 45.0;
pub const HOMINGSPEED: f32 = 5.0;
pub const NMOVESPEED: f32 = 2.0;
pub const OFFSETRATIO: f32 = 1.0/3.0;
pub const OFFSETSPEED: f32 = 1.5;
pub const NOFIGURESPEED: f32 = 4.5;
pub const TRANSPORTSPEED: f32 = 2.0;

pub mod motor {

    use core::f32;
    use std::ops::{Add, Sub};

    use rppal::gpio::{Error, OutputPin, Gpio};

    use crate::{delay, HOMINGSPEED, MMF, MMR, NOFIGURESPEED, OFFSETRATIO, OFFSETSPEED};

    #[derive(Debug)]
    pub enum MtrErrors {
        GpioCreationError,
        PinGettingError(Error),
        MotorDisabled
    }

    #[derive(Debug)]
    pub struct Mtr {
        pub xaxis: bool,
        pub dirpin: OutputPin,
        pub steppin: OutputPin,
        pub enb_pin: OutputPin,
    }


    impl Mtr {
        pub fn new(xaxis: bool, dp: u8, sp: u8, enbp: u8) -> Result<Self, MtrErrors>  {
            let gp = match Gpio::new() {
                Ok(gp) => gp,
                Err(_) => return Err(MtrErrors::GpioCreationError)
            };
            let dirpin = match Gpio::get(&gp, dp) {
                Ok(dp) => dp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            let steppin = match Gpio::get(&gp, sp) {
                Ok(sp) => sp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            let enablepin = match Gpio::get(&gp, enbp) {
                Ok(sp) => sp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            Ok(Mtr {
                xaxis: xaxis,
                dirpin: dirpin,
                steppin: steppin,
                enb_pin: enablepin,
            })

        }

        pub fn enable_motor(&mut self) {
            self.enb_pin.set_high();
        }

        pub fn disable_motor(&mut self) {
            self.enb_pin.set_low();
        }

        pub fn move_steps(&mut self, steps: u32, direction: bool, speed: f32, pos: &mut PosNow) -> Result<(), MtrErrors>{
            if self.enb_pin.is_set_low() {
                return Err(MtrErrors::MotorDisabled)
            };
            pos.update(self.xaxis, steps, direction);
            if direction {
                self.dirpin.set_high();
            } else {
                self.dirpin.set_low();
            };
            let del = rps_to_del(speed);
            for _ in 0..steps {
                self.steppin.set_high();
                delay::delaymics(del);
                self.steppin.set_low();
                delay::delaymics(del);
            };
            Ok(())
        }
    }

    fn rps_to_del(rps: f32) -> u32 {
        (5000.0/rps) as u32 / 2
    }

    pub fn diagonal(x: &mut Mtr, y: &mut Mtr, right: bool, up: bool, steps: u32, speed: f32, pos: &mut PosNow) -> Result<(), MtrErrors>{
        if x.enb_pin.is_set_low() || y.enb_pin.is_set_low() {
            return Err(MtrErrors::MotorDisabled)
        };
        pos.update(true, steps, right);
        pos.update(false, steps, up);
        if right {
            x.dirpin.set_high();
        } else {
            x.dirpin.set_low();
        };
        if up {
            y.dirpin.set_high();
        } else {
            y.dirpin.set_low();
        }
        let del = rps_to_del(speed);
        for _ in 0..steps {
            x.steppin.set_high();
            y.steppin.set_high();
            delay::delaymics(del);
            x.steppin.set_low();
            y.steppin.set_low();
            delay::delaymics(del);
        }
        Ok(())
    }

    #[derive(Debug)]
    pub struct Magnet {
        pub pin: OutputPin,
    }

    impl Magnet {
        pub fn new(pinnum: u8) -> Result<Self, MtrErrors> {
            let gp = match Gpio::new() {
                Ok(gp) => gp,
                Err(_) => return Err(MtrErrors::GpioCreationError)
            };
            let mgpin = match Gpio::get(&gp, pinnum) {
                Ok(mg) => mg.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            return Ok(Magnet {
                pin: mgpin
            })
        }

        pub fn status(&self) -> bool {
            return self.pin.is_set_high()
        }

        pub fn on(&mut self) {
            self.pin.set_high();
        }

        pub fn off(&mut self) {
            self.pin.set_low();
        }
    }

    #[derive(Debug)]
    pub struct PosNow {
        xmtr: i32,
        ymtr: i32
    }

    impl PosNow {
        pub fn new() -> Self {
            PosNow { xmtr: 0, ymtr: 0}
        }

        pub fn new_from_field(f: Field) -> Self {
            PosNow { xmtr: fields_to_steps_signed(f.0), ymtr: fields_to_steps_signed(f.1) }
        }

        pub fn update(&mut self, xaxis: bool, steps: u32, dir: bool) {
            if xaxis {
                if dir {
                    self.xmtr += steps as i32;
                } else {
                    self.xmtr -= steps as i32;
                };
            } else {
                if dir {
                    self.ymtr += steps as i32;
                } else {
                    self.ymtr -= steps as i32;
                };
            }
        }

        pub fn sfh_to_field(&self) -> Field {
            let mut x = 0.0;
            let mut y = 0.0;
            if self.xmtr != 0 {
                x = ((((self.xmtr/200) as f32 * MMR)/MMF).ceil()) - 0.5;
            };
            if self.ymtr != 0 {
                y = ((((self.ymtr/200) as f32 * MMR)/MMF).ceil()) - 0.5;
            };
            Field::from_tuple((x, y))
        }
    }

    #[derive(Debug)]
    #[derive(Clone, Copy)]
    pub enum MotorMoveType {
        StraightX(MotorMove),
        StraightY(MotorMove),
        Diagonal(MotorMove)
    }

    #[derive(Debug)]
    #[derive(Clone, Copy)]
    pub struct MotorMove {
        pub dir: bool,
        pub len: u32,
        pub speed: f32,
        pub dir2: bool,
        pub magnet: bool
    }

    impl MotorMove {
        pub fn new() -> Self {
            MotorMove {dir: true, len: 0, dir2: true, speed: 3.0, magnet: false}
        }

        pub fn new_values(dir: bool, len: u32, dir2: bool, speed: f32, magnet: bool) -> Self {
            MotorMove { dir, len, dir2, speed, magnet}
        }
    }

    #[derive(Debug)]
    pub struct MotorInstructions {
        pub instructions: Vec<MotorMoveType>
    }

    impl MotorInstructions {
        pub fn new() -> Self {
            MotorInstructions { instructions: Vec::new() }
        }

        pub fn append(&mut self, mut mi: MotorInstructions) {
            self.instructions.append(&mut mi.instructions);
        }

        pub fn from_vfield(field: Field, speed: f32, magnet: bool, pos: &mut PosNow) -> Self {
            let mut res = Vec::new();
            let xlen = fields_to_steps_signed(field.0) as i32;
            let ylen = fields_to_steps_signed(field.1) as i32;
            if xlen != 0 {
                res.push(MotorMoveType::StraightX(steps_to_motormove(xlen, speed, magnet)));
            };
            if ylen != 0 {
                res.push(MotorMoveType::StraightY(steps_to_motormove(ylen, speed, magnet)));
            }
            Self { instructions:  res}.write_to_pos(pos)
        }

        pub fn to_home(pos: &mut PosNow) -> Self {
            let x = -pos.xmtr;
            let y = -pos.ymtr;
            Self { instructions: vec![MotorMoveType::StraightX(steps_to_motormove(x, HOMINGSPEED, false)), MotorMoveType::StraightY(steps_to_motormove(y, HOMINGSPEED, false))] }.write_to_pos(pos)
        }

        pub fn field_to_field(f1: Field, f2: Field, speed: f32, magnet: bool, pos: &mut PosNow) -> Self {
            let f = f2-f1;
            let mut res = Self::new();
            if pos.sfh_to_field() != f1 {
                res.append(Self::from_vfield(f1 - pos.sfh_to_field(), NOFIGURESPEED, false, pos));
            };
            res.append(Self::from_vfield(f, speed, magnet, pos));
            res.write_to_pos(pos)
        }

        pub fn home_to_field(f: Field, pos: &mut PosNow) -> Self {
            Self::from_vfield(f, HOMINGSPEED, false, pos)
        }

        pub fn diagonal(f1: Field, f2: Field, speed: f32, magnet: bool, pos: &mut PosNow) -> Self {
            let mut res = Vec::new();
            let vf = f2 - f1;
            let mut len = fields_to_steps(vf.0.abs());
            if vf.0.abs() < vf.1.abs() {
                let ylen = fields_to_steps(vf.1.abs() - vf.0.abs());
                res.push(MotorMoveType::StraightY(MotorMove::new_values(vf.1.is_sign_positive(), ylen, true, speed, magnet)));
            };
            if vf.0.abs() > vf.1.abs() {
                len = fields_to_steps(vf.1.abs());
                let xlen = fields_to_steps(vf.0.abs() - vf.1.abs());
                res.push(MotorMoveType::StraightX(MotorMove::new_values(vf.0.is_sign_positive(), xlen, true, speed, magnet)));
            };
            res.push(MotorMoveType::Diagonal(MotorMove::new_values(vf.0.is_sign_positive(), len, vf.1.is_sign_positive(), speed, magnet)));
            return Self { instructions: res }.write_to_pos(pos)
        }

        pub fn print_out(&self) {
            println!("MotorInstructions:");
            for i in &self.instructions {
                println!("  {:?}", i);
            }
        }

        pub fn write_to_pos(self, pos: &mut PosNow) -> Self {
            let instructions = self.instructions.clone();
            for instruction in instructions {
                match instruction {
                    MotorMoveType::StraightX(mm) => pos.update(true, mm.len, mm.dir),
                    MotorMoveType::StraightY(mm) => pos.update(false, mm.len, mm.dir),
                    MotorMoveType::Diagonal(mm) => {
                        pos.update(true, mm.len, mm.dir);
                        pos.update(false, mm.len, mm.dir2);
                    }
                }
            }
            self
        }
    }

    pub struct OffSet {
        offset: (Option<bool>, Option<bool>),
        field: Field
    }

    impl OffSet {
        pub fn new(field: Field, x: Option<bool>, y: Option<bool>) -> Self {
            Self { offset: (x, y), field }
        }

        pub fn offset(&self, pos: &mut PosNow) -> MotorInstructions {
            let mut res = Vec::new();
            if pos.sfh_to_field() != self.field {
                res.append(&mut MotorInstructions::field_to_field(pos.sfh_to_field(), self.field, NOFIGURESPEED, false, pos).instructions)
            };
            match (self.offset.0, self.offset.1) {
                (Some(x), Some(y)) => {
                    res.push(MotorMoveType::Diagonal(MotorMove::new_values(x, fields_to_steps(OFFSETRATIO), y, OFFSETSPEED, true)));
                    res.push(MotorMoveType::Diagonal(MotorMove::new_values(!x, fields_to_steps(OFFSETRATIO), !y, OFFSETSPEED, false)));
                },
                (Some(x), None) => {
                    res.push(MotorMoveType::StraightX(MotorMove::new_values(x, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, true)));
                    res.push(MotorMoveType::StraightX(MotorMove::new_values(!x, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, false)));
                },
                (None, Some(y)) => {
                    res.push(MotorMoveType::StraightY(MotorMove::new_values(y, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, true)));
                    res.push(MotorMoveType::StraightY(MotorMove::new_values(!y, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, false)));
                },
                (None, None) => {}
            };
            MotorInstructions { instructions: res}.write_to_pos(pos)
        }

        pub fn resolve(self, pos: &mut PosNow) -> MotorInstructions {
            let mut res = Vec::new();
            if pos.sfh_to_field() != self.field {
                res.append(&mut MotorInstructions::field_to_field(pos.sfh_to_field(), self.field, NOFIGURESPEED, false, pos).instructions)
            };
            match (self.offset.0, self.offset.1) {
                (Some(x), Some(y)) => {
                    res.push(MotorMoveType::Diagonal(MotorMove::new_values(x, fields_to_steps(OFFSETRATIO), y, OFFSETSPEED, false)));
                    res.push(MotorMoveType::Diagonal(MotorMove::new_values(!x, fields_to_steps(OFFSETRATIO), !y, OFFSETSPEED, true)));
                },
                (Some(x), None) => {
                    res.push(MotorMoveType::StraightX(MotorMove::new_values(x, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, false)));
                    res.push(MotorMoveType::StraightX(MotorMove::new_values(!x, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, true)));
                },
                (None, Some(y)) => {
                    res.push(MotorMoveType::StraightY(MotorMove::new_values(y, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, false)));
                    res.push(MotorMoveType::StraightY(MotorMove::new_values(!y, fields_to_steps(OFFSETRATIO), true, OFFSETSPEED, true)));
                },
                (None, None) => {}
            };
            MotorInstructions { instructions: res}.write_to_pos(pos)
        }
    }


    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    pub struct Field(f32, f32);

    impl Sub for Field {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Add for Field {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    pub trait ToF32 {
        fn to_f32(self) -> f32;
    }

    impl ToF32 for f32 {
        fn to_f32(self) -> f32 {
            self
        }
    }

    impl ToF32 for usize {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }

    impl ToF32 for u32 {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }

    impl ToF32 for i32 {
        fn to_f32(self) -> f32 {
            self as f32
        }
    }

    impl Field {
        pub fn from_tuple<T: ToF32, S: ToF32>(t: (T, S)) -> Self {
            Field(t.0.to_f32(), t.1.to_f32())
        }

        pub fn ind_to_relative_ind<T: ToF32, S: ToF32>(ind: (T, S)) -> Self {
            let x: f32 = -6.5 + ind.1.to_f32();
            let y: f32 = 3.5 - ind.0.to_f32();
            Field(x, y)
        }

        pub fn from_field_usize(f: FieldUsize) -> Self {
            Field::ind_to_relative_ind(f.to_tuple())
        }
    }

    #[derive(Debug)]
    #[derive(PartialEq)]
    #[derive(Clone, Copy)]
    pub struct FieldUsize(pub usize, pub usize);

    impl Sub for FieldUsize {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    impl Add for FieldUsize {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self(self.0 + rhs.0, self.1 + rhs.1)
        }
    }

    pub trait ToUsize {
        fn to_usize(self) -> usize;
    }

    impl ToUsize for usize {
        fn to_usize(self) -> usize {
            self
        }
    }

    impl ToUsize for u32 {
        fn to_usize(self) -> usize {
            self as usize
        }
    }

    impl FieldUsize {
        pub fn from_tuple<T: ToUsize, S: ToUsize>(t: (T, S)) -> Self {
            Self(t.0.to_usize(), t.1.to_usize())
        }

        pub fn add_x(&self, x: usize) -> Self {
            Self(self.0, self.1 + x)
        }

        pub fn add_y(&self, y: usize) -> Self {
            Self(self.0 + y, self.1)
        }

        pub fn sub_x(&self, x: usize) -> Self {
            if self.1 > x {
                Self(self.0, self.1 - x)
            } else {
                self.clone()
            }
        }

        pub fn sub_y(&self, y: usize) -> Self {
            if self.1 > y {
                Self(self.0 - y, self.1)
            } else {
                self.clone()
            }
        }

        pub fn to_tuple(&self) -> (usize, usize) {
            (self.0, self.1)
        }
    }

    pub fn steps_to_motormove(ind: i32, speed: f32, magnet: bool) -> MotorMove {
        let xmovement = ind;
        let xmovement = if xmovement.is_negative() {
            MotorMove::new_values(false, xmovement.abs() as u32, true, speed, magnet)
        } else {
            MotorMove::new_values(true, xmovement as u32, true, speed, magnet)
        };
        xmovement
    }

    pub fn fields_to_steps(f: f32) -> u32 {
        (((MMF*f)/MMR)*200.0).round() as u32
    }

    pub fn fields_to_steps_signed(f: f32) -> i32 {
        (((MMF*f)/MMR)*200.0).round() as i32
    }
    
}

pub mod delay {
    use embedded_hal::delay::DelayNs;
    use rppal::hal::Delay;

    pub fn delayms(time: u32) {
        Delay.delay_ms(time);
    }

    pub fn delaymics(time: u32) {
        Delay.delay_us(time);
    }

    pub fn delayns(time: u32) {
        Delay.delay_ns(time);
    }
}