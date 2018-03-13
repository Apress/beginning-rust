/* It prints:
SOUTH*/
fn main() {
    enum CardinalPoint { North, South, West, East };
    let direction = CardinalPoint::South;
    match direction {
        CardinalPoint::North => print!("NORTH"),
        CardinalPoint::South => print!("SOUTH"),
        CardinalPoint::East => {},
        CardinalPoint::West => {},
    }
}
