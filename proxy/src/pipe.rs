use std::sync::Arc;

pub struct Pipe {
  reader: crate::Reader,
  writer: crate::Writer,
}

impl Pipe {
  pub fn new(reader: crate::Reader, writer: crate::Writer) -> Self {
    Pipe {
      reader,
      writer,
    }
  }

  pub fn read(&'static self) -> Result<(), rusb::Error> {
    self.reader.read(self)
  }

  pub fn write(&self, data: &mut [u8])  {
    self.writer.write(data);
  }
}