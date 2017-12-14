extern crate clap;
extern crate falcon;

use falcon::analysis;
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
        .arg(clap::Arg::with_name("function")
                .short("f")
                .long("function")
                .value_name("FUNCTION_NAME")
                .help("Function to analyze with Falcon")
                .required(true))
        .get_matches();

    let program_filename = matches.value_of("program").unwrap();
    let function_name = matches.value_of("function").unwrap();

    // Load our Elf
    let elf = match Elf::from_file(Path::new(program_filename)) {
        Ok(elf) => elf,
        Err(_) => { panic!("Error loading elf"); }
    };

    let program = elf.program().unwrap();

    // If we have a main function, let's print that
    let function = match program.functions()
                                .into_iter()
                                .filter(|f| f.name() == function_name)
                                .collect::<Vec<&il::Function>>()
                                .first() {
        Some(function) => (*function).clone(),
        None => {
            panic!(format!("Could not find function {}", function_name));
        }
    };

    let ud = match analysis::use_def(&function) {
        Ok(ud) => ud,
        Err(e) => { panic!("Error calculating use def chains: {}", e); }
    };

    for block in function.blocks() {
        for instruction in block.instructions() {
            println!("{}", instruction);
            let rfl = il::RefFunctionLocation::Instruction(block, instruction);
            let rpl = il::RefProgramLocation::new(&function, rfl);
            for location in ud[&rpl].locations() {
                println!("  {}", location);
            }
        }
    }
}