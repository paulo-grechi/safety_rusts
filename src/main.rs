use log::info;
use log4rs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	log4rs::init_file("/home/paulo/safety_rusts/src/log.yaml", Default::default()).unwrap();
	let path_s = std::env::args().nth(1).expect("no param");
	let pattern = std::env::args().nth(2).expect("no param");
	let path = std::path::PathBuf::from(path_s);
	let content = std::fs::read_to_string(&path)?;
	find_pattern(&content, &pattern);
	Ok(())
}

fn find_pattern(content: &str, pattern: &str){
	for line in content.lines(){
		if line.contains(pattern){
			info!("{}", line);
		}
	}
}
