fn main() {
  // mutを入れると再代入できる
  // let mut x = 5;
  // println!("The value of x is: {}", x); // xの値は{}です
  // x = 6;
  // println!("The value of x is: {}", x);

  // let x = 5;
  // // これは再代入ではなくシャドーイングと呼ぶ（更新に近い）
  // let x = x + 1;
  // let x = x * 2;

  // println!("The value of x is: {}", x);

  // 型が異なるのでmutを使った再代入はできない
  // let spaces = "   ";
  // let spaces = spaces.len();
  // println!("The number of space is: {}", spaces);

  // 静的型付言語なので，コンパイル時に型が判明していないといけない
  // : u32が必要
  // let guess: u32 = "42".parse().expect("Not a number!"); // 数字ではありません！
  // println!("The number is: {}", guess);

  // // 足し算
  // let sum: i32 = 5 + 10;

  // // 引き算
  // let difference: f64 = 95.5 - 4.3;

  // // 掛け算
  // let product: i32 = 4 * 30;

  // // 割り算
  // let quotient: f64 = 56.7 / 32.2;

  // // 余り
  // let remainder: i32 = 43 % 5;

  // println!("The sum is: {}", sum);
  // println!("The difference is: {}", difference);
  // println!("The product is: {}", product);
  // println!("The quotient is: {}", quotient);
  // println!("The remainder is: {}", remainder);

  // タプル
  // let tup: (i32, f64, u8) = (500, 6.4, 1);
  // let (x, y, z) = tup;

  // 分割代入とインデックスでとれる
  // println!("The tup[0] is: {}", x);
  // println!("The tup[1] is: {}", tup.2);

  // // 配列は固定長
  // let months = [
  //   "January",
  //   "February",
  //   "March",
  //   "April",
  //   "May",
  //   "June",
  //   "July",
  //   "August",
  //   "September",
  //   "October",
  //   "November",
  //   "December",
  // ];
  // // 繰り返し
  // for elm in &months {
  //   println!("The month is: {}", elm);
  // }

  // // 関数呼出し
  // let hoge = another_function(10);
  // println!("number is {}", hoge);
  // let number = 6;

  // let result = if number % 4 == 0 {
  //   // 数値は4で割り切れます
  //   "number is divisible by 4"
  // } else if number % 3 == 0 {
  //   // 数値は3で割り切れます
  //   "number is divisible by 3"
  // } else if number % 2 == 0 {
  //   // 数値は2で割り切れます
  //   "number is divisible by 2"
  // } else {
  //   // 数値は4、3、2で割り切れません
  //   "number is not divisible by 4, 3, or 2"
  // };
  // println!("result is {}", result);
}

// // 外部関数定義
// fn another_function(x: i32) -> i32 {
//   x * 2
// }
