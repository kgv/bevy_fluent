use crate::exts::PathExt;
use anyhow::Result;
use bevy::{
    asset::{Asset, AssetIo, AssetIoError},
    prelude::*,
    utils::tracing::{self, instrument},
};
use globset::Glob;
use std::path::{Path, PathBuf};

/// Extension methods for [`AssetIo`](bevy::asset::AssetIo)
pub trait AssetIoExt: AssetIo {
    /// Visit directory
    fn visit_directory(
        &self,
        directory: &Path,
        callback: &mut dyn FnMut(PathBuf),
    ) -> Result<(), AssetIoError>;

    fn walk_directory(&self, directory: &Path) -> Result<Vec<PathBuf>, AssetIoError>;
}

impl<T: AssetIo + ?Sized> AssetIoExt for T {
    fn visit_directory(
        &self,
        directory: &Path,
        callback: &mut dyn FnMut(PathBuf),
    ) -> Result<(), AssetIoError> {
        if self.is_dir(directory) {
            for path in self.read_directory(directory)? {
                if self.is_dir(&path) {
                    self.visit_directory(&path, callback)?;
                } else {
                    callback(path);
                }
            }
        }
        Ok(())
    }

    fn walk_directory(&self, directory: &Path) -> Result<Vec<PathBuf>, AssetIoError> {
        let mut paths = Vec::new();
        self.visit_directory(directory, &mut |path| paths.push(path))?;
        Ok(paths)
    }
}

/// Extension methods for [`AssetServer`](bevy::asset::AssetServer)
pub trait AssetServerExt {
    fn load_glob<T: Asset>(&self, path: &str) -> Result<Vec<Handle<T>>>;
}

impl AssetServerExt for AssetServer {
    #[instrument(fields(glob = ?glob), skip_all)]
    fn load_glob<T: Asset>(&self, glob: &str) -> Result<Vec<Handle<T>>> {
        let path = Path::new(glob);
        let matcher = Glob::new(glob)?.compile_matcher();
        let path = path
            .find_prefix(|path| self.asset_io().is_dir(path))
            .unwrap_or_else(|_| Path::new(""));
        trace!(base = ?path);
        Ok(self
            .asset_io()
            .walk_directory(path)?
            .into_iter()
            .filter_map(|path| matcher.is_match(&path).then(|| self.load(path)))
            .collect())
    }
}
