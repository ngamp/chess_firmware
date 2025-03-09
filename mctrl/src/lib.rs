const MMR: f32 = 14.135;
const MMF: f32 = 45.0;

pub const HOMINGSPEED: f32 = 5.0;
pub const NMOVESPEED: f32 = 2.0;

pub mod motor {

    use core::f32;
    use std::ops::Sub;

    use rppal::gpio::{Error, OutputPin, Gpio};

    use crate::{delay, HOMINGSPEED, MMF, MMR};

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
        pub steps_from_home: i32
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
                steps_from_home: 0
            })

        }

        pub fn enable_motor(&mut self) {
            self.enb_pin.set_high();
        }

        pub fn disable_motor(&mut self) {
            self.enb_pin.set_low();
        }

        pub fn move_steps(&mut self, steps: u32, direction: bool, speed: f32) -> Result<(), MtrErrors>{
            if self.enb_pin.is_set_low() {
                return Err(MtrErrors::MotorDisabled)
            };
            if direction {
                self.steps_from_home += steps as i32;
                self.dirpin.set_high();
            } else {
                self.steps_from_home -= steps as i32;
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

    pub fn diagonal(x: &mut Mtr, y: &mut Mtr, right: bool, up: bool, steps: u32, speed: f32) -> Result<(), MtrErrors>{
        if x.enb_pin.is_set_low() || y.enb_pin.is_set_low() {
            return Err(MtrErrors::MotorDisabled)
        };
        if right {
            x.steps_from_home += steps as i32;
            x.dirpin.set_high();
        } else {
            x.steps_from_home -= steps as i32;
            x.dirpin.set_low();
        };
        if up {
            y.steps_from_home += steps as i32;
            y.dirpin.set_high();
        } else {
            y.steps_from_home -= steps as i32;
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
    pub enum MotorMoveType {
        StraightX(MotorMove),
        StraightY(MotorMove),
        Diagonal(MotorMove),
        Adjust(MotorMove)
    }

    #[derive(Debug)]
    pub struct MotorMove {
        pub dir: bool,
        pub len: u32,
        pub speed: f32,
        pub dir2: bool
    }

    impl MotorMove {
        pub fn new() -> Self {
            MotorMove {dir: true, len: 0, dir2: true, speed: 3.0}
        }

        pub fn new_values(dir: bool, len: u32, dir2: bool, speed: f32) -> Self {
            MotorMove { dir, len, dir2, speed}
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

        pub fn from_vfield(field: Field, speed: f32) -> Self {
            let mut res = Vec::new();
            let xlen = (((MMF*field.0)/MMR)*200.0) as i32;
            let ylen = (((MMF*field.1)/MMR)*200.0) as i32;
            if xlen != 0 {
                res.push(MotorMoveType::StraightX(steps_to_motormove(xlen, speed)));
            };
            if ylen!= 0 {
                res.push(MotorMoveType::StraightY(steps_to_motormove(ylen, speed)));
            }
            Self { instructions:  res}
        }

        pub fn to_home(m1: &Mtr, m2: &Mtr) -> Self {
            let x = -m1.steps_from_home;
            let y = -m2.steps_from_home;
            Self { instructions: vec![MotorMoveType::StraightX(steps_to_motormove(x, HOMINGSPEED)), MotorMoveType::StraightY(steps_to_motormove(y, HOMINGSPEED))] }
        }

        pub fn field_to_field(f1: Field, f2: Field, speed: f32) -> Self {
            let f = f2-f1;
            Self::from_vfield(f, speed)
        }

        pub fn home_to_field(f: Field) -> Self {
            Self::from_vfield(f, HOMINGSPEED)
        }

        pub fn diagonal(f1: Field, f2: Field, speed: f32) -> Self {
            let mut res = Vec::new();
            let vf = f2 - f1;
            let mut len = (((MMF*(vf.0.abs()))/MMR)*200.0) as u32;
            if vf.0.abs() < vf.1.abs() {
                let ylen = (((MMF*(vf.1.abs() - vf.0.abs()))/MMR)*200.0) as u32;
                res.push(MotorMoveType::StraightY(MotorMove::new_values(vf.1.is_sign_positive(), ylen, true, speed)));
            };
            if vf.0.abs() > vf.1.abs() {
                len = (((MMF*(vf.1.abs()))/MMR)*200.0) as u32;
                let xlen = (((MMF*(vf.0.abs() - vf.1.abs()))/MMR)*200.0) as u32;
                res.push(MotorMoveType::StraightX(MotorMove::new_values(vf.0.is_sign_positive(), xlen, true, speed)));
            };
            res.push(MotorMoveType::Diagonal(MotorMove::new_values(vf.0.is_sign_positive(), len, vf.1.is_sign_positive(), speed)));
            return Self { instructions: res }
        }
    }

    pub struct Field(f32, f32);

    impl Sub for Field {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Self(self.0 - rhs.0, self.1 - rhs.1)
        }
    }

    pub trait ToF32 {
        fn to_f32(self) -> f32;
    }

    impl ToF32 for f32 {
        fn to_f32(self) -> f32 {
            self as f32
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
        pub fn from_tuple<T: ToF32>(t: (T, T)) -> Self {
            Field(t.0.to_f32(), t.1.to_f32())
        }

        pub fn ind_to_relative_ind<T: ToF32>(ind: (T, T)) -> Self {
            let x: f32 = -6.5 + ind.0.to_f32();
            let y: f32 = -3.5 + ind.1.to_f32();
            Field(x, y)
        }
    }

    pub fn steps_to_motormove(ind: i32, speed: f32) -> MotorMove {
        let xmovement = ind;
        let xmovement = if xmovement.is_negative() {
            MotorMove::new_values(false, xmovement.abs() as u32, true, speed)
        } else {
            MotorMove::new_values(true, xmovement as u32, true, speed)
        };
        xmovement
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