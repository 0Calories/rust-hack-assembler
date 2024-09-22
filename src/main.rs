use std::io;

mod code;
mod parser;
mod symbol_table;

fn main() {
    println!("Hello, world!");

    user_input_instruction();
}

fn user_input_instruction() {
    println!("Input the dest: ");
    let mut dest = String::new();
    io::stdin()
        .read_line(&mut dest)
        .expect("Failed to read dest");

    dest = (dest.trim()).to_string();

    println!("Input the comp: ");
    let mut comp = String::new();
        io::stdin()
            .read_line(&mut comp)
            .expect("Failed to read comp");

    comp = (comp.trim()).to_string();

    println!("Input the jump: ");
    let mut jump = String::new();
        io::stdin()
            .read_line(&mut jump)
            .expect("Failed to read jump");

    jump = (jump.trim()).to_string();

    let result = code::translate_c_instruction(&dest, &comp, &jump);

    println!("Translation: {}", result)
}
