mod libs;

fn main() {
    let search_word = "современные".to_string();
    let file_name = "file.txt";
    let vector = libs::grep(file_name, search_word);
    println!("{:#?}", vector);
}
