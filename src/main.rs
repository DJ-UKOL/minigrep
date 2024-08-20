use std::{env, process};
use minigrep::Config;

fn main() {
    // Позволяем программе читать любые переданные ей аргументы командной строки,
    let args: Vec<String> = env::args().collect();  // а затем собирать значения в вектор.
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);           // немедленно остановит программу и вернёт номер,
                                         // который был передан в качестве кода состояния выхода.
    });

    if let Err(e) = minigrep::run(config) {   // if let используется, чтобы проверить возвращает ли run значение Err
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}