use std::fmt;

#[derive(Copy, Clone)]
pub enum FlipDirection {
    Horizontal,
    Vertical
}

impl FlipDirection {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlipDirection::Horizontal => "Horizontal",
            FlipDirection::Vertical => "Vertical",
        }
    }
}

pub struct InputPoint {
    pub original_position: Position,
    pub flip_direction: FlipDirection
}

impl InputPoint {
    pub fn flip(&self) -> OutputPoint {
        let output_position = match self.flip_direction {
            FlipDirection::Horizontal => self.flip_horizontal(),
            FlipDirection::Vertical => self.flip_vertical(),
        };
        OutputPoint {
            original_position: Position {
                x: self.original_position.x,
                y: self.original_position.y,
            },
            flip_direction: self.flip_direction,
            output_position,
        }
    }

    fn flip_horizontal(&self) -> Position {
        Position {
            x: -self.original_position.x,
            y: self.original_position.y
        }
    }

    fn flip_vertical(&self) -> Position {
        Position {
            x: self.original_position.x,
            y: -self.original_position.y,
        }
    }
}

pub struct OutputPoint {
    pub original_position: Position,
    pub flip_direction: FlipDirection,
    pub output_position: Position
}

impl fmt::Display for OutputPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            r#"
            Original Position: ({}, {})
            Flip Direction: {}
            Output Position: ({}, {})
            "#,
            self.original_position.x,
            self.original_position.y,
            self.flip_direction.as_str(),
            self.output_position.x,
            self.output_position.y
         )
    }
}

pub struct Position {
    pub x: i8,
    pub y: i8
}