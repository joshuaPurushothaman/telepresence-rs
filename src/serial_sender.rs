use crate::robot_command::{RobotCommand, Direction};

pub struct SerialSender {
    command_recv: std::sync::mpsc::Receiver<RobotCommand>,
    port: Box<dyn serialport::SerialPort>,
}

impl SerialSender {
    pub fn try_create(
        path: String,
        baud_rate: u32,
        command_recv: std::sync::mpsc::Receiver<RobotCommand>,
    ) -> Result<Self, serialport::Error> {
        println!("Creating SerialSender with port: {path}, baudrate: {baud_rate}");

        let port = serialport::new(&path, baud_rate)
            .timeout(std::time::Duration::from_secs(3))
            .open()?;

        println!("Port opened successfully.");

        Ok(SerialSender { port, command_recv })
    }

    pub fn run_forever(&mut self) -> Result<(), serialport::Error> {
        // let mut send_stop_msg = move |s: &SerialSender| -> Result<(), serialport::Error> {
        //     let stop_buf = [127, 127, 127, b'\n'];
        //     self.port.write_all(&stop_buf)?;
        //     Ok(())
        // };

        for msg in self.command_recv.iter() {
            match msg {
                RobotCommand::EndProgram => {
                    println!("SerialSender: Ending program.");

                    send_stop_msg(&mut self.port)?;
                    return Ok(());
                }

                RobotCommand::Stop => send_stop_msg(&mut self.port)?,

                RobotCommand::MoveInDirection {
                    direction,
                    duration,
                } => {
                    let x: u8 = 127; // Not used for Romi since it can't strafe horizontally
                    let mut y: u8 = 127;
                    let mut rotation: u8 = 127;

                    // Send a stop command first
                    let stop_buf = [127, 127, 127, b'\n'];
                    self.port.write_all(&stop_buf)?;

                    const SPEED: u8 = 50;

                    match direction {
                        Direction::Forward => {
                            y = 127 + SPEED;
                        }
                        Direction::Backward => {
                            y = 127 - SPEED;
                        }
                        Direction::LeftTurn => {
                            rotation = 127 - SPEED;
                        }
                        Direction::RightTurn => {
                            rotation = 127 + SPEED;
                        }
                    }

                    let serial_buf = [x, y, rotation, b'\n'];
                    self.port.write_all(&serial_buf)?;

                    std::thread::sleep(duration);

                    send_stop_msg(&mut self.port)?
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        Ok(())
    }
}

// let mut send_stop_msg = move |s: &SerialSender| -> Result<(), serialport::Error> {
//     let stop_buf = [127, 127, 127, b'\n'];
//     self.port.write_all(&stop_buf)?;
//     Ok(())
// };
fn send_stop_msg(port: &mut Box<dyn serialport::SerialPort>) -> Result<(), serialport::Error> {
    let stop_buf = [127, 127, 127, b'\n'];
    port.write_all(&stop_buf)?;
    Ok(())
}
