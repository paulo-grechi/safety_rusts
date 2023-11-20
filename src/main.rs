use log::info;
use log4rs;
use std::{
	path::{Path,PathBuf},
	fs,
	io,
};
use homedir::get_my_home;

use safety_rusts::Contador;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	log4rs::init_file("log.yaml", Default::default()).unwrap();
	let m_path = get_my_home().unwrap().unwrap();
	let contador_m: Contador = Contador::new(0,0,0);
	let result = read_path(&m_path.as_path(), contador_m).unwrap();
	println!("{} pastas percorridas", result.folders);
	println!("{} arquivos lidos", result.files);
    Ok(())
}

fn find_pattern(content: &str, filename: &str){
	for line in content.lines(){
		match line {
			x if has_rm_sudo(x) => info!("{}", line),
			_ => (),
		}
	}
}

fn folders(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    Ok(fs::read_dir(dir)?
        .into_iter()
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path())
        .filter(|r| !r.display().to_string().contains("main.rs") && !r.display().to_string().contains("requests.log"))
        .collect())
}

fn read_path(paths: &Path,mut contador: Contador) -> Result<Contador, Box<dyn std::error::Error>> {
	for path in folders(&paths)? {
		 match fs::read_to_string(&path) {
			Ok(_) => {
				contador.files += 1;
				let file = fs::read_to_string(&path).unwrap();
				find_pattern(file, &path.display().to_string());
			},
			Err(_) => {
				if path.is_dir(){
					contador.folders += 1;
					contador = read_path(&path, contador).unwrap();
				} 
			}
		}
	}
	Ok(contador)
}

fn has_rm_sudo(line: &str) -> bool {
	line.starts_with("rm") || line.contains(" rm ") || line.starts_with("sudo") || line.contains(" sudo ")
	|| line.contains("&rm ") || line.contains("&sudo ")
}
