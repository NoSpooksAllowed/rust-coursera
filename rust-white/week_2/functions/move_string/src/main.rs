fn main() {
    let mut source = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let mut destination = vec!["z".to_string()];

    move_strings(&mut source, &mut destination);

    println!("destination: {:?}", destination);
    println!("source: {:?}", source);
}

fn move_strings(source: &mut Vec<String>, destination: &mut Vec<String>) {
    destination.extend(source.drain(..).collect::<Vec<String>>());
}
