fn main() {
  println!("Hello, word!");
}

// 

use std::io;

fn main(){
  println!("숫자를 추측해보겠습니다");
  println!("숫자를 입력해보세요");
  let mut guess = String::new();
  io::stdin().read_line(&mut guess)
      .expect("Failed to read line");
  println!("당신의 숫자는: {} 입니다.", guess);
}

// 

fn main() {
  println!("Hello, world!");
}
