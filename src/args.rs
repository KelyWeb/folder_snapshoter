
pub mod components {

    #[derive(Debug, Clone, PartialEq)]
    pub enum Param {
        WithTwo(String, String),
        With(String),
        Without 
    }

    pub type Key = String;
    
    #[derive(Debug, Clone, PartialEq)]
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
                                    match args.get(index + 2) {
                                        Some(next_arg2) => {
                                            match next_arg2.chars().next().unwrap() {

                                                '-' => {
                                                    compl_comands.push(
                                                        CompletedCommand { 
                                                            key: args[index].clone(), 
                                                            param: Param::With(next_arg.clone()) 
                                                        }
                                                    )
                                                },
                                                 _ => {
                                                    compl_comands.push(
                                                        CompletedCommand { 
                                                            key: args[index].clone(), 
                                                            param: Param::WithTwo(next_arg.clone(), next_arg2.clone()) 
                                                        }
                                                    )
                                                }
                                            }
                                        },
                                        None => {
                                            compl_comands.push(
                                                CompletedCommand { 
                                                    key: args[index].clone(), 
                                                    param: Param::With(next_arg.clone()) 
                                                }
                                            )
                                        }
                                    }
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

#[cfg(test)]
#[test]
fn test_parse_with_all_valid_args() {
    use self::components::parse_commands;
    use self::components::{CompletedCommand, Key, Param};
    
    let args = vec![String::from("-r"),
                                 String::from("-t"),
                                 String::from("one"),
                                 String::from("two"),
                                 String::from("ignored"),
                                 String::from("-t"),
                                 String::from("one"),];
    let parse_result = parse_commands(args);
    let parse_vector = vec![CompletedCommand {key: Key::from("-r"), param: Param::Without},
                                                CompletedCommand {key: Key::from("-t"), param: Param::WithTwo(String::from("one"), 
                                                                                                              String::from("two"))},
                                                CompletedCommand {key: Key::from("-t"), param: Param::With(String::from("one"))}];
    assert_eq!(parse_result, parse_vector);
}
#[test]
fn test_parse_without_valid_args() {
    use self::components::parse_commands;
    use self::components::{CompletedCommand};
    
    let args = vec![String::from("one"),
                    String::from("two"),
                    String::from("one"),
                    String::from("two")]; 
    let result_parse = parse_commands(args);
    assert_eq!(result_parse, Vec::<CompletedCommand>::new());
}