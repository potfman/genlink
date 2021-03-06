extern crate which;

use std::path::PathBuf;
use std::process::Command;

use crate::{LinkResult, Linker, OsStr, OsString};

/// General Linker Interface
pub struct GenLink<L> {
    linker: L,
    output: OsString,
    command: Command,
}

impl<L: Linker> GenLink<L> {
    /// Create new instance.
    pub fn new(linker: L) -> Self {
        let mut command = Command::new(linker.name());
        linker.preproc(&mut command);

        Self {
            linker,
            output: "a.out".into(),
            command,
        }
    }

    /// Check whether the linker exists on the `PATH`.
    pub fn ok(&self) -> bool {
        which::which(self.linker.name()).is_ok()
    }

    /// Set the output file.
    pub fn output<T: Into<OsString>>(&mut self, path: T) -> &mut Self {
        self.output = path.into();
        self
    }

    /// Add an object.
    pub fn obj<T: AsRef<OsStr>>(&mut self, path: T) -> LinkResult<&mut Self> {
        let path = std::fs::canonicalize(path.as_ref())?;
        self.linker.obj(&mut self.command, path.as_os_str());
        Ok(self)
    }

    /// Add objects.
    pub fn objs<T, TS>(&mut self, paths: TS) -> LinkResult<&mut Self>
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.obj(path)?;
        }
        Ok(self)
    }

    /// Add a dynamic library.
    pub fn dylib<T: AsRef<OsStr>>(&mut self, path: T) -> &mut Self {
        self.linker.dylib(&mut self.command, path.as_ref());
        self
    }

    /// Add libraries.
    pub fn dylibs<T, TS>(&mut self, paths: TS) -> &mut Self
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.dylib(path);
        }
        self
    }

    /// Add a static library.
    pub fn staticlib<T: AsRef<OsStr>>(&mut self, path: T) -> &mut Self {
        self.linker.staticlib(&mut self.command, path.as_ref());
        self
    }

    /// Add libraries.
    pub fn staticlibs<T, TS>(&mut self, paths: TS) -> &mut Self
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.staticlib(path);
        }
        self
    }

    /// Add a library path.
    pub fn path<T: AsRef<OsStr>>(&mut self, path: T) -> &mut Self {
        self.linker.path(&mut self.command, path.as_ref());
        self
    }

    /// Add library paths.
    pub fn paths<T, TS>(&mut self, paths: TS) -> &mut Self
    where
        T: AsRef<OsStr>,
        TS: IntoIterator<Item = T>,
    {
        for path in paths {
            self.path(path);
        }
        self
    }

    /// Link the objects and return the path of the emitted file.
    pub fn link(&mut self) -> LinkResult<PathBuf> {
        let dest = std::fs::canonicalize(&self.output)?;
        self.linker.output(&mut self.command, dest.as_os_str());
        let output = self.command.output()?;

        if output.status.success() {
            Ok(dest)
        } else {
            Err(output.into())
        }
    }
}
