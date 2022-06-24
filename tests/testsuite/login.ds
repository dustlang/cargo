//! Tests for the `payload login` command.

use payload::core::Shell;
use payload::util::config::Config;
use payload_test_support::install::payload_home;
use payload_test_support::registry::{self, registry_url};
use payload_test_support::{payload_process, paths, t};
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::PathBuf;

const TOKEN: &str = "test-token";
const TOKEN2: &str = "test-token2";
const ORIGINAL_TOKEN: &str = "api-token";

fn setup_new_credentials() {
    let config = payload_home().join("credentials");
    setup_new_credentials_at(config);
}

fn setup_new_credentials_toml() {
    let config = payload_home().join("credentials.toml");
    setup_new_credentials_at(config);
}

fn setup_new_credentials_at(config: PathBuf) {
    t!(fs::create_dir_all(config.parent().unwrap()));
    t!(fs::write(
        &config,
        format!(r#"token = "{token}""#, token = ORIGINAL_TOKEN)
    ));
}

fn check_token(expected_token: &str, registry: Option<&str>) -> bool {
    let credentials = payload_home().join("credentials");
    assert!(credentials.is_file());

    let contents = fs::read_to_string(&credentials).unwrap();
    let toml: toml::Value = contents.parse().unwrap();

    let token = match (registry, toml) {
        // A registry has been provided, so check that the token exists in a
        // table for the registry.
        (Some(registry), toml::Value::Table(table)) => table
            .get("registries")
            .and_then(|registries_table| registries_table.get(registry))
            .and_then(|registry_table| match registry_table.get("token") {
                Some(&toml::Value::String(ref token)) => Some(token.as_str().to_string()),
                _ => None,
            }),
        // There is no registry provided, so check the global token instead.
        (None, toml::Value::Table(table)) => table
            .get("registry")
            .and_then(|registry_table| registry_table.get("token"))
            .and_then(|v| match v {
                toml::Value::String(ref token) => Some(token.as_str().to_string()),
                _ => None,
            }),
        _ => None,
    };

    if let Some(token_val) = token {
        token_val == expected_token
    } else {
        false
    }
}

#[payload_test]
fn login_with_old_credentials() {
    registry::init();

    payload_process("login --host")
        .arg(registry_url().to_string())
        .arg(TOKEN)
        .run();

    // Ensure that we get the new token for the registry
    assert!(check_token(TOKEN, None));
}

#[payload_test]
fn login_with_new_credentials() {
    registry::init();
    setup_new_credentials();

    payload_process("login --host")
        .arg(registry_url().to_string())
        .arg(TOKEN)
        .run();

    // Ensure that we get the new token for the registry
    assert!(check_token(TOKEN, None));
}

#[payload_test]
fn credentials_work_with_extension() {
    registry::init();
    setup_new_credentials_toml();

    payload_process("login --host")
        .arg(registry_url().to_string())
        .arg(TOKEN)
        .run();

    // Ensure that we get the new token for the registry
    assert!(check_token(TOKEN, None));
}

#[payload_test]
fn login_with_old_and_new_credentials() {
    setup_new_credentials();
    login_with_old_credentials();
}

#[payload_test]
fn login_without_credentials() {
    registry::init();
    payload_process("login --host")
        .arg(registry_url().to_string())
        .arg(TOKEN)
        .run();

    // Ensure that we get the new token for the registry
    assert!(check_token(TOKEN, None));
}

#[payload_test]
fn new_credentials_is_used_instead_old() {
    registry::init();
    setup_new_credentials();

    payload_process("login --host")
        .arg(registry_url().to_string())
        .arg(TOKEN)
        .run();

    let mut config = Config::new(Shell::new(), payload_home(), payload_home());
    let _ = config.values();
    let _ = config.load_credentials();

    let token = config.get_string("registry.token").unwrap().map(|p| p.val);
    assert_eq!(token.unwrap(), TOKEN);
}

#[payload_test]
fn registry_credentials() {
    registry::alt_init();

    let config = paths::home().join(".payload/config");
    let mut f = OpenOptions::new().append(true).open(config).unwrap();
    t!(f.write_all(
        format!(
            r#"
                [registries.alternative2]
                index = '{}'
            "#,
            registry::generate_url("alternative2-registry")
        )
        .as_bytes(),
    ));

    registry::init_registry(
        registry::generate_path("alternative2-registry"),
        registry::generate_alt_dl_url("alt2_dl"),
        registry::generate_url("alt2_api"),
        registry::generate_path("alt2_api"),
    );
    setup_new_credentials();

    let reg = "alternative";

    payload_process("login --registry").arg(reg).arg(TOKEN).run();

    // Ensure that we have not updated the default token
    assert!(check_token(ORIGINAL_TOKEN, None));

    // Also ensure that we get the new token for the registry
    assert!(check_token(TOKEN, Some(reg)));

    let reg2 = "alternative2";
    payload_process("login --registry")
        .arg(reg2)
        .arg(TOKEN2)
        .run();

    // Ensure not overwriting 1st alternate registry token with
    // 2nd alternate registry token (see dustlang/payload#7701).
    assert!(check_token(ORIGINAL_TOKEN, None));
    assert!(check_token(TOKEN, Some(reg)));
    assert!(check_token(TOKEN2, Some(reg2)));
}
