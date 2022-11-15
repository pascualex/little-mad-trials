use crate::{board::BoardMode, laser::LaserMode, phases::Phase};

pub fn mobile_laser_board_phases() -> Vec<Phase<BoardMode>> {
    vec![
        Phase::new(BoardMode::Shown, 20.0),  // 20.0
        Phase::new(BoardMode::Exiting, 1.0), // 21.0
    ]
}

pub fn mobile_laser_phases() -> Vec<Phase<LaserMode>> {
    vec![
        // first round (0.0)
        Phase::new(LaserMode::Charging, 0.2), // 0.2
        Phase::new(LaserMode::Shooting, 0.2), // 0.4
        Phase::new(LaserMode::Ready, 0.8),    // 1.2
        Phase::new(LaserMode::Charging, 0.5), // 1.7
        Phase::new(LaserMode::Shooting, 0.2), // 1.9
        Phase::new(LaserMode::Ready, 0.8),    // 2.7
        Phase::new(LaserMode::Charging, 0.5), // 3.2
        Phase::new(LaserMode::Shooting, 0.2), // 3.4
        Phase::new(LaserMode::Ready, 1.0),    // 4.4
        // second round (4.4)
        Phase::new(LaserMode::Charging, 0.5), // 4.9
        Phase::new(LaserMode::Shooting, 0.2), // 5.1
        Phase::new(LaserMode::Ready, 0.8),    // 5.9
        Phase::new(LaserMode::Charging, 0.5), // 6.4
        Phase::new(LaserMode::Shooting, 0.2), // 6.6
        Phase::new(LaserMode::Ready, 0.8),    // 7.4
        Phase::new(LaserMode::Charging, 0.5), // 7.9
        Phase::new(LaserMode::Shooting, 0.2), // 8.1
        Phase::new(LaserMode::Ready, 1.0),    // 9.1
        // third round (9.1)
        Phase::new(LaserMode::Charging, 0.5), // 9.6
        Phase::new(LaserMode::Shooting, 0.2), // 9.8
        Phase::new(LaserMode::Ready, 0.3),    // 10.1
        Phase::new(LaserMode::Charging, 0.5), // 10.6
        Phase::new(LaserMode::Shooting, 0.2), // 10.8
        Phase::new(LaserMode::Ready, 0.3),    // 11.1
        Phase::new(LaserMode::Charging, 0.5), // 11.6
        Phase::new(LaserMode::Shooting, 0.2), // 11.8
        Phase::new(LaserMode::Ready, 1.0),    // 12.8
        // fourth round (12.8)
        Phase::new(LaserMode::Charging, 0.5), // 13.3
        Phase::new(LaserMode::Shooting, 0.2), // 13.5
        Phase::new(LaserMode::Ready, 0.3),    // 13.8
        Phase::new(LaserMode::Charging, 0.5), // 14.3
        Phase::new(LaserMode::Shooting, 0.2), // 14.5
        Phase::new(LaserMode::Ready, 0.3),    // 14.8
        Phase::new(LaserMode::Charging, 0.5), // 15.3
        Phase::new(LaserMode::Shooting, 0.2), // 15.5
        Phase::new(LaserMode::Ready, 1.0),    // 16.5
        // fifth round (16.5)
        Phase::new(LaserMode::Charging, 0.5), // 17.0
        Phase::new(LaserMode::Shooting, 0.2), // 17.2
        Phase::new(LaserMode::Ready, 0.0),    // 17.2
        Phase::new(LaserMode::Charging, 0.5), // 17.7
        Phase::new(LaserMode::Shooting, 0.2), // 17.9
        Phase::new(LaserMode::Ready, 0.0),    // 17.9
        Phase::new(LaserMode::Charging, 0.5), // 18.4
        Phase::new(LaserMode::Shooting, 0.2), // 18.6
        Phase::new(LaserMode::Ready, 0.0),    // 18.6
        Phase::new(LaserMode::Charging, 0.5), // 19.1
        Phase::new(LaserMode::Shooting, 0.2), // 19.3
        Phase::new(LaserMode::Ready, 0.0),    // 19.3
        Phase::new(LaserMode::Charging, 0.5), // 19.8
        Phase::new(LaserMode::Shooting, 0.2), // 20.0
    ]
}

pub fn upper_laser_board_phases() -> Vec<Phase<BoardMode>> {
    vec![
        Phase::new(BoardMode::Hidden, 3.4),   // 3.4
        Phase::new(BoardMode::Entering, 1.0), // 4.4
        Phase::new(BoardMode::Shown, 3.7),    // 8.1
        Phase::new(BoardMode::Exiting, 1.0),  // 9.1
        Phase::new(BoardMode::Hidden, 2.7),   // 11.8
        Phase::new(BoardMode::Entering, 1.0), // 12.8
        Phase::new(BoardMode::Shown, 7.2),    // 20.0
        Phase::new(BoardMode::Exiting, 1.0),  // 21.0
    ]
}

