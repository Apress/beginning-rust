/* It waits for a line typed from the console.
   After the user has typed such line, it prints:
Ok(<integer number>)
[<contents of the line>
]
*/
fn main() {
    let mut line = String::new();
    println!("{:?}", std::io::stdin().read_line(&mut line));
    println!("[{}]", line);
}
