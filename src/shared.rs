pub const DEBUG: bool = true;
pub const TRACE: bool = true;
pub const BAUD_RATE: u32 = 57_600;
pub const MAX_DELTATIME: u32 = 10_000;
pub type UsbSerial = ::arduino_hal::Usart<
    ::arduino_hal::pac::USART0,
    ::arduino_hal::port::Pin<arduino_hal::port::mode::Input, arduino_hal::hal::port::PE0>,
    ::arduino_hal::port::Pin<arduino_hal::port::mode::Output, arduino_hal::hal::port::PE1>,
>;
