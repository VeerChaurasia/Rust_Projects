use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let directory = if args.len()>1{
        PathBuf::from(&args[1])

    }else{
        PathBuf::from(".")
    };

    match fs::read_dir(directory){
        Ok(entries)=>{
            for entry in entries{
                match entry{
                    Ok(entry)=> println!("{}",entry.path().display()),
                    Err(e)=>eprint!("Error :{}",e),
                }
            }
            
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }

}
