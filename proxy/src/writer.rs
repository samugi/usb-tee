// src/Writer.rs
pub struct Writer {
  // fields
}

impl Writer {
  pub fn new() -> Self {
    Writer {
          // ...
      }
  }

  pub fn write(&self, data: &mut [u8]) {
    println!("YEEEEY writing data... {:?}", data);
  }
}
