use rand::Rng;

fn main() {
    println!("猜数字！");
    let hint_message = "请输入 1 至 100 间的整数（包含边界），输入 0 退出游戏。";

    loop {
        let secret_number = rand::thread_rng().gen_range(1..=100);

        println!("{hint_message}");

        loop {
            let mut guess = String::new();

            if let Err(_) = std::io::stdin().read_line(&mut guess) {
                println!("读取失败！");
                return;
            }

            match guess.trim().parse::<u32>() {
                Ok(0) => return,
                Ok(guess) if guess >= 1 && guess <= 100 => match guess.cmp(&secret_number) {
                    std::cmp::Ordering::Less => println!("太小了！"),
                    std::cmp::Ordering::Greater => println!("太大了！"),
                    std::cmp::Ordering::Equal => {
                        println!("你赢了！");
                        break;
                    }
                },
                _ => {
                    println!("{hint_message}");
                    continue;
                }
            }
        }
    }
}
