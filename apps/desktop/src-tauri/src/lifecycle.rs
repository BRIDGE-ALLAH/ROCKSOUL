use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use rocksoul_core::StoreError;
use uuid::Uuid;

const ACTIVE_MARKER_PREFIX: &str = "rocksoul-active-session-";
const ACTIVE_MARKER_SUFFIX: &str = ".json";

pub(crate) struct LifecycleSession {
    marker_path: PathBuf,
}

pub(crate) struct SessionStart {
    pub session: LifecycleSession,
    pub previous_unclean_exit: bool,
}

impl LifecycleSession {
    /// Detect and clear stale active-session markers, then durably create this
    /// process marker before the database is opened or migrations are run.
    pub(crate) fn begin(data_dir: &Path) -> io::Result<SessionStart> {
        fs::create_dir_all(data_dir)?;

        let mut stale_markers = Vec::new();
        for entry in fs::read_dir(data_dir)?.take(1024) {
            let entry = entry?;
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if !name.starts_with(ACTIVE_MARKER_PREFIX) || !name.ends_with(ACTIVE_MARKER_SUFFIX) {
                continue;
            }

            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)?;
            if metadata.file_type().is_file() {
                stale_markers.push(path);
            }
        }

        let previous_unclean_exit = !stale_markers.is_empty();
        for stale in stale_markers {
            fs::remove_file(stale)?;
        }

        let run_id = Uuid::new_v4();
        let marker_path = data_dir.join(format!(
            "{ACTIVE_MARKER_PREFIX}{run_id}{ACTIVE_MARKER_SUFFIX}"
        ));
        let started_at_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();

        let mut marker = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&marker_path)?;
        if let Err(error) = writeln!(
            marker,
            "schema_version=1\nrun_id={run_id}\nstarted_at_unix_ms={started_at_unix_ms}"
        )
        .and_then(|()| marker.sync_all())
        {
            let _ = fs::remove_file(&marker_path);
            return Err(error);
        }

        Ok(SessionStart {
            session: Self { marker_path },
            previous_unclean_exit,
        })
    }

    /// This must be called only after the database pool has been closed.
    pub(crate) fn finish_cleanly(self) -> io::Result<()> {
        fs::remove_file(self.marker_path)
    }
}

pub(crate) fn safe_store_error_code(error: &StoreError) -> &'static str {
    match error {
        StoreError::Migration(_) => "database_migration_failed",
        StoreError::Database(_) => "database_connection_failed",
        StoreError::Io(_) => "application_data_io_failed",
        _ => "database_startup_failed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{error::Error, fs};
    use tempfile::tempdir;

    #[test]
    fn clean_exit_clears_marker_and_unclean_exit_is_detected() -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;

        let first = LifecycleSession::begin(directory.path())?;
        let first_marker = first.session.marker_path.clone();
        assert!(first_marker.is_file());
        assert!(!first.previous_unclean_exit);

        drop(first);
        assert!(first_marker.is_file());

        let second = LifecycleSession::begin(directory.path())?;
        assert!(second.previous_unclean_exit);
        assert!(!first_marker.exists());

        second.session.finish_cleanly()?;
        let third = LifecycleSession::begin(directory.path())?;
        assert!(!third.previous_unclean_exit);
        third.session.finish_cleanly()?;
        Ok(())
    }

    #[test]
    fn marker_is_metadata_only_and_does_not_contain_application_paths()
    -> Result<(), Box<dyn Error>> {
        let directory = tempdir()?;
        let started = LifecycleSession::begin(directory.path())?;
        let marker = fs::read_to_string(&started.session.marker_path)?;
        assert!(marker.contains("schema_version=1"));
        assert!(marker.contains("run_id="));
        assert!(marker.contains("started_at_unix_ms="));
        assert!(!marker.contains(directory.path().to_string_lossy().as_ref()));
        started.session.finish_cleanly()?;
        Ok(())
    }

    #[test]
    fn safe_error_codes_do_not_echo_sensitive_error_details() {
        let secret = "C:\\Users\\synthetic\\AppData\\token=DO_NOT_LOG";
        let error = StoreError::Io(io::Error::other(secret));
        let code = safe_store_error_code(&error);
        assert_eq!(code, "application_data_io_failed");
        assert!(!code.contains("DO_NOT_LOG"));
        assert!(!code.contains("C:\\Users"));
    }
}
