#[derive(Debug, Clone)]
enum Mode {
  Repl,
  File(String)
}

#[derive(Debug)]
pub struct Config {
  mode: Mode
}

impl Config {
  pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
    args.next();

    let arg = match args.next() {
      Some(arg) => arg,
      None => return Err("Invalid arguments")
    };

    let mode = if arg.to_lowercase() == "repl" {
      Mode::Repl
    } else {
      Mode::File(arg)
    };

    Ok(Self { mode })
  }

  fn mode(&self) -> Mode {
    self.mode.clone()
  }

}