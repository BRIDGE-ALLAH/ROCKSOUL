use serde::Serialize;
use std::{
    collections::BTreeMap,
    fs,
    path::Path,
};

const MAX_ENTRIES: usize = 1_500;
const MAX_DEPTH: usize = 5;
const MAX_SAMPLE_FILES: usize = 40;

const IGNORED_DIRS: &[&str] = &[
    ".git", ".rocksoul", "node_modules", "target", "dist", "build",
    "coverage", ".next", ".nuxt", ".venv", "venv", "vendor",
    ".cache", ".idea",
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InspectionReport {
    pub root_path: String,
    pub entries_seen: usize,
    pub files_scanned: usize,
    pub directories_scanned: usize,
    pub skipped_sensitive: usize,
    pub skipped_symlinks: usize,
    pub inaccessible: usize,
    pub truncated: bool,
    pub extensions: BTreeMap<String, usize>,
    pub sample_files: Vec<String>,
}

pub fn inspect_directory(path: &Path) -> Result<InspectionReport, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("Cannot inspect selected folder: {error}"))?;

    if metadata.file_type().is_symlink() {
        return Err("The selected folder is a symbolic link. Choose its real directory instead.".into());
    }
    if !metadata.is_dir() {
        return Err("The selected path is not a directory.".into());
    }

    let root = fs::canonicalize(path)
        .map_err(|error| format!("Cannot resolve selected folder: {error}"))?;

    let mut report = InspectionReport {
        root_path: root.display().to_string(),
        entries_seen: 0,
        files_scanned: 0,
        directories_scanned: 0,
        skipped_sensitive: 0,
        skipped_symlinks: 0,
        inaccessible: 0,
        truncated: false,
        extensions: BTreeMap::new(),
        sample_files: Vec::new(),
    };

    walk(&root, &root, 0, &mut report);
    Ok(report)
}

fn walk(root: &Path, current: &Path, depth: usize, report: &mut InspectionReport) {
    if depth > MAX_DEPTH || report.truncated {
        return;
    }

    let entries = match fs::read_dir(current) {
        Ok(entries) => entries,
        Err(_) => {
            report.inaccessible += 1;
            return;
        }
    };

    report.directories_scanned += 1;

    for entry in entries {
        if report.entries_seen >= MAX_ENTRIES {
            report.truncated = true;
            break;
        }

        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                report.entries_seen += 1;
                report.inaccessible += 1;
                continue;
            }
        };
        report.entries_seen += 1;

        let name = entry.file_name();
        let name = name.to_string_lossy();
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(_) => {
                report.inaccessible += 1;
                continue;
            }
        };

        // Never follow links. Canonical containment also protects against a directory
        // being replaced between directory-entry inspection and recursion.
        if file_type.is_symlink() {
            report.skipped_symlinks += 1;
            continue;
        }

        if file_type.is_dir() {
            if IGNORED_DIRS.iter().any(|ignored| name.eq_ignore_ascii_case(ignored)) {
                continue;
            }
            if depth < MAX_DEPTH {
                let canonical = match fs::canonicalize(&path) {
                    Ok(canonical) => canonical,
                    Err(_) => {
                        report.inaccessible += 1;
                        continue;
                    }
                };
                if canonical != path || !canonical.starts_with(root) {
                    report.skipped_symlinks += 1;
                    continue;
                }
                walk(root, &canonical, depth + 1, report);
            }
            continue;
        }

        if !file_type.is_file() {
            continue;
        }

        if is_sensitive_name(&name) {
            report.skipped_sensitive += 1;
            continue;
        }

        report.files_scanned += 1;
        let extension = path
            .extension()
            .and_then(|value| value.to_str())
            .map(|value| value.to_ascii_lowercase())
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "[no extension]".to_string());
        *report.extensions.entry(extension).or_insert(0) += 1;

        if report.sample_files.len() < MAX_SAMPLE_FILES {
            if let Ok(relative) = path.strip_prefix(root) {
                report.sample_files.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
}

fn is_sensitive_name(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower == ".env"
        || lower.starts_with(".env.")
        || lower == ".npmrc"
        || lower == ".netrc"
        || lower == ".git-credentials"
        || lower == "id_rsa"
        || lower == "id_ed25519"
        || lower.contains("secret")
        || lower.contains("credential")
        || lower.ends_with(".pem")
        || lower.ends_with(".key")
        || lower.ends_with(".p12")
        || lower.ends_with(".pfx")
        || lower.ends_with(".kdbx")
        || lower.ends_with(".token")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn inventory_skips_secrets_and_generated_directories() {
        let temp = tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("src")).expect("src");
        fs::create_dir_all(temp.path().join("node_modules/pkg")).expect("node_modules");
        fs::write(temp.path().join("src/main.rs"), "not read").expect("source");
        fs::write(temp.path().join(".env"), "SECRET=value").expect("env");
        fs::write(temp.path().join("id_ed25519"), "private").expect("key");
        fs::write(temp.path().join("node_modules/pkg/index.js"), "ignored").expect("generated");

        let report = inspect_directory(temp.path()).expect("inspection");
        assert_eq!(report.files_scanned, 1);
        assert_eq!(report.skipped_sensitive, 2);
        assert_eq!(report.sample_files, vec!["src/main.rs"]);
        assert!(report.entries_seen <= MAX_ENTRIES);
    }

    #[cfg(unix)]
    #[test]
    fn inventory_never_follows_symlinks() {
        use std::os::unix::fs::symlink;
        let temp = tempdir().expect("tempdir");
        fs::write(temp.path().join("real.txt"), "content").expect("real file");
        symlink(temp.path().join("real.txt"), temp.path().join("linked.txt")).expect("symlink");

        let report = inspect_directory(temp.path()).expect("inspection");
        assert_eq!(report.files_scanned, 1);
        assert_eq!(report.skipped_symlinks, 1);
        assert_eq!(report.sample_files, vec!["real.txt"]);
    }

    #[test]
    fn rejects_a_file_as_a_workspace_root() {
        let temp = tempdir().expect("tempdir");
        let file = temp.path().join("not-a-folder.txt");
        fs::write(&file, "x").expect("file");
        assert!(inspect_directory(&file).is_err());
    }
}
