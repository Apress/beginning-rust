// ILLEGAL
fn main() {
    enum CardinalPoint { North, South, West, East };
    let direction = CardinalPoint::South;
    if direction == CardinalPoint::North { }
}
