use assert_cmd::Command;

#[test]
fn no_filename() {
    let mut command = Command::cargo_bin("vm").unwrap();
    // Not giving a binary program for the virtual machine is a failure
    command.assert().failure();
}

#[test]
fn invalid_filename() {
    let mut command = Command::cargo_bin("vm").unwrap();
    // Using a non-existent binary file name is a failure
    command.arg("/tmp/nonexistent-djahdfc").assert().failure();
}

#[test]
fn invalid_file_content() {
    let mut command = Command::cargo_bin("vm").unwrap();
    // Cargo.toml is not a valid virtual machine binary program
    command.arg("Cargo.toml").assert().failure();
}

// Execute a virtual machine program and return the result
fn exec(bin: &str) -> String {
    let mut command = Command::cargo_bin("vm").unwrap();
    let command = command.arg(format!("examples/{bin}.bin"));
    String::from_utf8(command.output().unwrap().stdout).unwrap()
}

#[test]
fn execute_example_hello_world() {
    insta::assert_snapshot!(exec("hello_world"), @r###"
    Hello, world!
    "###);
}

#[test]
fn execute_example_count() {
    insta::assert_snapshot!(exec("count"), @r###"
    I will count from 1 to 10 (included)
    1 2 3 4 5 6 7 8 9 10
    "###);
}

#[test]
fn execute_example_factorial() {
    insta::assert_snapshot!(exec("factorial"), @r###"
    I will compute some factorials for you
    fact(1) = 1
    fact(2) = 2
    fact(3) = 6
    fact(4) = 24
    fact(5) = 120
    fact(6) = 720
    fact(7) = 5040
    fact(8) = 40320
    fact(9) = 362880
    fact(10) = 3628800
    I'm done!
    "###);
}
