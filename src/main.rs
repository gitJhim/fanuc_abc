use std::fs::File;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::collections::HashMap;

mod alphabet;

const fileName: &str = "fanucABC.ls";

fn main() {
    createLSFile();

    let alphabet = alphabet::alphanetHashMap();    
    let word = getWord();


    for c in word.chars() {
        println!("{}", &alphabet[&c.to_string()]);
        appendToFile(&alphabet[&c.to_string()]);
    }
}

fn createLSFile() -> std::io::Result<()> {
    let mut file = File::create(fileName.to_owned())?;
    file.write_all(b"
    /PROG  PROG_NEW
    /ATTR
    OWNER		= MNEDITOR;
    COMMENT		= \"\";
    PROG_SIZE	= 172;
    CREATE		= DATE 22-09-08  TIME 12:39:20;
    MODIFIED	= DATE 22-09-09  TIME 09:39:44;
    FILE_NAME	= ;
    VERSION		= 0;
    LINE_COUNT	= 3;
    MEMORY_SIZE	= 532;
    PROTECT		= READ_WRITE;
    TCD:  STACK_SIZE	= 0,
          TASK_PRIORITY	= 50,
          TIME_SLICE	= 0,
          BUSY_LAMP_OFF	= 0,
          ABORT_REQUEST	= 0,
          PAUSE_REQUEST	= 0;
    DEFAULT_GROUP	= 1,*,*,*,*;
    CONTROL_CODE	= 00000000 00000000;
    /MN
    ")?;

    Ok(())
}

fn appendToFile(command: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(fileName)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", command) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn getWord() -> String {
    let mut line = String::new();
    println!("Enter a word:");
    std::io::stdin().read_line(&mut line);
    return line;
}
// Grippers
// Open:  .7in
// Close: .18in
