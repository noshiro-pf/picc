use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    panic!("引数の個数が正しくありません\n");
  }

  let value_str = &args[1];
  // let value_num = value_str.parse::<i32>().unwrap();

  let chars = value_str.chars().collect();

  // printf("  mov rax, %ld\n", strtol(p, &p, 10));
  // println!("input: {}", value_str);
  println!(".intel_syntax noprefix");
  println!(".globl main");
  println!("main:");
  println!("  mov rax, {}", value_num);
  println!("  ret");
}
