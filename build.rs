fn main() {
    uniffi::generate_scaffolding("src/crypto_service.udl").expect("Build script panics can be ignored");;
}