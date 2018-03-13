// It does nothing.
fn main() {
    const MAXIMUM_POWER: u16 = 600;
    enum VehicleKind {
        Motorcycle,
        Car,
        Truck,
    }
    struct VehicleData {
        kind: VehicleKind,
        registration_year: u16,
        registration_month: u8,
        power: u16,
    }
    let vehicle = VehicleData {
        kind: VehicleKind::Car,
        registration_year: 2003,
        registration_month: 11,
        power: 120,
    };
    if vehicle.power > MAXIMUM_POWER {
        println!("Too powerful");
    }
}
