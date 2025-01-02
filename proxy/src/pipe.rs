pub struct Pipe <'a> {
  reader: & 'a mut crate::Reader,
  writer: & 'a mut crate::Writer,
}

impl<'a> Pipe<'a> {
  pub fn new(reader: &'a mut crate::Reader, writer: &'a mut crate::Writer) -> Self {
    Pipe {
      reader,
      writer,
    }
  }

  pub fn write(&mut self, data: &mut [u8])  {
    self.writer.write(data);
  }

  pub fn read(&mut self) -> Result<(), rusb::Error> {
    self.reader.read(|data| {
      self.writer.write(data);
    })
  }
}
