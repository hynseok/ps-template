use std::io::{ Write, BufRead };

struct Stdin {
  stdin: std::io::BufReader<std::io::StdinLock<'static>>,
}

impl Stdin {
  fn new() -> Self {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let stdin = std::io::BufReader::new(stdin);
    
    Stdin { stdin }
  }

  fn read_line(&mut self) -> String {
    let mut input = String::new();
    self.stdin.read_line(&mut input).unwrap();

    input
  }
}

struct Stdout {
  stdout: std::io::BufWriter<std::io::StdoutLock<'static>>
}

impl Stdout {
  fn new() -> Self {
    let stdout = std::io::stdout();
    let stdout = stdout.lock();
    let stdout = std::io::BufWriter::new(stdout);

    Stdout { stdout }
  }

  fn writeln(&mut self, str: String) {
    writeln!(self.stdout, "{}", str).unwrap();
  }
}

fn main() {
    let mut stdin = Stdin::new();
    let mut stdout = Stdout::new();

    let line = stdin.read_line();

    stdout.writeln(format!("Your Input is \"{}\"", line.trim().to_string()));
}