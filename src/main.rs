use std::fs::File;
use std::io::Write;

use reqwest;
use structopt::StructOpt;

use cli::Cli;

mod cli;

const CONTEXT: &str = "/service/rest/v1/search/assets/download";

fn main() {
    // Get args
    let args: Cli = Cli::from_args();
    // Check if URL end with "/"
    let url_checked: &str;
    if args.url.ends_with("/") {
        url_checked = &args.url[0..args.url.chars().count() - 1];
    } else {
        url_checked = args.url.as_str();
    }
    // Check if args.path exists and is a directory
    if !args.path.exists() || !args.path.is_dir() {
        println!("Error: The informed path is not a directory or not exists");
        std::process::exit(-9);
    }
    // Split artifact
    let artifact_parts = args.artifact.split(":").collect::<Vec<&str>>();
    if artifact_parts.len() != 3 {
        println!("Error: Artifact is not in the correct format \"groupId:artifactId:version\"");
        std::process::exit(-2);
    }
    // Check if sort is present
    let request_string = if args.sort.is_some() {
        format!(
            "{}{}?repository={}&maven.groupId={}&maven.artifactId={}&maven.baseVersion={}&maven.extension={}&sort={}",
            url_checked,
            CONTEXT,
            args.repo,
            artifact_parts[0],
            artifact_parts[1],
            artifact_parts[2],
            args.extension,
            args.sort.unwrap()
        )
    } else {
        format!(
            "{}{}?repository={}&maven.groupId={}&maven.artifactId={}&maven.baseVersion={}&maven.extension={}",
            url_checked,
            CONTEXT,
            args.repo,
            artifact_parts[0],
            artifact_parts[1],
            artifact_parts[2],
            args.extension
        )
    };
    // Make the request
    let client = reqwest::blocking::Client::new();
    let builder;
    if args.auth.is_some() {
        builder = client
            .get(&request_string)
            .header("Authorization", format!("Basic {}", args.auth.unwrap()));
    } else {
        builder = client.get(&request_string);
    }
    let req = builder.send();
    // Check the result
    if req.is_err() {
        println!("Error during the request: {}", req.err().unwrap());
        std::process::exit(-3);
    }
    let res = req.unwrap();
    if !res.status().is_success() {
        println!("Error during the request: {}", res.text().unwrap());
        std::process::exit(-4);
    }
    // Create file name
    let file_name = format!(
        "{}-{}.{}",
        artifact_parts[1], artifact_parts[2], args.extension
    );
    // Write downloaded file
    let content = res.bytes();
    if content.is_err() {
        println!("Error during while writing file: {}", content.err().unwrap());
        std::process::exit(-5);
    }
    let path_final = args.path.join(file_name);
    let out = File::create(&path_final);
    if out.is_err() {
        println!("Error during while writing file: {}", out.err().unwrap());
        std::process::exit(-6);
    }
    let mut out_checked = out.unwrap();
    if content.is_err() {
        println!("Error during while writing file: {}", content.err().unwrap());
        std::process::exit(-7);
    }
    let result_file = out_checked.write_all(&*content.unwrap());
    if result_file.is_err() {
        println!("Error during while writing file: {}", result_file.err().unwrap());
        std::process::exit(-8);
    }

    println!("File download at: {}", path_final.display());
}
