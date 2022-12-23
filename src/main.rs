use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Data{
    count: i32,
    value: Vec<String>,
}

fn grep(
    mut file: File,
    search_word: String,
) -> Data
{
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

fn main() {
    let search_word = "современные".to_string();
    let file = File::open("file.txt").expect("don't open =(");
    let vector = grep(file, search_word);
    println!("{:#?}", vector);
}
