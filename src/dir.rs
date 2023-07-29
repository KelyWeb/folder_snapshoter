use std::ffi::OsString;
use std::fs::{ReadDir, read_dir};


#[derive(Debug, Clone)]
enum DirEntryFiles {
    File(OsString, u64),
    Dir {
        name: OsString,
        files: Vec<DirEntryFiles>
    }
}

impl DirEntryFiles {

    fn new_dir(dir_name: OsString) -> DirEntryFiles {

        DirEntryFiles::Dir { name: dir_name, files: Vec::new() }
    }
    fn new_file(file_name: OsString, file_size: u64) -> DirEntryFiles {

        DirEntryFiles::File(file_name, file_size)
    }

    fn add_file_to_dir(&mut self, file_metadata: (std::ffi::OsString, u64)) {

        match *self {

            DirEntryFiles::File(_, _) => {},
            DirEntryFiles::Dir { ref mut name, ref mut files } => {

                files.push(DirEntryFiles::File(file_metadata.0, file_metadata.1))
            }
        }
    }
    fn add_dir_to_dir(&mut self, dir: DirEntryFiles) {

        match *self {

            DirEntryFiles::File(_, _) => {},
            DirEntryFiles::Dir { ref mut name, ref mut files } => {

                files.push(dir);
            }
        }
    }
}

fn collect_files(dir_path: ReadDir, dir_entry: &mut DirEntryFiles) {

    dir_path.map(|entity_res| {
        match entity_res {
            Ok(entity) => {

                if entity.metadata().unwrap().is_file() == true {

                    dir_entry.add_file_to_dir((entity.file_name(), entity.metadata().unwrap().len()));
                } else {

                    let mut next_dir = DirEntryFiles::new_dir(entity.file_name());
                    collect_files(read_dir(entity.path()).expect("Read file Error"), 
                                    &mut next_dir);
                    dir_entry.add_dir_to_dir(next_dir);
                }
            },
            Err(_) => {}
        }
    }).collect()    
}