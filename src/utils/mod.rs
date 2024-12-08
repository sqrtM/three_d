use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub(crate) fn parse_file_line_by_line<P, F>(path: P, mut process_line: F) -> io::Result<()>
where
    P: AsRef<Path>,
    F: FnMut(String),
{
    let file = File::open(path)?;

    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        process_line(line);
    }

    Ok(())
}
