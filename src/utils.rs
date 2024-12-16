use alloc::vec::Vec;

pub(crate) fn read_all<R, E>(stream: R) -> Result<Vec<u8>, E>
where
    R: embedded_io::Read<Error = E>,
    E: embedded_io::Error,
{
    let mut buf = Vec::new();
    read_all_into(stream, &mut buf)?;
    Ok(buf)
}

pub(crate) fn read_all_into<R, E>(mut stream: R, buf: &mut Vec<u8>) -> Result<(), E>
where
    R: embedded_io::Read<Error = E>,
    E: embedded_io::Error,
{
    const CHUNK_SIZE: usize = 64;
    let mut filled_size = 0;
    loop {
        buf.resize(filled_size + CHUNK_SIZE, 0);
        let gained_size = stream.read(&mut buf[filled_size..])?;
        if gained_size == 0 {
            break;
        }
        filled_size += gained_size;
    }
    buf.truncate(filled_size);
    buf.shrink_to_fit();
    Ok(())
}

/// Read stream into the buffer, trying to 100% fill the buffer.
///
/// Returns the number of bytes read.
pub(crate) fn read_into<R, E>(mut stream: R, buf: &mut [u8]) -> Result<usize, E>
where
    R: embedded_io::Read<Error = E>,
    E: embedded_io::Error,
{
    let mut buf = buf;
    let mut filled = 0;
    while !buf.is_empty() {
        let n = stream.read(buf)?;
        if n == 0 {
            break;
        }
        filled += n;
        buf = &mut buf[n..];
    }
    Ok(filled)
}

pub(crate) fn write_all<W, E>(mut stream: W, buf: &[u8]) -> Result<usize, E>
where
    W: embedded_io::Write<Error = E>,
    E: embedded_io::Error,
{
    let mut buf = buf;
    let mut filled = 0;
    while !buf.is_empty() {
        let n = stream.write(buf)?;
        if n == 0 {
            break;
        }
        filled += n;
        buf = &buf[n..];
    }
    Ok(filled)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_all() {
        let given = alloc::vec![1, 2, 3, 4];
        let res = read_all(&given[..]).unwrap();
        assert_eq!(res, given);
    }
}
