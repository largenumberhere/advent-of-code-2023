use std::io::Read;

fn main() {
    let mut args = std::env::args();
    let _ = args.next()
        .expect("Invalid args given! program argument 0 should always be the current file's name");
    
    let input_file = args.next()
        .expect("Fatal error: Please give an file name as argument 1");

    if input_file == "--help" || input_file == "-help" || input_file == "/help" {
        println!("Small utility to get the entrypoint address of a 64-bit elf file.\nThe first and only argument is the file name. Prints the address in hexadecimal.");
        return;
    }


    if let Some(v) = args.next() {
        panic!("Only one argument expected");
    }

    std::mem::drop(args);

    let mut file = std::fs::File::open(input_file)
        .expect("Failed to open file given");

    let mut elf_header = [0; 64];

    file.read_exact(&mut elf_header)
        .expect("Failed to read first 64 bytes from file");

    // check for magic elf header values
    let magic: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];
    assert_eq!(magic, elf_header[0..4], "File is probably not in ELF format! Did not contain _ident[EI_MAG0] through e_ident[EI_MAG3]");

    // check for 64-bit
    assert_eq!(2 ,elf_header[0x04], "Only 64-bit ELF file is supported");

    // check for little endian
    assert_eq!(1, elf_header[0x05], "Only little endian ELF files are supported");

    // read the 64-bit little endian entrypoint value

    let entry_start:[u8; 8] = elf_header[0x18.. (0x18+8)].try_into().expect("oops");

    let value = i64::from_le_bytes(entry_start);

    println!("0x{:X}", value);



}
