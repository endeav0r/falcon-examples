extern crate clap;
extern crate falcon;

use falcon::il;
use falcon::loader::{Elf, Loader};
use std::path::Path;


fn main () {
    let matches = clap::App::new("falcon-example-0")
        .version("0.2.1")
        .about("An example of Falcon usage")
        .arg(clap::Arg::with_name("program")
                .short("p")
                .long("program")
                .value_name("FILE")
                .help("Program to analyze with Falcon")
                .required(true))
        .get_matches();

    let program_filename = matches.value_of("program").unwrap();

    // Load our Elf
    let elf = match Elf::from_file(Path::new(program_filename)) {
        Ok(elf) => elf,
        Err(_) => { panic!("Error loading elf"); }
    };

    // Let's print out all the functions we've found
    println!("Functions by address");
    for function_entry in elf.function_entries().unwrap() {
        match *function_entry.name() {
            Some(ref name) => {
                println!("0x{:x} {}", function_entry.address(), name);
            },
            None => {
                println!("0x{:x}", function_entry.address());
            }
        }
    }

    println!("");

    // Get the IL Program for the ELF
    let program = elf.program().unwrap();

    // For every function, print the number of blocks
    println!("Number of blocks in function");
    for function in program.functions() {
        println!("{} {}", function.name(), function.blocks().len());
    }

    println!("");

    // If we have a main function, let's print that
    if let Some(function) = program.functions()
                                   .into_iter()
                                   .filter(|f| f.name() == "main")
                                   .collect::<Vec<&il::Function>>()
                                   .first() {
        println!("{}", function.control_flow_graph());
    }
}