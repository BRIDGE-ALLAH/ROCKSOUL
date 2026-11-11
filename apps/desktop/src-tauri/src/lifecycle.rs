use std::{fs, io, path::{Path, PathBuf}};

const RUNNING_MARKER: &str = "rocksoul.running";

#[derive(Debug)]
pub struct LifecycleMarker {
    marker_path: PathBuf,
    previous_unclean_exit: bool,
}

impl LifecycleMarker {
    pub fn begin(app_data_dir: &Path) -> io::Result<Self> {
        fs::create_dir_all(app_data_dir)?;
        let marker_path = app_data_dir.join(RUNNING_MARKER);

        let previous_unclean_exit = match fs::create_dir(&marker_path) {
            Ok(()) => false,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let metadata = fs::symlink_metadata(&marker_path)?;
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Rocksoul lifecycle marker is not a regular directory",
                    ));
                }
                true
            }
            Err(error) => return Err(error),
        };

        Ok(Self { marker_path, previous_unclean_exit })
    }

    pub fn previous_unclean_exit(&self) -> bool {
        self.previous_unclean_exit
    }

    /// Call only after durable database work has been closed successfully.
    pub fn mark_clean_shutdown(&self) -> io::Result<()> {
        match fs::remove_dir(&self.marker_path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    #[cfg(test)]
    fn marker_path(&self) -> &Path {
        &self.marker_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn first_launch_creates_a_running_marker_without_a_recovery_warning() {
        let directory = tempdir().expect("temporary app data directory");
        let marker = LifecycleMarker::begin(directory.path()).expect("begin first session");
        assert!(!marker.previous_unclean_exit());
        assert!(marker.marker_path().is_dir());
    }

    #[test]
    fn dropped_session_is_reported_and_clean_shutdown_resets_the_marker() {
        let directory = tempdir().expect("temporary app data directory");
        {
            let first = LifecycleMarker::begin(directory.path()).expect("begin first session");
            assert!(!first.previous_unclean_exit());
            // Dropping without cleanup simulates a process that did not exit cleanly.
        }
        let recovered = LifecycleMarker::begin(directory.path()).expect("begin recovery session");
        assert!(recovered.previous_unclean_exit());
        recovered.mark_clean_shutdown().expect("clean recovered session");
        assert!(!recovered.marker_path().exists());
        let next = LifecycleMarker::begin(directory.path()).expect("begin next session");
        assert!(!next.previous_unclean_exit());
        next.mark_clean_shutdown().expect("clean final session");
    }

    #[test]
    fn a_non_directory_marker_is_rejected() {
        let directory = tempdir().expect("temporary app data directory");
        fs::write(directory.path().join(RUNNING_MARKER), "not a marker directory")
            .expect("create invalid marker");
        let error = LifecycleMarker::begin(directory.path()).expect_err("reject marker file");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[cfg(unix)]
    #[test]
    fn a_symlink_marker_is_rejected() {
        use std::os::unix::fs::symlink;
        let directory = tempdir().expect("temporary app data directory");
        let target = directory.path().join("target");
        fs::create_dir(&target).expect("create symlink target");
        symlink(&target, directory.path().join(RUNNING_MARKER)).expect("create symlink marker");
        let error = LifecycleMarker::begin(directory.path()).expect_err("reject symlink marker");
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }
}
