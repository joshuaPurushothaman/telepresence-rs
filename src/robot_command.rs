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
    MoveInDirection {
        direction: Direction,
        duration: Duration,
    },
    Stop,
    EndProgram,
}
