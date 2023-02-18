use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main()-> std::io::Result<()> {

    let input:Vec<String> = args().skip(1).collect();
    let mut total_lines = 0;
    let mut total_words = 0;
    for arg in input{
        let f = File::open(&arg)?;
        let reader = BufReader::new(f);
        let mut word_count = 0;
        let mut line_count = 0;
        for line in reader.lines(){
            match line {
                Ok(l) => {
                    word_count += l.split_whitespace().count();
                    line_count += 1;
                }
                _ => {}
            }
        }

        println!("{} {} {}", arg, word_count, line_count);
        total_words += word_count;
        total_lines += line_count;
    }

    if args().len() > 2 {
        println!("Total:{} {} {}", args().len()-1, total_words, total_lines);
    }

    Ok(())
}
