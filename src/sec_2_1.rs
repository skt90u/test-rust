use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn guess_number() {

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("你生成的數字是: {}", secret_number);

    loop {
        let mut guess = String::new();

        println!("請猜測一個數");
        io::stdin().read_line(& mut guess).expect("無法讀取行");

        println!("你猜測的數字是: {}", guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}