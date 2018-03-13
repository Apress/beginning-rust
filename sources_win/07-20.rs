/* It prints:
S*/
fn main() {
    enum CardinalPoint { North, South, West, East };
    let direction = CardinalPoint::South;
    print!("{}", match direction {
        CardinalPoint::North => 'N',
        CardinalPoint::South => 'S',
        _ => '*',
    });
}
