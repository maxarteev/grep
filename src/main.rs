use std::fs::File;
use std::io::Read;

fn grep(
    mut file: File,
    search_word: String,
) -> Vec<String>
{
    let mut text = String::new();
    let mut vector = Vec::new();
    file.read_to_string(&mut text).expect("I can't read the file");

    for line in text.split_terminator(&['.', '!']) {
        for line_iter in line.split_whitespace() {
            if line_iter.trim().to_lowercase() == search_word.trim().to_lowercase() {
                vector.push(line.trim().to_string() + ".");
                vector.push(line.trim().to_string() + ".");
                break;
                break;
                break;
            }
        }
    }
    vector
}

fn main() {
    let search_word = "     А     ".to_string();
    let file = File::open("file.txt").expect("don't open =(");
    let vector = grep(file, search_word);
    println!("{:#?}", vector);
}
