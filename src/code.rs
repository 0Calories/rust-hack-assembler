use phf::phf_map;

// Maps string representations of the comp portion of c-instructions to their binary equivalent
static COMP_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    "0"     => "0101010",
    "1"     => "0111111",
    "-1"    => "0111010",
    "D"     => "0001100",
    "A"     => "0110000",
    "M"     => "1110000",
    "!D"    => "0001101",
    "!A"    => "0110001",
    "!M"    => "1110001",
    "-D"    => "0001111",
    "-A"    => "0110011",
    "-M"    => "1110011",
    "D+1"   => "0011111",
    "A+1"   => "0110111",
    "M+1"   => "1110111",
    "D-1"   => "0001110",
    "A-1"   => "0110010",
    "M-1"   => "1110010",
    "D+A"   => "0000010",
    "D+M"   => "1000010",
    "D-A"   => "0010011",
    "D-M"   => "1010011",
    "A-D"   => "0000111",
    "M-D"   => "1000111",
    "D&A"   => "0000000",
    "D&M"   => "1000000",
    "D|A"   => "0010101",
    "D|M"   => "1010101"
};

// Maps string representations of the dest portion of c-instructions to their binary equivalent
static DEST_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    ""      => "000",
    "M"     => "001",
    "D"     => "010",
    "MD"    => "011",
    "A"     => "100",
    "AM"    => "101",
    "AD"    => "110",
    "AMD"   => "111"
};

    // Maps string representations of the jump portion of c-instructions to their binary equivalent
    static JUMP_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    ""      => "000",
    "JGT"   => "001",
    "JEQ"   => "010",
    "JGE"   => "011",
    "JLT"   => "100",
    "JNE"   => "101",
    "JLE"   => "110",
    "JMP"   => "111"
};

static A_INSTR_PREFIX: &str = "011";
static C_INSTR_PREFIX: &str = "111";

pub fn translate_a_instruction() {

}

pub fn translate_c_instruction(dest: &str, comp: &str, jump: &str) -> String {
    print!("Dest:{}, Comp:{}, Jump:{}", dest, comp, jump);
    let dest_binary = DEST_TABLE.get(dest).expect("Invalid dest portion of instruction!");
    let comp_binary = COMP_TABLE.get(comp).expect("Invalid comp portion of instruction!");
    let jump_binary = JUMP_TABLE.get(jump).expect("Invalid jump portion of instruction!");

    let result = format!("{}{}{}{}", C_INSTR_PREFIX, comp_binary, dest_binary, jump_binary);
    result
}
