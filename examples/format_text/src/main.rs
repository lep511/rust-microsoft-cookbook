use std::fmt::Write as FmtWrite;
use std::fmt;

#[derive(Debug)]
enum Place {
    Start,
    Middle,
    End,
}

pub struct Writer<'a> {
    writer: &'a mut dyn fmt::Write,
    count: usize,
    place: Place,
}

impl<'a> Writer<'a> {
    pub fn new(writer: &'a mut dyn fmt::Write, count: usize) -> Self {
        Writer {
            writer,
            count,
            place: Place::Start,
        }
    }

    pub fn continuation(&mut self) -> &mut Self {
        self.place = Place::Middle;
        self
    }
}

impl fmt::Write for Writer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let lines: Vec<Option<&str>> = s.split('\n')
            .map(Some)
            .collect::<Vec<_>>()
            .into_iter()
            .enumerate()
            .flat_map(|(i, line)| {
                if i == 0 { 
                    vec![line] 
                } else { 
                    vec![None, line]
                }
            })
            .collect();

        for line in lines {
            match self.place {
                Place::Start => {
                    write!(self.writer, "{:>count$}", "", count = self.count)?;
                    self.place = Place::Middle;
                }
                Place::End => {
                    writeln!(self.writer)?;
                    self.place = Place::Start;
                }
                Place::Middle => {} // Do nothing for middle lines
            }

            match line {
                None => self.place = Place::End,
                Some(line) => {
                    write!(self.writer, "{}", line)?;
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), std::fmt::Error> {
    let mut output = String::new(); // Our output buffer
    { // Limit the scope of the mutable borrow of output
        let mut writer = Writer::new(&mut output, 4); // 4-space indentation

        writer.write_str("This is the first line.\n")?;
        writer.continuation().write_str("This is a continuation.\n")?;
        writer.write_str("This is another line in the same block.\n")?;
        writer.write_str("This is a new block.\n")?;
        writer.write_str("Another line in the new block.\n")?;
    }

    println!("{}", output);

    Ok(())
}