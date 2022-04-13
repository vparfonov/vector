#[rustversion::before(1.34)]
compile_error!("`err-derive` depends on `proc-macro-error`, which requires rustc >= 1.34");

fn main() {
    #[cfg(test)]
    skeptic::generate_doc_tests(&["README.md"]);
}
