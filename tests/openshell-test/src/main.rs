// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

//! `openshell-test`: `OpenShell`'s end-to-end tests as one program, to run
//! against a gateway without cargo or nextest. It holds the conformance
//! suite for now.
//!
//! The tests are registered by `#[testkit::test]` in the suite crates; see
//! `testkit::program` for the commands.

use std::process::ExitCode;

// A dependency that is never named is not linked, and its tests would be
// missing. Name each suite crate here.
use conformance as _;

fn main() -> ExitCode {
    testkit::main()
}
