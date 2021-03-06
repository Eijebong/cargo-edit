extern crate assert_cli;
#[macro_use]
extern crate pretty_assertions;
extern crate tempdir;
extern crate toml;

use std::process;
mod utils;
use utils::{clone_out_test, execute_command, get_toml};

/// Check 'failure' deps are not present
fn no_manifest_failures(manifest: &toml::Value) -> bool {
    manifest.get("dependencies.failure").is_none() &&
        manifest.get("dev-dependencies.failure").is_none() &&
        manifest.get("build-dependencies.failure").is_none()
}

#[test]
fn adds_dependency() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "my-package"], &manifest);

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["my-package"];
    assert_eq!(val.as_str().unwrap(), "my-package--CURRENT_VERSION_TEST");
}

fn upgrade_test_helper(upgrade_method: &str, expected_prefix: &str) {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    let upgrade_arg = format!("--upgrade={0}", upgrade_method);

    execute_command(&["add", "my-package", upgrade_arg.as_str()], &manifest);

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["my-package"];

    let expected_result = format!("{0}my-package--CURRENT_VERSION_TEST", expected_prefix);
    assert_eq!(val.as_str().unwrap(), expected_result);
}

#[test]
fn adds_dependency_with_upgrade_none() {
    upgrade_test_helper("none", "=");
}
#[test]
fn adds_dependency_with_upgrade_patch() {
    upgrade_test_helper("patch", "~");
}
#[test]
fn adds_dependency_with_upgrade_minor() {
    upgrade_test_helper("minor", "^");
}
#[test]
fn adds_dependency_with_upgrade_all() {
    upgrade_test_helper("all", ">=");
}

#[test]
fn adds_dependency_with_upgrade_bad() {
    upgrade_test_helper("an_invalid_string", "");
}

#[test]
fn adds_multiple_dependencies() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "my-package1", "my-package2"], &manifest);

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["my-package1"];
    assert_eq!(val.as_str().unwrap(), "my-package1--CURRENT_VERSION_TEST");
    let val = &toml["dependencies"]["my-package2"];
    assert_eq!(val.as_str().unwrap(), "my-package2--CURRENT_VERSION_TEST");
}

#[test]
fn adds_dev_build_dependency() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());
    assert!(toml.get("build-dependencies").is_none());

    execute_command(&["add", "my-dev-package", "--dev"], &manifest);
    execute_command(&["add", "my-build-package", "--build"], &manifest);

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["my-dev-package"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-dev-package--CURRENT_VERSION_TEST"
    );
    let val = &toml["build-dependencies"]["my-build-package"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-build-package--CURRENT_VERSION_TEST"
    );

    // cannot run with both --dev and --build at the same time
    let call = process::Command::new("target/debug/cargo-add")
        .args(&["add", "failure", "--dev", "--build"])
        .arg(format!("--manifest-path={}", &manifest))
        .output()
        .unwrap();

    assert!(!call.status.success());
    assert!(no_manifest_failures(&get_toml(&manifest)));
}

#[test]
fn adds_multiple_dev_build_dependencies() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());
    assert!(toml.get("dev-dependencies").is_none());
    assert!(toml.get("build-dependencies").is_none());
    assert!(toml.get("build-dependencies").is_none());

    execute_command(
        &["add", "my-dev-package1", "my-dev-package2", "--dev"],
        &manifest,
    );
    execute_command(
        &["add", "my-build-package1", "--build", "my-build-package2"],
        &manifest,
    );

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["my-dev-package1"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-dev-package1--CURRENT_VERSION_TEST"
    );
    let val = &toml["dev-dependencies"]["my-dev-package2"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-dev-package2--CURRENT_VERSION_TEST"
    );
    let val = &toml["build-dependencies"]["my-build-package1"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-build-package1--CURRENT_VERSION_TEST"
    );
    let val = &toml["build-dependencies"]["my-build-package2"];
    assert_eq!(
        val.as_str().unwrap(),
        "my-build-package2--CURRENT_VERSION_TEST"
    );
}

