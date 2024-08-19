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

    for line in search(&config.query, &contents) {
        println!("{line}");
    }
    Ok(())
}

// Принимает запрос и текст для поиска, а возвращает только те строки из текста,
// которые содержат запрос
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { //время жизни назначем content, т.к. он содержить текст и мы возвращаем часть текста
    let mut results = Vec::new();
    for line in contents.lines() {  // Метод lines возвращает итератор
        if line.contains(query) {   // проверяем содержит ли текущая строка нашу искомую строку
            results.push(line);     // сохранение совпавшей строки
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // принимает запрос и текст для поиска, а возвращает только те строки из текста,
    // которые содержат запрос
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.";       // не помещать символ новой строки в начало содержимого этого строкового литерала


        assert_eq!(vec!["save, fast, productive."], search(query, contents));
    }
}