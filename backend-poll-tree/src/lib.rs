extern crate notify_backend as backend;
extern crate walkdir;

use backend::prelude::*;
use backend::Buffer;
use std::{fmt, sync::Arc};

#[derive(Debug)]
pub struct Backend {
    buffer: Buffer,
    reg: Arc<MioRegistration>,
    trees: Vec<String>,
    watches: Vec<PathBuf>,
}

impl NotifyBackend for Backend {
    fn new(_paths: Vec<PathBuf>) -> NewBackendResult {
        Err(BackendError::NotImplemented)
    }

    fn capabilities() -> Vec<Capability> {
        vec![
            Capability::FollowSymlinks,
            Capability::WatchFiles,
            Capability::WatchFolders,
            Capability::WatchNewFolders,
            Capability::WatchRecursively,
        ]
    }

    fn driver(&self) -> Box<Evented> {
        Box::new(self.reg.clone())
    }
}

impl Drop for Backend {
    fn drop(&mut self) {}
}

impl Stream for Backend {
    type Item = StreamItem;
    type Error = StreamError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.buffer.closed() {
            return self.buffer.poll()
        }

        self.buffer.poll()
    }
}