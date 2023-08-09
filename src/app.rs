#[path = "./args.rs"] mod args;
#[path = "./dir.rs"] mod dir;
use std::collections::hash_map::DefaultHasher;
use std::ffi::OsStr;
use std::fs::{File, metadata, OpenOptions};
use std::hash::{Hasher, Hash};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{PathBuf, Path};
use args::components::{parse_commands, Args, CompletedCommand, Param};
use chrono::{DateTime, Local, Utc};
use dir::{DirEntryFiles, collect_files, compare_root_dirs, add_root_dir, compare_snaps};


pub const CONFIG_FILE: &str = "./config";
pub struct Application;

impl Application {

    pub fn start(args_c: Vec<String>) {

        let mut args = parse_commands(args_c);
        if args.is_empty() {
            Application::help();
            return;
        }

        let mut file = Application::config_file();
        Application::is_change_work_dir_action(&mut args, &mut file);
        let dir = match Application::read_work_directory(&mut file) {
            Ok(path) => path,
            Err(error) => panic!("{}", error)
        };
        Application::is_make_snap_action(&mut args, PathBuf::from(dir).as_path());
        Application::is_compare_snaps_actions(&mut args);
        // snapshot -d <<path to dir for snap>>  
        // snapshot -s <<path wo work dir>>
        // snapshot -c <<snap1>> <<snap2>>

        //let temp = String::from("folder");
        //Application::create_snaps_folder(&dir, &temp).unwrap();
    }

    fn is_compare_snaps_actions(args: &mut Args) {

        match args.iter().find(|arg| arg.key == "-c") {
            Some(compare_action) => {
                match &compare_action.param {
                    Param::WithTwo(first, second) => {
                        let metadata_snap1 = std::fs::metadata(first.as_str())
                                                        .expect("Snap1 invalid path");
                        let metadata_snap2 = std::fs::metadata(first.as_str())
                                                        .expect("Snap2 invalid path");
                        if !metadata_snap1.is_file() && !metadata_snap2.is_file() {
                            panic!("Snap must be a file");
                        }
                        let snaps = Application::load_snaps(
                                                            PathBuf::from(first.as_str()).as_path(), 
                                                            PathBuf::from(second.as_str()).as_path());
                        let mut compare_output_result = String::new();
                        if !compare_root_dirs((&snaps.0, &snaps.1)) {
                            panic!("Root dir aren't the same");
                        }
                        add_root_dir(&mut compare_output_result, &snaps.0);
                        compare_snaps((&snaps.0, &snaps.1), (3, 3), &mut compare_output_result);
                        println!("{}", compare_output_result);
                    }
                    Param::With(_) => panic!("Use -c arg with two aprams. -c <snap1 path> <snap2 path>"),
                    Param::Without => panic!("Use -c arg with two aprams. -c <snap1 path> <snap2 path>")
                }
            }
            None => {}
        }
    }

    fn load_snaps(snap1: &Path, snap2: &Path) -> (DirEntryFiles, DirEntryFiles) {

        let mut snaps: (DirEntryFiles, DirEntryFiles) = (DirEntryFiles::new(), DirEntryFiles::new());
        let mut snap1_file = OpenOptions::new()
                                        .read(true)
                                        .open(snap1)
                                        .expect("File open error");
        let mut snap2_file = OpenOptions::new()
                                        .read(true)
                                        .open(snap2)
                                        .expect("File open error");
        snaps.0 = DirEntryFiles::read_from_file(&mut snap1_file);
        snaps.1 = DirEntryFiles::read_from_file(&mut snap2_file);
        snaps
    }

