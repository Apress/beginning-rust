// ILLEGAL
fn main() {
    enum CardinalPoint { North, South, West, East };
    if CardinalPoint::South < CardinalPoint::North { }
}
