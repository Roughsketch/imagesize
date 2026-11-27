#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "jpeg")]
fn jpeg_test() {
    let dim = size("tests/images/jpg/test.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 690,
            height: 298
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn jpeg_extra_info_test() {
    let dim = size("tests/images/jpg/extra.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1500,
            height: 844
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn issue_9_test() {
    let dim = size("tests/images/jpg/issue-9.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1360,
            height: 1904
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn jpg_unexpected_eof() {
    let dim = size("tests/images/jpg/unexpected_eof.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 3047,
            height: 2008
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn jpg_unexpected_eof_2() {
    let dim = size("tests/images/jpg/unexpected_eof_2.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 4980,
            height: 3321
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn jpg_unexpected_eof_3() {
    let dim = size("tests/images/jpg/unexpected_eof_3.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 2995,
            height: 1998
        }
    );
}

#[test]
#[cfg(feature = "jpeg")]
fn jpg_wrong_size() {
    let dim = size("tests/images/jpg/wrong_size.jpg").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1080,
            height: 1080
        }
    );
}

#[cfg(feature = "jpeg")]
mod markers {
    use imagesize::{blob_size, image_type, reader_size, ImageError, ImageSize, ImageType};
    use std::io::{self, BufRead, Cursor, Read, Seek, SeekFrom};
    use std::ops::Range;

    const EXPECTED: ImageSize = ImageSize {
        width: 32,
        height: 16,
    };

    fn frame(marker: u8) -> [u8; 13] {
        // One-component frame: length, precision, height, width, component data.
        [0xFF, marker, 0, 11, 8, 0, 16, 0, 32, 1, 1, 0x11, 0]
    }

    fn jpeg(segments: &[u8], frame_marker: u8) -> Vec<u8> {
        let mut data = vec![0xFF, 0xD8];
        data.extend_from_slice(segments);
        data.extend_from_slice(&frame(frame_marker));
        data
    }

    #[test]
    fn fill_bytes_before_metadata_and_frames() {
        for marker in [0xC0, 0xC2] {
            for fill_count in [0, 1, 4] {
                let mut segments = vec![0xFF; fill_count];
                segments.extend_from_slice(&[0xFF, 0xE0, 0, 4, 0x12, 0x34]);
                segments.resize(segments.len() + fill_count, 0xFF);
                let data = jpeg(&segments, marker);

                assert_eq!(image_type(&data).unwrap(), ImageType::Jpeg);
                assert_eq!(blob_size(&data).unwrap(), EXPECTED);
            }
        }
    }

    #[test]
    fn fill_bytes_in_existing_image() {
        let mut data = include_bytes!("images/jpg/test.jpg").to_vec();
        data.insert(2, 0xFF);
        assert_eq!(
            blob_size(&data).unwrap(),
            ImageSize {
                width: 690,
                height: 298,
            }
        );
    }

    #[test]
    fn all_supported_frame_markers() {
        for marker in [
            0xC0, 0xC1, 0xC2, 0xC3, 0xC5, 0xC6, 0xC7, 0xC9, 0xCA, 0xCB, 0xCD, 0xCE, 0xCF,
        ] {
            assert_eq!(blob_size(&jpeg(&[], marker)).unwrap(), EXPECTED);
        }
    }

    #[test]
    fn non_frame_markers_are_skipped() {
        for marker in [0xC4, 0xC8, 0xCC] {
            let mut segment = frame(marker);
            segment[6] = 1;
            segment[8] = 2;
            assert_eq!(blob_size(&jpeg(&segment, 0xC0)).unwrap(), EXPECTED);
        }
    }

    #[test]
    fn metadata_with_markers_and_embedded_jpeg_is_opaque() {
        let mut payload = vec![0xFF, 0, 0xFF, 0xFF, 0xFF, 0xDA, 0xFF, 0xD9];
        payload.extend_from_slice(include_bytes!("images/jpg/test.jpg"));

        for marker in [0xE1, 0xFE] {
            let mut segment = vec![0xFF, marker];
            segment.extend_from_slice(&u16::try_from(payload.len() + 2).unwrap().to_be_bytes());
            segment.extend_from_slice(&payload);
            assert_eq!(blob_size(&jpeg(&segment, 0xC0)).unwrap(), EXPECTED);
        }
    }

    #[test]
    fn standalone_tem_has_no_length() {
        assert_eq!(
            blob_size(&jpeg(&[0xFF, 0x01, 0xFF, 0xFF, 0x01], 0xC0)).unwrap(),
            EXPECTED
        );
    }

    #[test]
    fn unexpected_standalone_and_scan_markers_are_corrupt() {
        for marker in 0xD0..=0xDA {
            let data = jpeg(&[0xFF, marker], 0xC0);
            assert!(
                matches!(blob_size(&data), Err(ImageError::CorruptedImage)),
                "marker {marker:02X}"
            );
        }
    }

    #[test]
    fn invalid_marker_boundaries_are_corrupt() {
        for segment in [&[0xFF, 0x00][..], &[0xFF, 0xFF, 0x00], &[0x12, 0x34]] {
            // A valid empty segment keeps type detection independent of the bad boundary.
            let mut segments = vec![0xFF, 0xFE, 0, 2];
            segments.extend_from_slice(segment);
            assert!(matches!(
                blob_size(&jpeg(&segments, 0xC0)),
                Err(ImageError::CorruptedImage)
            ));
        }
    }

    #[test]
    fn segment_lengths_are_checked() {
        for length in [0, 1] {
            assert!(matches!(
                blob_size(&jpeg(&[0xFF, 0xE0, 0, length], 0xC0)),
                Err(ImageError::CorruptedImage)
            ));
        }
        assert_eq!(
            blob_size(&jpeg(&[0xFF, 0xFE, 0, 2], 0xC0)).unwrap(),
            EXPECTED
        );

        for length in 0..11 {
            let mut data = jpeg(&[], 0xC0);
            data[5] = length;
            assert!(matches!(blob_size(&data), Err(ImageError::CorruptedImage)));
        }
    }

    #[test]
    fn truncation_remains_an_io_error() {
        let data = jpeg(&[0xFF, 0xE0, 0, 4, 0x12, 0x34, 0xFF, 0xFF], 0xC0);
        let dimensions_end = data.len() - 4;
        for end in 2..dimensions_end {
            // Use the typed entry point to also cover inputs shorter than the type sniff.
            let result = ImageType::Jpeg.reader_size(&mut Cursor::new(&data[..end]));
            assert!(
                matches!(result, Err(ImageError::IoError(ref e)) if e.kind() == io::ErrorKind::UnexpectedEof),
                "truncated at byte {end}: {result:?}"
            );
        }
        assert_eq!(blob_size(&data[..dimensions_end]).unwrap(), EXPECTED);
    }

    struct RecordingReader<'a> {
        inner: Cursor<&'a [u8]>,
        reads: Vec<Range<u64>>,
        seeks: Vec<(u64, u64)>,
    }

    impl Read for RecordingReader<'_> {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            let start = self.inner.position();
            let count = self.inner.read(buffer)?;
            self.reads.push(start..self.inner.position());
            Ok(count)
        }
    }

    impl BufRead for RecordingReader<'_> {
        fn fill_buf(&mut self) -> io::Result<&[u8]> {
            self.inner.fill_buf()
        }

        fn consume(&mut self, amount: usize) {
            let start = self.inner.position();
            self.inner.consume(amount);
            self.reads.push(start..self.inner.position());
        }
    }

    impl Seek for RecordingReader<'_> {
        fn seek(&mut self, position: SeekFrom) -> io::Result<u64> {
            let start = self.inner.position();
            let end = self.inner.seek(position)?;
            self.seeks.push((start, end));
            Ok(end)
        }
    }

    #[test]
    fn skips_large_metadata_and_stops_after_dimensions() {
        // Put the large payload beyond the public API's initial 12-byte type sniff.
        let mut data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0, 8, 1, 2, 3, 4, 5, 6];
        data.extend_from_slice(&[0xFF, 0xE1, 0xFF, 0xFF]);
        let payload_start = data.len();
        data.resize(payload_start + usize::from(u16::MAX) - 2, 0xFF);
        let payload = payload_start as u64..data.len() as u64;
        let dimensions_end = data.len() as u64 + 9;
        data.extend_from_slice(&frame(0xC0));
        data.extend_from_slice(&[0xFF; 64]);

        let mut reader = RecordingReader {
            inner: Cursor::new(&data),
            reads: Vec::new(),
            seeks: Vec::new(),
        };
        assert_eq!(reader_size(&mut reader).unwrap(), EXPECTED);
        assert_eq!(reader.inner.position(), dimensions_end);
        assert!(reader.seeks.contains(&(payload.start, payload.end)));
        assert!(reader.reads.iter().all(|read| {
            (read.end <= payload.start || read.start >= payload.end) && read.end <= dimensions_end
        }));
    }
}
