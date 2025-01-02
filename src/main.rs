use std::sync::Arc;

use proxy::Reader;
use proxy::Writer;
use proxy::Pipe;

use lazy_static::lazy_static;


// FIXME please, get rid of the static lifetime
lazy_static! {
  static ref PIPE: Arc<Pipe> = Arc::new(Pipe::new(Reader::new(), Writer::new()));
}

fn main() {
  PIPE.read().unwrap();
}
