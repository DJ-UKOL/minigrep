use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,      // Строка, которую мы ищем
    pub file_path: String,  // Путь к файлу
    pub ignore_case: bool,  // учитывать регистр или нет
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
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case, })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {      // Box<dyn Error> функция будет возвращать тип реализующий типаж Error, ошибки могут быть разных типов в разных случаях. dyn - динамический
    let contents = fs::read_to_string(config.file_path)?;    // открывает файл и возвращает содержимое файла. ?-проброс ошибки
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
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

// Функция поиска без учета регистра
fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();   // преобразовываем строку в нижний регистр
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
save, fast, productive.
Pick three.
Duct tape.";       // не помещать символ новой строки в начало содержимого этого строкового литерала

        assert_eq!(vec!["save, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}