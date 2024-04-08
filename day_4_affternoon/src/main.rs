#![allow(dead_code, unused, unused_variables)]

use anyhow::{bail, Context, Result};
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io::Read;
use std::{fs, io};
use thiserror::Error;

// error macros with thiserror and anyhow
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

#[derive(Debug)]
enum ReadUsernameError {
    IoError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl Display for ReadUsernameError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "IO error: {e}"),
            Self::EmptyUsername(path) => write!(f, "Found no username in {path}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn read_username_simplified(path: &str) -> Result<String, ReadUsernameError> {
    let username_file_result = fs::File::open(path);
    let mut username = String::new();
    let r = username_file_result?
        .read_to_string(&mut username)
        .map(|_| username.clone())
        .map_err(|e| ReadUsernameError::IoError(e));

    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    r
}

fn read_username_simplified_err(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
    .with_context(|| format!("Failed to open {path}"))?
    .read_to_string(&mut username)
        .context("Failed to read")?;

    if username.is_empty() {
        bail!(EmptyUsernameError(path.to_string()));
    }
    Ok(username)
}

// you can wrap error into dyn Error so that you don't need case for each separate
fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut count_str = String::new();
    fs::File::open(path)?.read_to_string(&mut count_str)?;
    let count: i32 = count_str.parse()?;
    Ok(count)
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("invalid_config.dat");
    let username2 = read_username_simplified("invalid_config.dat");
    let username3 = read_username_simplified("config.dat");
    let username4 = read_username_simplified("config_empty.dat");
    println!("username or error: {username:?}");
    println!("username or error: {username2:?}");
    println!("username or error: {username3:?}");
    println!("username or error: {username4:?}");

    // with anyhow and thiserror you get similar err handling to Golang and this prints out stacktrace as well
    println!("read_username_simplified_err ---");
    match read_username_simplified_err("invalid_config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
    match read_username_simplified_err("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }

    // will lead to error as it has alphabetics inside the file
    println!("read_count ---");
    fs::write("count.dat", "1i3").unwrap();
    match read_count("count.dat") {
        Ok(count) => println!("Count: {count}"),
        Err(err) => println!("Error: {err}"),
    }

    fs::remove_file("count.dat").unwrap(); // cleanup

    // does not exist -> IOError
    match read_count("invalid_count.dat") {
        Ok(count) => println!("Count: {count}"),
        Err(err) => println!("Error: {err}"),
    }
}
