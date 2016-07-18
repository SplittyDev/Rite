use spin::Mutex;
use cpuio::{inb, outb};

macro_rules! serial_data { ($port:expr) => ($port + 0) }
macro_rules! serial_ier { ($port:expr) => ($port + 1) }
macro_rules! serial_fifo { ($port:expr) => ($port + 2) }
macro_rules! serial_line { ($port:expr) => ($port + 3) }
macro_rules! serial_modem { ($port:expr) => ($port + 4) }
macro_rules! serial_line_status { ($port:expr) => ($port + 5) }

/// A serial writer to COM1.
pub static COM1: Mutex<SerialWriter> = Mutex::new(SerialWriter {
    port: 0x3F8,
    irq: 4,
});

/// A serial writer to COM2.
pub static COM2: Mutex<SerialWriter> = Mutex::new(SerialWriter {
    port: 0x2F8,
    irq: 3,
});

/// A serial writer to COM3.
pub static COM3: Mutex<SerialWriter> = Mutex::new(SerialWriter {
    port: 0x3E8,
    irq: 4,
});

/// A serial writer to COM4.
pub static COM4: Mutex<SerialWriter> = Mutex::new(SerialWriter {
    port: 0x2E8,
    irq: 3,
});

/// The `SerialWriter` type.
pub struct SerialWriter {
    /// The port.
    port: u16,

    /// The IRQ.
    irq: u8,
}

/// The `::core::fmt::Write` implementation for `SerialWriter`.
impl ::core::fmt::Write for SerialWriter {
    /// Writes a string.
    #[inline(always)]
    fn write_str(&mut self, string: &str) -> ::core::fmt::Result {
        for byte in string.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

/// The `SerialWriter` implementation.
impl SerialWriter {
    /// Initializes the serial writer.
    #[inline(always)]
    pub fn init(&self) {
        unsafe {
            outb(0x00, serial_data!(self.port));
            outb(0x80, serial_line!(self.port));
            outb(0x03, serial_data!(self.port));
            outb(0x00, serial_ier!(self.port));
            outb(0x03, serial_line!(self.port));
            outb(0xc7, serial_fifo!(self.port));
            outb(0x0b, serial_modem!(self.port));
        }
    }

    /// Sets the baud rate.
    #[inline(always)]
    pub fn set_baud_rate(&self, divisor: u32) {
        unsafe {
            outb(0x80, serial_line!(self.port));
            outb(((divisor >> 8) & 0x00FF) as u8, serial_data!(self.port));
            outb((divisor & 0x00FF) as u8, serial_data!(self.port));
        }
    }

    /// Sets the line.
    #[inline(always)]
    pub fn set_line(&self) {
        unsafe {
            outb(0x03, serial_line!(self.port));
        }
    }

    /// Enables the buffer.
    #[inline(always)]
    pub fn enable_buffer(&self) {
        unsafe {
            outb(0xc7, serial_fifo!(self.port));
        }
    }

    /// Tests if the buffer is empty.
    #[inline(always)]
    pub fn buffer_is_empty(&self) -> bool {
        unsafe { (inb(serial_line_status!(self.port)) & 0x20) > 0 }
    }

    /// Tests if the buffer contains data.
    #[inline(always)]
    pub fn buffer_contains_data(&self) -> bool {
        unsafe { (inb(serial_line_status!(self.port)) & 1) > 0 }
    }

    /// Writes a byte.
    #[inline(always)]
    pub fn write_byte(&self, byte: u8) {
        while !self.buffer_is_empty() {}
        unsafe {
            outb(byte, self.port);
        }
    }

    /// Writes a string.
    #[inline(always)]
    pub fn write_str(&self, string: &str) {
        for byte in string.bytes() {
            self.write_byte(byte);
        }
    }

    /// Reads a byte.
    #[inline(always)]
    pub fn read_byte(&self) -> u8 {
        while !self.buffer_contains_data() {}
        unsafe { inb(self.port) }
    }

    /// Reads a char.
    #[inline(always)]
    pub fn read_char(&self) -> char {
        self.read_byte() as char
    }
}
