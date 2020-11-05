use cargo_toml::Manifest;
use pipe_trait::*;
use std::{
    fs::{copy, read_dir, read_to_string, write},
    path::PathBuf,
};
use structopt::*;
use structopt_utilities::StructOptUtils;

#[derive(Debug, StructOpt)]
#[structopt(name = "script")]
enum CliArgs {
    /// Generate `pkgbuild/PKGBUILD` from `template/PKGBUILD`
    #[structopt(name = "pkgbuild")]
    GeneratePkgBuild {
        /// Build profile
        #[structopt(possible_values = &["release", "debug"])]
        profile: String,
    },

    /// Determine whether to deploy [GitHub Actions]
    #[structopt(name = "should-deploy")]
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

    match CliArgs::strict_from_args() {
        CliArgs::GeneratePkgBuild { profile } => {
            for entry in read_dir(&crate_container).expect("read crate container") {
                let manifest_path = entry
                    .expect("read individual entry")
                    .path()
                    .join("Cargo.toml");
                if !manifest_path.exists() {
                    return;
                }

                eprintln!("{}", manifest_path.to_string_lossy());

                let binary_names: Vec<_> = manifest_path
                    .pipe_ref(Manifest::from_path)
                    .expect("load manifest content")
                    .bin
                    .into_iter()
                    .map(|product| product.name.expect("access bin.name"))
                    .collect();

                let binary_checksums = binary_names
                    .iter()
                    .map(|_| "SKIP")
                    .collect::<Vec<_>>()
                    .join(" ");

                let pkgbuild_directory = workspace_root.join("pkgbuild").to_path_buf();

                write(
                    pkgbuild_directory.join("PKGBUILD"),
                    workspace_root
                        .join("template")
                        .join("PKGBUILD")
                        .pipe(read_to_string)
                        .expect("read PKGBUILD template")
                        .replace("BINARY_NAMES", &binary_names.join(" "))
                        .replace("BINARY_CHECKSUMS", &binary_checksums),
                )
                .expect("write content to PKGBUILD");

                for name in &binary_names {
                    eprintln!("  → {}", name);
                    copy(
                        workspace_root.join("target").join(&profile).join(name),
                        pkgbuild_directory.join(name),
                    )
                    .expect("copy binary file to pkgbuild location");
                }
            }
        }

        CliArgs::ShouldDeploy { git_ref } => {
            fn remove_prefix(prefix: &'static str) -> impl FnOnce(&str) -> &str {
                move |text| {
                    if text.starts_with(prefix) {
                        &text[prefix.len()..]
                    } else {
                        text
                    }
                }
            }

            let git_ref = git_ref
                .as_str()
                .pipe(remove_prefix("refs/heads/"))
                .pipe(remove_prefix("refs/tags/"))
                .pipe(remove_prefix("refs/branches/"));

            println!("::set-output name=git_ref::{}", git_ref);
            println!("::set-output name=should_deploy::{}", git_ref == "master");
        }
    }
}
