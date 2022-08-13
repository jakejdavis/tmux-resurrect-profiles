use clap::{Parser, Subcommand};

mod cmds {
    pub mod create;
    pub mod select;
}
use cmds::create::create;
use cmds::select::select;

#[derive(Debug, Parser)] 
#[clap(name = "tmux-resurrect-profiles")]
#[clap(bin_name = "tmux-resurrect-profiles")]
#[clap(about = "CLI for switching between multiple tmux-resurrect profiles", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "Select a profile", long_about = None)]
    Select {
        #[clap(value_parser, required = false, help = "Name of profile to select")]
        profile: Option<String>,
    },

    #[clap(about = "Select a profile and launch tmux", long_about = None)]
    Launch {
        #[clap(value_parser, required = false, help = "Name of profile to launch")]
        profile: Option<String>,
    },

    #[clap(arg_required_else_help = true, about = "Create a profile from existing tmux-resurrect saves", long_about = None)]
    Create {
        #[clap(value_parser, help = "Name of profile to create")]
        profile: String,
    }
}

fn main() {
    let args = Cli::parse();

    let result = match args.command {
        Commands::Select { profile } => select(&profile), 
        Commands::Launch { profile } =>
            select(&profile).and_then(|_| {
                std::process::Command::new("tmux")
                    .arg("kill-server")
                    .spawn()?
                    .wait()?;

                std::process::Command::new("tmux")
                    .spawn()?
                    .wait()?;
                Ok(())
            }),
        Commands::Create { profile } => create(&profile)
    };

    // Output error if there was one
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
