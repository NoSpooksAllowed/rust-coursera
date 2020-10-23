extern crate argparse;

#[macro_use]
extern crate json;

use argparse::{ArgumentParser, Store};
use std::env;
use std::fs;
use std::io::Write;
use std::path;

struct ArgumentsValues {
    key: String,
    value: String,
}

fn main() {
    let mut storage_path = env::temp_dir();
    storage_path.push("storage.data");

    let args = parse();

    if !args.key.is_empty() && !args.value.is_empty() {
        put(&storage_path, &args);
    } else if !args.key.is_empty() {
        let data = get(&storage_path, &args.key);
        if data.len() == 0 {
            println!("None");
        } else {
            for i in 0..data.len() {
                print!("{}", data[i]);

                if i < data.len() - 1 {
                    print!(", ");
                }
            }

            println!();
        }
    } else {
        println!("usage ./storage --key [key_name] --val [value]");
    }
}

fn parse() -> ArgumentsValues {
    let mut args = ArgumentsValues {
        key: String::from(""),
        value: String::from(""),
    };

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut args.key)
            .add_option(&["--key"], Store, "you have to enter a key value");

        ap.refer(&mut args.value)
            .add_option(&["--val"], Store, "you have to ener a key value");
        ap.parse_args_or_exit();
    }

    args
}

fn read_data(storage_path: &path::PathBuf) -> json::JsonValue {
    if !storage_path.exists() {
        return json::JsonValue::new_object();
    }

    let raw_data = fs::read_to_string(storage_path).expect("something went wrong reading the file");

    if !raw_data.is_empty() {
        return json::parse(&raw_data).unwrap();
    }

    json::JsonValue::new_object()
}

fn put(storage_path: &path::PathBuf, args: &ArgumentsValues) {
    let mut data = read_data(storage_path);

    if data[&args.key].is_empty() {
        data[&args.key] = array![args.value.clone()];
    } else {
        data[&args.key].push(args.value.clone()).unwrap();
    }
    write_data(storage_path, &data);
}

fn write_data(storage_path: &path::PathBuf, data: &json::JsonValue) {
    let mut f = std::fs::File::create(storage_path).expect("create failed");
    f.write_all(data.dump().as_bytes()).expect("write failed");
}

fn get(storage_path: &path::PathBuf, key: &str) -> json::JsonValue {
    let data = read_data(storage_path);
    if data[key].is_empty() {
        return array![];
    }

    data[key].clone()
}
