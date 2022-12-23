use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Data{
    count: i32,
    value: Vec<String>,
}

pub fn grep(
    file_name: &str,
    search_word: String,
) -> Data
{
    let mut file = File::open(file_name).expect("file not found");
    let mut text = String::new();
    let mut data = Data{
        count: 0,
        value: vec![],
    };
    file.read_to_string(&mut text).expect("I can't read the file");
    for line in text.split_terminator(&['.', '!']) {
        for line_iter in line.split_whitespace() {
            if line_iter.trim().to_lowercase() == search_word.trim().to_lowercase() {
                data.value.push(line.trim().to_string() + ".");
                data.count +=1;
                break;
            }
        }
    }
    data
}