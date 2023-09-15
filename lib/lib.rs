mod zeit;

pub trait Game {
    /// Returns the LaTeX code for the puzzle
    fn latex(&self) -> String;
}


#[derive(Debug)]
enum Module {
    Zeit
}

impl Module {
    const MODULES : [&str; 1]= ["zeit"];
}


pub fn from_config(config: Config) -> Result<impl Game, String> {
    match &config.module {
        Module::Zeit => zeit::get_game(config)
    }
}
        

impl TryFrom<String> for Module {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &s[..] {
            "zeit" => Ok(Self::Zeit),
            _ => Err(format!("No module with identifier \"{}\" found.\nModule keywords are {:?}", s, Module::MODULES))
        }
    }
}

#[derive(Debug)]
pub struct Config {
    module: Module,
    args: Vec<String>,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result< Self, String >{
        args.pop(); // discard bin name 
        let module: Module = args
            .pop()
            .ok_or(String::from("No arguments found"))?
            .try_into()?;
        Ok(Config {module, args})
    }
}
