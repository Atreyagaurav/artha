use std::io::Read;
use std::io::Write;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;
use timeout_readwrite::TimeoutReader;

pub struct SpellChecker {
    aspell: Child,
}

impl SpellChecker {
    pub fn new() -> Option<Self> {
        let aspell = match Command::new("aspell")
            .args(["-l", "ne", "munch"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Error loading aspell: {e}");
                return None;
            }
        };

        Some(SpellChecker { aspell })
    }

    /// Reads the output from ispell
    fn read_str(&mut self) -> Option<String> {
        let mut res = String::new();
        _ = TimeoutReader::new(self.aspell.stdout.as_mut()?, Duration::from_millis(200))
            .read_to_string(&mut res);
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    /// Write to aspell stdin
    fn write_str(&mut self, text: &str) -> Result<(), std::io::Error> {
        // First, clear ispell's stdout just in case
        _ = self.read_str();
        if let Some(stdin) = self.aspell.stdin.as_mut() {
            stdin.write_all(text.as_bytes())?;
            stdin.write_all(b"\n")?;
            stdin.flush()?;
        }
        Ok(())
    }

    pub fn get_root(&mut self, word: &str) -> Option<String> {
        // eprintln!("Getting Root: {word}");
        self.write_str(word).ok()?;

        // eprintln!("Wrote to aspell: {word}");
        // thread::sleep(Duration::new(1, 0));
        let out = self.read_str()?;

        // eprintln!("Aspell Output: {out}");
        out.split(' ')
            .nth(1)?
            .split('/')
            .next()
            .map(ToString::to_string)
    }
}