pub fn upper_laser_phases() -> Vec<Phase<LaserMode>> {
    vec![
        Phase::new(LaserMode::Ready, 4.4), // 4.4
        // second round (4.4)
        Phase::new(LaserMode::Charging, 0.5), // 4.9
        Phase::new(LaserMode::Shooting, 0.2), // 5.1
        Phase::new(LaserMode::Ready, 2.3),    // 7.4
        Phase::new(LaserMode::Charging, 0.5), // 7.9
        Phase::new(LaserMode::Shooting, 0.2), // 8.1
        Phase::new(LaserMode::Ready, 4.7),    // 12.8
        // fourth round (12.8)
        Phase::new(LaserMode::Charging, 0.5), // 13.3
        Phase::new(LaserMode::Shooting, 0.2), // 13.5
        Phase::new(LaserMode::Ready, 1.3),    // 14.8
        Phase::new(LaserMode::Charging, 0.5), // 15.3
        Phase::new(LaserMode::Shooting, 4.7), // 20.0
    ]
}

pub fn middle_laser_board_phases() -> Vec<Phase<BoardMode>> {
    vec![
        Phase::new(BoardMode::Hidden, 4.9),   // 4.9
        Phase::new(BoardMode::Entering, 1.0), // 5.9
        Phase::new(BoardMode::Shown, 0.7),    // 6.6
        Phase::new(BoardMode::Exiting, 1.0),  // 7.6
        Phase::new(BoardMode::Hidden, 4.2),   // 11.8
        Phase::new(BoardMode::Entering, 1.0), // 12.8
        Phase::new(BoardMode::Shown, 2.7),    // 14.5
        Phase::new(BoardMode::Exiting, 1.0),  // 15.5
    ]
}

pub fn middle_laser_phases() -> Vec<Phase<LaserMode>> {
    vec![
        Phase::new(LaserMode::Ready, 4.4),
        // second round (4.4)
        Phase::new(LaserMode::Ready, 1.5),    // 5.9
        Phase::new(LaserMode::Charging, 0.5), // 6.4
        Phase::new(LaserMode::Shooting, 0.2), // 6.6
        Phase::new(LaserMode::Ready, 6.2),    // 12.8
        // fourth round (12.8)
        Phase::new(LaserMode::Charging, 0.5), // 13.3
        Phase::new(LaserMode::Shooting, 0.2), // 13.5
        Phase::new(LaserMode::Ready, 0.3),    // 13.8
        Phase::new(LaserMode::Charging, 0.5), // 14.3
        Phase::new(LaserMode::Shooting, 0.2), // 14.5
    ]
}

pub fn lower_laser_board_phases() -> Vec<Phase<BoardMode>> {
    vec![
        Phase::new(BoardMode::Hidden, 3.4),   // 3.4
        Phase::new(BoardMode::Entering, 1.0), // 4.4
        Phase::new(BoardMode::Shown, 3.7),    // 8.1
        Phase::new(BoardMode::Exiting, 1.0),  // 9.1
        Phase::new(BoardMode::Hidden, 3.7),   // 12.8
        Phase::new(BoardMode::Entering, 1.0), // 13.8
        Phase::new(BoardMode::Shown, 6.2),    // 20.0
        Phase::new(BoardMode::Exiting, 1.0),  // 21.0
    ]
}

pub fn lower_laser_phases() -> Vec<Phase<LaserMode>> {
    vec![
        Phase::new(LaserMode::Ready, 4.4), // 4.4
        // second round (4.4)
        Phase::new(LaserMode::Charging, 0.5), // 4.9
        Phase::new(LaserMode::Shooting, 0.2), // 5.1
        Phase::new(LaserMode::Ready, 2.3),    // 7.4
        Phase::new(LaserMode::Charging, 0.5), // 7.9
        Phase::new(LaserMode::Shooting, 0.2), // 8.1
        Phase::new(LaserMode::Ready, 4.7),    // 12.8
        // fourth round (12.8)
        Phase::new(LaserMode::Ready, 1.0),    // 13.8
        Phase::new(LaserMode::Charging, 0.5), // 14.3
        Phase::new(LaserMode::Shooting, 0.2), // 14.5
        Phase::new(LaserMode::Ready, 0.3),    // 14.8
        Phase::new(LaserMode::Charging, 0.5), // 15.3
        Phase::new(LaserMode::Shooting, 4.7), // 20.0
    ]
}
