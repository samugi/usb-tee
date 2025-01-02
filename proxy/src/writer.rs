use std::{fs::OpenOptions, io::Write, time::Duration};

use bytes::Bytes;
use usb_gadget::{
  function::custom::{EndpointDirection, EndpointSender},
  udcs, Class, Config, Gadget, Id, RegGadget, Strings,
};

use usb_gadget::function::hid::Hid;

use std::fs;
use std::io;

const MOUSE_REPORT_DESCRIPTOR: [u8; 74] = [
    0x05, 0x01,       // USAGE_PAGE (Generic Desktop)
    0x09, 0x02,       // USAGE (Mouse)
    0xA1, 0x01,       // COLLECTION (Application)
    0x09, 0x01,       //   USAGE (Pointer)
    0xA1, 0x00,       //   COLLECTION (Physical)

    // 5 Buttons (bits) + 3 bits padding => 1 byte
    0x05, 0x09,       //   USAGE_PAGE (Button)
    0x19, 0x01,       //   USAGE_MINIMUM (Button 1)
    0x29, 0x05,       //   USAGE_MAXIMUM (Button 5)
    0x15, 0x00,       //   LOGICAL_MINIMUM (0)
    0x25, 0x01,       //   LOGICAL_MAXIMUM (1)
    0x95, 0x05,       //   REPORT_COUNT (5 buttons)
    0x75, 0x01,       //   REPORT_SIZE (1 bit each)
    0x81, 0x02,       //   INPUT (Data,Var,Abs)
    0x95, 0x01,       //   REPORT_COUNT (1)
    0x75, 0x03,       //   REPORT_SIZE (3) => pad out the rest of this byte
    0x81, 0x01,       //   INPUT (Cnst,Ary,Abs) => unused bits

    // X, Y, Wheel (3 bytes total, signed, relative)
    0x05, 0x01,       //   USAGE_PAGE (Generic Desktop)
    0x09, 0x30,       //   USAGE (X)
    0x09, 0x31,       //   USAGE (Y)
    0x09, 0x38,       //   USAGE (Wheel)
    0x15, 0x81,       //   LOGICAL_MINIMUM (-127)
    0x25, 0x7F,       //   LOGICAL_MAXIMUM (127)
    0x75, 0x08,       //   REPORT_SIZE (8 bits)
    0x95, 0x03,       //   REPORT_COUNT (3 fields => X, Y, Wheel)
    0x81, 0x06,       //   INPUT (Data,Var,Rel)

    // 4 vendor-defined bytes (Usage Page = 0xFF00)
    0x06, 0x00, 0xFF, //   USAGE_PAGE (Vendor 0xFF00)
    0x09, 0x01,       //   USAGE (Vendor usage 1)
    0x09, 0x02,       //   USAGE (Vendor usage 2)
    0x09, 0x03,       //   USAGE (Vendor usage 3)
    0x09, 0x04,       //   USAGE (Vendor usage 4)
    0x15, 0x00,       //   LOGICAL_MINIMUM (0)
    0x26, 0xFF, 0x00, //   LOGICAL_MAXIMUM (255)
    0x75, 0x08,       //   REPORT_SIZE (8 bits)
    0x95, 0x04,       //   REPORT_COUNT (4 => 4 bytes)
    0x81, 0x02,       //   INPUT (Data,Var,Abs)

    0xC0,             // END_COLLECTION (Physical)
    0xC0              // END_COLLECTION (Application)
];

// src/Writer.rs
pub struct Writer {
  gadget: RegGadget,
}

impl Writer {
  pub fn new() -> Self {
    // get available USB device controllers in the system
    let udcs = udcs().unwrap();
    let first = udcs.first().unwrap();

    println!("first {:?}", first);

    usb_gadget::remove_all().expect("cannot remove all gadgets");

    let mut builder = Hid::builder();
    builder.protocol = 2;
    builder.sub_class = 1;
    builder.report_len = 3;
    builder.report_desc = MOUSE_REPORT_DESCRIPTOR.to_vec();
    let (_hid, hid_function) = builder.build();

    let gadget = Gadget::new(
      Class::interface_specific(),
      Id::new(0x1d6b, 0x0104),
      Strings::new("Meow", "Gadget", "1234"),
    )
    .with_config(Config::new("Config 1").with_function(hid_function))
    .bind(&first)
    .expect("Failed binding to udc");

    // sleep a while
    std::thread::sleep(Duration::from_secs(1));

    let writer = Writer { gadget };
    writer
  }

  pub fn write(&mut self, data: &mut [u8]) {
    println!("Data to write is {:?}", data);

    let path = "/dev/hidg0"; // TODO: load dynamically
    println!("sending to path {:?}", path);

    let mut file = OpenOptions::new()
      .write(true)
      .open(path)
      .expect("Failed to open file");

    file.write_all(data).expect("Failed to write to file");
    file.flush().expect("Failed to flush file");

    println!("written data... {:?}", data);
  }
}
