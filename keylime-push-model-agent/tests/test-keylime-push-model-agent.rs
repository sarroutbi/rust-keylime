// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Keylime Authors
use assert_cmd::Command;
use predicates::prelude::*;

const KEYLIME_PUSH_MODEL_AGENT_BINARY: &str = "keylime_push_model_agent";
const ERROR_SENDING_REQUEST: &str = "error sending request";
const REQUEST_SENT_SUCCESSFULLY: &str = "Request sent successfully";

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn print_help_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(KEYLIME_PUSH_MODEL_AGENT_BINARY)?;
        cmd.arg("-h");
        cmd.assert().success().stdout(predicate::str::contains(
            KEYLIME_PUSH_MODEL_AGENT_BINARY,
        ));
        Ok(())
    }

    #[test]
    fn connection_error_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(KEYLIME_PUSH_MODEL_AGENT_BINARY)?;
        cmd.arg("-v")
            .arg("http://1.2.3.4:5678")
            .arg("--timeout")
            .arg("10");
        cmd.assert()
            .success()
            .stderr(predicate::str::contains(ERROR_SENDING_REQUEST));
        Ok(())
    }
    #[test]
    fn mockoon_based_test() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin(KEYLIME_PUSH_MODEL_AGENT_BINARY)?;
        cmd.arg("-v")
            .arg("http://localhost:3000")
            .arg("--timeout")
            .arg("1000");
        // Check if MOCKOON environment variable is set and, if so, assume
        // that the mock server is running in port 3000 and response will be received
        // in less than 1 second.
        if std::env::var("MOCKOON").is_ok() {
            cmd.assert()
                .success()
                .stdout(predicate::str::contains(REQUEST_SENT_SUCCESSFULLY));
        } else {
            // If MOCKOON is not set, assume that the server is not running
            // and we expect an error.
            cmd.assert()
                .success()
                .stderr(predicate::str::contains(ERROR_SENDING_REQUEST));
        }
        Ok(())
    }
}
