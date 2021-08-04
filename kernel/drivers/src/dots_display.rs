use kernel::hil::led::Led;
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::ErrorCode;

pub const DRIVER_NUM: usize = 0xa0001;

const DIGITS:[u32; 10] = [
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
    0b11111_10011_10101_11001_11111,
];

pub struct DotsDisplay<'a, L: Led> {
    leds: &'a [&'a L; 25],
}

impl<'a, L: Led> DotsDisplay<'a, L> {
    pub fn new(leds: &'a [&'a L; 25]) -> DotsDisplay<'a, L> {
        // if leds.len() != 25 {
        //     panic! ("DotsDisplay needs a slice of 25 LEDs, you supplied {}", leds.len());
        // }
        DotsDisplay { leds }
    }

    fn display (&self, digit: char) {
        let digit_index = digit as usize - '0' as usize;
        let current_digit = DIGITS[digit_index];
        for index in 0..25 {
            let bit = (current_digit >> (24-index)) & 0x1;
            if bit == 1 {
                self.leds[index].on();
            }
            else
            {
                self.leds[index].off();
            }
        }
    }
}

impl<'a, L: Led> SyscallDriver for DotsDisplay<'a, L> {
    fn command(
        &self,
        command_num: usize,
        r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            // print digit
            1 => match char::from_u32(r2 as u32) {
                Some(digit) => {
                    if digit >= '0' && digit <= '9' {
                        self.display (digit);
                        CommandReturn::success()
                    } else {
                        CommandReturn::failure(ErrorCode::INVAL)
                    }
                }
                None => CommandReturn::failure(ErrorCode::INVAL),
            },
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}
