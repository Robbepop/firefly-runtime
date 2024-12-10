use alloc::vec::Vec;

pub(crate) fn read_all<R, E>(mut stream: R) -> Result<Vec<u8>, E>
where
    R: embedded_io::Read<Error = E>,
    E: embedded_io::Error,
{
    let mut result = Vec::new();
    loop {
        let size = stream.read(&mut result[..])?;
        if size == 0 {
            break;
        }
    }
    result.shrink_to_fit();
    Ok(result)
}
