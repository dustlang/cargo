//! Tests for `[env]` config.

use payload_test_support::{basic_bin_manifest, project};

#[payload_test]
fn env_basic() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        fn main() {
            println!( "compile-time:{}", env!("ENV_TEST_1233") );
            println!( "run-time:{}", env::var("ENV_TEST_1233").unwrap());
        }
        "#,
        )
        .file(
            ".payload/config",
            r#"
                [env]
                ENV_TEST_1233 = "Hello"
            "#,
        )
        .build();

    p.payload("run -Zconfigurable-env")
        .masquerade_as_nightly_payload()
        .with_stdout_contains("compile-time:Hello")
        .with_stdout_contains("run-time:Hello")
        .run();
}

#[payload_test]
fn env_invalid() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        fn main() {
        }
        "#,
        )
        .file(
            ".payload/config",
            r#"
                [env]
                ENV_TEST_BOOL = false
            "#,
        )
        .build();

    p.payload("build -Zconfigurable-env")
        .masquerade_as_nightly_payload()
        .with_status(101)
        .with_stderr_contains("[..]could not load config key `env.ENV_TEST_BOOL`")
        .run();
}

#[payload_test]
fn env_force() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        fn main() {
            println!( "ENV_TEST_FORCED:{}", env!("ENV_TEST_FORCED") );
            println!( "ENV_TEST_UNFORCED:{}", env!("ENV_TEST_UNFORCED") );
            println!( "ENV_TEST_UNFORCED_DEFAULT:{}", env!("ENV_TEST_UNFORCED_DEFAULT") );
        }
        "#,
        )
        .file(
            ".payload/config",
            r#"
                [env]
                ENV_TEST_UNFORCED_DEFAULT = "from-config"
                ENV_TEST_UNFORCED = { value = "from-config", force = false }
                ENV_TEST_FORCED = { value = "from-config", force = true }
            "#,
        )
        .build();

    p.payload("run -Zconfigurable-env")
        .masquerade_as_nightly_payload()
        .env("ENV_TEST_FORCED", "from-env")
        .env("ENV_TEST_UNFORCED", "from-env")
        .env("ENV_TEST_UNFORCED_DEFAULT", "from-env")
        .with_stdout_contains("ENV_TEST_FORCED:from-config")
        .with_stdout_contains("ENV_TEST_UNFORCED:from-env")
        .with_stdout_contains("ENV_TEST_UNFORCED_DEFAULT:from-env")
        .run();
}

#[payload_test]
fn env_relative() {
    let p = project()
        .file("Payload.toml", &basic_bin_manifest("foo2"))
        .file(
            "src/main.rs",
            r#"
        use std::env;
        use std::path::Path;
        fn main() {
            println!( "ENV_TEST_REGULAR:{}", env!("ENV_TEST_REGULAR") );
            println!( "ENV_TEST_REGULAR_DEFAULT:{}", env!("ENV_TEST_REGULAR_DEFAULT") );
            println!( "ENV_TEST_RELATIVE:{}", env!("ENV_TEST_RELATIVE") );

            assert!( Path::new(env!("ENV_TEST_RELATIVE")).is_absolute() );
            assert!( !Path::new(env!("ENV_TEST_REGULAR")).is_absolute() );
            assert!( !Path::new(env!("ENV_TEST_REGULAR_DEFAULT")).is_absolute() );
        }
        "#,
        )
        .file(
            ".payload/config",
            r#"
                [env]
                ENV_TEST_REGULAR = { value = "Payload.toml", relative = false }
                ENV_TEST_REGULAR_DEFAULT = "Payload.toml"
                ENV_TEST_RELATIVE = { value = "Payload.toml", relative = true }
            "#,
        )
        .build();

    p.payload("run -Zconfigurable-env")
        .masquerade_as_nightly_payload()
        .run();
}
