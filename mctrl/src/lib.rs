pub mod motor {
    use embedded_hal::delay::DelayNs;
    use rppal::{gpio::{self, Error, OutputPin}, hal::Delay};

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
        pub delay: Delay,
    }
    impl Mtr {
        pub fn new(xaxis: bool, dp: u8, sp: u8, enbp: u8) -> Result<Self, MtrErrors>  {
            let gp = match gpio::Gpio::new() {
                Ok(gp) => gp,
                Err(_) => return Err(MtrErrors::GpioCreationError)
            };
            let dirpin = match gpio::Gpio::get(&gp, dp) {
                Ok(dp) => dp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            let steppin = match gpio::Gpio::get(&gp, sp) {
                Ok(sp) => sp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            let enablepin = match gpio::Gpio::get(&gp, enbp) {
                Ok(sp) => sp.into_output_low(),
                Err(rr) => return Err(MtrErrors::PinGettingError(rr))
            };
            let dl = Delay::new();
            Ok(Mtr {
                xaxis: xaxis,
                dirpin: dirpin,
                steppin: steppin,
                enb_pin: enablepin,
                delay: dl
            })

        }

        pub fn enable_motor(&mut self) {
            self.enb_pin.set_high();
        }

        pub fn disable_motor(&mut self) {
            self.enb_pin.set_low();
        }

        pub fn move_steps(&mut self, steps: u32, direction: bool, speed: u32) -> Result<(), MtrErrors>{
            if self.enb_pin.is_set_low() {
                return Err(MtrErrors::MotorDisabled)
            };
            if direction {
                self.dirpin.set_high();
            } else {
                self.dirpin.set_low();
            };
            for _ in 0..steps {
                self.steppin.set_high();
                self.delay.delay_ns(speed);
                self.steppin.set_low();
                self.delay.delay_ns(speed);
            };
            Ok(())
        }
    }
}

pub mod magnet {
    use rppal::gpio::{OutputPin, Gpio};

    use crate::motor::MtrErrors;


    #[derive(Debug)]
    pub struct Magnet {
        pin: OutputPin,
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


}
