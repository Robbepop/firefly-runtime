use alloc::vec::Vec;

pub(crate) fn read_all<R, E>(mut stream: R) -> Result<Vec<u8>, E>
where
    R: embedded_io::Read<Error = E>,
    E: embedded_io::Error,
{
    const CHUNK_SIZE: usize = 64;
    let mut result = Vec::new();
    let mut filled_size = 0;
    loop {
        result.resize(filled_size + CHUNK_SIZE, 0);
        let gained_size = stream.read(&mut result[filled_size..])?;
        if gained_size == 0 {
            break;
        }
        filled_size += gained_size;
    }
    result.truncate(filled_size);
    result.shrink_to_fit();
    Ok(result)
}
