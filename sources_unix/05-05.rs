// ILLEGAL
fn main() {
    let mut x = ["a"]; // array of strings
    x[0] = 3;
    x[-1] = "b";
    x[0.] = "b";
    x[false] = "b";
    x["0"] = "b";
}
