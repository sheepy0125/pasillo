//! An interactive "debugger" or "monitor," [`HallwayMonitor`], meant for debug scenarios to look
//! through memory.

#[cfg(debug_assertions)]
use crate::{
    console::{print, println, read_line},
    debug::memory::MARKERS,
    utils::decompose_uninit_array,
};

#[cfg(debug_assertions)]
use avr_device::interrupt;

#[cfg(debug_assertions)]
const HELP_MESSAGE: &str = r#"Commands:
help/h - <--
exit/e - <--
cli - [cl]ear [i]nterrupts
ena - [ena]ble interrupts
r<0xLEN> - [r]ead memory
w<0xBYTE>(,<0bBYTE>...) - [w]rite bytes
cw<0xBYTE>,0xLEN - [c]opy [w]rite
j - [j]ump to a function pointer
a<0xLEN> - [a]dvance pointer
b<0xLEN> - [b]acktrack pointer
lm - [l]ist [m]arkers
m<name> - point to [m]arker
g<0xPOS> - [g]oto position"#;

/// Helper method to parse `input`[`offset`..`offset+len`] to a `usize` in `radix`
#[cfg(debug_assertions)]
fn helper_parse(input: &str, offset: usize, len: usize, radix: u32, default: usize) -> usize {
    let parse = input.split_at_checked(offset).unwrap_or(("", "")).1;
    let parse = parse.split_at_checked(len).unwrap_or((parse, "")).0;
    let parse = parse.trim_start_matches("0x");
    usize::from_str_radix(parse, radix).unwrap_or(default)
}

/// An interactive "debugger" to examine memory. Only available with debug assertions.
#[cfg(debug_assertions)]
#[derive(Default)]
pub struct HallwayMonitor {
    pos: Option<*const u8>,
}
#[cfg(debug_assertions)]
impl HallwayMonitor {
    pub fn new() -> Self {
        Self::default()
    }
    /// Run an interactive hallway monitor.
    ///
    /// # Safety
    /// This disables interrupts and blocks everything while waiting for user input.
    pub unsafe fn interactive(&mut self) {
        interrupt::disable_save();
        println!("Welcome to Hallway Monitor!");
        self.help();
        loop {
            self.status();
            print!("> ");
            let input_stack_str = unsafe { read_line::<96>() };
            let input = input_stack_str.as_ref().trim();
            match input {
                _help if input.starts_with('h') => self.help(),
                _exit if input.starts_with('e') => break,
                _clear_interrupts if input.starts_with("cli") => {
                    interrupt::disable_save();
                }
                _enable_interrupts if input.starts_with("ena") => unsafe {
                    interrupt::enable();
                },
                _read_memory if input.starts_with('r') => unsafe {
                    self.read_memory(helper_parse(input, 1, 8, 16, 8))
                },
                _write_memory if input.starts_with('w') => unsafe {
                    let original_pos = self.pos;
                    let mut offset = 0;
                    while offset < input.len() {
                        offset += 1;
                        let len = match input_stack_str.inner[offset + 1] {
                            b'x' => 4,
                            _ => 2,
                        };
                        self.write_memory(helper_parse(input, offset, len, 16, 0));
                        self.advance(1);
                        offset += len;
                    }
                    self.pos = original_pos;
                },
                _copy_write_memory if input.starts_with("cw") => unsafe {
                    let original_pos = self.pos;
                    let len = match input_stack_str.inner[3] {
                        b'x' => 4,
                        _ => 2,
                    };
                    let byte = helper_parse(input, 2, len, 16, 0);
                    (0..helper_parse(input, 1 + len + 2, 4, 16, 1)).for_each(|_| {
                        self.write_memory(byte);
                        self.advance(1);
                    });
                    self.pos = original_pos;
                },
                _jump if input.starts_with('j') => {
                    if let Some(ptr) = self.pos {
                        let trampoline: fn() -> ! = unsafe { core::mem::transmute(ptr) };
                        trampoline();
                    }
                }
                _forward if input.starts_with('a') => {
                    self.advance(helper_parse(input, 1, 8, 16, 1))
                }
                _back if input.starts_with('b') => self.backtrack(helper_parse(input, 1, 8, 16, 1)),
                _list_markers if input.starts_with("lm") => self.list_markers(),
                _marker if input.starts_with('m') => unsafe {
                    self.marker(helper_parse(input, 1, 8, 16, 0))
                },
                _goto if input.starts_with('g') => {
                    self.pos = Some(helper_parse(
                        input,
                        1,
                        8,
                        16,
                        self.pos.map(|ptr| ptr.addr()).unwrap_or_default(),
                    ) as *const u8)
                }
                _ => println!("invalid input `{}`\nsee `help`", input),
            }
        }
        unsafe { interrupt::enable() }
    }

    fn help(&self) {
        println!("{}", HELP_MESSAGE);
    }
    fn status(&self) {
        match self.pos {
            Some(ptr) => println!("Position: 0x{:x}", ptr.addr()),
            None => {
                println!(
                    "No position specified.\nHINT: Point to a marker first!\nHINT: See `help`."
                )
            }
        }
    }
    unsafe fn read_memory(&self, len: usize) {
        let ptr = match self.pos {
            Some(ptr) => ptr,
            None => return,
        };
        (0..len * 8).for_each(|offset| {
            let pos = unsafe { ptr.add(offset) };
            print!("0x{:02x} ", unsafe { *pos });
            if offset != 0 && (offset + 1) % 8 == 0 {
                println!("<-- @0x{:x}", pos.addr() + 1);
            }
        });
    }
    unsafe fn write_memory(&self, byte: usize) {
        let ptr = match self.pos {
            Some(ptr) => ptr,
            None => return,
        };
        *ptr.cast_mut() = byte.try_into().unwrap_or(u8::MAX);
    }
    /// Advance the pointer by `by` bits.
    fn advance(&mut self, by: usize) {
        self.pos = unsafe { self.pos.map(|ptr| ptr.add(by)) };
    }
    /// Backtrack the pointer by `by` bits.
    fn backtrack(&mut self, by: usize) {
        self.pos = unsafe { self.pos.map(|ptr| ptr.sub(by)) };
    }
    fn list_markers(&self) {
        unsafe { &decompose_uninit_array(MARKERS.inner)[..MARKERS.len] }
            .iter()
            .enumerate()
            .for_each(|(idx, (name, _))| println!("{} --> {}", idx, name))
    }
    unsafe fn marker(&mut self, idx: usize) {
        if idx >= unsafe { MARKERS.len } {
            println!("Marker not found\nHINT: Use `lm` to list markers");
            return;
        }
        let (name, pos) = unsafe { MARKERS.inner[idx] }.assume_init();
        println!("Pointing to marker `{}`!", name);
        self.pos = Some(pos);
    }
}

/// A dummy hallway monitor
#[cfg(not(debug_assertions))]
pub struct HallwayMonitor;
#[cfg(not(debug_assertions))]
impl HallwayMonitor {
    pub fn new() -> Self {
        Self {}
    }
    pub unsafe fn interactive(&mut self) {}
}
