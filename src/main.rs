use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() {
    let html_pattern = Regex::new(".html?$").unwrap();

    std::env::args().skip(1).for_each(|filename| {
        File::open(&filename)
            .map(|src| {
                let replaced = html_pattern.replace(&filename, ".blk.html");
                let dist: String = replaced.into();
                let mut writer = BufWriter::new(File::create(dist).unwrap());
                let reader = BufReader::new(src);
                let mut complete = false;
                for line in reader.lines() {
                    line.map(|line| {
                        if complete {
                            writeln!(&mut writer, "{}", line).unwrap();
                        } else {
                            if line.contains("<body>") {
                                writeln!(
                                    &mut writer,
                                    "{}",
                                    line.replace(
                                        "<body>",
                                        "<body style=\"background-color: #202020;\">"
                                    )
                                )
                                .unwrap();
                                complete = true;
                            } else {
                                writeln!(&mut writer, "{}", line).unwrap();
                            }
                        }
                    })
                    .unwrap();
                }
            })
            .unwrap();
    });
}
