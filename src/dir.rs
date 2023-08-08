use std::ffi::{OsString, OsStr};
use std::fs::{ReadDir, read_dir, File, DirEntry};
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::{Result, Value};
use std::collections::LinkedList;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DirEntryFiles {
    File(OsString, u64),
    Dir {
        name: OsString,
        files: Vec<DirEntryFiles>
    }
}

impl std::cmp::PartialEq for DirEntryFiles {

    fn eq(&self, other: &Self) -> bool {

        if let DirEntryFiles::Dir { name, files } = self {
            if let DirEntryFiles::Dir { name, files } = other {
                true
            } else {
                false
            }
        } else {
            if let DirEntryFiles::File(name, size) = other {
                true
            } else {
                false
            }
        }
    }
}

impl DirEntryFiles {

    pub fn new() -> Self {
        DirEntryFiles::Dir { name: OsString::new(), files: Vec::new() }
    }

    pub fn new_dir(dir_name: OsString) -> DirEntryFiles {

        DirEntryFiles::Dir { name: dir_name, files: Vec::new() }
    }
    pub fn new_file(file_name: OsString, file_size: u64) -> DirEntryFiles {

        DirEntryFiles::File(file_name, file_size)
    }

    pub fn add_file_to_dir(&mut self, file_metadata: (std::ffi::OsString, u64)) {

        match *self {

            DirEntryFiles::File(_, _) => {},
            DirEntryFiles::Dir { ref mut name, ref mut files } => {

                files.push(DirEntryFiles::File(file_metadata.0, file_metadata.1))
            }
        }
    }
    pub fn add_dir_to_dir(&mut self, dir: DirEntryFiles) {

        match *self {

            DirEntryFiles::File(_, _) => {},
            DirEntryFiles::Dir { ref mut name, ref mut files } => {

                files.push(dir);
            }
        }
    }

    pub fn debug_files(&self, spaces: usize, sticks: usize) {

        match *self {
            DirEntryFiles::File(ref name, ref size) => {
                for space in 0..spaces {
                    print!(" ");
                }
                print!("|");
                for stick in 0..sticks {
                    print!("-");
                }
                println!("{} {}", name.to_str().unwrap(), size);
            },
            DirEntryFiles::Dir { ref name, ref files } => {
                for space in 0..spaces {
                    print!(" ");
                }
                print!("|");
                for stick in 0..sticks {
                    print!("-");
                }
                println!("{}", name.to_str().unwrap());
                for next_file in files {

                    next_file.debug_files(spaces + sticks, sticks + 3);
                }
            }
        }
    }

    pub fn write_to_file(&self, file: &mut File) {

        let serialized = serde_json::to_string(self)
                                            .expect("Serialize DirEntryFiles Error");
        match file.write_all(serialized.as_bytes()) {
            Err(error) => panic!("{}", error),
            _ => {}
        }
    }
    pub fn read_from_file(file: &mut File) -> DirEntryFiles {

        let mut read_buffer = String::new();
        file.read_to_string(&mut read_buffer)
            .expect("File read error");
        let deser_obj: DirEntryFiles = serde_json::from_str(read_buffer.as_str()).unwrap();
        deser_obj
    }
}

