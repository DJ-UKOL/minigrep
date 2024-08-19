use std::{env, fs, process};
use std::error::Error;

fn main() {
    // Позволяем программе читать любые переданные ей аргументы командной строки,
    let args: Vec<String> = env::args().collect();  // а затем собирать значения в вектор.
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);           // немедленно остановит программу и вернёт номер,
                                         // который был передан в качестве кода состояния выхода.
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {   // if let используется, чтобы проверить возвращает ли run значение Err
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {      // Box<dyn Error> функция будет возвращать тип реализующий типаж Error, ошибки могут быть разных типов в разных случаях. dyn - динамический
    let contents = fs::read_to_string(config.file_path)?;    // открывает файл и возвращает содержимое файла. ?-проброс ошибки
    println!("With text:\n{contents}");
    Ok(())
}

struct Config {
    query: String,      // Строка, которую мы ищем
    file_path: String,  // Путь к файлу
}

impl Config {

    // Конструктор для структуры Config
    fn build(args: &[String]) -> Result<Config, &'static str> {     // 'static - статическое время жизни которое равно времени выполнения программы
        // нужно проверять, что срез достаточно длинный, перед попыткой доступа по индексам 1 и 2
        if args.len() < 3 {
            return Err("not enough argument");  // Если меньше 3, то возвращаем ошибку
        }
        // &args[0] - имя программы ("target/debug/minigrep")
        let query = args[1].clone();       // .clone() делает полную копию данных для экземпляра Config для владения
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}