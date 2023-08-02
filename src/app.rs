#[path = "./args.rs"] mod args;
use std::collections::hash_map::DefaultHasher;
use std::fs::{File, metadata};
use std::hash::{Hasher, Hash};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{PathBuf, Path};
use args::components::{parse_commands, Args, CompletedCommand, Param};


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

        

        
        // snapshot -d <<path to dir for snap>>  
        // snapshot -s <<path wo work dir>>
        // snapshot -c <<snap1>> <<snap2>>

        //let temp = String::from("folder");
        //Application::create_snaps_folder(&dir, &temp).unwrap();
    }

    fn is_change_work_dir_action(args: &mut Args, work_dir_file: &mut File) {

        match args.iter().find(|arg| arg.key == "-s") {
            Some(arg) => {
                match &arg.param {
                    Param::With(arg_path) => {
                        work_dir_file.set_len(0).expect("Trucate file error");
                        match std::fs::metadata(arg_path.as_str()) {
                            Ok(_) => {
                                work_dir_file.write_all(arg_path.as_bytes()).expect("Error write new work path to file");
                                work_dir_file.seek(SeekFrom::Start(0));
                            },
                            Err(_) => panic!("Invalid file path. Use -s 'path'")
                        }
                    },
                    Param::Without => panic!("Arg -s needs a path. Use -s 'path'")
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
                            .open("./config")
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
                    .open("./config");
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

    fn create_snaps_folder(work_dir: &String, snap_folder_path: &String) -> std::io::Result<()> {

        let mut hasher = DefaultHasher::default();
        snap_folder_path.hash(&mut hasher);
        let hash_r = hasher.finish();
        let complete_path = Path::new(work_dir.as_str()).join(hash_r.to_string());
        std::fs::create_dir(complete_path)
    }

    fn help() {

    }
}