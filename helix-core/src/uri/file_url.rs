use std::path::{Path, PathBuf};

use url::Url;

#[cfg(not(target_os = "motor"))]
pub fn from_file_path(path: impl AsRef<Path>) -> Result<Url, ()> {
    Url::from_file_path(path)
}

#[cfg(target_os = "motor")]
pub fn from_file_path(path: impl AsRef<Path>) -> Result<Url, ()> {
    motor_url_from_path(path.as_ref(), false)
}

#[cfg(not(target_os = "motor"))]
pub fn from_directory_path(path: impl AsRef<Path>) -> Result<Url, ()> {
    Url::from_directory_path(path)
}

#[cfg(target_os = "motor")]
pub fn from_directory_path(path: impl AsRef<Path>) -> Result<Url, ()> {
    motor_url_from_path(path.as_ref(), true)
}

#[cfg(not(target_os = "motor"))]
pub fn to_file_path(url: &Url) -> Result<PathBuf, ()> {
    url.to_file_path()
}

#[cfg(target_os = "motor")]
pub fn to_file_path(url: &Url) -> Result<PathBuf, ()> {
    motor_path_from_url(url)
}

#[cfg(any(target_os = "motor", test))]
fn motor_url_from_path(path: &Path, directory: bool) -> Result<Url, ()> {
    use std::path::Component;

    if !path.is_absolute() {
        return Err(());
    }

    let path = helix_stdx::path::normalize(path);
    let mut url = Url::parse("file:///").map_err(|_| ())?;
    let mut segments = url.path_segments_mut().map_err(|_| ())?;
    segments.clear();

    let mut has_segment = false;
    for component in path.components() {
        match component {
            Component::RootDir => (),
            Component::Normal(component) => {
                segments.push(component.to_str().ok_or(())?);
                has_segment = true;
            }
            Component::CurDir => (),
            Component::ParentDir | Component::Prefix(_) => return Err(()),
        }
    }
    if directory && has_segment {
        segments.push("");
    }
    drop(segments);
    Ok(url)
}

#[cfg(any(target_os = "motor", test))]
fn motor_path_from_url(url: &Url) -> Result<PathBuf, ()> {
    use percent_encoding::percent_decode_str;

    if url.scheme() != "file" || !matches!(url.host_str(), None | Some("localhost")) {
        return Err(());
    }

    let mut path = PathBuf::from(std::path::MAIN_SEPARATOR_STR);
    for segment in url.path_segments().ok_or(())? {
        let segment = percent_decode_str(segment).decode_utf8().map_err(|_| ())?;
        path.push(segment.as_ref());
    }
    Ok(path)
}

#[cfg(all(test, not(windows)))]
mod tests {
    use super::*;

    #[test]
    fn motor_file_urls_match_host_urls_for_utf8_paths() {
        for path in ["/", "/tmp/file", "/tmp/a b#?%", "/tmp/λ\\name"] {
            let path = Path::new(path);
            assert_eq!(motor_url_from_path(path, false), Url::from_file_path(path));
        }

        let directory = Path::new("/tmp/a b");
        assert_eq!(
            motor_url_from_path(directory, true),
            Url::from_directory_path(directory)
        );
        assert!(motor_url_from_path(Path::new("relative"), false).is_err());
    }

    #[test]
    fn motor_file_urls_round_trip_utf8_paths() {
        let path = Path::new("/tmp/a b#?%/λ\\name");
        let url = motor_url_from_path(path, false).unwrap();
        assert_eq!(motor_path_from_url(&url), Ok(path.into()));

        let localhost = Url::parse("file://localhost/tmp/a%20b").unwrap();
        assert_eq!(motor_path_from_url(&localhost), Ok("/tmp/a b".into()));
    }

    #[test]
    fn motor_file_urls_reject_remote_hosts_and_non_utf8_paths() {
        assert!(motor_path_from_url(&Url::parse("https://example.com/a").unwrap()).is_err());
        assert!(motor_path_from_url(&Url::parse("file://example.com/a").unwrap()).is_err());
        assert!(motor_path_from_url(&Url::parse("file:///tmp/%FF").unwrap()).is_err());
    }
}
