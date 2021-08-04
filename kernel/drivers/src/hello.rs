use kernel::process::{Error, ProcessId};
use kernel::syscall::{CommandReturn, SyscallDriver};
use kernel::{debug, ErrorCode};
use core::cell::Cell;

pub const DRIVER_NUM: usize = 0xa0000;

pub struct Hello {
    n: Cell<u32>
}

impl Hello {
    pub fn new() -> Hello {
        Hello {
            n: Cell::new (0)
        }
    }
}

impl SyscallDriver for Hello {
    fn command(
        &self,
        command_num: usize,
        r2: usize,
        _r3: usize,
        _process_id: ProcessId,
    ) -> CommandReturn {
        match command_num {
            0 => CommandReturn::success(),
            // print
            1 => {
                debug!("Hello, n is {}", self.n.get());
                CommandReturn::success()
            }
            // up
            2 => {
                self.n.set (self.n.get () + 1);
                CommandReturn::success()
            }
            // down
            3 => {
                if self.n.get () > 0 {
                    self.n.set (self.n.get() - 1);
                    CommandReturn::success()
                }
                else
                {
                    CommandReturn::failure (ErrorCode::INVAL)
                }
            }
            // set
            4 => {
                self.n.set (r2 as u32);
                CommandReturn::success()
            }
            // get
            5 => {
                CommandReturn::success_u32(self.n.get())
            }
            _ => CommandReturn::failure(ErrorCode::NOSUPPORT),
        }
    }

    fn allocate_grant(&self, _process_id: ProcessId) -> Result<(), Error> {
        Ok(())
    }
}
