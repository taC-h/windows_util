use std::{
    process::Command,
    env,
    io
};

fn main() -> io::Result<()> {
    let path = env::current_dir()?;
    let name = path.to_str().unwrap().split("\\").collect::<Vec<&str>>().pop().unwrap();
    let output =
        Command::new("cmd")
        .args(&[
            "/C",
            "mklink",
            &format!("..\\..\\{}.exe",name),
            &format!("dev\\{0}\\target\\release\\{0}.exe",name)
            ])
        .output()?;
    let if_none = r#"
    this shell is no use utf-8
    execute command "chcp 65001" so that UTF-8 can be displayed
    "#;
    println!("stdout:{:?}",std::str::from_utf8(&output.stdout).unwrap_or(if_none));
    eprintln!("stderr{:?}",std::str::from_utf8(&output.stderr).unwrap_or(if_none));
    std::process::exit(output.status.code().unwrap())
}
