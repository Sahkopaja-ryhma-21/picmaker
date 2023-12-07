use std::error::Error;

const DEFAULT_BAUD_RATE: u32 = 9600;

pub fn upload_data(payload: Box<[u8]>) -> Result<(), Box<dyn Error>> {
    let ports = serialport::available_ports()?;
    let chosen = match ports.len() {
        1 => {
            let p = ports.get(0).expect("Port to be still connected").port_name.clone();
            println!("Only one port connected {p}");
            p
        },
        _ => {
            inquire::Select::new(
                "Select serial to upload",
                ports.iter().map(|x| x.port_name.clone()).collect(),
            )
            .prompt().expect("You need atleast one board connected")
        }
    };
    let baud_rate = DEFAULT_BAUD_RATE;
        /*inquire::CustomType::<u32>::new(&format!("Enter baud rate (default: {DEFAULT_BAUD_RATE})"))
            .with_default(DEFAULT_BAUD_RATE)
            .prompt()?;*/
    let mut open_port = serialport::new(chosen, baud_rate).open()?;
    open_port.write_all(&payload)?;
    Ok(open_port.flush()?)
}
