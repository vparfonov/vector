// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

<<<<<<<< HEAD:vendor/opendal/src/services/fs/appender.rs
use async_trait::async_trait;
use bytes::Bytes;
use tokio::io::AsyncWriteExt;

use super::error::parse_io_error;
use crate::raw::*;
use crate::*;

pub struct FsAppender<F> {
    f: F,
}

impl<F> FsAppender<F> {
    pub fn new(f: F) -> Self {
        Self { f }
    }
}

#[async_trait]
impl oio::Append for FsAppender<tokio::fs::File> {
    async fn append(&mut self, bs: Bytes) -> Result<()> {
        self.f.write_all(&bs).await.map_err(parse_io_error)?;

        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        self.f.sync_all().await.map_err(parse_io_error)?;

        Ok(())
    }
}
========
//! Utilities for opendal testing.

mod read;
pub use read::ReadAction;
pub use read::ReadChecker;

mod write;
pub use write::WriteAction;
pub use write::WriteChecker;

mod utils;
pub use utils::init_test_service;
pub use utils::TEST_RUNTIME;
>>>>>>>> 75e70ace9f (LOG-5296: Update vector to v0.37.x):vendor/opendal/src/raw/tests/mod.rs
