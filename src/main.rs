use std::io;
use rand::Rng;

fn main() {
    println!("数当てゲーム");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("シークレットナンバーはこれ: {}", secret_number);

    println!("どの数だとおもう？ = ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込み失敗");

    println!("入力値: {}", guess);
}
