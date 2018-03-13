// ILLEGAL
fn main() {
    /* This is /* a valid*/
    comment, even /* if /* it contains
    comments*/ inside */itself.  */
    /* This /* instead is not allowed in Rust,
    while in C is tolerated (but it may generate a warning).*/
}

