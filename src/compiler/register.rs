#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Register {
    value: u8,
}

impl Register {
    pub fn new(value: u8) -> Result<Register, String> {
        if value < 32 {
            Ok(Register { value })
        } else {
            Err(format!("Register {} out of range (0-31)", value))
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}
