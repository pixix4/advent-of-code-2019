#[derive(Debug)]
pub struct OpCode {
    code: i32,
}

impl OpCode {
    pub fn new(code: i32) -> Self {
        OpCode { code }
    }

    pub fn op(&self) -> i32 {
        self.code % 100
    }

    pub fn mode(&self, pos: i32) -> i32 {
        let position = (10 as i32).pow(pos as u32 + 1);
        (self.code / position % 10)
    }
}

#[derive(Debug)]
pub enum AllocationMode {
    Forbidden,
    AtWrite,
    DefaultTo(i32),
}
