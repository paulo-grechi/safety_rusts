use log::info;
use log4rs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	log4rs::init_file("/home/paul/safety_rusts-master/src/log.yaml", Default::default()).unwrap();
	let args: Vec<String> = std::env::args().collect();
	let path_s = parse_args(&args);
	let path = std::path::PathBuf::from(path_s);
	let content = std::fs::read_to_string(&path)?;
	find_pattern(&content);
	Ok(())
}

fn find_pattern(content: &str){
	for line in content.lines(){
		match Some(line){
			Some(x) if x.contains("rm") || x.contains("sudo") => info!("{}", line),
			Some(_) => print!(""),
			None => print!("")
		}
	}
}

fn parse_args(args: &[String]) -> &str{
	let path_s = &args[1];
	
	path_s
}
