pub mod compression;

/* standard use */
use std::io;
use std::io::Read;
use std::path::Path;

/* project use */
use crate::error::Error;
use crate::level::Level;

/// Similar to [sniff](crate::sniff) but readable stream is now sendable
pub fn sniff<'a>(
    in_stream: Box<dyn io::Read + Send + 'a>,
) -> Result<(Box<dyn io::Read + Send + 'a>, compression::Format), Error> {
    let (first_bytes, in_stream) = crate::utils::get_first_five(in_stream)?;

    let cursor = io::Cursor::new(first_bytes);
    match compression::bytes2type(first_bytes) {
        e @ compression::Format::Gzip
        | e @ compression::Format::Bzip
        | e @ compression::Format::Lzma
        | e @ compression::Format::Zstd => Ok((Box::new(cursor.chain(in_stream)), e)),
        _ => Ok((Box::new(cursor.chain(in_stream)), compression::Format::No)),
    }
}

/// Similar to [get_reader](crate::get_reader) but readable stream is now sendable
pub fn get_reader<'a>(
    in_stream: Box<dyn io::Read + Send + 'a>,
) -> Result<(Box<dyn io::Read + Send + 'a>, compression::Format), Error> {
    // check compression
    let (in_stream, compression) = sniff(in_stream)?;

    // return readable and compression status
    match compression {
        compression::Format::Gzip => compression::new_gz_decoder(in_stream),
        compression::Format::Bzip => compression::new_bz2_decoder(in_stream),
        compression::Format::Lzma => compression::new_lzma_decoder(in_stream),
        compression::Format::Zstd => compression::new_zstd_decoder(in_stream),
        compression::Format::No => Ok((in_stream, compression::Format::No)),
    }
}

/// Similar to [get_writer](crate::get_writer) but writable stream is now sendable
pub fn get_writer<'a>(
    out_stream: Box<dyn io::Write + Send + 'a>,
    format: compression::Format,
    level: Level,
) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
    match format {
        compression::Format::Gzip => compression::new_gz_encoder(out_stream, level),
        compression::Format::Bzip => compression::new_bz2_encoder(out_stream, level),
        compression::Format::Lzma => compression::new_lzma_encoder(out_stream, level),
        compression::Format::Zstd => compression::new_zstd_encoder(out_stream, level),
        compression::Format::No => Ok(Box::new(out_stream)),
    }
}

/// Similar to [from_path](crate::from_path) but readable stream is now sendable
pub fn from_path<'a, P: AsRef<Path>>(
    path: P,
) -> Result<(Box<dyn io::Read + Send + 'a>, compression::Format), Error> {
    let readable = io::BufReader::new(std::fs::File::open(path)?);
    get_reader(Box::new(readable))
}

