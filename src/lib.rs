extern crate libc;

#[macro_use]
extern crate bitflags;

pub mod ffi {
    use libc::{c_int, c_short};

    bitflags! {
        #[repr(C)]
        flags PollEvent: u16 {
            const NONE    = 0x0000,
            const POLLIN  = 0x0001,
            const POLLPRI = 0x0002,
            const POLLOUT = 0x0004,
            const POLLERR = 0x0008,
            const POLLHUP = 0x0010,
            const POLLNVAL= 0x0020,
        }
    }

    #[repr(C)]
    pub struct PollFD {
        pub fd : c_int,
        pub events : PollEvent,
        pub revents : PollEvent
    }

    impl PollFD {
        pub fn new(fd : c_int, events : PollEvent) -> PollFD {
            PollFD {
                fd: fd,
                events: events,
                revents: NONE
            }
        }
    }

    extern {
        pub fn poll(fds : * const PollFD, nfds : c_short, timeout : c_int) -> c_int;
    }
}

use std::os::unix::io::RawFd;
use std::sync::mpsc::{Receiver, channel};
use std::thread::{JoinHandle};

pub struct InputPoller {
    pub rcv : Receiver<()>,
    pub th : JoinHandle<()>
}

impl InputPoller {
    pub fn new(fd : RawFd) -> InputPoller {
        let (snd, rcv) = channel();
        let th = std::thread::spawn(move || {
            loop {
                let fds = [ffi::PollFD::new(fd, ffi::POLLIN)];
                let mut res : i32;
                unsafe { res = ffi::poll(fds.as_ptr(), 1, -1); }
                if res >= 0 {
                    snd.send(()).unwrap();
                } else {
                    return;
                }
            }
        });

        InputPoller {
            rcv: rcv,
            th: th
        }
    }
}

#[test]
fn ffi_poll() {
    let fds = [ffi::PollFD::new(0, ffi::POLLIN)];
    unsafe { ffi::poll(fds.as_ptr(), 1, 0); }
}

