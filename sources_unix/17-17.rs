/* If this program is called with two command-line arguments,
   the first being the name of a readable file,
   and the second being the name of a writeable file,
   possibly already existing, it does not print anything,
   but it copies the first file onto the second one.
   CAUTION: If the destination file already exists,
   it will be silently overwritten!
*/
fn main() {
    use std::io::Read;
    use std::io::Write;
    let mut command_line: std::env::Args = std::env::args();
    command_line.next().unwrap();
    let source = command_line.next().unwrap();
    let destination = command_line.next().unwrap();
    let mut file_in = std::fs::File::open(source).unwrap();
    let mut file_out = std::fs::File::create(destination).unwrap();
    let mut buffer = [0u8; 4096];
    loop {
        let nbytes = file_in.read(&mut buffer).unwrap();
        file_out.write(&buffer[..nbytes]).unwrap();
        if nbytes < buffer.len() { break; }
    }
}
