extern crate cc;
extern crate glob;

use std::path::Path;

fn main() {
    let common_dir = Path::new("pqclean/common");
    let common_files = vec![
        common_dir.join("fips202.c"),
        common_dir.join("aes.c"),
        common_dir.join("sp800-185.c"),
    ];

    cc::Build::new()
        .flag("-std=c99")
        .include("pqclean/common")
        .files(common_files.into_iter())
        .compile("pqclean_common");

    // Link in pqcrypto_internals
    println!("cargo:rustc-link-lib=pqcrypto_internals");

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIIIc-classic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIIIc-classic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIIIc-cyclic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIIIc-cyclic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIIIc-cyclic-compressed/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIIIc-cyclic-compressed_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIa-classic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIa-classic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIa-cyclic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIa-cyclic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowIa-cyclic-compressed/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowIa-cyclic-compressed_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowVc-classic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowVc-classic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowVc-cyclic/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowVc-cyclic_clean");
    }

    {
        let mut builder = cc::Build::new();
        let target_dir = Path::new("pqclean/crypto_sign/rainbowVc-cyclic-compressed/clean");
        let scheme_files = glob::glob(target_dir.join("*.c").to_str().unwrap()).unwrap();
        builder
            .flag("-std=c99")
            .include("include")
            .include("pqclean/common")
            .include(target_dir)
            .files(
                scheme_files
                    .into_iter()
                    .map(|p| p.unwrap().to_string_lossy().into_owned()),
            );
        builder.compile("rainbowVc-cyclic-compressed_clean");
    }
}
