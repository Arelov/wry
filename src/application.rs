// Copyright 2020-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Re-exported Tao APIs
//!
//! This module re-export [tao] APIs for user to create application windows. To learn more about
//! how to use tao, please see [its documentation](https://crates.io/crates/tao).
//!
//! [tao]: https://crates.io/crates/tao

#[cfg(feature = "tao")]
pub use tao::*;

#[cfg(feature = "winit")]
pub use winit::*;
