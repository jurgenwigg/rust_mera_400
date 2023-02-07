use crate::registers::Registers;

pub struct CPU {
    pub reg: Registers,
    // pub bus: AddressBus,
    // halt: bool,
    // pub debug: Debug,
    // int: Option<u8>,
    // nmi: bool,
    // im: u8,
    // iff1: bool,
    // iff2: bool,
    // slice_duration: u32,
    // // Defaults to 35000 cycles per 16ms slice (2.1 Mhz).
    // // cycles = clock speed in Hz / required frames-per-second
    // slice_max_cycles: u32,
    // slice_current_cycles: u32,
    // slice_start_time: SystemTime,
}

impl CPU {
    /// Creates a new CPU instance. 'Size' will be its top address.
    pub fn new(size: u16) -> CPU {
        CPU {
            reg: Registers::new(),
            // alt: Registers::new(),
            // bus: AddressBus::new(size),
            // halt: false,
            // debug: Debug::new(),
            // int: None,
            // nmi: false,
            // im: 0,
            // iff1: false,
            // iff2: false,
            // slice_duration: 16,
            // slice_max_cycles: 35000,
            // slice_current_cycles: 0,
            // slice_start_time: SystemTime::now(),
        }
    }
}