pub fn collect_files(dir_path: ReadDir, dir_entry: &mut DirEntryFiles) {

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

pub fn compare_root_dirs(snaps: (&DirEntryFiles, &DirEntryFiles)) -> bool {
    if let DirEntryFiles::Dir { name, files } = snaps.0 {
        if let DirEntryFiles::Dir { name: subname, files: subfiles } = snaps.1 {
            if name == subname {
                return true;
            }
        } 
    } 
    false
}

pub fn compare_snaps(snaps: (&DirEntryFiles, &DirEntryFiles), tabs: (usize, usize), output: &mut String) {

    let snap1_list = if let DirEntryFiles::Dir { name, files } = snaps.0 {
        files.iter().collect::<LinkedList<_>>()
    } else { LinkedList::new() };
    let snap2_list = if let DirEntryFiles::Dir { name, files } = snaps.1 {
        files.iter().collect::<LinkedList<_>>()
    } else { LinkedList::new() };

    for file in snap1_list {
        if let DirEntryFiles::File(f_name, f_size) = file {
            match search_match(&snap2_list, file) {
                Some(file_dup) => {
                    add_output_line_mod(output, tabs, (file, file_dup));
                },
                None => {
                    add_output_line_deleted(output, tabs, file);
                }
            }
        } else {
            match search_match(&snap2_list, file) {
                Some(file_dup) => {
                    add_output_line_mod(output, tabs, (file, file_dup));
                    compare_snaps((file, file_dup), (tabs.0 + 3, tabs.1 + 3), output);
                },
                None => {
                    add_output_line_deleted(output, tabs, file);
                    compare_snaps((file, &DirEntryFiles::new()), (tabs.0 + 3, tabs.1 + 3), output);
                }
            }
        }
    }
}

fn search_match<'a>(list_with_files: &'a LinkedList<&DirEntryFiles>, file: &DirEntryFiles) -> Option<&'a DirEntryFiles> {

    for file_s in list_with_files {
        if compare_files(file_s, file) {
            return Some(file_s);
        }
    }
    None
}

fn compare_files(file1: &DirEntryFiles, file2: &DirEntryFiles) -> bool {

    if file1 == file2 {
        if let DirEntryFiles::Dir { name, files } = file1 {
            if let DirEntryFiles::Dir { name: sub_name, files: sub_files } = file2 {
                if name == sub_name {
                    return true;
                } else {
                    return false;
                }
            }
        }
        if let DirEntryFiles::File(file_name, file_size) = file1 {
            if let DirEntryFiles::File(file_name_s, file_size_s) = file2 {
                if file_name == file_name_s {
                    return true
                } else {
                    return false;
                }
            }
        }
    }
    false
}

pub fn add_output_line_mod(output: &mut String, tabs: (usize, usize), files: (&DirEntryFiles,&DirEntryFiles)) {

    let mut line = String::new();
    match files.0 {
        DirEntryFiles::Dir { name, files } => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            line.push('|');
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(name.to_str().unwrap());
        },
        DirEntryFiles::File(file_name, file_size) => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(file_name.to_str().unwrap());
            line.push(' ');
            if let DirEntryFiles::File(file_name_sub, file_size_sub) = files.1 {
                if *file_size == *file_size_sub {
                    line.push_str(file_size.to_string().as_str());
                    line.push_str(" Bytes");
                } else {
                    line.push_str(file_size.to_string().as_str());
                    line.push_str(" Bytes");
                    line.push_str(" -> ");
                    line.push_str(file_size_sub.to_string().as_str());
                    line.push_str(" Bytes");
                }
            }
        }
    }
    line.push('\n');
    output.push_str(line.as_str());
}

pub fn add_output_line_new(output: &mut String, tabs: (usize, usize), file: &DirEntryFiles) {

    let mut line = String::new();
    match file {
        DirEntryFiles::Dir { name, files } => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            line.push('|');
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(name.to_str().unwrap());
            line.push_str(" new");
        },
        DirEntryFiles::File(file_name, file_size) => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(file_name.to_str().unwrap());
            line.push_str(" new");
        }
    }
    line.push('\n');
    output.push_str(line.as_str());
}

pub fn add_output_line_deleted(output: &mut String, tabs: (usize, usize), file: &DirEntryFiles) {

    let mut line = String::new();
    match file {
        DirEntryFiles::Dir { name, files } => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            line.push('|');
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(name.to_str().unwrap());
            line.push_str(" deleted");
        },
        DirEntryFiles::File(file_name, file_size) => {
            for space in 0..tabs.0 {
                line.push(' ');
            }
            for stick in 0..tabs.1 {
                line.push('-');
            }
            line.push_str(file_name.to_str().unwrap());
            line.push_str(" deleted");
        }
    }
    line.push('\n');
    output.push_str(line.as_str());
}

pub fn add_root_dir(output: &mut String, dir: &DirEntryFiles) {
    if let DirEntryFiles::Dir { name, files } = dir {
        output.push_str(name.to_str().unwrap());
        output.push('\n');
    }
}