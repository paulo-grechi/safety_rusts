fn main() -> Result<(), Box<dyn std::error::Error>> {
	let path_s = std::env::args().nth(1).expect("no params");
	let pattern = std::env::args().nth(2).expect("no param");
	let path = std::path::PathBuf::from(path_s);
	let content = std::fs::read_to_string(&path)?;
	find_pattern(&content, &pattern);
	Ok(())
}

fn find_pattern(content: &str, pattern: &str){
	for line in content.lines(){
		if line.contains(pattern){
			println!("{}", line);
		}
	}	
}
