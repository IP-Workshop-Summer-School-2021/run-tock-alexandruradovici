use kernel::hil::led::Led;
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};

pub const DRIVER_NUM: usize = 0xa0002;

const DIGITS: [u32; 10] = [
    // 0
    0b11111_10011_10101_11001_11111,
    // 1
    0b00100_01100_00100_00100_01110,
    // 2
    0b11110_00001_01110_10000_11111,
    // 3
    0b11110_00001_11110_00001_11110,
    // 4
    0b10000_10000_10100_11111_00100,
    // 5
    0b11111_10000_11110_00001_11110,
    // 6
    0b11111_10000_11111_10001_11111,
    // 7
    0b11111_00001_00010_00100_00100,
    // 8
    0b11111_10001_11111_10001_11111,
    // 9
    0b11111_10001_11111_00001_11111,
];

pub struct DotsTextDisplay<'a, L: Led, A: Alarm<'a>> {
    leds: &'a [&'a L; 25],
    alarm: &'a A
}

impl<'a, L: Led, A: Alarm<'a>> DotsTextDisplay<'a, L, A> {
    pub fn new(leds: &'a [&'a L; 25], alarm: &'a A) -> DotsTextDisplay<'a, L, A> {
        // if leds.len() != 25 {
        //     panic! ("DotsTextDisplay needs a slice of 25 LEDs, you supplied {}", leds.len());
        // }

        DotsTextDisplay { leds, alarm }
    }

    pub fn set_timeout (&self) {
        self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(1000));
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

impl<'a, L: Led, A: Alarm<'a>> SyscallDriver for DotsTextDisplay<'a, L, A> {
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

impl<'a, L: Led, A: Alarm<'a>> AlarmClient for DotsTextDisplay<'a, L, A> {
    fn alarm(&self) {
        debug! ("fired");
        self.set_timeout();
    }
}