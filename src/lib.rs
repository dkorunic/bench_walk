use project_root::get_project_root;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Clones the Linux kernel repository into `sub_dir` (relative to the project
/// root) on first use, returning the absolute path to that directory.
///
/// # Errors
///
/// Returns an [`Error`] if the project root cannot be determined or if the
/// `git clone`/`sync` commands cannot be spawned.
pub fn prepare_test_dir(sub_dir: &str) -> Result<PathBuf, Error> {
    let target_dir = get_project_root()?;
    let target_dir = target_dir.as_path().join(sub_dir);

    if !target_dir.exists() {
        println!("Cloning to {}", target_dir.display());

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

/// Walks `root` by shelling out to the system `find` command.
///
/// # Panics
///
/// Panics if the `find` command cannot be spawned.
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

pub fn isideload_walkdir(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in isideload_walkdir::WalkDir::new(root) {}
}

/// Walks `root` using the `walker` crate.
///
/// # Panics
///
/// Panics if `root` cannot be opened.
pub fn walker_walkdir(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in walker::Walker::new(root)
        .expect("walker: unable to open root directory")
    {}
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
/// Walks `root` using the `walkdir_minimal` crate.
///
/// # Panics
///
/// Panics if `root` cannot be opened.
pub fn walkdir_minimal(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in walkdir_minimal::WalkDir::new(root)
        .expect("walkdir_minimal: unable to open root directory")
    {}
}
pub fn fs_walk_serial(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in fs_walk::WalkOptions::new().walk(root) {}
}
/// Builds an `fsindex` configuration that traverses the whole tree (no
/// `.gitignore` handling, hidden files included) and skips reading file
/// contents, so it measures traversal rather than I/O throughput.
fn fsindex_config() -> fsindex::Config {
    fsindex::Config::builder()
        .respect_gitignore(false)
        .include_hidden(true)
        .read_contents(false)
        .build()
}
pub fn fsindex_serial(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in fsindex::FileIndexer::with_config(root, fsindex_config()).files()
    {
    }
}
pub fn jwalk_serial(root: impl AsRef<Path>) {
    let root = root.as_ref();

    // `skip_hidden` defaults to true; disable it so jwalk traverses the full
    // tree (including `.git`) like every other crate here.
    for _ in jwalk::WalkDir::new(root)
        .parallelism(jwalk::Parallelism::Serial)
        .skip_hidden(false)
    {}
}
pub fn async_walkdir(root: impl AsRef<Path>) {
    use futures_lite::stream::StreamExt;

    let root = root.as_ref();

    futures_lite::future::block_on(async {
        let mut entries = async_walkdir::WalkDir::new(root);
        while entries.next().await.is_some() {}
    });
}
pub fn jwalk_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    for _ in jwalk::WalkDir::new(root).skip_hidden(false) {}
}
/// Walks `root` using the `scandir` crate, which traverses in parallel
/// internally (backed by a `jwalk` fork).
///
/// # Panics
///
/// Panics if `root` cannot be opened or if traversal fails.
pub fn scandir_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    // `store = Some(false)` keeps scandir from retaining every entry in
    // memory; `collect()` still drains the full traversal off its channel.
    // `skip_hidden` defaults to true, so disable it to walk the full tree.
    let mut walk = scandir::Walk::new(root, Some(false))
        .expect("scandir: unable to open root directory")
        .skip_hidden(false);
    walk.collect().expect("scandir: traversal failed");
}
pub fn swdir_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    swdir::Swdir::new()
        .root_path(root)
        .recurse(swdir::Recurse::Unlimited)
        .max_threads(num_cpus::get())
        .sort_order(swdir::SortOrder::Filesystem)
        .clear_filters()
        .walk();
}
pub fn fsindex_parallel(root: impl AsRef<Path>) {
    let root = root.as_ref();

    fsindex::FileIndexer::with_config(root, fsindex_config()).files_parallel();
}
