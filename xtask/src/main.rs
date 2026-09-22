use anyhow::{Context, Result, bail};
use book_converter::LibraryConverter;
use clap::{Parser, Subcommand};
use std::env::consts::DLL_PREFIX;
use std::env::consts::DLL_SUFFIX;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const NUMWORKS_TARGET: &str = "thumbv7em-none-eabihf";
const SIMULATOR_ARTIFACT: &str = "numworks_ereader";

#[derive(Parser)]
#[command(about = "Development tools for numworks-ereader")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the e-reader in the simulator.
    Sim {
        #[arg(required = true, value_name = "EPUB")]
        books: Vec<PathBuf>,
    },

    /// Build and install the e-reader on a calculator.
    Deploy {
        #[arg(required = true, value_name = "EPUB")]
        books: Vec<PathBuf>,
    },
}

struct Xtask {
    root: PathBuf,
    library: PathBuf,
}

impl Xtask {
    fn new() -> Self {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask must be inside the project root")
            .to_path_buf();
        let library = root.join("target/ereader/library.nwlib");

        Self { root, library }
    }

    fn build_library(&self, books: &[PathBuf]) -> Result<()> {
        let directory = self
            .library
            .parent()
            .expect("library must have a parent directory");

        fs::create_dir_all(directory)?;

        let data = LibraryConverter::new()
            .convert(books)
            .map_err(|_| anyhow::anyhow!("failed to build library"))?;

        fs::write(&self.library, data)?;

        Ok(())
    }

    fn build_simulator(&self) -> Result<PathBuf> {
        let status = Command::new("cargo")
            .current_dir(&self.root)
            .args(["build", "--lib"])
            .status()
            .context("failed to run cargo build")?;

        if !status.success() {
            bail!("failed to build simulator app");
        }

        let artifact = self
            .root
            .join("target/debug")
            .join(format!("{DLL_PREFIX}{SIMULATOR_ARTIFACT}{DLL_SUFFIX}"));

        if !artifact.exists() {
            bail!("simulator artifact not found: {}", artifact.display());
        }

        Ok(artifact)
    }

    fn sim(&self, books: &[PathBuf]) -> Result<()> {
        self.build_library(books)?;

        let app = self.build_simulator()?;

        let simulator = env::var_os("EPSILON_SIMULATOR")
            .map(PathBuf::from)
            .unwrap_or_else(|| self.root.join("epsilon.bin"));

        let status = Command::new(&simulator)
            .current_dir(&self.root)
            .env("NUMWORKS_EREADER_LIBRARY", &self.library)
            .arg("--nwb")
            .arg(app)
            .status()
            .with_context(|| format!("failed to run {}", simulator.display()))?;

        if !status.success() {
            bail!("simulator exited unsuccessfully");
        }

        Ok(())
    }

    fn deploy(&self, books: &[PathBuf]) -> Result<()> {
        self.build_library(books)?;

        let status = Command::new("cargo")
            .current_dir(&self.root)
            .args(["run", "--target", NUMWORKS_TARGET])
            .status()
            .context("failed to run cargo")?;

        if !status.success() {
            bail!("deployment failed");
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let xtask = Xtask::new();

    match cli.command {
        Commands::Sim { books } => {
            xtask.sim(&books)?;
        }
        Commands::Deploy { books } => {
            xtask.deploy(&books)?;
        }
    }

    Ok(())
}
