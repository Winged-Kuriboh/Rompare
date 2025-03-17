use clap::{Arg, Command};
use itertools::Itertools;
use itertools::EitherOrBoth::{Both, Left, Right};
use colored::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let args = Command::new("compare")
        .arg_required_else_help(true)
        .arg(Arg::new("file1").required(true).index(1))
        .arg(Arg::new("file2").required(true).index(2))
        .get_matches();

    let file1_path = args.get_one::<String>("file1").unwrap();
    let file2_path = args.get_one::<String>("file2").unwrap();


    let file1 = File::open(file1_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Error opening file1 '{}': {}", file1_path, e)))?;

    let file2 = File::open(file2_path)
        .map_err(|e| io::Error::new(e.kind(), format!("Error opening file2 '{}': {}", file2_path, e)))?;

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    compare_and_display(reader1.lines(), reader2.lines(), &mut writer)?;

    Ok(())
}

fn compare_and_display<B1, B2, W>(
    lines1: B1,
    lines2: B2,
    writer: &mut W,
) -> io::Result<()>
where
    B1: Iterator<Item = io::Result<String>>,
    B2: Iterator<Item = io::Result<String>>,
    W: Write,
{
    writeln!(
        writer,
        "{:<10} {:<40} {:<40}",
        "Line".bold(),
        "File 1".bold(),
        "File 2".bold()
    )?;
    writeln!(writer, "{}", "-".repeat(90))?;

    for (line_number, pair) in lines1.zip_longest(lines2).enumerate() {
        match pair {
            Both(Ok(line1), Ok(line2)) if line1 == line2 => {
                writeln!(writer, "{:<10} {:<40} {:<40}", line_number + 1, line1, line2)?;
            }
            Both(Ok(line1), Ok(line2)) => {
                writeln!(
                    writer,
                    "{:<10} {:<40} {:<40}",
                    line_number + 1,
                    line1.green(),
                    line2.red()
                )?;
            }
            Left(Ok(line1)) => {
                writeln!(
                    writer,
                    "{:<10} {:<40} {:<40}",
                    line_number + 1,
                    line1.green(),
                    ""
                )?;
            }
            Right(Ok(line2)) => {
                writeln!(
                    writer,
                    "{:<10} {:<40} {:<40}",
                    line_number + 1,
                    "",
                    line2.red()
                )?;
            }

            Left(Err(e)) => {
                writeln!(writer, "fail reading File 1 at line {}: {}", line_number + 1, e)?;
            }
            Right(Err(e)) => {
                writeln!(writer, "fail reading File 2 at line {}: {}", line_number + 1, e)?;
            }
            _ => writeln!(writer, "Unknown error at line {}", line_number + 1)?,
        }
    }

    writer.flush()?;
    Ok(())
}