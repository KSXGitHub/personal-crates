use cargo_toml::Manifest;
use clap::Parser;
use pipe_trait::*;
use std::{
    env,
    fs::{copy, read_dir, read_to_string, write, OpenOptions},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, Parser)]
#[clap(name = "script")]
enum CliArgs {
    /// Generate `pkgbuild/PKGBUILD` from `template/PKGBUILD`
    #[clap(name = "pkgbuild")]
    GeneratePkgBuild {
        /// Build profile
        #[clap(value_parser = ["release", "debug"])]
        profile: String,
    },

    /// Determine whether to deploy [GitHub Actions]
    #[clap(name = "should-deploy")]
    ShouldDeploy {
        /// Current branch/tag/ref, e.g. ${{ github.ref }}
        git_ref: String,
    },
}

fn main() {
    let workspace_root = env!("CARGO_MANIFEST_DIR")
        .pipe(PathBuf::from)
        .parent()
        .expect("get parent of CARGO_MANIFEST_DIR")
        .to_path_buf();

    let crate_container = workspace_root.join("crates");

    match CliArgs::parse() {
        CliArgs::GeneratePkgBuild { profile } => {
            let pkgbuild_directory = workspace_root.join("pkgbuild");
            let mut binary_names = Vec::new();

            for entry in read_dir(&crate_container).expect("read crate container") {
                let manifest_path = entry
                    .expect("read individual entry")
                    .path()
                    .join("Cargo.toml");

                if !manifest_path.exists() {
                    continue;
                }

                eprintln!("{}", manifest_path.display());

                let local_binary_names: Vec<_> = manifest_path
                    .pipe_ref(Manifest::from_path)
                    .expect("load manifest content")
                    .bin
                    .into_iter()
                    .map(|product| product.name.expect("access bin.name"))
                    .collect();

                binary_names.extend_from_slice(&local_binary_names);

                for name in &local_binary_names {
                    eprintln!("  → {name}");
                    copy(
                        workspace_root.join("target").join(&profile).join(name),
                        pkgbuild_directory.join(name),
                    )
                    .expect("copy binary file to pkgbuild location");
                }
            }

            let binary_checksums = binary_names
                .iter()
                .map(|_| "SKIP")
                .collect::<Vec<_>>()
                .join(" ");

            write(
                pkgbuild_directory.join("PKGBUILD"),
                workspace_root
                    .join("template")
                    .join("PKGBUILD")
                    .pipe(read_to_string)
                    .expect("read PKGBUILD template")
                    .replace("VERSION", version())
                    .replace("BINARY_NAMES", &binary_names.join(" "))
                    .replace("BINARY_CHECKSUMS", &binary_checksums),
            )
            .expect("write content to PKGBUILD");
        }

        CliArgs::ShouldDeploy { git_ref } => {
            fn remove_prefix(prefix: &'static str) -> impl FnOnce(&str) -> &str {
                move |text| text.strip_prefix(prefix).unwrap_or(text)
            }

            let mut github_output = match env::var_os("GITHUB_OUTPUT")
                .map(|path| OpenOptions::new().create(true).append(true).open(path))
            {
                Some(Ok(file)) => Some(file),
                Some(Err(error)) => {
                    eprintln!("warning: Failed to open $GITHUB_OUTPUT");
                    eprintln!("{error}");
                    None
                }
                None => None,
            };

            macro_rules! set_output {
                ($name:expr, $value:expr) => {{
                    let name = $name;
                    let value = $value;
                    if let Some(file) = &mut github_output {
                        writeln!(file, "{name}={value}").unwrap_or_else(|error| {
                            panic!("Failed to write variable {name:?} in file {file:?}: {error}",)
                        });
                    } else {
                        println!("::set-output name={name}::{value}");
                    }
                }};
            }

            let git_ref = git_ref
                .as_str()
                .pipe(remove_prefix("refs/heads/"))
                .pipe(remove_prefix("refs/tags/"))
                .pipe(remove_prefix("refs/branches/"));

            let version = version();
            let should_deploy = git_ref == version;
            let build_profile = if should_deploy { "release" } else { "debug" };
            dbg!(git_ref, version, should_deploy, build_profile);
            set_output!("git_ref", git_ref);
            set_output!("should_deploy", should_deploy);
            set_output!("build_profile", build_profile);
        }
    }
}

fn version() -> &'static str {
    include_str!("../../version").trim()
}
