use stack_vec::StackVec;
use console::{kprint, kprintln, CONSOLE};

/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>
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

fn echo(args: &[&str]) {
    use std::fmt::Write;

    let mut console = CONSOLE.lock();

    console.write_str("\n");

    for arg in args[1..].iter() {
        console.write_str(arg);
        console.write_str(" ");
    }
}

/*
 * ASCII Constants
 */
const BACKSPACE: u8 = 8;
const BELL: u8 = 7;
const DELETE:u8 = 127;

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns: it is perpetually in a shell loop.
pub fn shell(prefix: &str) -> ! {
    use std::str;
    use std::io::Write;

    loop {
        let mut buf_storage = [0u8; 512];
        let mut buf = StackVec::new(&mut buf_storage);

        kprint!("{} ", prefix);

        loop {
            let byte = CONSOLE.lock().read_byte();

            if byte == b'\r' || byte == b'\n' {
                let cmd: &str = match str::from_utf8(&buf.as_slice()) {
                    Ok(s) => s,
                    Err(_) => {
                        kprintln!("error: failed to parse command\r");
                        break
                    }
                };

                let mut str_buf = [""; 64];

                let command = match Command::parse(cmd, &mut str_buf) {
                    Ok(command) => command,
                    Err(Error::Empty) => break,
                    Err(Error::TooManyArgs) => {
                        kprintln!("\nerror: too many arguments\r");
                        break
                    }
                };

                match command.path() {
                    "echo" => {
                        echo(command.args.as_slice());
                        break
                    }
                    _ => {
                        kprintln!("\nunknown command: {}\r", command.path());
                        break
                    }
                }

            } else if byte == DELETE || byte == BACKSPACE {
                let mut console = CONSOLE.lock();

                if !(buf.is_empty()) {
                    console.write(&[BACKSPACE, b' ', BACKSPACE]);
                    buf.pop();
                }

            } else if byte >= 32 && byte <= 126 {
                let mut console = CONSOLE.lock();

                console.write_byte(byte);
                buf.push(byte);
            } else {
                let mut console = CONSOLE.lock();

                console.write_byte(BELL);
            }
        }

        kprintln!("\r")
    }
}
