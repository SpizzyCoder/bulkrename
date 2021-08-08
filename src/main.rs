use std::{fs,env};
use std::fs::ReadDir;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!["=============================="];
    println!["Syntax : bulkrename <SearchExpression> <ReplaceExpression>"];
    println!["Example: bulkrename 2020 2021"];
    println!["=============================="];
    return
  }

  let search_str: &str = &args[1];
  let replace_str: &str = &args[2];

  // Get an iterator from current dir
  let current_dir_iterator: ReadDir = match fs::read_dir("./") {
    Ok(iterator) => iterator,
    Err(error) => {
      eprintln!["Couldn't open the current dir [Error: {}]",error];
      return
    }
  };

  for object in current_dir_iterator {
    // Get the current object as string
    let current_object: String = match object {
      Ok(object_entry) => object_entry.path().display().to_string(),
      Err(error) => {
        eprintln!["Couldn't get object as string [Error: {}]",error];
        continue
      }
    };

    // Replace substrings in string
    let renamed_object: String = current_object.replace(search_str,replace_str);

    // Ignore not renamed objects
    if renamed_object == current_object {
      continue
    }

    // Rename the object
    match fs::rename(&current_object,&renamed_object) {
      Ok(_) => println!["{} -> {}",current_object,renamed_object],
      Err(error) => eprintln!["ERROR {} -> {} [Error: {}]",current_object,renamed_object,error]
    };
  }
}
