mod filefinder;
use std::error::Error;

use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, Editor};
use std::time::Instant;
use std::{
    io::{stdout, BufWriter, Write},
    path::PathBuf,
};

const DISPLAY_LIMIT: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let mut my_readline = DefaultEditor::new()?;

    println!("Fsleu -File Finder-");

    let mut app = FileFinder {
        stack_vec: vec![],
        extention: String::new(),
        searchword: String::new(),
        deselection: String::new(),
    };

    app.set_extension(&mut my_readline)?;
    app.manage_token(&mut my_readline)?;

    Ok(())
}

struct FileFinder {
    stack_vec: Vec<PathBuf>,
    extention: String,
    searchword: String,
    deselection: String,
}

impl FileFinder {
    fn search(&mut self) -> Result<(), Box<dyn Error>> {
        let mut output_buffer = BufWriter::new(stdout());
        self.stack_vec =
            filefinder::item_search(".", &self.extention, &self.searchword, &self.deselection)?;

        // 表示する項目数の制限
        let limitnum = self.stack_vec.len().min(DISPLAY_LIMIT);
        writeln!(output_buffer, "--------\nResults\n--------")?;

        for it in self.stack_vec[..limitnum].iter() {
            writeln!(output_buffer, "{}", it.to_string_lossy())?;
        }

        writeln!(
            output_buffer,
            "extention: {},word: {} => {} hits",
            self.extention,
            self.searchword,
            self.stack_vec.len()
        )?;

        output_buffer.flush()?;
        Ok(())
    }

    fn set_extension(
        &mut self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let rline = my_readline.readline("extension >>")?;
            if !rline.is_empty() {
                self.extention = rline;
                println!("Extention:{}", self.extention);
                break;
            } else {
                println!("please input search word... '*' is wildcard.");
            }
        }
        Ok(())
    }

    fn set_deselection(
        &mut self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<(), Box<dyn Error>> {
        loop {
            let rline = my_readline.readline("deselection >>")?;

            if !rline.is_empty() {
                self.deselection = rline;
                println!("Deselection:{}", self.deselection);
                break;
            } else {
                println!("please input deselection word...");
            }
        }
        Ok(())
    }

    fn open_file(
        &mut self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<(), Box<dyn Error>> {
        let limitnum = 100;

        loop {
            let mut output_buffer = BufWriter::new(stdout());
            for (i, path) in self.stack_vec.iter().enumerate() {
                if i >= limitnum {
                    writeln!(
                        output_buffer,
                        "There are more than {limitnum} applicable items"
                    )?;
                    break;
                }

                writeln!(output_buffer, "{}:{:?}", i, path)?;
            }

            output_buffer.flush()?;

            match my_readline.readline("select number >>") {
                Ok(rline) => match rline.as_str() {
                    "q" | "quit" | "@q" | "@quit" => {
                        break;
                    }

                    _ => match rline.parse::<usize>() {
                        Ok(n) if n < self.stack_vec.len() => {
                            filefinder::opendir(&self.stack_vec[n])?;
                        }
                        Ok(_) => {
                            println!("Wrong number.");
                        }

                        Err(e) => println!("{e}"),
                    },
                },

                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }

    fn manage_token(
        &mut self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<(), Box<dyn Error>> {
        let mut output_buffer = BufWriter::new(stdout());

        loop {
            writeln!(
                output_buffer,
                "\n\n@ext << change extention, @open << open file, @dsele << deselection, 
                @dsclear << deselection clear, @q or @quit << exit"
            )?;

            if !self.deselection.is_empty() {
                writeln!(output_buffer, "\ndeselection word:{}", self.deselection)?;
            }

            output_buffer.flush()?;

            let readline = my_readline.readline(format!("ext:{} =>>", self.extention).as_str())?;

            let in_token = readline.trim();

            match in_token {
                "@quit" | "@q" => break,

                "@ext" | "@e" | "@ex" => {
                    self.deselection.clear();
                    self.set_extension(my_readline)?;
                }
                "@open" | "@o" | "@op" => {
                    self.open_file(my_readline)?;
                }
                "@dsele" => {
                    self.set_deselection(my_readline)?;
                }
                "@dsclear" => self.deselection.clear(),
                _ => {
                    let searchword = in_token.to_string();
                    self.searchword = searchword;
                    let start = Instant::now();
                    self.search()?;
                    let end = Instant::now();
                    println!("------------------------------------");
                    println!(
                        "Search process took {} milliseconds",
                        end.duration_since(start).as_millis()
                    );
                }
            }
        }
        Ok(())
    }
}
