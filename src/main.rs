use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    clear();
    print_hex(3,3,255,255,255);
    print_hex(5,10,255,255,255);
    print_hex(7,3,255,255,255);

    flush();
}

fn flush(){
    io::stdout().flush().unwrap();
}

fn clear(){
    print!("\u{1B}[2J");
    flush();
}

fn set_cursor_pos(x: u8, y: u8){
    print!("\u{1B}[{};{}H",x,y)
}

fn set_colour(r:u8,g:u8,b:u8){
    print!("\u{1B}[38;2;{r};{g};{b}m");
}

fn print_hex(x:u8,y:u8,r:u8,g:u8,b:u8){
    set_colour(r,g,b);
    set_cursor_pos(x,y+2);
    print!("_____");
    set_cursor_pos(x+1,y+1);
    print!("/     \\");
    set_cursor_pos(x+2,y);
    print!("/       \\");
    set_cursor_pos(x+3,y);
    print!("\\       /");
    set_cursor_pos(x+4,y+1);
    print!("\\_____/");
    flush();
}

// Not working yet
/*
fn get_mouse(){
    print!("\u{1B}[?1002h");
    thread::sleep(time::Duration::from_millis(1000));
    print!("\u{1B}[?0h");
}*/

/* Example map
                 _______
                / _____ \
          _____/ /     \ \_____
         / _____/  311  \_____ \
   _____/ /     \       /     \ \_____
  / _____/  221  \_____/  412  \_____ \
 / /     \       /     \       /     \ \
/ /  131  \_____/  322  \_____/  513  \ \
\ \       /     \       /     \       / /
 \ \_____/  232  \_____/  423  \_____/ /
 / /     \       /     \       /     \ \
/ /  142  \_____/  333  \_____/  524  \ \
\ \       /     \       /     \       / /
 \ \_____/  243  \_____/  434  \_____/ /
 / /     \       /     \       /     \ \
/ /  153  \_____/  344  \_____/  535  \ \
\ \       /     \       /     \       / /
 \ \_____/  254  \_____/  445  \_____/ /
  \_____ \       /     \       / _____/
        \ \_____/  355  \_____/ /
         \_____ \       / _____/
               \ \_____/ /
                \_______/
*/