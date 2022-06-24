//! Tests for the `payload logout` command.

use payload_test_support::install::payload_home;
use payload_test_support::{payload_process, registry};
use std::fs;

#[payload_test]
fn gated() {
    registry::init();
    payload_process("logout")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr(
            "\
[ERROR] the `payload logout` command is unstable, pass `-Z unstable-options` to enable it
See https://github.com/dustlang/payload/issues/8933 for more information about \
the `payload logout` command.
",
        )
        .run();
}

/// Checks whether or not the token is set for the given token.
fn check_config_token(registry: Option<&str>, should_be_set: bool) {
    let credentials = payload_home().join("credentials");
    let contents = fs::read_to_string(&credentials).unwrap();
    let toml: toml::Value = contents.parse().unwrap();
    if let Some(registry) = registry {
        assert_eq!(
            toml.get("registries")
                .and_then(|registries| registries.get(registry))
                .and_then(|registry| registry.get("token"))
                .is_some(),
            should_be_set
        );
    } else {
        assert_eq!(
            toml.get("registry")
                .and_then(|registry| registry.get("token"))
                .is_some(),
            should_be_set
        );
    }
}

fn simple_logout_test(reg: Option<&str>, flag: &str) {
    registry::init();
    let msg = reg.unwrap_or("crates.io");
    check_config_token(reg, true);
    payload_process(&format!("logout -Z unstable-options {}", flag))
        .masquerade_as_nightly_payload()
        .with_stderr(&format!(
            "\
[UPDATING] [..]
[LOGOUT] token for `{}` has been removed from local storage
",
            msg
        ))
        .run();
    check_config_token(reg, false);

    payload_process(&format!("logout -Z unstable-options {}", flag))
        .masquerade_as_nightly_payload()
        .with_stderr(&format!(
            "\
[LOGOUT] not currently logged in to `{}`
",
            msg
        ))
        .run();
    check_config_token(reg, false);
}

#[payload_test]
fn default_registry() {
    simple_logout_test(None, "");
}

#[payload_test]
fn other_registry() {
    registry::alt_init();
    simple_logout_test(Some("alternative"), "--registry alternative");
}
