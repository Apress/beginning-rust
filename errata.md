# Errata for *Beginning Rust*

On **page 72** [technical accuracy]:
 
let i = 8;
let j = 8_000_000_000;
print!("{} {}", i, j);
```

This program will print: ``"8 -589934592"``.

While I run as below in Ubuntu:

```
fn main() {
    let i = 8;
    let j = 8_000_000_000;
    print!("{} {}", i, j);
}
```

I cannot see the same result, but get following error message:

```
[Running]
error: literal out of range for `i32`
 --> tempCodeRunnerFile.rs:3:13
  |
3 |     let j = 8_000_000_000;
  |             ^^^^^^^^^^^^^
  |
  = note: `#[deny(overflowing_literals)]` on by default
  = note: the literal `8_000_000_000` does not fit into the type `i32` whose range is `-2147483648..=2147483647`

error: aborting due to previous error
```

Question: is that mean you configure to have `u32` not `i32` by default?


***

On **page xx** [Summary of error]:
 
Details of error here. Highlight key pieces in **bold**.

***
