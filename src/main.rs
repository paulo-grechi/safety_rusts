use log::info;
use log4rs;
use std::{
	path::{Path,PathBuf},
	fs,
	io,
	env::consts::FAMILY,
	time::SystemTime,
	thread
};
use homedir::get_my_home;
// Era usado para fazer a contagem dos arquivos e pastas lidas, mas com threads nao funcionou direito ainda
//  estou investigando como fazer.
use safety_rusts::Contador;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Inicializa o arquivo de log, para fazer a gravacao das linhas que podem conter script malicioso
	log4rs::init_file("log.yaml", Default::default()).unwrap();
	let mut children = vec![];	
	let sys_time = SystemTime::now();
	// Usando a crate homedir busca o caminho para o diretorio home do usuario atual
	let m_path = get_my_home().unwrap().unwrap();
	let _contador_m: Contador = Contador::new(0,0,0);
//	let _result = read_path(&m_path.as_path(), contador_m).unwrap();
	for path in folders(&m_path)?{
		// cria uma nova thread e adiciona ela para a lista de threads, todos os arquivos e diretorios na pasta home
		// do usuario atual terao sua propria thread de execucao
		children.push(thread::spawn(move || {
//			read_path(&path, contador_m).unwrap();	
			// le o caminho atual, determinando se o caminho corresponde a um diretorio ou arquivo
			let _ = read_path(&path);
		}));
	}

	for child in children{
		// espera o encerramento da execucao da thread
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
	// percorre as linhas do arquivo
	for line in content.lines(){
		// esta funcao determina se o sistema que esta sendo usado e linux ou windows, e busca os comandos
		// especificos de cada sistema
		if has_rm_sudo(line){
			// faz a gravacao da linha e do nome do arquivo no log
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
		// caso seja um diretorio, faz a busca dos arquivos dentro dele mesmo para procurar os caminhos a vasculhar
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
				// caso seja um arquivo faz a leitura dele, e passa para a funcao de vasculhamento
				let file = fs::read_to_string(&paths).unwrap();
				find_pattern(&file, &paths.display().to_string());
			},
			Err(_) => {
				//println!("{}", er);
			}
		}
	}
	Ok(())
}

fn has_rm_sudo(line: &str) -> bool {
	if FAMILY == "unix"{
		return line.starts_with("rm") || line.contains(" rm ") || line.starts_with("sudo") || line.contains(" sudo ")
		|| line.contains("&rm ") || line.contains("&sudo ");
	} else {
		return false;
	}
}
