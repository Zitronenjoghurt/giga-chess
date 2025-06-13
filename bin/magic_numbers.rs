use giga_chess::engine::magic_numbers::{find_bishop_magics, find_rook_magics};

fn main() {
    print_bishop_magics();
    print_rook_magics();
}

fn print_bishop_magics() {
    let bishop_magics = find_bishop_magics();
    println!("const BISHOP_MAGICS: [u64; 64] = [");
    for (i, &magic) in bishop_magics.iter().enumerate() {
        print_magic(i, magic);
    }
    println!("\n];");
}

fn print_rook_magics() {
    let rook_magics = find_rook_magics();
    println!("const ROOK_MAGICS: [u64; 64] = [");
    for (i, &magic) in rook_magics.iter().enumerate() {
        print_magic(i, magic);
    }
    println!("\n];");
}

fn print_magic(i: usize, magic: u64) {
    if i % 8 == 0 && i != 0 {
        println!();
    }
    print!("0x{:016X}", magic);
    if i < 63 {
        print!(", ");
    }
}
