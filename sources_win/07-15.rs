/* It prints:
Error n. 1200 in module X*/
fn main() {
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
    }

    let outcome = Result::Failure(1200, 'X');

    match outcome {
        Result::Success(value) =>
            print!("Result: {}", value),
        Result::Failure(error_code, module) =>
            print!("Error n. {} in module {}",
                error_code, module),
        Result::Uncertainty => {},
    }
}
