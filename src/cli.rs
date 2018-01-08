use std::path::Path;

pub enum Func {
    Help,
    Execute,
    Error,
}

pub fn arg_parse(args: &Vec<String>) -> Func {
    
    let argc = args.len();
    
    if argc == 1 || (argc == 2 && args[1] == "-h") {
        return Func::Help
    }

    
    if argc == 2 {
        
        if Path::new(&args[1]).exists() {
            
            return Func::Execute;
            
        } else {
            
            println!("Cannot open {:?}", &args[1]);
            return Func::Error;
            
        }
        
    } else {
        
        return Func::Error;
        
    }
}
