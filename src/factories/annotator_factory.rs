use alvarium_annotator::constants;
use crate::SdkAnnotator;
use crate::annotations::{PkiAnnotator, SourceAnnotator, TlsAnnotator, TpmAnnotator};
use crate::config::SdkInfo;
use crate::errors::{Error, Result};


pub fn new_annotator(kind: constants::AnnotationType, cfg: SdkInfo) -> Result<Box<SdkAnnotator>> {
    if !kind.is_base_annotation_type() {
        return Err(Error::NotKnownProvider(kind.kind().to_string()))
    }

    match kind.kind() {
        "source" => Ok(Box::new(SourceAnnotator::new(&cfg)?)),
        "pki" => Ok(Box::new(PkiAnnotator::new(&cfg)?)),
        "tls" => Ok(Box::new(TlsAnnotator::new(&cfg)?)),
        "tpm" => Ok(Box::new(TpmAnnotator::new(&cfg)?)),
        _ => Err(Error::NotKnownProvider(kind.kind().to_string()))
    }
}