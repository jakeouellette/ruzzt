use std::{error::Error, fs,fs::File, io, path::Path, collections::HashMap};

mod tsv_reader;
use zzt_file_format::{World, Board};

fn main() -> Result<(), Box<dyn Error>>{
    println!("Loading...");
    let param = std::env::args().nth(1)
    .ok_or("Missing root path argument")?;
    let root = Path::new(&param);
    println!("Loading... {}", &root.to_str().unwrap());
    let mapping_file = root.join("mapping.tsv");
    let zzt_in = root.join("CAVEBASE.ZZT");
    println!("Loading... {}", &zzt_in.to_str().unwrap());
    let mut input_file = std::fs::File::open(zzt_in).map_err(|e| format!("{:?}", e))?;

    let mut world : World = World::parse(&mut input_file).unwrap();

    let zzt_out = root.join("CAVENEW.ZZT");
    println!("Targeting... {}", &zzt_out.to_str().unwrap());

    let old_to_new_boardnames = tsv_reader::read_tsv(&mapping_file)?;
    let new_boards = get_boards(root, &world).unwrap();
    for board in &new_boards {
        println!("{}", board.1.meta_data.board_name.to_string(false));
    }
    let mut updated_boards = Vec::new(); 
    for board in world.boards {
        let name = board.meta_data.board_name.to_string(false); 

        if let Some(board_name) = old_to_new_boardnames.get(&name) {
            if let Some(new_board) = new_boards.get(board_name) {
                println!("Updating [{}] to board: [{}]", name, board_name);
                updated_boards.push(new_board.clone()); 
            } else {
                println!("Keeping (no file): [{}]", name);
                updated_boards.push(board.clone()); 
            }
        } else {
            println!("Keeping (no board assigned): [{}]", name);
            updated_boards.push(board.clone()); 
        }
    }
    world.boards = updated_boards;
    let mut out_file = File::create(zzt_out).unwrap();
    // let mut output_file = std::fs::File::open(zzt_out).map_err(|e| format!("{:?}", e))?;
	
    let _ = world.write(&mut out_file);
    
// tsv_reader::test();
  Ok(())
}

pub fn get_boards(file_path: &Path, world: &World) -> Result<HashMap<String, Board>, Box<dyn Error>> {
    let brds: Vec<String> = list_files(file_path, "brd").unwrap();
    let mut brds_by_name =  HashMap::new();
    for brd in brds {
        let mut input_file = std::fs::File::open(file_path.join(brd)).map_err(|e| format!("{:?}", e))?;
        let board = Board::parse(&mut input_file, world.world_header.world_type).unwrap();
        brds_by_name.insert(board.meta_data.board_name.to_string(false), board);
    }
    Ok(brds_by_name)
}


fn list_files(dir: &Path, extension: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_str().unwrap().to_ascii_lowercase() == extension.to_ascii_lowercase() { 
                        files.push(path.to_string_lossy().to_string());
                    }
                }
            }
        }
    }
    Ok(files)
}