pub struct Neighbours(u8);

impl Neighbours {
    pub fn from_bytes(bytes: u8) -> Self {
        Self(bytes)
    }

    pub(crate) fn get_raw(&self) -> u8 {
        self.0
    }
}
