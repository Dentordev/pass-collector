
use std::collections::HashMap;
use std::fs::read_to_string;
use clap::Parser;
use std::fs::File;
use std::io::prelude::*;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version = "v0.0.2", about, long_about = None)]
struct Args {
    /// Files to input
    #[arg(num_args(0..))]
    /// List of files to accept. Username, password format
    files: Vec<std::path::PathBuf>,
    /// Number of passwords to output.
    #[arg(short, long, default_value_t = 50)]
    limit: u32,
    /// Weather or not to output results to a textfile
    #[arg(short, long)]
    output: Option<std::path::PathBuf>,

    #[arg(short, long, default_value_t = ',')]
    delimiter:char
}




struct PasswordCollector {
    passwords_map:HashMap<String, u32>
}

impl PasswordCollector {
    pub fn new() -> Self{
        Self{passwords_map:HashMap::new()}
    }

    pub fn read(&mut self, src:&std::path::PathBuf, delimiter:&char){
        for mut line in read_to_string(src).unwrap().lines().map(String::from){
            let idx = line.find(*delimiter);
            if idx.is_none(){
                continue;
            }
            *self.passwords_map.entry(line.split_off(idx.unwrap() + 1)).or_insert(1u32) += 1;
        }
    }

    pub fn common_passwords_and_limit(&self, limit:u32) -> Vec<String>{
        let mut passdict: HashMap<String, u32>= self.passwords_map.clone();
        let mut outvec = Vec::new();
        for _ in 0..limit{
            // finish if we're empty
            if passdict.is_empty(){
                break;
            }
            let s = passdict.iter().max_by(|a, b| a.1.cmp(&b.1)).map(|(k, _v)| k).unwrap();
            outvec.push(s.clone().to_string());
            passdict.remove(&s.clone());
        }
        return outvec;
    }

    pub fn write_common_password_to_file(&self, out:&std::path::PathBuf, limit:u32){
        let mut fp = File::create(out).unwrap();
        for line in self.common_passwords_and_limit(limit){
            writeln!(fp, "{}", line).unwrap();
        }
    }
}



fn main() {
    let args = Args::parse();
    let mut collector = PasswordCollector::new();
    for a in args.files{
        collector.read(&a, &args.delimiter);
    }
    match args.output {
        Some(o) => {collector.write_common_password_to_file(&o, args.limit);},
        None => {
            for password in collector.common_passwords_and_limit(args.limit){
                println!("{password}");
            }
        }
    }
}