#[test]
fn adds_specified_version() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &["add", "versioned-package", "--vers", ">=0.1.1"],
        &manifest,
    );

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["versioned-package"];
    assert_eq!(val.as_str().expect("not string"), ">=0.1.1");

    // cannot run with both --dev and --build at the same time
    let call = process::Command::new("target/debug/cargo-add")
        .args(&["add", "failure", "--vers", "invalid version string"])
        .arg(format!("--manifest-path={}", &manifest))
        .output()
        .unwrap();

    assert!(!call.status.success());
    assert!(no_manifest_failures(&get_toml(&manifest)));
}

#[test]
fn adds_specified_version_with_inline_notation() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "versioned-package@>=0.1.1"], &manifest);

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["versioned-package"];
    assert_eq!(val.as_str().expect("not string"), ">=0.1.1");
}

#[test]
fn adds_multiple_dependencies_with_versions() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &["add", "my-package1@>=0.1.1", "my-package2@0.2.3"],
        &manifest,
    );

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["my-package1"];
    assert_eq!(val.as_str().expect("not string"), ">=0.1.1");
    let val = &toml["dependencies"]["my-package2"];
    assert_eq!(val.as_str().expect("not string"), "0.2.3");
}

#[test]
fn adds_multiple_dependencies_with_some_versions() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "my-package1", "my-package2@0.2.3"], &manifest);

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["my-package1"];
    assert_eq!(
        val.as_str().expect("not string"),
        "my-package1--CURRENT_VERSION_TEST"
    );
    let val = &toml["dependencies"]["my-package2"];
    assert_eq!(val.as_str().expect("not string"), "0.2.3");
}

#[test]
fn adds_git_source_using_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &[
            "add",
            "git-package",
            "--git",
            "http://localhost/git-package.git",
        ],
        &manifest,
    );

    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["git-package"];
    assert_eq!(
        val.as_table().unwrap()["git"].as_str().unwrap(),
        "http://localhost/git-package.git"
    );

    // check this works with other flags (e.g. --dev) as well
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());

    execute_command(
        &["add", "git-dev-pkg", "--git", "http://site/gp.git", "--dev"],
        &manifest,
    );

    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["git-dev-pkg"];
    assert_eq!(
        val.as_table().unwrap()["git"].as_str().unwrap(),
        "http://site/gp.git"
    );
}

#[test]
fn adds_local_source_using_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "local", "--path", "/path/to/pkg"], &manifest);

    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["local"];
    assert_eq!(
        val.as_table().unwrap()["path"].as_str().unwrap(),
        "/path/to/pkg"
    );

    // check this works with other flags (e.g. --dev) as well
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());

    execute_command(
        &["add", "local-dev", "--path", "/path/to/pkg-dev", "--dev"],
        &manifest,
    );

    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["local-dev"];
    assert_eq!(
        val.as_table().unwrap()["path"].as_str().unwrap(),
        "/path/to/pkg-dev"
    );
}

#[test]
#[cfg(feature = "test-external-apis")]
fn adds_git_source_without_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &["add", "https://github.com/killercup/cargo-edit.git"],
        &manifest,
    );

    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["cargo-edit"];
    assert_eq!(
        val.as_table().unwrap()["git"].as_str().unwrap(),
        "https://github.com/killercup/cargo-edit.git"
    );

    // check this works with other flags (e.g. --dev) as well
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());

    execute_command(
        &[
            "add",
            "https://github.com/killercup/cargo-edit.git",
            "--dev",
        ],
        &manifest,
    );

    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["cargo-edit"];
    assert_eq!(
        val.as_table().unwrap()["git"].as_str().unwrap(),
        "https://github.com/killercup/cargo-edit.git"
    );
}

#[test]
fn adds_local_source_without_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    let (tmpdir, _) = clone_out_test("tests/fixtures/add/local/Cargo.toml.sample");
    let tmppath = tmpdir.into_path();
    let tmpdirstr = tmppath.to_str().unwrap();

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", tmpdirstr], &manifest);

    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["foo-crate"];
    assert_eq!(val.as_table().unwrap()["path"].as_str().unwrap(), tmpdirstr);

    // check this works with other flags (e.g. --dev) as well
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");
    let toml = get_toml(&manifest);
    assert!(toml.get("dev-dependencies").is_none());

    execute_command(&["add", tmpdirstr, "--dev"], &manifest);

    let toml = get_toml(&manifest);
    let val = &toml["dev-dependencies"]["foo-crate"];
    assert_eq!(val.as_table().unwrap()["path"].as_str().unwrap(), tmpdirstr);
}

