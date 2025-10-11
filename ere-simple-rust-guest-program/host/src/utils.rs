use std::path::Path;

pub(crate) fn load_compiled_guest_program(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(path)
}
