use std::sync::Arc;

use proxy::Reader;
use proxy::Writer;
use proxy::Pipe;

fn main() {
  let mut reader = Reader::new();
  let mut writer = Writer::new();
  let mut pipe = Pipe::new(&mut reader, &mut writer);
  pipe.read().unwrap();
}
