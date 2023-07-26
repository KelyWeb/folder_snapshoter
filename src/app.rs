use std::collections::hash_map::DefaultHasher;
use std::fs::{File, metadata};
use std::hash::{Hasher, Hash};
use std::io::Read;
use std::path::{PathBuf, Path};

pub struct Application;

impl Application {

    pub fn start() {

        let mut file = Application::config_file();
        let dir = match Application::read_work_directory(&mut file) {

            Ok(path) => path,
            Err(error) => panic!("{}", error)
        };

        let temp = String::from("folder");
        Application::create_snaps_folder(&dir, &temp).unwrap();
    }

    fn config_file() -> File {

        let file_try_find = Application::check_config_file();

        let file = match file_try_find {

            Ok(file_r) => file_r,
            Err(error) => {

                match error.kind() {

                    std::io::ErrorKind::NotFound => {

                        match Application::create_config_file() {
                            Ok(file) => file,
                            Err(error) => match error.kind() {

                                _ => panic!("{}", error)
                            }
                        }
                    },
                    _ => panic!("{}", error)
                }
            }
        };
        file
    }

    fn check_config_file() -> Result<File, std::io::Error> {

        let file_r = File::open("./config");

        match file_r {
            Ok(file) => Ok(file),
            Err(err) => Err(err)
        }
    }

    fn create_config_file() -> Result<File, std::io::Error> {

        let file = match File::create("config") {

            Ok(file) => file,
            Err(err) => return Err(err)
        };
        Ok(file)
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
}