/// Similar to [to_path](crate::to_path) but writable stream is now sendable
pub fn to_path<'a, P: AsRef<Path>>(
    path: P,
    format: compression::Format,
    level: Level,
) -> Result<Box<dyn io::Write + Send + 'a>, Error> {
    let writable = io::BufWriter::new(std::fs::File::create(path)?);
    get_writer(Box::new(writable), format, level)
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::NamedTempFile;

    pub(crate) const SHORT_FILE: &'static [u8] = &[0o037, 0o213, 0o0, 0o0];
    pub(crate) const GZIP_FILE: &'static [u8] = &[0o037, 0o213, 0o0, 0o0, 0o0];
    pub(crate) const BZIP_FILE: &'static [u8] = &[0o102, 0o132, 0o0, 0o0, 0o0];
    pub(crate) const LZMA_FILE: &'static [u8] = &[0o375, 0o067, 0o172, 0o130, 0o132];
    pub(crate) const LOREM_IPSUM: &'static [u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut ultricies scelerisque diam, a scelerisque enim sagittis at.";
    pub(crate) const ZSTD_FILE: &'static [u8] = &[0x28, 0xb5, 0x2f, 0xfd, 0];

    mod compress_uncompress {
        use super::*;

        #[test]
        fn no_compression() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer =
                    get_writer(Box::new(wfile), compression::Format::No, Level::One).unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::No);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[cfg(feature = "gz")]
        #[test]
        fn gzip() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer =
                    get_writer(Box::new(wfile), compression::Format::Gzip, Level::Six).unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Gzip);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        #[cfg(not(feature = "bz2"))]
        fn no_bzip2_feature() {
            assert!(
                get_writer(Box::new(vec![]), compression::Format::Bzip, Level::Six).is_err(),
                "bz2 disabled, this assertion should fail"
            );

            assert!(
                get_reader(Box::new(&BZIP_FILE[..])).is_err(),
                "bz2 disabled, this assertion should fail"
            );
        }

        #[cfg(feature = "bz2")]
        #[test]
        fn bzip() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer =
                    get_writer(Box::new(wfile), compression::Format::Bzip, Level::Six).unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Bzip);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        #[cfg(not(feature = "lzma"))]
        fn no_lzma_feature() {
            assert!(
                get_writer(Box::new(vec![]), compression::Format::Lzma, Level::Six).is_err(),
                "lzma disabled, this assertion should fail"
            );

            assert!(
                get_reader(Box::new(&LZMA_FILE[..])).is_err(),
                "lzma disabled, this assertion should fail"
            );
        }

        #[cfg(feature = "lzma")]
        #[test]
        fn lzma() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer =
                    get_writer(Box::new(wfile), compression::Format::Lzma, Level::Six).unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Lzma);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }

        #[test]
        #[cfg(not(feature = "zstd"))]
        fn no_zstd_feature() {
            assert!(
                get_writer(Box::new(vec![]), compression::Format::Zstd, Level::Six).is_err(),
                "zstd disabled, this assertion should fail"
            );

            assert!(
                get_reader(Box::new(&ZSTD_FILE[..])).is_err(),
                "zstd disabled, this assertion should fail"
            );
        }

        #[cfg(feature = "zstd")]
        #[test]
        fn zstd() {
            let ofile = NamedTempFile::new().expect("Can't create tmpfile");

            {
                let wfile = ofile.reopen().expect("Can't create tmpfile");
                let mut writer =
                    get_writer(Box::new(wfile), compression::Format::Zstd, Level::Six).unwrap();
                writer
                    .write_all(LOREM_IPSUM)
                    .expect("Error during write of data");
            }

            let rfile = ofile.reopen().expect("Can't create tmpfile");
            let (mut reader, compression) =
                get_reader(Box::new(rfile)).expect("Error reading from tmpfile");

            assert_eq!(compression, compression::Format::Zstd);

            let mut buffer = Vec::new();
            reader
                .read_to_end(&mut buffer)
                .expect("Error during reading");
            assert_eq!(LOREM_IPSUM, buffer.as_slice());
        }
    }

    mod compression_format_detection {
        use super::*;

        #[test]
        fn gzip() {
            let (_, compression) = sniff(Box::new(GZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Gzip);
        }

        #[test]
        fn bzip() {
            let (_, compression) = sniff(Box::new(BZIP_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Bzip);
        }

        #[test]
        fn lzma() {
            let (_, compression) = sniff(Box::new(LZMA_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Lzma);
        }

        #[test]
        fn zstd() {
            let (_, compression) = sniff(Box::new(ZSTD_FILE)).expect("Error in read file");
            assert_eq!(compression, compression::Format::Zstd);
        }

        #[test]
        fn too_short() {
            let result = sniff(Box::new(SHORT_FILE));
            assert!(result.is_err());
        }

        #[test]
        fn no_compression() {
            let (_, compression) = sniff(Box::new(LOREM_IPSUM)).expect("Error in read file");
            assert_eq!(compression, compression::Format::No);
        }
    }
}
