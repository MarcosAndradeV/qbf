use std::{env, io, process};

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next().expect("Program not provided.");
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "help" => return usage(),
            _ => {}
        }
        match compile_file(arg) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1)
            }
        }
    } else {
        usage()
    }
}

fn compile_file(_arg: String) -> Result<String, String> {
    todo!()
}

fn usage() -> io::Result<()> {
    println!(
        r###"
        Usage: qbf <file.bf>
        "###
    );
    Ok(())
}
