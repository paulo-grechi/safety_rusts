use log::info;
use log4rs;
use std::{
	path::{Path,PathBuf},
	fs,
	io,
//	env::consts::FAMILY,
	time::SystemTime,
	thread
};
use homedir::get_my_home;

use safety_rusts::Contador;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	log4rs::init_file("log.yaml", Default::default()).unwrap();
	let mut children = vec![];	
	let sys_time = SystemTime::now();
	let m_path = get_my_home().unwrap().unwrap();
	let _contador_m: Contador = Contador::new(0,0,0);
//	let _result = read_path(&m_path.as_path(), contador_m).unwrap();
	for path in folders(&m_path)?{
		children.push(thread::spawn(move || {
//			read_path(&path, contador_m).unwrap();	
			let _ = read_path(&path);
		}));
	}

	for child in children{
		child.join().unwrap();
	}
//	println!("{} pastas percorridas", result.folders);
//	println!("{} arquivos lidos", result.files);
	let new_sys_time = SystemTime::now();
	let difference = new_sys_time.duration_since(sys_time)
	    .expect("Clock may have gone backwards");
    println!("Tempo de execucao: {difference:?}");
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
        .map(|r| r.unwrap().path())
        .filter(|r| !r.display().to_string().contains("main.rs") && !r.display().to_string().contains("requests.log"))
        .collect())
}

fn read_path(paths: &Path) -> Result<(), Box<dyn std::error::Error>> { // ,mut contador: Contador) -> Result<Contador, Box<dyn std::error::Error>> {
	if paths.is_dir() {
		for path in folders(&paths).unwrap() {
			 match fs::read_to_string(&path) {
				Ok(_) => {
					//contador.files += 1;
					let file = fs::read_to_string(&path).unwrap();
					find_pattern(&file, &path.display().to_string());
				},
				Err(_) => {
					if path.is_dir(){
						//contador.folders += 1;
						//contador = read_path(&path, contador).unwrap();
						read_path(&path).unwrap();
					}
				}
			}
		}	
	} else {
		 match fs::read_to_string(&paths) {
			Ok(_) => {
				//contador.files += 1;
				let file = fs::read_to_string(&paths).unwrap();
				find_pattern(&file, &paths.display().to_string());
			},
			Err(_) => {
				println!("deu erro");
			}
		}
	}
	Ok(())
}

fn has_rm_sudo(line: &str) -> bool {
//	if FAMILY == "unix"{
		return line.starts_with("rm") || line.contains(" rm ") || line.starts_with("sudo") || line.contains(" sudo ")
		|| line.contains("&rm ") || line.contains("&sudo ");
//	} else {
//		return false;
//	}
}
