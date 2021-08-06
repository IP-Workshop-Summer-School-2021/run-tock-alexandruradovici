use kernel::grant::Grant;
use kernel::hil::led::Led;
use kernel::hil::time::{Alarm, AlarmClient, ConvertTicks};
use kernel::process::{Error, ProcessId};
use kernel::processbuffer::{ReadOnlyProcessBuffer, ReadableProcessBuffer};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};
use core::cell::Cell;
use kernel::errorcode::into_statuscode;

pub const DRIVER_NUM: usize = 0xa0002;

#[derive(Default)]
pub struct AppData {
    buffer: ReadOnlyProcessBuffer,
}

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

#[derive(Clone,Copy,PartialEq)]
enum State {
    Idle,
    Printing{
        process_id: ProcessId,
        ms: usize,
        position: usize,
        len: usize
    },
}

pub struct DotsTextDisplay<'a, L: Led, A: Alarm<'a>> {
    leds: &'a [&'a L; 25],
    alarm: &'a A,
    app_data: Grant<AppData, 1>,
    state: Cell<State>,
}

impl<'a, L: Led, A: Alarm<'a>> DotsTextDisplay<'a, L, A> {
    pub fn new(
        leds: &'a [&'a L; 25],
        alarm: &'a A,
        app_data: Grant<AppData, 1>,
    ) -> DotsTextDisplay<'a, L, A> {
        // if leds.len() != 25 {
        //     panic! ("DotsTextDisplay needs a slice of 25 LEDs, you supplied {}", leds.len());
        // }

        DotsTextDisplay {
            leds,
            alarm,
            app_data,
            state: Cell::new(State::Idle),
        }
    }

    fn display_next_digit(&self) -> Result<(), ErrorCode> {
        match self.state.get () {
            State::Printing{
                process_id,
                ms,
                position,
                len
            } => {
                let res = self.app_data.enter(process_id, |data, upcalls| {
                    let res = data.buffer.enter (|buffer| {
                        if position < buffer.len() && position < len {
                            self.display(buffer[position].get() as char);
                            
                            self.state.set (State::Printing{
                                process_id,
                                ms,
                                position: position+1,
                                len
                            });
                            true
                        }
                        else 
                        {
                            upcalls.schedule_upcall (0, (0,0,0)).ok();
                            self.state.set(State::Idle);
                            false
                        }
                    });
                    if let Err(error) = res {
                        upcalls.schedule_upcall (0, (into_statuscode (error.into()),0,0)).ok();
                    };
                    res
                });
                match res {
                    Ok(Ok(next_print)) => {
                        if next_print {
                            self.alarm.set_alarm(self.alarm.now(), self.alarm.ticks_from_ms(ms as u32));
                        }
                        Ok(())
                    },
                    Ok(Err(error)) => {
                        self.state.set(State::Idle);
                        Err(error.into())
                    }
                    Err(error) => 
                    {
                        self.state.set(State::Idle);
                        Err(error.into())
                    }
                }
            }
            State::Idle => {
                Err(ErrorCode::FAIL)
                // unreachable! ()
            }
        }
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
}

impl<'a, L: Led, A: Alarm<'a>> SyscallDriver for DotsTextDisplay<'a, L, A> {
    fn command(
        &self,
        command_num: usize,
        r2: usize,
        r3: usize,
        process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            // print digit
            1 => {
                if self.state.get() == State::Idle {
                    self.state.set(State::Printing{
                        process_id,
                        ms: r3,
                        position: 0,
                        len: r2
                    });
                    if let Err(error) = self.display_next_digit() {
                        CommandReturn::failure(error)
                    }
                    else
                    {
                        CommandReturn::success()
                    }
                    /*
                    match self.display_next_digit() {
                        Err(error) => ...
                        _ => ...
                    }
                    */
                } else {
                    CommandReturn::failure(ErrorCode::BUSY)
                }
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, process_id: ProcessId) -> Result<(), Error> {
        self.app_data.enter(process_id, |_, _| {})
    }

    fn allow_readonly(
        &self,
        process_id: ProcessId,
        allow_num: usize,
        mut buffer: ReadOnlyProcessBuffer,
    ) -> Result<ReadOnlyProcessBuffer, (ReadOnlyProcessBuffer, ErrorCode)> {
        match allow_num {
            0 => {
                let res = self.app_data.enter(process_id, |data, _| {
                    core::mem::swap(&mut data.buffer, &mut buffer);
                });
                match res {
                    Ok(()) => Ok(buffer),
                    Err(error) => Err((buffer, error.into())),
                }
            }
            _ => Err((buffer, ErrorCode::NOSUPPORT)),
        }
    }
}

impl<'a, L: Led, A: Alarm<'a>> AlarmClient for DotsTextDisplay<'a, L, A> {
    fn alarm(&self) {
        debug!("fired");
        let _ = self.display_next_digit();
    }
}
