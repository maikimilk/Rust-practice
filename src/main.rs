use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("数当てゲーム");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("シークレットナンバーはこれ: {}", secret_number);

    println!("どの数だとおもう？ = ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("読み込み失敗");

        let guess:u32 = guess
        .trim()
        .parse()
        .expect("数を入力してください。");

    println!("入力値: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("小さすぎです。"),
        Ordering::Greater => println!("大きすぎです。"),
        Ordering::Equal => println!("正解です。")
    }
}
