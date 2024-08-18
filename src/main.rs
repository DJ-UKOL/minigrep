use std::{env, fs};

fn main() {
    // Позволяем программе читать любые переданные ей аргументы командной строки,
    let args: Vec<String> = env::args().collect();  // а затем собирать значения в вектор.
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)    // открывает файл и возвращает содержимое файла
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,      // Строка, которую мы ищем
    file_path: String,  // Путь к файлу
}

impl Config {

    // Конструктор для структуры Config
    fn new(args: &[String]) -> Config {
        // &args[0] - имя программы ("target/debug/minigrep")
        let query = args[1].clone();       // .clone() делает полную копию данных для экземпляра Config для владения
        let file_path = args[2].clone();

        Config { query, file_path }      // возвращаем структуру
    }
}