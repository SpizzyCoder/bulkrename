use std::fs;
use std::path::PathBuf;

use clap::Parser;

/// Simple program to list directories
#[derive(Parser,Debug)]
#[clap(version,about)]
pub struct Args {
  /// Search string
  #[clap()]
  search_str: String,

  /// Replace string
  #[clap()]
  replace_str: String,
}

fn main() {
  let args: Args = Args::parse();

  let objects_as_strings = match fs::read_dir(".") {
    Ok(iterator) => iterator.filter_map(|x| x.ok()) // Get all the valid items
                            .filter_map(|x| path_to_string(x.path())),
    Err(error) => {
      eprintln!["Failed to open the current dir [Error: {}]",error];
      return
    }
  };

  for cur_object in objects_as_strings {
    let renamed_object: String = cur_object.replace(&args.search_str,&args.replace_str);

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
