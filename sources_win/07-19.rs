/* It prints:
OK*/
fn main() {
    enum Result {
        Success(f64),
        Failure(u16, char),
        Uncertainty,
    }

    let outcome = Result::Success(23.67); 

    match outcome {
        Result::Success(_) => print!("OK"),
        Result::Failure(error_code, module) =>
            print!("Error n. {} in module {}",
                error_code, module),
        Result::Uncertainty => {},
    }
}
