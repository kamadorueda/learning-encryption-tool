use std::fs::File;
use std::io::Read;
use std::io::Write as _;
use std::path::Path;

pub(crate) fn decrypt(password: &[u8], in_: &Path, out: &Path) -> anyhow::Result<()> {
    let mut in_reader = File::open(in_)?;
    let mut out_writer = File::create(out)?;

    let additional_data_len = {
        let mut buf = [0; 8];
        in_reader.read_exact(&mut buf)?;
        u64::from_le_bytes(buf) as usize
    };

    let mut additional_data = vec![0; additional_data_len];
    in_reader.read_exact(&mut additional_data)?;

    let mut password_salt = [0; 32];
    in_reader.read_exact(&mut password_salt)?;

    let key = crate::crypto::pbkdf::pbkdf::<32>(password, &password_salt);

    let mut nonce = [0; 7];
    in_reader.read_exact(&mut nonce)?;

    for chunk in crate::crypto::decrypt::decrypt(&key, &nonce, &additional_data, in_reader) {
        let chunk = chunk?;

        out_writer.write_all(&chunk)?;
    }

    Ok(())
}
