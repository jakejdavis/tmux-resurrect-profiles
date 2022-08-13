use std::fs;
use std::env;
use dialoguer::{
    Select,
    theme::ColorfulTheme,
    console::Term
};

const RESURRECT_EXCLUDE_LIST: &'static [&'static str] = &["profiles"];

pub fn create(profile: &str) -> std::io::Result<()> {
    let resurrect_dir = env::var("HOME").unwrap() + "/.tmux/resurrect/";
    let profile_dir = resurrect_dir.to_owned() + "/profiles/";

    // Create the profile directory if it doesn't exist
    if !fs::metadata(&profile_dir).is_ok() {
        fs::create_dir(&profile_dir).unwrap();
    }


    // Get a list of existing profiles in resurrect directory
    let files = fs::read_dir(&resurrect_dir).unwrap();
    let names = files.filter_map(|entry| {
      entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
      )
    }).filter(|n| !RESURRECT_EXCLUDE_LIST.contains(&n.as_str())).collect::<Vec<String>>();
    
    // Ask the user to select an existing profile
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&names)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    // Create the symlink to the selected profile
    let save_path;
    match selection {
        Some(index) => {
            save_path = resurrect_dir.to_owned() + &names[index]; 
        }
        None => {
            println!("Nothing selected");
            return Ok(());
        }
    }

    let profile_path = profile_dir.to_owned() + profile;
    fs::copy(&save_path, &profile_path)?;
    println!("Created profile {} at {}", profile, &profile_path);

    return Ok(());
}
