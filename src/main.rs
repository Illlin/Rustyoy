use std::io;
use std::io::Write;

fn main() {
    set_cursor_pos(3,3);
    print!("\u{1B}[1;31mHello, world! ");
    flush();
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("Error getting guess");
}

fn flush(){
    io::stdout().flush().unwrap();
}

fn clear(){
    print!("\u{1B}[J");
    flush();
}

fn set_cursor_pos(x: u8, y: u8){
    print!("\u{1B}[{};{}H",x,y)
}