use crate::decompressors::tar_common::tar_extract;
use crate::{DecompressError, Decompression, Decompressor, ExtractOpts, Listing};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};
use tar::Archive;

use super::tar_common::tar_list;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?i)\.t(ar\.zst|zst)$").unwrap();
}

fn build_archive(archive: &Path) -> Result<Archive<Box<dyn Read>>, DecompressError> {
    let fd = BufReader::new(File::open(archive)?);
    let out: Archive<Box<dyn Read>> = Archive::new(Box::new(zstd::stream::read::Decoder::new(fd)?));
    Ok(out)
}

#[derive(Default)]
pub struct Tarzst {
    re: Option<Regex>,
}

impl Tarzst {
    #[must_use]
    pub fn new(re: Option<Regex>) -> Self {
        Self { re }
    }
    #[must_use]
    pub fn build(re: Option<Regex>) -> Box<Self> {
        Box::new(Self::new(re))
    }
}

impl Decompressor for Tarzst {
    fn test(&self, archive: &Path) -> bool {
        archive
            .file_name()
            .and_then(std::ffi::OsStr::to_str)
            .map_or(false, |f| self.re.as_ref().unwrap_or(&*RE).is_match(f))
    }

    fn list(&self, archive: &Path) -> Result<Listing, DecompressError> {
        Ok(Listing {
            id: "tarzst",
            entries: tar_list(&mut build_archive(archive)?)?,
        })
    }

    fn decompress(
        &self,
        archive: &Path,
        to: &Path,
        opts: &ExtractOpts,
    ) -> Result<Decompression, DecompressError> {
        Ok(Decompression {
            id: "tarzst",
            files: tar_extract(&mut build_archive(archive)?, to, opts)?,
        })
    }
}
