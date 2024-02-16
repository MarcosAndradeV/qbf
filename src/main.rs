use std::{env, fs::File, io::{self, Read}, path::PathBuf, process};

fn main() -> io::Result<()> {
    let mut args = env::args();
    args.next().expect("Program not provided.");
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "help" => return usage(),
            _ => {

            }
        }
        let file = PathBuf::from(arg);
        if !file.exists() {

        }
        let mut file = File::open(file)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        match compile_file(buf) {
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

fn compile_file(source: Vec<u8>) -> Result<Vec<u8>, String> {
    let mut output = vec![];
    let mut source = source.into_iter();
    while let Some(ch) = source.next() {
        match ch {
            b'+' => output.extend("".as_bytes()),
            _ => {}
        }
    }
    
    Ok(output)
}

fn usage() -> io::Result<()> {
    println!(
        r###"
        Usage: qbf [option] <file.bf>
        Option:
            help - Prints this message.
        "###
    );
    Ok(())
}
