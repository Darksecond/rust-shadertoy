
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use regex::Regex;
use std::fmt::Write;

lazy_static! {
    static ref INCLUDE_REGEX: Regex = Regex::new("^\\s*#\\s*include\\s+\"([[:print:]]+)\"\\s*$").unwrap();
}

#[derive(Debug)]
pub struct Source {
    path: PathBuf,
    source: String
}

impl Source {
    pub fn new(path: &str) -> Source {
        Source {
            source: read_file(&path),
            path: PathBuf::from(path),
        }
    }
}

#[derive(Debug)]
pub struct Resolver<'a> {
    sources: HashMap<&'a str, Source>
}

impl<'a> Resolver<'a> {
    pub fn new() -> Self {
        Resolver {
            sources: HashMap::new()
        }
    }
    pub fn push(&mut self, path: &'a str) {
        let source = Source::new(path);
        self.sources.insert(&path, source);
    }

    //TODO push_glob

    pub fn resolve(&self, path: &str) -> Option<String> {
        match self.sources.get(path) {
            Some(source) => {
                let output = self.parse(source);
                Some(output)
            },
            None => None
        }
    }

    fn parse(&self, source: &Source) -> String {
        let mut output = String::new();
        for (_, line) in source.source.lines().enumerate() {
            if let Some(cap) = INCLUDE_REGEX.captures(line) {
                let include = self.sources.get(&cap[1]).expect("File should exist");
                write!(&mut output, "{}\n", self.parse(include)).unwrap();
            } else {
                write!(&mut output, "{}\n", line).unwrap();
            }
        }
        output
    }
}

/// Helper function which reads the contents of a file as a string.
fn read_file<P: AsRef<Path>>(p: P) -> String {
    let mut file = File::open(p).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}
