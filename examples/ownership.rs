use std::io::Result;

fn case1() -> Result<()> {
  let a = 42; // aが42の所有権を獲得
  {
    let b = a + 10; // bが52の所有権を獲得
    println!("{}", b);
    // bのメモリが破棄
  }
  // aのメモリが破棄
  Ok(())
}

fn case2() -> Result<()> {
  let a = 42;
  let b = a + 10;
  println!("{}", a);
  println!("{}", b);
  Ok(())
}

fn case3() -> Result<()> {
  // 不変参照。参照先の値を変えられない。
  let x = 42;
  let y = &x; // 変数xに対する不変参照。yは不変参照型をもつ不変変数。
  let mut z = &x; // 変数xに対する不変参照。zは不変参照型をもつ可変変数。

  // 可変参照。参照先の値を変えられる。
  let mut a = 42;
  let b = &mut a;
  let mut c = &mut a;
  Ok(())
}

fn case4() -> Result<()> {
  // Borrowing rules(借用規則)
  // 不変参照は同一の変数に対して複数定義できる。
  let a = 42;
  let b = &a;
  let c = &a;
  // 可変参照は同一の変数に対して同時に複数の定義はできない。
  let mut x = 42;
  let mut y = &mut x;
  let mut z = &mut x;
  println!("{}", x);


  Ok(())
}

fn main() -> Result<()> {
  case2();
  Ok(())
}