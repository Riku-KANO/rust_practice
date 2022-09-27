use std::thread;
use std::time::Duration;
use std::io::Result;


fn case1() -> Result<()> {
  // thread::spawnでスレッドの生成
  // thread::sleepの間にスレッドの実行を中止し別のスレッドの実行を始める。
  // thread::spawnの引数はクロージャ
  thread::spawn(|| {
    for i in 1..10 {
      println!("number {} from the spawened thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("numer {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  Ok(())
}

fn case2() -> Result<()> {
  // 途中で同期させる
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("number {} from the spawened thread!", i);
      thread::sleep(Duration::from_millis(1));
    }
  });
  for i in 1..5 {
    println!("number {} from the main thread!", i);
    thread::sleep(Duration::from_millis(1));
  }
  handle.join().unwrap();
  Ok(())
}

fn case3() {
  todo!();
}

fn main() -> Result<()> {
  // case1();
  case2();
  Ok(())
}