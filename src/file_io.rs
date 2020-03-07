use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::io::LineWriter;

pub fn generate_file(project_info: HashMap<String, String>) -> Result<(), Box<Error>> {
    let file = File::create("README.md")?;
    let mut file = LineWriter::new(file);

    for (k, v) in project_info.iter() {
        let text = String::from(format!("{}\n{}", k, v));
        file.write_all(text.as_bytes())?;
    }
    file.flush()?;

    Ok(())
}
