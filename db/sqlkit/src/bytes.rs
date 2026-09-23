// Lab 02 — your readers go here, above the tests.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::warmup::{has_sqlite_magic, SQLITE_MAGIC};

    fn fixture(name: &str) -> Vec<u8> {
        let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name);
        std::fs::read(&p).unwrap_or_else(|e| panic!("fixture {name}: {e}"))
    }

    #[test]
    fn big_endian_readers_agree_with_the_header() {
        todo!()
    }

    #[test]
    fn endianness_is_a_choice_the_file_made() {
        todo!()
    }

    #[test]
    fn short_slices_return_none() {
        todo!()
    }

    #[test]
    fn varint_matches_measured_bytes() {
        todo!()
    }

    #[test]
    fn hexdump_line_is_fixed_width() {
        todo!()
    }
}
