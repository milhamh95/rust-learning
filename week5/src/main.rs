mod point;

fn main() {
    let input1 = point::InputPoint {
        original_position: point::Position {x: 5, y: 10},
        flip_direction: point::FlipDirection::Horizontal,
    };
    
    let output1 = input1.flip();
    println!("{}", output1);

    let input2 = point::InputPoint {
        original_position: point::Position {x: 5, y: 10},
        flip_direction: point::FlipDirection::Vertical,
    };
    
    let output2 = input2.flip();
    println!("{}", output2);
}
