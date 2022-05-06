use std::{fs,env};
use std::path::PathBuf;

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

  let objects_as_strings = match fs::read_dir(".") {
    Ok(iterator) => iterator.filter_map(|x| x.ok()) // Get all the valid items
                            .filter_map(|x| path_to_string(x.path())),
    Err(error) => {
      eprintln!["Failed to open the current dir [Error: {}]",error];
      return
    }
  };

  for cur_object in objects_as_strings {
    let renamed_object: String = cur_object.replace(search_str,replace_str);

    // Ignore not renamed objects
    if renamed_object == cur_object {
      continue
    }

    // Rename the object
    match fs::rename(&cur_object,&renamed_object) {
      Ok(_) => println!["{} -> {}",cur_object,renamed_object],
      Err(error) => eprintln!["ERROR {} -> {} [Error: {}]",cur_object,renamed_object,error]
    };
  }
}

fn path_to_string(path: PathBuf) -> Option<String> {
  if path.to_str().is_none() {
    return None
  } else {
    return Some(path.to_str().unwrap().to_string())
  }
}
