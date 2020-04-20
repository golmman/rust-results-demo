use crate::is_zero::is_zero;
use crate::is_zero::is_zero2;

mod error;
mod is_zero;

fn file_exists(file_name: &str) -> std::io::Result<String> {
    match file_name {
        "testfile.txt" => Ok(String::from("file exists")),
        _ => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "file not found",
        )),
    }
}

fn create_file_name(
    base_name: &str,
    extension: &str,
) -> crate::error::Result<String> {
    if base_name.len() > 8 {
        return Err(crate::error::Error::new(
            crate::error::ErrorKind::BaseNameTooLong,
            "base name too long",
        ));
    }

    if extension.len() > 3 {
        return Err(crate::error::Error::new(
            crate::error::ErrorKind::ExtensionTooLong,
            "extension too long",
        ));
    }

    Ok(format!("{}.{}", base_name, extension))
}

fn check_file_and_zero(
    file_name: &str,
    number: i32,
) -> std::result::Result<bool, Box<dyn std::error::Error>> {
    file_exists(file_name)?;
    is_zero(number)?;
    Ok(true)
}

fn check_file_and_zero2(
    file_name: &str,
    number: i32,
) -> crate::error::Result<bool> {
    file_exists(file_name)?;
    is_zero(number)?;
    Ok(true)
}

fn check_file_and_zero3(
    base_name: &str,
    extension: &str,
    number: i32,
) -> crate::error::Result<bool> {
    let file_name = create_file_name(base_name, extension)?;
    file_exists(&file_name)?;
    is_zero(number)?;
    Ok(true)
}

fn convert_check(base_name: &str, extension: &str, number: i32) -> String {
    let check_result = check_file_and_zero3(base_name, extension, number);

    match check_result {
        Ok(res) => format!("0 {}", res),
        Err(err) => match err.kind() {
            crate::error::ErrorKind::IoErrorKind(io_error_kind) => {
                format!("1 {:?} {}", io_error_kind, err.message())
            }
            crate::error::ErrorKind::BaseNameTooLong => {
                format!("2 {}", err.message())
            }
            crate::error::ErrorKind::ExtensionTooLong => {
                format!("3 {}", err.message())
            }
            crate::error::ErrorKind::Positive => format!("4 {}", err.message()),
            crate::error::ErrorKind::Negative => format!("5 {}", err.message()),
        },
    }
}

fn main() {
    println!("Hello, world!");

    println!("{:?}", is_zero(0));
    println!("{:?}", is_zero(-50));
    println!("{:?}", is_zero(6));

    println!("{:?}", is_zero2(0));
    println!("{:?}", is_zero2(-50));
    println!("{:?}", is_zero2(6));

    println!("{:?}", check_file_and_zero("testfile.txt", 0));
    println!("{:?}", check_file_and_zero("nonsense.txt", 0));
    println!("{:?}", check_file_and_zero("testfile.txt", 1));

    println!("{:?}", check_file_and_zero2("testfile.txt", 0));
    println!("{:?}", check_file_and_zero2("nonsense.txt", 0));
    println!("{:?}", check_file_and_zero2("testfile.txt", 1));

    println!("{:?}", check_file_and_zero3("testfile", "txt", 0));
    println!("{:?}", check_file_and_zero3("testfil", "txt", 0));
    println!("{:?}", check_file_and_zero3("testfile1", "txt", 0));
    println!("{:?}", check_file_and_zero3("testfile", "txt1", 0));
    println!("{:?}", check_file_and_zero3("testfile", "txt", 2));
    println!("{:?}", check_file_and_zero3("testfile", "txt", -1));

    println!("{:?}", convert_check("testfile", "txt", 0));
    println!("{:?}", convert_check("testfil", "txt", 0));
    println!("{:?}", convert_check("testfile1", "txt", 0));
    println!("{:?}", convert_check("testfile", "txt1", 0));
    println!("{:?}", convert_check("testfile", "txt", 2));
    println!("{:?}", convert_check("testfile", "txt", -1));
}
