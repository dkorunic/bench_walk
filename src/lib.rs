use project_root::get_project_root;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

#[allow(clippy::missing_errors_doc)]
pub fn prepare_test_dir(sub_dir: &str) -> Result<PathBuf, Error> {
    let target_dir = get_project_root()?;
    let target_dir = target_dir.as_path().join(sub_dir);

    if !target_dir.exists() {
        println!("Cloning to {target_dir:?}");

        Command::new("git")
            .args([
                "clone",
                "https://github.com/torvalds/linux.git",
                "--depth",
                "1",
                &target_dir.display().to_string(),
            ])
            .output()?;
        Command::new("sync").output()?;

        println!("Cloning completed.");
    }

    Ok(target_dir)
}

pub fn find_walkdir(root: impl AsRef<Path>) {
    let root = root.as_ref();

    Command::new("find").arg(root).output().unwrap();
}

pub fn fts_walkdir(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in fts::walkdir::WalkDir::new(
        fts::walkdir::WalkDirConf::new(root).no_metadata(),
    ) {}
}

pub fn regular_walkdir(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in walkdir::WalkDir::new(root) {}
}

pub fn ignore_serial(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in ignore::WalkBuilder::new(root)
        .hidden(false)
        .standard_filters(false)
        .build()
    {}
}
pub fn ignore_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    ignore::WalkBuilder::new(root)
        .hidden(false)
        .standard_filters(false)
        .threads(num_cpus::get())
        .build_parallel()
        .run(move || Box::new(move |_| ignore::WalkState::Continue));
}
pub fn jwalk_serial(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in jwalk::WalkDir::new(root).parallelism(jwalk::Parallelism::Serial)
    {
    }
}
pub fn jwalk_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in jwalk::WalkDir::new(root) {}
}
