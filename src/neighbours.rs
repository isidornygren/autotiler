#[derive(Debug)]
pub struct Neighbours(u8);

impl Neighbours {
    /// Returns a neighbours struct from a byte representing the comparison state of neighbours.
    ///
    /// Input is a byte which represents [nw, n, ne, w, e, sw, s, se]
    pub fn from_byte(bytes: u8) -> Self {
        Self(bytes)
    }

    pub fn from_values(
        north_west: bool,
        north: bool,
        north_east: bool,
        west: bool,
        east: bool,
        south_west: bool,
        south: bool,
        south_east: bool,
    ) -> Self {
        let mut neighbours_byte = 0b00000000;
        if north_west {
            neighbours_byte |= 0b10000000;
        }
        if north {
            neighbours_byte |= 0b01000000;
        }
        if north_east {
            neighbours_byte |= 0b00100000;
        }
        if west {
            neighbours_byte |= 0b00010000;
        }
        if east {
            neighbours_byte |= 0b00001000;
        }
        if south_west {
            neighbours_byte |= 0b00000100;
        }
        if south {
            neighbours_byte |= 0b00000010;
        }
        if south_east {
            neighbours_byte |= 0b00000001;
        }
        Self::from_byte(neighbours_byte)
    }

    pub fn from_array(neighbours: [bool; 8]) -> Self {
        Self::from_values(
            neighbours[0],
            neighbours[1],
            neighbours[2],
            neighbours[3],
            neighbours[4],
            neighbours[5],
            neighbours[6],
            neighbours[7],
        )
    }

    pub(crate) fn north_west(&self) -> bool {
        self.get_raw() & 0b11010000 == 0b11010000
    }

    pub(crate) fn north(&self) -> bool {
        self.get_raw() & 0b01000000 == 0b01000000
    }

    pub(crate) fn north_east(&self) -> bool {
        self.get_raw() & 0b01101000 == 0b01101000
    }

    pub(crate) fn west(&self) -> bool {
        self.get_raw() & 0b00010000 == 0b00010000
    }

    pub(crate) fn east(&self) -> bool {
        self.get_raw() & 0b00001000 == 0b00001000
    }

    pub(crate) fn south_west(&self) -> bool {
        self.get_raw() & 0b00010110 == 0b00010110
    }

    pub(crate) fn south(&self) -> bool {
        self.get_raw() & 0b00000010 == 0b00000010
    }

    pub(crate) fn south_east(&self) -> bool {
        self.get_raw() & 0b00001011 == 0b00001011
    }

    pub(crate) fn get_raw(&self) -> u8 {
        self.0
    }
}
