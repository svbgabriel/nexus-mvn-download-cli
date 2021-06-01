use std::path::PathBuf;

use structopt::StructOpt;

/// Download an artifact from a maven repository in a Nexus Server
#[derive(StructOpt)]
pub struct Cli {
    /// URL of the Nexus Server
    #[structopt(short = "u", long = "url")]
    pub url: String,
    /// The repository name
    #[structopt(short = "r", long = "repository")]
    pub repo: String,
    /// The artifact in gradle style groupId:artifactId:version
    #[structopt(short = "a", long = "artifact")]
    pub artifact: String,
    /// The extension of artifact, like jar
    #[structopt(short = "e", long = "extension")]
    pub extension: String,
    /// Sorting method, necessary if multiple versions are found. Useful with SNAPSHOT
    #[structopt(short = "s", long = "sort")]
    pub sort: Option<String>,
    /// A base64 encoded of username:password, required if the repository is not public
    #[structopt(short = "c", long = "credentials")]
    pub auth: Option<String>,
    /// The path to the directory where the artifact will be saved
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    pub path: PathBuf,
}
