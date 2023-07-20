use std::fs::File;

pub struct Application;

impl Application {

    pub fn start() {

        let file = Application::config_file();
        
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
}