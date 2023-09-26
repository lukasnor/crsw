use strum_macros::{ EnumVariantNames, Display };
use strum::VariantNames;

pub trait Game {
    /// Returns the LaTeX code for the puzzle
    fn latex(&self) -> String;

    /// Returns a formatted string showing the solution (not necessarily LaTeX)
    fn solution(&self) -> String;
}

mod zeit;

#[derive(Debug, EnumVariantNames, PartialEq, Eq, Display)]
#[strum(serialize_all="lowercase")]
pub enum Module {
    Zeit, Foo
}

impl TryFrom<String> for Module {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match &s[..] {
            "zeit" => Ok(Self::Zeit),
            _ => Err(format!(
                "No module with identifier \"{}\" found.\nModule keywords are {:?}",
                s,
                Module::VARIANTS
            )),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub module: Module,
    args: Vec<String>,
}

impl Config {
    pub fn build(mut args: Vec<String>) -> Result<Self, String> {
        args.pop(); // discard bin name
        let module: Module = args
            .pop()
            .ok_or(String::from("No arguments found"))?
            .try_into()?;
        Ok(Config { module, args })
    }

    pub fn execute(&mut self) -> Result<(), String> {
        match &self.module {
            Module::Zeit => zeit::execute(self),
            Module::Foo => todo!(),
        }
    }
}
