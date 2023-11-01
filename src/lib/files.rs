#![allow(dead_code)]

use std::fs::{read_to_string, File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn read_lines_iterable<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub trait Instantiable {
    fn new(values: Vec<&str>) -> Self;
}

pub fn read_lines_into_structs<T>(filename: &str, separator: &str) -> Vec<T>
where
    T: Instantiable,
{
    let mut vec: Vec<T> = Vec::new();
    let lines_res = read_lines_iterable(filename);

    if let Ok(lines) = lines_res {
        for line in lines {
            if let Ok(l) = line {
                let values: Vec<&str> = l.split(separator).collect();
                let result = T::new(values);

                vec.push(result);
            }
        }
    }

    return vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_lines() -> io::Result<()> {
        let mut lines = read_lines_iterable("test/test_input.txt")?;

        assert_eq!("forward 4", lines.next().unwrap().unwrap());
        assert_eq!("down 9", lines.next().unwrap().unwrap());
        assert_eq!("forward 6", lines.next().unwrap().unwrap());
        assert_eq!("down 5", lines.next().unwrap().unwrap());
        assert_eq!("up 2", lines.next().unwrap().unwrap());

        assert!(lines.next().is_none());

        Ok(())
    }

    #[test]
    fn test_read_lines_into_structs() -> io::Result<()> {
        #[derive(Debug, PartialEq)]
        struct Command {
            dir: String,
            value: i32,
        }

        impl Instantiable for Command {
            fn new(values: Vec<&str>) -> Self {
                Command {
                    dir: values[0].to_string(),
                    value: values[1].parse::<i32>().unwrap(),
                }
            }
        }

        let commands = read_lines_into_structs::<Command>("test/test_input.txt", " ");

        assert_eq!(
            commands[0],
            Command {
                dir: String::from("forward"),
                value: 4
            }
        );

        assert_eq!(
            commands[4],
            Command {
                dir: String::from("up"),
                value: 2
            }
        );

        Ok(())
    }
}
