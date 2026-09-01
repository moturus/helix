#[macro_use]
extern crate helix_view;

pub mod application;
pub mod args;
pub mod commands;
pub mod compositor;
pub mod config;
pub mod events;
pub mod health;
pub mod job;
pub mod keymap;
pub mod ui;

use std::path::Path;

use futures_util::Future;
mod handlers;

use ignore::DirEntry;
use url::Url;

#[cfg(windows)]
fn true_color() -> bool {
    true
}

#[cfg(not(any(windows, target_os = "motor")))]
fn true_color() -> bool {
    if matches!(
        std::env::var("COLORTERM").map(|v| matches!(v.as_str(), "truecolor" | "24bit")),
        Ok(true)
    ) {
        return true;
    }

    match termini::TermInfo::from_env() {
        Ok(t) => {
            t.extended_cap("RGB").is_some()
                || t.extended_cap("Tc").is_some()
                || (t.extended_cap("setrgbf").is_some() && t.extended_cap("setrgbb").is_some())
        }
        Err(_) => false,
    }
}

#[cfg(target_os = "motor")]
fn true_color() -> bool {
    motor_true_color(std::env::var("COLORTERM").ok().as_deref())
}

#[cfg(any(target_os = "motor", test))]
fn motor_true_color(colorterm: Option<&str>) -> bool {
    matches!(colorterm, Some("truecolor" | "24bit"))
}

/// Function used for filtering dir entries in the various file pickers.
fn filter_picker_entry(entry: &DirEntry, root: &Path, dedup_symlinks: bool) -> bool {
    // We always want to ignore popular VCS directories, otherwise if
    // `ignore` is turned off, we end up with a lot of noise
    // in our picker.
    if matches!(
        entry.file_name().to_str(),
        Some(".git" | ".pijul" | ".jj" | ".hg" | ".svn")
    ) {
        return false;
    }

    // We also ignore symlinks that point inside the current directory
    // if `dedup_links` is enabled.
    if dedup_symlinks && entry.path_is_symlink() {
        return entry
            .path()
            .canonicalize()
            .ok()
            .is_some_and(|path| !path.starts_with(root));
    }

    true
}

/// Opens URL in external program.
#[cfg(not(target_os = "motor"))]
fn open_external_url_callback(
    url: Url,
) -> impl Future<Output = Result<job::Callback, anyhow::Error>> + Send + 'static {
    let commands = open::commands(url.as_str());
    async {
        for cmd in commands {
            let mut command: tokio::process::Command = cmd.into();
            if command.output().await.is_ok() {
                return Ok(job::Callback::Editor(Box::new(|_| {})));
            }
        }
        Ok(job::Callback::Editor(Box::new(move |editor| {
            editor.set_error("Opening URL in external program failed")
        })))
    }
}

#[cfg(target_os = "motor")]
fn open_external_url_callback(
    url: Url,
) -> impl Future<Output = Result<job::Callback, anyhow::Error>> + Send + 'static {
    unsupported_external_url_callback(url)
}

#[cfg(any(target_os = "motor", test))]
async fn unsupported_external_url_callback(_url: Url) -> Result<job::Callback, anyhow::Error> {
    Err(anyhow::anyhow!(MOTOR_EXTERNAL_URL_ERROR))
}

#[cfg(any(target_os = "motor", test))]
const MOTOR_EXTERNAL_URL_ERROR: &str = "Opening external URLs is unsupported on Motor OS";

#[cfg(test)]
mod motor_tests {
    use super::{motor_true_color, unsupported_external_url_callback, MOTOR_EXTERNAL_URL_ERROR};
    use url::Url;

    #[test]
    fn motor_true_color_uses_only_colorterm() {
        assert!(motor_true_color(Some("truecolor")));
        assert!(motor_true_color(Some("24bit")));
        assert!(!motor_true_color(Some("yes")));
        assert!(!motor_true_color(None));
    }

    #[tokio::test]
    async fn motor_external_url_is_explicitly_unsupported() {
        let result =
            unsupported_external_url_callback(Url::parse("https://example.com").unwrap()).await;
        let error = match result {
            Ok(_) => panic!("external URL unexpectedly succeeded"),
            Err(error) => error,
        };
        assert_eq!(error.to_string(), MOTOR_EXTERNAL_URL_ERROR);
    }
}
