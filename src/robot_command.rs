use std::time::Duration;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
    LeftTurn,
    RightTurn,
}

#[derive(Debug)]
pub enum RobotCommand {
    /// Speed from [0, 127]
    MoveInDirection {
        direction: Direction,
        duration: Duration,
        speed: u8,
    },
    Stop,
    EndProgram,
}
