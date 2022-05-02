pub struct Buffer<'a> {
    bytes:&'a[u8],
    position: usize,
    length: usize,
}

impl<'a> Buffer<'a> {
    pub fn new(bytes: &[u8]) -> Buffer {
        Buffer {
            bytes,
            position: 0,
            length: bytes.len(),
        }
    }

    pub fn get(&self) -> Option<&'a u8> {
        return if self.is_empty() { None } else { Some(&self.bytes[self.position]) }
    }

    pub fn is_empty(&self) -> bool {
        self.position >= self.length
    }

    pub fn next(&mut self, step: usize) {
        if !self.is_empty() {
            self.position += step
        }
    }

    pub fn take_bytes(&self, length: usize) -> Vec<u8> {
        Vec::from(&self.bytes[self.position..(self.position+ length)])
    }

    pub fn position(&self) -> usize {
        self.position
    }
}