use rusb::{Context, DeviceHandle, Error, UsbContext, TransferType, Direction};
use std::time::Duration;

fn foo() -> rusb::Result<()> {
  let vendor_id = 0x17ef;
  let product_id = 0x608d;

  let context = Context::new()?;
  let devices = context.devices()?;

  for device in devices.iter() {
    println!("looping...");
    let device_desc = device.device_descriptor().unwrap();

    if device_desc.vendor_id() != vendor_id || device_desc.product_id() != product_id {
      continue;
    }

    println!(
      "Found device: Bus {:03} Device {:03} ID {:04x}:{:04x}, speed: {:?}",
      device.bus_number(),
      device.address(),
      device_desc.vendor_id(),
      device_desc.product_id(),
      device.speed(),
    );

    let mut handle = device.open()?;
    println!("got the handle");

    // Often HID devices have interface #0 for the basic mouse/keyboard reports
    // If the kernel driver is attached (common on Linux), detach it first
    // (Windows does not require this call, and on macOS you also won't do this)
    handle.set_auto_detach_kernel_driver(true)?;
    let config_desc = device.active_config_descriptor()?;
    for interface in config_desc.interfaces() {
      for interface_desc in interface.descriptors() {
        let interface_number = interface_desc.interface_number();

        // Claim the interface. If the kernel is using it, auto-detach
        // above should handle that. If not, you may need handle.detach_kernel_driver(interface_number) explicitly.
        match handle.claim_interface(interface_number) {
          Ok(_) => println!("Claimed interface {}", interface_number),
          Err(e) => {
            eprintln!("Could not claim interface {}: {:?}", interface_number, e);
            continue;
          }
        };

        // Look for an interrupt IN endpoint.
        for endpoint_desc in interface_desc.endpoint_descriptors() {
          if endpoint_desc.transfer_type() == TransferType::Interrupt
            && endpoint_desc.direction() == Direction::In
          {
            let endpoint_address = endpoint_desc.address();
            println!(
              "Found interrupt IN endpoint: 0x{:02x} on interface {}",
              endpoint_address, interface_number
            );

            // Simple read loop
            let mut buf = [0u8; 8]; // size depends on your device
            loop {
              match handle.read_interrupt(endpoint_address, &mut buf, Duration::from_millis(500)) {
                Ok(len) => {
                  println!("Read {} bytes: {:?}", len, &buf[..len]);
                }
                Err(e) => {
                  // Timeout or other error
                  eprintln!("read_interrupt error: {:?}", e);
                  // Decide whether to break or continue
                  // break;
                }
              }
            }
          }
        }

        // If we get here, we may want to release the interface.
        // But keep in mind once we break out of the read loop we’re done.
        // handle.release_interface(interface_number)?;

        // let data = handle.read_interrupt(endpoint, buf, timeout);
        break;
      }
    }
  }
  Ok(())
}

fn main() {
  foo().unwrap();
}
