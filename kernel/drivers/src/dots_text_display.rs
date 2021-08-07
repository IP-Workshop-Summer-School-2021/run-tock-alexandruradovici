use core::cell::Cell;
use kernel::hil::led::Led;
use kernel::hil::text_screen::{TextScreen, TextScreenClient};
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::utilities::cells::{OptionalCell, TakeCell};
use kernel::ErrorCode;

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
    alarm: &'a A,
    buffer: TakeCell<'static, [u8]>,
    position: Cell<usize>,
    print_len: Cell<usize>,

    client: OptionalCell<&'a dyn TextScreenClient>, // Option<Cell<...>>

    supplied_buffer: TakeCell<'static, [u8]>,
    command_in_progress: Cell<bool>,

    ms: Cell<u32>,
}

impl<'a, L: Led, A: Alarm<'a>> DotsTextDisplay<'a, L, A> {
    pub fn new(
        leds: &'a [&'a L; 25],
        alarm: &'a A,
        buffer: &'static mut [u8],
    ) -> DotsTextDisplay<'a, L, A> {
        // if leds.len() != 25 {
        //     panic! ("DotsTextDisplay needs a slice of 25 LEDs, you supplied {}", leds.len());
        // }

        DotsTextDisplay {
            leds,
            alarm,
            buffer: TakeCell::new(buffer),
            position: Cell::new(0),
            print_len: Cell::new(0),
            client: OptionalCell::empty(),
            supplied_buffer: TakeCell::empty(),
            command_in_progress: Cell::new(false),
            ms: Cell::new(500),
        }
    }

    pub fn init(&self, ms: u32) {
        self.ms.set(ms);
        self.display_next_digit();
    }

    fn display_next_digit(&self) {
        self.buffer.map(|buffer| {
            if self.position.get() < buffer.len() && self.position.get() < self.print_len.get() {
                self.display(buffer[self.position.get()] as char);
                self.position.set(self.position.get() + 1);
            } else {
                self.position.set(0);
            }
        });
        self.alarm
            .set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(self.ms.get()));
    }

    fn display(&self, digit: char) {
        let digit_index = digit as usize - '0' as usize;
        let current_digit = DIGITS[digit_index];
        for index in 0..25 {
            let bit = (current_digit >> (24 - index)) & 0x1;
            if bit == 1 {
                self.leds[index].on();
            } else {
                self.leds[index].off();
            }
        }
    }

    fn clear(&self) {
        for index in 0..25 {
            self.leds[index].off();
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
            // set speed
            1 => {
                if r2 < 10_000 {
                    self.ms.set(r2 as u32);
                    CommandReturn::success()
                } else {
                    CommandReturn::failure(ErrorCode::INVAL)
                }
            }
            // get speed
            2 => CommandReturn::success_u32(self.ms.get()),
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a, L: Led, A: Alarm<'a>> AlarmClient for DotsTextDisplay<'a, L, A> {
    fn alarm(&self) {
        self.supplied_buffer.take().map(|buffer| {
            self.client
                .map(move |client| client.write_complete(buffer, self.print_len.get(), Ok(())));
        });
        if self.command_in_progress.get() {
            self.client.map(|client| client.command_complete(Ok(())));
            self.command_in_progress.set(false);
        }
        self.display_next_digit();
    }
}

impl<'a, L: Led, A: Alarm<'a>> TextScreen<'a> for DotsTextDisplay<'a, L, A> {
    fn set_client(&self, client: Option<&'a dyn TextScreenClient>) {
        if let Some(client) = client {
            self.client.replace(client);
        } else {
            self.client.clear();
        }
    }

    fn get_size(&self) -> (usize, usize) {
        self.buffer.map_or((0, 0), |buffer| (buffer.len(), 1))
    }

    fn print(
        &self,
        supplied_buffer: &'static mut [u8],
        len: usize,
    ) -> Result<(), (ErrorCode, &'static mut [u8])> {
        if self.supplied_buffer.is_none() {
            if self.buffer.is_some() {
                self.buffer.map(|own_buffer| {
                    // => context &'b mut supplied_buffer
                    let print_len = if len < own_buffer.len() {
                        len
                    } else {
                        own_buffer.len()
                    };
                    for i in 0..print_len {
                        own_buffer[i] = supplied_buffer[i]
                    }
                    self.print_len.set(print_len);
                });
                // this is wrong
                // self.client.map(move |client| client.write_complete(supplied_buffer, len, Ok(())));
                self.supplied_buffer.replace(supplied_buffer);
                Ok(())
            } else {
                Err((ErrorCode::NOMEM, supplied_buffer))
            }
        } else {
            Err((ErrorCode::BUSY, supplied_buffer))
        }
    }

    fn set_cursor(&self, _x_position: usize, _y_position: usize) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn hide_cursor(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn show_cursor(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn blink_cursor_on(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn blink_cursor_off(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn display_on(&self) -> Result<(), ErrorCode> {
        if !self.command_in_progress.get() {
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }

    fn display_off(&self) -> Result<(), ErrorCode> {
        Err(ErrorCode::NOSUPPORT)
    }

    fn clear(&self) -> Result<(), ErrorCode> {
        if !self.command_in_progress.get() {
            self.print_len.set(0);
            self.clear();
            Ok(())
        } else {
            Err(ErrorCode::BUSY)
        }
    }
}
