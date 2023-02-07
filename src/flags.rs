
pub struct Flags {
    pub z: bool,                // zero                 : bit 0, register 0
    pub m: bool,                // minus                : bit 1, register 0
    pub v: bool,                // parity / overflow    : bit 2, register 0
    pub c: bool,                // carry                : bit 3, register 0
    pub l:bool,                 // less                 : bit 4, register 0
    pub e:bool,                 // equal                : bit 5, register 0
    pub g: bool,                // greater              : bit 6, register 0
    pub y: bool,                // auxiliary carry      : bit 7, register 0
    pub x: bool,                // unused               : bit 8, register 0
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
            z: false,
            m: false,
            v: false,
            c: false,
            l: false,
            e: false,
            g: false,
            y: false,
            x: false,
        }
    }
}
