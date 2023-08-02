
pub mod components {

    #[derive(Debug, Clone)]
    pub enum Param {
        
        With(String),
        Without 
    }

    pub type Key = String;
    
    #[derive(Debug, Clone)]
    pub struct CompletedCommand {

        pub key: Key,
        pub param: Param
    }

    pub type Args = Vec::<CompletedCommand>;

    impl CompletedCommand {

        fn new(key_t: Key, param_t: Param) -> Self {
            Self {
                key: key_t,
                param: param_t
            }
        }
    }

    pub fn parse_commands(args: Vec<String>) -> Args {

        let mut compl_comands = Args::new();

        for index in 0..args.len() {

            match key_check(args[index].as_str()) {

                true => {

                    match args.get(index + 1) {

                        Some(next_arg) => {

                            match next_arg.chars().next().unwrap() {

                                '-' => {
                                    compl_comands.push(
                                        CompletedCommand { 
                                            key: args[index].clone(), 
                                            param: Param::Without 
                                        }
                                    )
                                },
                                 _  => {
                                    compl_comands.push(
                                        CompletedCommand { 
                                            key: args[index].clone(), 
                                            param: Param::With(next_arg.clone()) 
                                        }
                                    )
                                }
                            }
                        },
                        None => {
                            compl_comands.push(
                                CompletedCommand { 
                                    key: args[index].clone(), 
                                    param: Param::Without 
                                }
                            )
                        }
                    }
                },
                false => {}
            }
        }
        compl_comands
    }

    pub fn key_check(key: &str) -> bool {

        match key.chars().next() {

            Some(ch) => {
                match ch {
                    '-' => true,
                     _  => false
                }
            },
            None => false
        }
    }
}