use std::io::Cursor;
use std::fs;
use anyhow::Context;
use cfn_guard::{
    commands::{
        validate::{OutputFormatType, ShowSummaryType},
        Executable,
    },
    utils::{
        reader::{ReadBuffer, Reader},
        writer::{WriteBuffer, Writer},
    },
    CommandBuilder, ValidateBuilder,
};

#[derive(Debug)]
pub struct Payload {
    pub data: String,
    pub rules: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    // Specify the file path
    let files = vec!["event1.json", "event2.json"];

    for file_path in files {

        // Read the file content into a String
        let payload = fs::read_to_string(file_path)?;    
        let mut reader = Reader::new(ReadBuffer::Cursor(Cursor::new(Vec::from(
            payload.as_bytes(),
        ))));
        let mut writer = Writer::new_with_err(WriteBuffer::Vec(vec![]), WriteBuffer::Vec(vec![]));
    
        let cmd = ValidateBuilder::default()
            .payload(true)
            .output_format(OutputFormatType::JSON)
            .structured(true)
            .show_summary(vec![ShowSummaryType::None])
            .try_build()
            .context("failed to build validate command")?;
    
        cmd.execute(&mut writer, &mut reader)?;
    
        let content = writer.stripped().context("failed to read from writer")?;
        println!("File: {}\n{}", file_path, content);
        println!("\n---------------------------------------------------------------------\n");
    }

    Ok(())
}