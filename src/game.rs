use crossterm::style::Stylize;
use guess_game::{ flush_stdout, read_string_to_int, GameError };
use indicatif::{ProgressBar, ProgressStyle};
use std::cmp::Ordering;


pub struct Game {
    min: i32,
    max: i32,
    goal: i32,
}

impl Game {
    pub fn new(min: i32, max: i32) -> Self {
        let goal = rand::random_range(min..=max);
        Self { min, max, goal }
    }

    pub fn run(&self) -> Result<(), GameError> {

        let pb = ProgressBar::new(10);

        pb.set_style(
            ProgressStyle::with_template("{bar:40.cyan/blue} {pos}/{len} 次机会")
                .unwrap()
                .progress_chars("█░"),
        );

        println!("我已经生成了 {} - {} 之间的随机数字，等你来猜！", self.min, self.max);
        let mut num = 0;


        loop {
            println!("请输入数字：");

            num += 1;
            pb.inc(1);
            flush_stdout();

            let guess = read_string_to_int();
            if let Ok(value) = guess {
                if (value - self.goal).abs() < 3 && value != self.goal {
                    println!("接近了");
                }

                match value.cmp(&self.goal) {
                    Ordering::Greater => println!("太大了"),
                    Ordering::Less => println!("太小了"),
                    Ordering::Equal => {
                        println!("猜对了");
                        println!("你用了 {num} 次猜对了！");
                        pb.finish_and_clear();
                        break;
                    }
                }

                if num >= 10 {
                    println!("{}", format!("💀 机会用完了，答案是 {}", &self.goal).to_string().red());
                    pb.finish_and_clear();
                    break;
                }
            } else {
                println!("⚠️ 请输入一个有效的整数！");
            }
        }

        Ok(())
    }
}