#[test]
fn package_kinds_are_mutually_exclusive() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    let call = process::Command::new("target/debug/cargo-add")
        .args(&["add", "failure"])
        .args(&["--vers", "0.4.3"])
        .args(&["--git", "git://git.git"])
        .args(&["--path", "/path/here"])
        .arg(format!("--manifest-path={}", &manifest))
        .output()
        .unwrap();

    assert!(!call.status.success());
    assert!(no_manifest_failures(&get_toml(&manifest)));
}

#[test]
fn adds_optional_dependency() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &[
            "add",
            "versioned-package",
            "--vers",
            ">=0.1.1",
            "--optional",
        ],
        &manifest,
    );

    // dependency present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["dependencies"]["versioned-package"]["optional"];
    assert_eq!(val.as_bool().expect("optional not a bool"), true);
}

#[test]
fn adds_multiple_optional_dependencies() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &["add", "--optional", "my-package1", "my-package2"],
        &manifest,
    );

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    assert!(&toml["dependencies"]["my-package1"]["optional"]
        .as_bool()
        .expect("optional not a bool"));
    assert!(&toml["dependencies"]["my-package2"]["optional"]
        .as_bool()
        .expect("optional not a bool"));
}

#[test]
fn adds_dependency_with_target_triple() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("target").is_none());

    execute_command(
        &["add", "--target", "i686-unknown-linux-gnu", "my-package1"],
        &manifest,
    );

    // dependencies present afterwards
    let toml = get_toml(&manifest);

    let val = &toml["target"]["i686-unknown-linux-gnu"]["dependencies"]["my-package1"];
    assert_eq!(val.as_str().unwrap(), "my-package1--CURRENT_VERSION_TEST");
}

#[test]
fn adds_dependency_with_target_cfg() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("target").is_none());

    execute_command(&["add", "--target", "cfg(unix)", "my-package1"], &manifest);

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    let val = &toml["target"]["cfg(unix)"]["dependencies"]["my-package1"];

    assert_eq!(val.as_str().unwrap(), "my-package1--CURRENT_VERSION_TEST");
}

#[test]
fn adds_dependency_with_custom_target() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    execute_command(
        &["add", "--target", "x86_64/windows.json", "my-package1"],
        &manifest,
    );

    // dependencies present afterwards
    let toml = get_toml(&manifest);
    // Get package by hand because toml-rs does not currently handle escaping dots in get()
    let target = &toml["target"];
    if let toml::Value::Table(ref table) = *target {
        let win_target = &table["x86_64/windows.json"];
        let val = &win_target["dependencies"]["my-package1"];
        assert_eq!(val.as_str().unwrap(), "my-package1--CURRENT_VERSION_TEST");
    } else {
        panic!("target is not a table");
    }
}


#[test]
#[cfg(feature = "test-external-apis")]
fn adds_dependency_normalized_name() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    assert_cli::Assert::command(&[
        "target/debug/cargo-add",
        "add",
        "linked_hash_map",
        &format!("--manifest-path={}", manifest),
    ]).succeeds()
        .prints("WARN: Added `linked-hash-map` instead of `linked_hash_map`")
        .unwrap();

    // dependency present afterwards
    let toml = get_toml(&manifest);
    assert!(toml["dependencies"].get("linked-hash-map").is_some());
}


#[test]
#[should_panic]
fn fails_to_add_dependency_with_empty_target() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // Fails because target parameter must be a valid target
    execute_command(&["add", "--target", "", "my-package1"], &manifest);
}

#[test]
#[should_panic]
fn fails_to_add_optional_dev_dependency() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    // Fails because optional dependencies must be in `dependencies` table.
    execute_command(
        &[
            "add",
            "versioned-package",
            "--vers",
            ">=0.1.1",
            "--dev",
            "--optional",
        ],
        &manifest,
    );
}

