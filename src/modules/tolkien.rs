use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::str::FromStr;
use crate::modules::Module;

pub struct Tolkien {}

impl Module for Tolkien {
    fn handle(&self, input: String) -> String {
        let file = File::open("./silmarillion.txt");

        let line = usize::from_str(input.as_str())
            .ok()
            .or_else(|| File::open("./silmarillion-line.txt")
                .ok()
                .and_then(|file| Some(BufReader::new(file)))
                .and_then(|buffer| Some(buffer.lines().nth(0).unwrap().unwrap()))
                .and_then(|line| usize::from_str(line.as_str()).ok()))
            .unwrap_or(0);

        match file {
            Err(e) => format!("Sorry, ik kan geen Tolkien voorlezen op het moment. {e:?}"),
            Ok(mut file) => {
                let mut contents = String::new();

                match file.read_to_string(&mut contents) {
                    Err(e) => format!("Sorry, ik kan geen Tolkien voorlezen op het moment. {e:?}"),
                    Ok(_) => {
                        let content = contents.split("\n\n").nth(line).unwrap_or(&"Deze regel snap ik niet.");

                        if input.is_empty()
                        {
                            File::create("./silmarillion-line.txt")
                                .unwrap().write_all((line + 1).to_string().as_bytes())
                                .expect("Oops");
                        }

                        content.to_string()
                    }
                }
            }
        }
    }
}