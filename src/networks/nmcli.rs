use core::panic;
use std::{
    io::Error,
    process::{Command, Output},
    string::FromUtf8Error,
};

pub fn get_available_networks() -> Result<Vec<Vec<String>>, Error> {
    let output = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()?;

    match convert_stdout(output) {
        Ok(x) => Ok(x
            .lines()
            .skip(1)
            .map(|l| {
                let mut parsed = l[8..]
                    .split(' ')
                    .filter(|f| f.len() > 0)
                    .map(|f| f.to_string())
                    .collect::<Vec<String>>();
                parsed.push(l[..1].to_string());
                return parsed;
            })
            .collect()),
        Err(x) => panic!("Error in string convertion - {}", x.to_string()),
    }
}

pub fn get_active_network() -> Result<String, Error> {
    let output = Command::new("nmcli")
        .arg("con")
        .arg("show")
        .arg("--active")
        .output()?;

    match convert_stdout(output) {
        Ok(x) => Ok(x
            .lines()
            .skip(1)
            .take(1)
            .map(|x| {
                x.split(' ')
                    .take(1)
                    .map(|f| f.to_string())
                    .collect::<String>()
            })
            .collect::<String>()),
        Err(x) => panic!("Error in string convertion - {}", x.to_string()),
    }
}

pub fn connect_network(network: String, password: String) -> Result<bool, Error> {
    Command::new("nmcli")
        .arg("device wifi connect")
        .arg(network)
        .arg("password")
        .arg(password)
        .output()?;

    Ok(true)
}

fn convert_stdout(output: Output) -> Result<String, FromUtf8Error> {
    Ok(String::from_utf8(output.stdout)?)
}
