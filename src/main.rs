use url::Url;
use zbox::RepoOpener;

fn main() {
    zbox::init_env();
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("test-fs");

    if let Err(err) = std::fs::remove_dir_all(&path) {
        if err.kind() != std::io::ErrorKind::NotFound {
            panic!("Unable to remove archive file: {}", err);
        }
    }

    let url = Url::from_file_path(&path).unwrap();

    eprintln!("Opening URL: {}", &url);
    RepoOpener::new()
        .create_new(true)
        .open(&format!("{}", url), "abcdef")
        .expect("opening archive failed");

    eprintln!("Success!");
}