    fn is_make_snap_action(args: &mut Args, work_dir: &Path) {

        match args.iter().find(|arg| arg.key == "-s") {
            Some(arg) => {
                match &arg.param{
                    Param::With(param) => {
                        match std::fs::metadata(param.as_str()) {
                            Ok(_) => {
                                let complete_path = Application::create_snaps_folder(work_dir, param);
                                let local_data: DateTime<Local> = Utc::now().with_timezone(&Local);
                                let snap_name = local_data.time()
                                                          .to_string()
                                                          .chars().map(|symbol| match symbol {
                                                            ':' => '.',
                                                            _   => symbol
                                                          }).collect::<String>();

                                println!("{}", complete_path.join(snap_name.as_str()).display());

                                match OpenOptions::new()
                                                .create(true)
                                                .write(true)
                                                .open(complete_path.join(snap_name.as_str())) 
                                                {
                                                    Ok(mut file) => {
                                                        let mut dir_entry = DirEntryFiles::new_dir(
                                                                            PathBuf::from(param.as_str()).file_name().unwrap().to_os_string());
                                                        collect_files(std::fs::read_dir(PathBuf::from(param.as_str())).unwrap(), &mut dir_entry);

                                                        dir_entry.write_to_file(&mut file);
                                                        dir_entry.debug_files(0, 3);
                                                    },
                                                    Err(error) => panic!("File open error")
                                                }
                            },
                            Err(_) => panic!("Invalid dir path")
                        }
                    },
                    Param::WithTwo(_, _) => panic!("Use -s arg with one param. Use -s <path>"),
                    Param::Without => panic!("Invalid snap path. Use -s <path>")
                }
            }
            None => {}
        }
    }

    fn is_change_work_dir_action(args: &mut Args, work_dir_file: &mut File) {

        match args.iter().find(|arg| arg.key == "-d") {
            Some(arg) => {
                match &arg.param {
                    Param::With(arg_path) => {
                        work_dir_file.set_len(0).expect("Trucate file error");
                        match std::fs::metadata(arg_path.as_str()) {
                            Ok(_) => {
                                work_dir_file.write_all(arg_path.as_bytes()).expect("Error write new work path to file");
                                work_dir_file.seek(SeekFrom::Start(0));
                            },
                            Err(_) => panic!("Invalid file path. Use -d 'path'")
                        }
                    },
                    Param::WithTwo(_, _) => panic!("Arg -s need only one param. Use -d 'path"),
                    Param::Without => panic!("Arg -s needs a path. Use -d 'path'")
                }
            },
            None => {}
        }
    }

    fn config_file() -> File { // Find or create file that stores a path to work directory

        let file_try_find = Application::check_config_file();

        let file = match file_try_find {
            Ok(file_r) => file_r,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::NotFound => {
                        Application::create_config_file();
                        File::options()
                            .read(true)
                            .write(true)
                            .open(CONFIG_FILE)
                            .expect("Open config file error")
                    },
                    _ => panic!("{}", error)
                }
            }
        };
        file
    }

    fn check_config_file() -> Result<File, std::io::Error> {

        let file_r = File::options()
                    .read(true)
                    .write(true)
                    .open(CONFIG_FILE);
        match file_r {
            Ok(file) => Ok(file),
            Err(err) => Err(err)
        }
    }

    fn create_config_file() {
        File::create("config")
                .expect("Error create config file");
    }

    fn read_work_directory(file: &mut File) -> Result<String, std::io::Error> {

        let mut file_buffer = String::new();
        let read_res = file.read_to_string(&mut file_buffer);

        match read_res {

            Ok(read_s) => {

                if read_s == 0 {

                    Ok(String::from("."))

                } else {
                    let dir_path = file_buffer.lines().next().unwrap();
                    match metadata(dir_path) {
                        Ok(mdata) => {
                            match mdata.is_dir() {
                                true  => Ok(String::from(dir_path)),
                                false => Ok(String::from("."))
                            }
                        },
                        Err(error) => Err(error)
                    }
                }
            },
            Err(error) => Err(error)
        }
    }

    fn create_snaps_folder(work_dir: &Path, snap_folder_path: &String) -> PathBuf {

        let mut hasher = DefaultHasher::default();
        snap_folder_path.hash(&mut hasher);
        let hash_r = hasher.finish();
        let complete_path = Path::new(work_dir).join(hash_r.to_string());
        match std::fs::create_dir(complete_path.as_path()) {
            Ok(_) => complete_path,
            Err(error) => {
                match error.kind() {
                    std::io::ErrorKind::AlreadyExists => complete_path,
                    _ => panic!("Create snap dir error")
                }
            }
        }
    }

    fn help() {

    }
}