#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Register {
    value: u8,
}

impl Register {
    pub fn new(value: u8) -> Result<Register, String> {
        if value <= 15 {
            Ok(Register { value })
        } else {
            Err(format!("Register value {} out of range", value))
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShiftAmount {
    value: u8,
}

impl ShiftAmount {
    pub fn new(value: u8) -> Result<ShiftAmount, String> {
        if value <= 31 {
            Ok(ShiftAmount { value })
        } else {
            Err(format!("Shift amount value {} out of range", value))
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FunctionCode {
    value: u8,
}

impl FunctionCode {
    pub fn new(value: u8) -> Result<FunctionCode, String> {
        if value <= 127 {
            Ok(FunctionCode { value })
        } else {
            Err(format!("Function code value {} out of range", value))
        }
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }
}
