use rusb::{Context, Error, UsbContext, TransferType, Direction};
use std::time::Duration;
use std::thread;
use std::sync::Arc;

fn foo() -> rusb::Result<()> {
  let device_ids = "17ef:608d"; // mouse
  // let device_ids = "090c:1000"; // usb drive
  // let device_ids = "046d:0a44"; // headset

  let mut ids = device_ids.split(":");

  let Some(vendor_id) = ids.next() else {
    return Err(Error::Other);
  }; //0x17ef;
  println!("vendor_id is {}", vendor_id);
  let v_id = u16::from_str_radix(vendor_id, 16).unwrap();

  let Some(product_id) = ids.next() else {
    return Err(Error::Other);
  }; //0x608d;
  let p_id = u16::from_str_radix(product_id, 16).unwrap();

  let context = Context::new()?;
  let devices = context.devices()?;

  let mut threads = Vec::new();

  for device in devices.iter() {
    println!("looping...");
    let device_desc = device.device_descriptor().unwrap();

    if device_desc.vendor_id() != v_id || device_desc.product_id() != p_id {
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

    let handle = Arc::new(device.open()?);
    println!("got the handle");

    handle.set_auto_detach_kernel_driver(true)?;

    let config_desc = device.active_config_descriptor()?;
  
    for interface in config_desc.interfaces() {
      for interface_desc in interface.descriptors() {
        let cloned_handle = Arc::clone(&handle);
        let interface_number = interface_desc.interface_number();

        // Claim the interface. If the kernel is using it, auto-detach
        // above should handle that. If not, you may need handle.detach_kernel_driver(interface_number) explicitly.
        match cloned_handle.claim_interface(interface_number) {
          Ok(_) => println!("Claimed interface {}", interface_number),
          Err(e) => {
            eprintln!("Could not claim interface {}: {:?}", interface_number, e);
            continue;
          }
        };

        // Look for an interrupt IN endpoint.
        for endpoint_desc in interface_desc.endpoint_descriptors() {
          if endpoint_desc.transfer_type() == TransferType::Interrupt {
            if endpoint_desc.direction() == Direction::In {
              let endpoint_address = endpoint_desc.address();
              println!(
                "Found interrupt IN endpoint: 0x{:02x} on interface {}",
                endpoint_address, interface_number
              );
                
              // determine packet size for this endpoint
              let packet_size = endpoint_desc.max_packet_size() as usize;
              let mut buf = vec![0u8; packet_size];

              let cloned_handle = Arc::clone(&handle);
              let t = thread::spawn(move || {
                // read loop
                loop {
                  match cloned_handle.read_interrupt(endpoint_address, &mut buf, Duration::from_secs(60)) {
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
              });
              threads.push(t);

            } else {
              unimplemented!();
            }
            
          } else if endpoint_desc.transfer_type() == TransferType::Bulk {
            if endpoint_desc.direction() == Direction::In {
              let endpoint_address = endpoint_desc.address();
              println!(
                "Found bulk IN endpoint: 0x{:02x} on interface {}",
                endpoint_address, interface_number
              );

              let packet_size = endpoint_desc.max_packet_size() as usize;
              let mut buf = vec![0u8; packet_size];

              // TODO: we should probably start a thread here
              let cloned_handle = Arc::clone(&handle);
              let t = thread::spawn(move || {
                loop {
                  match cloned_handle.read_bulk(endpoint_address, &mut buf, Duration::from_secs(5)) {
                    Ok(len) => {
                      println!("Read {} bytes: {:?}", len, &buf[..len]);
                    }
                    Err(e) => {
                      eprintln!("read_bulk error: {:?}", e)
                    }
                  }
                }
              });
              threads.push(t);

            } else {
              unimplemented!();
            }

          } else if endpoint_desc.transfer_type() == TransferType::Isochronous {
            if endpoint_desc.direction() == Direction::In {
              let endpoint_address = endpoint_desc.address();
              println!(
                "Found isochronous IN endpoint: 0x{:02x} on interface {}",
                endpoint_address, interface_number
              );
              unimplemented!();

            } else {
              unimplemented!();
            }

          } else if endpoint_desc.transfer_type() == TransferType::Control {
            unimplemented!();

          } else {
            unimplemented!();
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

  for t in threads {
    t.join().unwrap();
  }
  Ok(())
}

fn main() {
  foo().unwrap();
}
