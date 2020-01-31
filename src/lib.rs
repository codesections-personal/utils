use atty::Stream;
pub use clap::crate_name;
pub use run_script::run_script;
use std::error::Error;

pub fn sh(script: &str) -> Result<(String, String), Box<dyn Error>> {
    let (code, stdout, stderr) = run_script!(script)?;
    if code != 0 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            stderr,
        )))
    } else {
        Ok((stdout, stderr))
    }
}

pub fn dependencies(dependencies: Vec<&str>) -> Result<(), Box<dyn Error>> {
    for dependency in dependencies {
        sh(&format!("which {}", dependency))?;
    }
    Ok(())
}

pub trait Die<T> {
    fn unwrap_or_die(self) -> T;
    fn or_die(self) -> T;
}

impl<T, E> Die<T> for Result<T, E>
where
    E: std::fmt::Display,
{
    fn unwrap_or_die(self) -> T {
        match self {
            Ok(t) => t,
            Err(err) if atty::is(Stream::Stdout) => {
                eprintln!("Err: {}", err);
                std::process::exit(1);
            }
            Err(err) => {
                eprint!("{}", err);
                std::process::exit(1);
            }
        }
    }
    fn or_die(self) -> T {
        self.unwrap_or_die()
    }
}
