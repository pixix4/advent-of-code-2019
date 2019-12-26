#[derive(Debug)]
pub struct OpCode {
    code: i128,
}

impl OpCode {
    pub fn new(code: i128) -> Self {
        OpCode { code }
    }

    pub fn op(&self) -> i128 {
        self.code % 100
    }

    pub fn mode(&self, pos: i128) -> i128 {
        let position = (10 as i128).pow(pos as u32 + 1);
        (self.code / position % 10)
    }
}
