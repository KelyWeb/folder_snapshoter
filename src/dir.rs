
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