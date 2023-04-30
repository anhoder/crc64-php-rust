use phper_test::{cli::test_php_scripts, utils::get_lib_path};
use std::{
    env,
    path::{Path, PathBuf},
};

#[test]
fn test_php() {
    test_php_scripts(
        get_lib_path(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target"),
            "crc64",
        ),
        &[&Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("php")
            .join("test.php")],
    );
}
