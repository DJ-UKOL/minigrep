use std::{env, fs};

fn main() {
    // Позволяем программе читать любые переданные ей аргументы командной строки,
    let args: Vec<String> = env::args().collect();  // а затем собирать значения в вектор.

    // &args[0] - имя программы ("target/debug/minigrep")
    let query = &args[1];       // Строка, которую мы ищем
    let file_path = &args[2];   // Путь к файлу

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path)    // открывает файл и возвращает содержимое файла
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}