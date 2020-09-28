use shim::io;
use shim::path::{Path, PathBuf};

use stack_vec::StackVec;

use pi::atags::Atags;

use fat32::traits::FileSystem;
use fat32::traits::{Dir, Entry};

use crate::console::{kprint, kprintln, CONSOLE};
use core::str::from_utf8;
use crate::ALLOCATOR;
use crate::FILESYSTEM;

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns.
pub fn shell(prefix: &str) -> ! {
    const CMD_LEN: usize = 512;
    const ARG_LEN: usize = 64;
    kprintln!("Welcome to RustOS");
    kprintln!();
    loop {
        let mut cmd_buf = [0u8; CMD_LEN];
        let mut arg_buf = [""; ARG_LEN];

        kprintln!();
        kprint!("{}", prefix);

        let mut i = 0;
        'cmd: loop {
            if i == CMD_LEN {
                kprintln!();
                kprintln!("command length exceeds {}", CMD_LEN);
                break 'cmd;
            }

            let byte = CONSOLE.lock().read_byte();
            if byte == b'\n' || byte == b'\r' {
                kprintln!();
                let cmd = from_utf8(&cmd_buf[0..i]).expect("can't convert byte array to string");
                match Command::parse(cmd, &mut arg_buf) { // enter
                    Err(Error::Empty) => {},
                    Err(Error::TooManyArgs) => {
                        kprintln!("error: too many arguments");
                    },
                    Ok(cmd) => {
                        process_command(cmd);
                    }
                }
                break 'cmd;
            } else if byte == 8 || byte == 127 { // backspace
                if i == 0 {
                    continue 'cmd;
                }
                kprint!("\u{8} \u{8}");
                i -= 1
            } else {
                cmd_buf[i] = byte;
                CONSOLE.lock().write_byte(byte);
                i += 1;
            }
        }
    }
}

fn process_command<'a>(cmd: Command<'a>) {
    match cmd.path() {
        "echo" => {
            if cmd.args.len() > 1 {
                kprint!("{}", cmd.args[1]);
                for arg in cmd.args[2..].iter() {
                    kprint!(" {}", arg);
                }
            }
            kprintln!();
        },
        unknown => {
            kprintln!("unknown command: {}", unknown);
        }
    }
}