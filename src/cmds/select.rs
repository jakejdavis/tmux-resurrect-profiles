use std::fs::metadata;
use std::fs::read_dir;
use std::fs::remove_file;
use std::os::unix::fs;
use std::env;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};


pub fn select(profile_opt: &Option<std::string::String>) -> std::io::Result<()> {
    let resurrect_dir = env::var("HOME").unwrap() + "/.tmux/resurrect/";
    let profile_dir = resurrect_dir.to_owned() + "profiles/";
    
    // If profile not specified show dialog
    let profile: std::string::String;
    if profile_opt.is_none() { 
        let files = read_dir(&profile_dir).unwrap();
        let names = files.filter_map(|entry| {
          entry.ok().and_then(|e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
          )
        })
        .filter(|e| !e.starts_with("."))
        .collect::<Vec<String>>();
        
        // Ask the user to select an existing profile
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&names)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

       match selection {
        Some(index) => {
               profile = names[index].to_string();
           },
           None => {
               println!("Nothing selected");
               return Ok(());
           }
        }
    } else {
        profile = profile_opt.as_ref().unwrap().to_string();
    }


    let profile_path = profile_dir.to_owned() + &profile;
    let last_symlink = resurrect_dir.to_owned() + "last";

    if !metadata(&profile_path).is_ok() {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Profile not found")); 
    }
   
    // delete symlink if it exists
    remove_file(&last_symlink).unwrap_or_default();
    fs::symlink(&profile_path, &last_symlink)?;

    println!("Updated current profile");

    return Ok(());
}
