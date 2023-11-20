use std::{error::Error, fmt::Display, time::Duration};

const DEFAULT_BAUD_RATE:u32 = 9000;

pub fn upload_data(payload: Box<[u8]>) -> Result<(), Box<dyn Error>> {
    let ports = serialport::available_ports()?;
    let chosen = inquire::Select::new(
        "Select serial to upload",
        ports.iter().map(|x| x.port_name.clone()).collect(),
    )
    .prompt()?;
    let baud_rate = inquire::CustomType::<u32>::new(&format!("Enter baud rate (default: {DEFAULT_BAUD_RATE})")).with_default(DEFAULT_BAUD_RATE).prompt()?;
    let mut open_port = serialport::new(chosen, baud_rate).timeout(Duration::from_millis(10)).open()?;
    Ok(open_port.write_all(&payload)?)
}
