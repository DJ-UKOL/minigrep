use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,      // Строка, которую мы ищем
    pub file_path: String,  // Путь к файлу
}

impl Config {

    // Конструктор для структуры Config
    pub fn build(args: &[String]) -> Result<Config, &'static str> {     // 'static - статическое время жизни которое равно времени выполнения программы
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {      // Box<dyn Error> функция будет возвращать тип реализующий типаж Error, ошибки могут быть разных типов в разных случаях. dyn - динамический
    let contents = fs::read_to_string(config.file_path)?;    // открывает файл и возвращает содержимое файла. ?-проброс ошибки
    println!("With text:\n{contents}");
    Ok(())
}