#[test]
#[should_panic]
fn fails_to_add_multiple_optional_dev_dependencies() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependencies not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());
    assert!(toml.get("dependencies").is_none());

    // Fails because optional dependencies must be in `dependencies` table.
    execute_command(
        &["add", "--optional", "my-package1", "my-package2", "--dev"],
        &manifest,
    );
}

#[test]
#[should_panic]
#[cfg(feature = "test-external-apis")]
fn fails_to_add_inexistent_git_source_without_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(
        &["add", "https://github.com/killercup/fake-git-repo.git"],
        &manifest,
    );
}

#[test]
#[should_panic]
fn fails_to_add_inexistent_local_source_without_flag() {
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");

    // dependency not present beforehand
    let toml = get_toml(&manifest);
    assert!(toml.get("dependencies").is_none());

    execute_command(&["add", "./tests/fixtures/local"], &manifest);
}

fn overwite_dependency_test(first_command: &[&str], second_command: &[&str], expected: &str) {
    // First, add a dependency.
    let (_tmpdir, manifest) = clone_out_test("tests/fixtures/add/Cargo.toml.sample");
    execute_command(first_command, &manifest);

    // Then, overwite with the latest version
    execute_command(second_command, &manifest);

    // Verify that the dependency is as expected.
    let toml = get_toml(&manifest);
    let expected = r#"
        [package]
        name = "cargo-list-test-fixture"
        version = "0.0.0"
    "#.to_string() + expected;
    let expected_dep: toml::Value = toml::from_str(&expected).unwrap();
    assert_eq!(expected_dep, toml);
}

#[test]
fn overwrite_version_with_version() {
    overwite_dependency_test(
        &["add", "versioned-package", "--vers", "0.1.1", "--optional"],
        &["add", "versioned-package"],
        r#"
            [dependencies.versioned-package]
            version = "versioned-package--CURRENT_VERSION_TEST"
            optional = true
        "#,
    )
}

#[test]
fn overwrite_version_with_git() {
    overwite_dependency_test(
        &["add", "versioned-package", "--vers", "0.1.1", "--optional"],
        &["add", "versioned-package", "--git", "git://git.git"],
        r#"
            [dependencies.versioned-package]
            git = "git://git.git"
            optional = true
        "#,
    )
}

#[test]
fn overwrite_version_with_path() {
    overwite_dependency_test(
        &["add", "versioned-package", "--vers", "0.1.1", "--optional"],
        &["add", "versioned-package", "--path", "../foo"],
        r#"
            [dependencies.versioned-package]
            path = "../foo"
            optional = true
        "#,
    )
}

#[test]
fn overwrite_git_with_path() {
    overwite_dependency_test(
        &[
            "add",
            "versioned-package",
            "--git",
            "git://git.git",
            "--optional",
        ],
        &["add", "versioned-package", "--path", "../foo"],
        r#"
            [dependencies.versioned-package]
            path = "../foo"
            optional = true
        "#,
    )
}

#[test]
fn overwrite_path_with_version() {
    overwite_dependency_test(
        &["add", "versioned-package", "--path", "../foo"],
        &["add", "versioned-package"],
        r#"
            [dependencies]
            versioned-package = "versioned-package--CURRENT_VERSION_TEST"
        "#,
    )
}

#[test]
fn no_argument() {
    assert_cli::Assert::command(&["target/debug/cargo-add", "add"])
        .fails_with(1)
        .prints_error_exactly(
            r"Invalid arguments.

Usage:
    cargo add <crate> [--dev|--build|--optional] [--vers=<ver>|--git=<uri>|--path=<uri>] [options]
    cargo add <crates>... [--dev|--build|--optional] [options]
    cargo add (-h|--help)
    cargo add --version",
        )
        .unwrap();
}

#[test]
fn unknown_flags() {
    assert_cli::Assert::command(&["target/debug/cargo-add", "add", "foo", "--flag"])
        .fails_with(1)
        .prints_error_exactly(
            r"Unknown flag: '--flag'

Usage:
    cargo add <crate> [--dev|--build|--optional] [--vers=<ver>|--git=<uri>|--path=<uri>] [options]
    cargo add <crates>... [--dev|--build|--optional] [options]
    cargo add (-h|--help)
    cargo add --version",
        )
        .unwrap();
}
