use clap::{CommandFactory, Parser};
use std::path::PathBuf;

use meread::{comrak_config::ComrakConfig, export::export, watch_and_serve};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to markdown file or directory containing README.md
    #[arg(default_value = ".")]
    path: PathBuf,

    /// If supplied, will export the markdown file to HTML in the specified directory
    #[arg(long, short)]
    export_dir: Option<PathBuf>,

    /// Whether to overwrite the export directory if it exists
    #[arg(long, short)]
    force: bool,

    /// Address to bind the server to
    #[arg(long, short, default_value = "localhost:3000")]
    address: String,

    /// Whether to open the browser on serve
    #[arg(long, short)]
    open: bool,

    /// Render page in light-mode style
    #[arg(long, short)]
    light_mode: bool,

    /// Print manpage to stdout and exit
    #[arg(long)]
    generate_manpage: bool,
}

fn main() -> color_eyre::Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .display_location_section(cfg!(debug_assertions))
        .install()?;

    let args = Args::parse();

    if args.generate_manpage {
        let cmd = Args::command();
        clap_mangen::Man::new(cmd).render(&mut std::io::stdout())?;
        return Ok(());
    }

    let markdown_file_path = if args.path.is_dir() {
        args.path.join("README.md")
    } else {
        args.path
    };

    let comrak_config = ComrakConfig::new(args.light_mode)?;

    if let Some(export_dir) = &args.export_dir {
        export(
            &markdown_file_path,
            export_dir,
            args.force,
            args.light_mode,
            &comrak_config,
        )?;
        return Ok(());
    }

    watch_and_serve(
        &markdown_file_path,
        args.light_mode,
        comrak_config,
        &args.address,
        args.open,
    )
}
