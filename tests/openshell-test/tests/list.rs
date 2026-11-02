// SPDX-FileCopyrightText: Copyright (c) 2026 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0

//! The program links the conformance suite's tests.

use std::process::Command;

#[test]
fn lists_the_conformance_tests() {
    let output = Command::new(env!("CARGO_BIN_EXE_openshell-test"))
        .arg("list")
        .output()
        .expect("run openshell-test list");
    assert!(output.status.success(), "{output:?}");
    let stdout = String::from_utf8(output.stdout).expect("utf-8");
    let rows: Vec<Vec<&str>> = stdout
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    assert!(
        rows.iter().all(|row| row[0] == "conformance"),
        "only conformance tests:\n{stdout}"
    );
    for name in [
        "smoke::create_exec_delete",
        "lifecycle::restart_stopped_sandbox",
    ] {
        assert!(
            rows.iter().any(|row| row[1] == name),
            "{name} is listed:\n{stdout}"
        );
    }
}
