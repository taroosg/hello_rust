fn main() {
  // mutを入れると再代入できる
  // let mut x = 5;
  // println!("The value of x is: {}", x); // xの値は{}です
  // x = 6;
  // println!("The value of x is: {}", x);

  let x = 5;
  // これは再代入ではなくシャドーイングと呼ぶ（更新に近い）
  let x = x + 1;
  let x = x * 2;

  println!("The value of x is: {}", x);

  // 型が異なるのでmutを使った再代入はできない
  let spaces = "   ";
  let spaces = spaces.len();
  println!("The number of space is: {}", spaces);
}
