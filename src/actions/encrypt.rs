use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Context;

use crate::actions::show::Metadata;

pub(crate) fn encrypt(password: &[u8], in_: &Path, out: &Path) -> anyhow::Result<()> {
    let in_reader = File::open(in_)?;
    let mut out_writer = File::create(out)?;

    let in_metadata = Metadata {
        name: in_
            .file_name()
            .context("input file does not contain a name")?
            .to_owned()
            .into_string()
            .map_err(|_| anyhow::anyhow!("input file name must be UTF-8"))?,
        size_bytes: in_reader.metadata()?.len(),
        algo: String::from("AES-GCM"),
    };

    let additional_data = serde_json::to_string(&in_metadata)?.into_bytes();
    let additional_data_len: u64 = additional_data.len() as u64;

    let password_salt = crate::crypto::random_bytes::random_bytes::<32>()?;
    let key = crate::crypto::pbkdf::pbkdf::<32>(password, &password_salt);
    let nonce = crate::crypto::random_bytes::random_bytes::<7>()?;

    out_writer.write_all(&additional_data_len.to_le_bytes())?;
    out_writer.write_all(&additional_data)?;
    out_writer.write_all(&password_salt)?;
    out_writer.write_all(&nonce)?;

    for chunk in crate::crypto::encrypt::encrypt(&key, &nonce, &additional_data, in_reader) {
        let chunk = chunk?;

        out_writer.write_all(&chunk)?;
    }

    Ok(())
}
