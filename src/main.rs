use clap::{clap_app, crate_version};
use failure::Error;
use regex::Regex;

#[derive(Debug)]
struct Record {
    line: usize,
    text: String,
}

fn process_file<P: AsRef<std::path::Path>>(p: P, re: Regex) -> Result<Vec<Record>, Error> {
    let mut res = Vec::new();
    let bts = std::fs::read(p).map_err(|e| e)?;

    if let Ok(ss) = String::from_utf8(bts) {
        for (i, l) in ss.lines().enumerate() {
            if re.is_match(l) {
                res.push(Record {
                    line: i + 1,
                    text: l.to_string(),
                })
            }
        }
    }
    Ok(res)
}

fn main() {
    let cp = clap_app!(
      rgrep =>
      (version: crate_version!())
      (about: "Minimal version of grep written in Rust!")
      (author: "Gaurav Saini")
      (@arg pattern: +required "Regex pattern to search")
      (@arg file: -f --file +takes_value "Input file")
    )
    .get_matches();
    let p = process_file(
        cp.value_of("file").ok_or("File not chosen").unwrap(),
        Regex::new(cp.value_of("pattern").unwrap()).unwrap(),
    );

    if let Ok(v) = p {
        for i in v {
            println!("Line {}: {}", i.line, i.text);
        }
    }
}
