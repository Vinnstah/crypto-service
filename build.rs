use uniffi::TargetLanguage;
use uniffi_bindgen::generate_bindings;

fn main() {
    uniffi::generate_scaffolding("src/crypto_service.udl")
        .expect("Build script panics can be ignored");

    let udl_file = "src/crypto_service.udl";
    let out_dir = "./bindings/";
    uniffi::generate_bindings(
        udl_file.into(),
        None,
        vec![TargetLanguage::Swift],
        Some(out_dir.into()),
        None,
        None,
        true,
    )
    .expect("Generate binding panic");
}
