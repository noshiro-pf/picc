pub struct Lexer {
  input: Vec<char>,
  position: usize,
}

impl Lexer {
  pub fn new(input: Vec<char>) -> Lexer {
    Lexer {
      input: input,
      position: 0,
    }
  }

  fn current(&self) -> Option<&char> {
    self.input.get(self.position)
  }

  fn pos(&self) -> usize {
    self.position
  }

  fn peek(&self) -> Option<&char> {
    self.input.get(self.position + 1)
  }

  fn next(&mut self) {
    self.position += 1;
  }
}
