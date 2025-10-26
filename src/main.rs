use guess_game::{flush_stdout, read_input_string};
use crossterm::style::{Stylize};
use game::Game;
mod game;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Guess Game",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()) as Box<dyn eframe::App>)),
    )
}

/* fn main() {
    println!("{}", "🎮 欢迎来到猜数字游戏".green().bold());

    loop {
        if let Err(err) = Game::new(0, 100).run() {
            eprintln!("{err}");
        }

        if !go_on() {
            println!("{}", "🎮 游戏结束，欢迎下次再来玩！".cyan());
            break;
        }
    }
} */

fn go_on() -> bool {
    loop {
        println!("是否继续？(y/n): ");
        flush_stdout();

        match read_input_string().unwrap_or_default().as_str() {
            "y" | "Y" => return true,
            "n" | "N" => return false,
            _ => println!("只能输入 【y/n】"),
        }
    }
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 18,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("请输入正整数");
                ui.text_edit_singleline(&mut self.name);
            });

            if ui.button("Increment").clicked() {
                self.age += 1;
            }

        });
    }
}