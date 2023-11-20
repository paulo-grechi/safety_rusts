use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::env;

fn main(){
	let profile = env::var_os("PROFILE").unwrap();
	let yaml = fs::read_to_string("./src/log.yaml").unwrap();
	if profile == "release" {
		let dest_path = Path::new("target/release/log.yaml");
		let mut file = File::create(&dest_path.display().to_string()).expect("Unable to create file");
		file.write_all(yaml.as_bytes()).unwrap();
	} else {
		let dest_path = Path::new("target/debug/log.yaml");
		let mut file = File::create(&dest_path.display().to_string()).expect("Unable to create file");
		file.write_all(yaml.as_bytes()).unwrap();	
	}
}
