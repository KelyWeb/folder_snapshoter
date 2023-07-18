
pub mod components {

    #[derive(Debug, Clone)]
    pub enum Param {
        
        With(String),
        Without
    }

    pub type Key = String;

    #[derive(Debug, Clone)]
    pub struct CompletedCommand {

        key: Key,
        param: Param
    }
}