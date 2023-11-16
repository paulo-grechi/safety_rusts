
use log::info;
use log4rs;
use homedir::get_my_home;

use std::{
	fs,
	path::{Path,PathBuf},
	io
};

fn main() -> Result<(), Box<dyn std::error::Error>>{
	log4rs::init_file("./log.yaml", Default::default()).unwrap();
	let m_path = get_my_home().unwrap().unwrap();
	read_path(&m_path.as_path());	
	Ok(())
}

fn find_pattern(content: &str, filename: &str){
	for line in content.lines(){
		if has_rm_sudo(line){
			info!("{} => {}", filename, line);
		}
	}
}

fn folders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path()) // This is safe, since we only have the Ok variants
        .filter(|r| !r.display().to_string().contains("main.rs") && !r.display().to_string().contains("requests.log"))
        .collect())
}

fn read_path(paths: &Path){
	for path in folders(&paths).unwrap(){
		match fs::read_to_string(&path){
			Ok(_) => {
				let file = fs::read_to_string(&path).unwrap();
				find_pattern(&file, &path.display().to_string());
			},
			Err(_) => {
				if path.is_dir() {
					read_path(&path);	
				}
			}
		}
	}
}

fn has_rm_sudo(line: &str) -> bool {
	line.starts_with("rm") || line.contains(" rm ") || line.starts_with("sudo") || line.contains(" sudo ")
	|| line.contains("&rm ") || line.contains("&sudo ")
}
