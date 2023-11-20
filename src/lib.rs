pub struct Contador {
	pub lines: u128,
	pub folders: u128,
	pub files: u128
}

impl Contador{
	pub fn new(p_lines: u128, p_folders: u128, p_files: u128) -> Contador{
		Contador{
			lines: p_lines,
			folders: p_folders,
			files: p_files
		}
	}
}
