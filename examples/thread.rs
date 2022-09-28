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

struct User {
  id: usize,
  name: String,
  num: i32,
}

impl User {
  fn new(id: usize, name: String) -> Self {
    return Self{id: id, name: name, num: 0};
  }

  fn read(&self, server: &Server, idx: usize) -> i32 {
    return server.data[idx];
  }
  fn write(&self, server: &mut Server, idx: usize, val: i32) -> () {
    server.data[idx] = val;
  }
}

struct Server {
  id: usize,
  data: Vec<i32>
}

impl Server {
  fn new(id: usize, n: usize) -> Self {
    let data = vec![0; n];
    return Self{id: id, data: data};
  }
}

use std::sync::{Mutex, Arc};
// Arc: atomically reference counted
// Arcはデータ共有用の型
// Mutexは共有データ編集のための型 
// lockをかけながら処理

fn case3() -> Result<()> {
  let SLEEP_TIME = 1;
  let user1 = User::new(1, "aoki".to_string());
  let user2 = User::new(2, "suzuki".to_string());
  let _server = Server::new(0, 10);
  let server = Arc::new(Mutex::new(_server));
  let mut handles = vec![];

  for user in [user1, user2] {
    let server = Arc::clone(&server);
    let handle = thread::spawn(move || {
      for i in 0..20 {
        let uread = user.read(&server.lock().unwrap(), i%10);
        println!("User{} access the data [{}] located in {}", user.id, uread, i%10);
        thread::sleep(Duration::from_millis(SLEEP_TIME));
        user.write(&mut *server.lock().unwrap(), i%10, uread + 1);
        println!("User{} access the data and update data to [{}] located in {}", user.id, uread, i%10);
        thread::sleep(Duration::from_millis(SLEEP_TIME));
      }
    }); 
    handles.push(handle);
  }
  for handle in handles {
    handle.join().unwrap();
  }

  println!("Server Last State: {:?}", server.lock().unwrap().data);
  Ok(())
}

fn main() -> Result<()> {
  // case1();
  // case2();
  case3();
  Ok(())
}