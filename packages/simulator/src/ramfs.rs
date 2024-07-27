use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
pub type Error = linux::AttachMemFSError;
#[cfg(target_os = "macos")]
pub type Error = macos::AttachMemFSError;

pub struct RamFS {
    #[cfg(target_os = "linux")]
    inner: Option<linux::LinuxDevShmFS>,
    #[cfg(target_os = "macos")]
    inner: Option<macos::MacOSMemFS>,
}

impl RamFS {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            #[cfg(target_os = "linux")]
            inner: Some(linux::LinuxDevShmFS::new()?),
            #[cfg(target_os = "macos")]
            inner: Some(macos::MacOSMemFS::new(2048 * 256)?),
        })
    }

    pub fn path(&self) -> &Path {
        self.inner.as_ref().unwrap().path()
    }
}

impl Drop for RamFS {
    fn drop(&mut self) {
        #[cfg(target_os = "macos")]
        self.inner.take().unwrap().cleanup().unwrap();
    }
}

mod linux {
    use std::path::{Path, PathBuf};

    use snafu::Snafu;

    pub struct LinuxDevShmFS {
        path: PathBuf,
    }

    impl LinuxDevShmFS {
        pub fn new() -> Result<Self, AttachMemFSError> {
            let path = PathBuf::from("/dev/shm");
            if !path.is_dir() {
                PathMissingSnafu { path: &path }.fail()?;
            }
            Ok(Self { path })
        }

        pub fn path(&self) -> &Path {
            &self.path
        }
    }

    #[derive(Debug, Snafu)]
    pub enum AttachMemFSError {
        #[snafu(display("In-memory filesystem not available (expected at /dev/shm)"))]
        PathMissing { path: PathBuf },
    }
}

/// https://gist.github.com/htr3n/344f06ba2bb20b1056d7d5570fe7f596
mod macos {
    use std::{
        path::{Path, PathBuf},
        process::Command,
        string::FromUtf8Error,
    };

    use snafu::{ResultExt, Snafu};

    pub struct MacOSMemFS {
        disk: PathBuf,
        volume_path: PathBuf,
    }

    impl MacOSMemFS {
        /// Create an in-memory filesystem on macOS with a size specified in blocks.
        ///
        /// # Arguments
        ///
        /// * `blocks` - The number of 512-byte blocks to allocate for the volume. 2048 blocks is roughly 1MiB.
        pub fn new(blocks: u32) -> Result<Self, AttachMemFSError> {
            // Allocate a new RAM disk with the specified number of blocks
            let ram_url = format!("ram://{blocks}");
            let disk = Command::new("hdiutil")
                .args(["attach", "-nomount", &ram_url])
                .output()
                .context(SubprocessExecutionFailedSnafu)?;
            if !disk.status.success() {
                let stderr = String::from_utf8(disk.stderr).ok();
                SubprocessFailedSnafu { stderr }.fail()?;
            }
            let disk = String::from_utf8(disk.stdout).context(PathNotUtf8Snafu)?;
            let disk = PathBuf::from(disk.trim());
            if !disk.exists() {
                PathMissingSnafu { path: &disk }.fail()?;
            }

            // Format it as an APFS volume so we can place files on it
            // TODO: Handle duplicate volume paths
            let volume_name = "VEX V5 Memory";
            let volume = Command::new("diskutil")
                .args(["eraseVolume", "APFS", volume_name])
                .arg(&disk)
                .output()
                .context(SubprocessExecutionFailedSnafu)?;
            if !volume.status.success() {
                let stderr = String::from_utf8(volume.stderr).ok();
                SubprocessFailedSnafu { stderr }.fail()?;
            }

            Ok(Self {
                disk,
                volume_path: PathBuf::from("/Volumes").join(volume_name),
            })
        }

        pub fn path(&self) -> &Path {
            &self.volume_path
        }

        pub fn cleanup(self) -> Result<(), AttachMemFSError> {
            let detach = Command::new("hdiutil")
                .arg("detach")
                .arg(&self.disk)
                .output()
                .context(SubprocessExecutionFailedSnafu)?;
            if !detach.status.success() {
                let stderr = String::from_utf8(detach.stderr).ok();
                SubprocessFailedSnafu { stderr }.fail()?;
            }
            Ok(())
        }
    }

    #[derive(Debug, Snafu)]
    pub enum AttachMemFSError {
        #[snafu(display("Failed to execute subprocess"))]
        SubprocessExecutionFailed { source: std::io::Error },
        #[snafu(display("Attachment failed: {stderr:?}"))]
        SubprocessFailed { stderr: Option<String> },
        #[snafu(display("Volume/disk path is not valid UTF-8"))]
        PathNotUtf8 { source: FromUtf8Error },
        #[snafu(display("Volume/disk missing after attachment (expected at {path:?})"))]
        PathMissing { path: PathBuf },
    }
}
