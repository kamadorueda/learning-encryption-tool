use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct Metadata {
    pub(crate) name: String,
    pub(crate) size_bytes: u64,
    pub(crate) algo: String,
}

pub(crate) fn show(in_: &Path) -> anyhow::Result<()> {
    let mut in_reader = File::open(in_)?;

    let additional_data_len = {
        let mut buf = [0; 8];
        in_reader.read_exact(&mut buf)?;
        u64::from_le_bytes(buf) as usize
    };

    let mut additional_data = vec![0; additional_data_len];
    in_reader.read_exact(&mut additional_data)?;

    let metadata = serde_json::from_slice::<Metadata>(&additional_data)?;

    println!("{:#?}", metadata);

    Ok(())
}
