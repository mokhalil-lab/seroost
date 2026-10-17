use std::{
    fs::File,
    io::{self, Result},
    println,
};

use xml::{EventReader, reader::XmlEvent};

#[derive(Debug)]

struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while !self.content.is_empty() && self.content[0].is_whitespace() {
            self.content = &self.content[1..]
        }
    }

    fn chop(&mut self, n: usize) -> &'a [char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    fn chop_while<P>(&mut self, mut predicate: P) -> &'a [char]
    where
        P: FnMut(&char) -> bool,
    {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.chop(n)
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.is_empty() {
            return None;
        }

        if self.content[0].is_numeric() {
            return Some(self.chop_while(|c| c.is_numeric()));
        }

        if self.content[0].is_alphabetic() {
            return Some(self.chop_while(|c| c.is_alphabetic()));
        }

        Some(self.chop(1))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a [char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

fn read_entire_xml_file(filepath: &str) -> io::Result<String> {
    let file = File::open(filepath)?;

    let er = EventReader::new(file);

    let mut content = String::new();

    for event in er.into_iter() {
        if let Ok(XmlEvent::Characters(text)) = event {
            content.push_str(&text);
            content.push(' ');
        }
    }

    Ok(content)
}

fn main() -> Result<()> {
    let content: Vec<char> = read_entire_xml_file("docs.gl/gl4/glClear.xhtml")?
        .chars()
        .collect();
    let lexer = Lexer::new(&content);
    for token in lexer {
        println!("{}", token.iter().collect::<String>())
    }
    Ok(())
}
