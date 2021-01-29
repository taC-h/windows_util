use std::{
    process::Command,
    env,
    io,
    path::PathBuf,
};
use dirs::home_dir;

fn main() -> io::Result<()> {
    let (devpath, name) = gen_dev_path()?;
    let modpath = gen_py_mod_dir(&format!{"{}.pyd",name});
    let output =
        Command::new("cmd")
        .args(&[
            "/C",
            "mklink",
            &format!("{}",modpath.display()),
            &format!("{}", devpath.display())
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

fn gen_py_mod_dir(name: &str) -> PathBuf {
    let mut env = home_dir().unwrap();
    [
        "anaconda3",
        "Lib",
        "site-packages",
        "mymod",
        name,
    ].iter().for_each(|p|env.push(p));
    env
}

fn gen_dev_path() -> io::Result<(PathBuf,String)> {
    let mut current = env::current_dir()?;
    let tmp = current.clone();
    let name = tmp.to_str().unwrap().split("\\").collect::<Vec<&str>>().pop().unwrap();
    [
        "target",
        "release",
        &format!("{}.dll",name)
    ].iter().for_each(|p|current.push(p));
    Ok((current, name.into()))
}