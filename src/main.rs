mod filefinder;

use std::time::Instant;
use std::{
    env,
    io::{stdout, BufWriter, Write},
    path::PathBuf,
};

use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, Editor};

const DISPLAY_LIMIT: usize = 100;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_readline = DefaultEditor::new()?;
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All))?;
    println!("Fsleu -File Finder-");

    let mut app = FileFinder {
        stack_vec: vec![],
        extention: String::new(),
        searchword: String::new(),
    };

    app.extention = app.set_extension(&mut my_readline)?;
    app.manage_token(&mut my_readline)?;

    Ok(())
}

struct FileFinder {
    stack_vec: Vec<PathBuf>,
    extention: String,
    searchword: String,
}

impl FileFinder {
    fn search(&self) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
        let cur_path = env::current_dir()?;
        filefinder::item_search(&cur_path, &self.extention, &self.searchword)
    }

    fn display(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_buffer = BufWriter::new(stdout());
        // 表示する項目数の制限
        let limitnum = self.stack_vec.len().min(DISPLAY_LIMIT);
        writeln!(output_buffer, "--------\nResults\n--------")?;

        for it in self.stack_vec[..limitnum].iter() {
            writeln!(output_buffer, "{}", it.display())?;
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
        &self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        loop {
            let rline = my_readline.readline("extension >>")?;

            if !rline.is_empty() {
                // self.extention = rline;
                println!("Extention:{}", self.extention);
                return Ok(rline);
            }
            println!("please input search word... '*' is wildcard.");
        }
    }

    fn open_file(
        &self,
        my_readline: &mut Editor<(), FileHistory>,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut output_buffer = BufWriter::new(stdout());

        loop {
            writeln!(
                output_buffer,
                "\n\n@ext << change extention, @open << open file, @q or @quit << exit"
            )?;

            output_buffer.flush()?;

            let readline = my_readline.readline(format!("ext:{} =>>", self.extention).as_str())?;

            let in_token = readline.trim();

            output_buffer.execute(Clear(ClearType::All))?;

            match in_token {
                "@quit" | "@q" => break,

                "@ext" | "@e" | "@ex" => {
                    self.extention = self.set_extension(my_readline)?;
                }
                "@open" | "@o" | "@op" => {
                    self.open_file(my_readline)?;
                }
                _ => {
                    let searchword = in_token.to_string();
                    self.searchword = searchword;
                    let start = Instant::now();
                    self.stack_vec = self.search()?;
                    self.display()?;
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
