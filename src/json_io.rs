use serde::Deserialize;

use embedded_svc::io::Read;

#[derive(Debug)]
pub enum SerdeError<E> {
    IoError(E),
    SerdeError,
}

pub fn read_buf<'a, R, T>(read: R, buf: &'a mut [u8]) -> Result<T, SerdeError<R::Error>>
where
    R: Read,
    T: Deserialize<'a>,
{
    let read_len = try_read_full(read, buf).map_err(|(e, _)| SerdeError::IoError(e))?;

    let (result, _) =
        serde_json_core::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

    Ok(result)
}

fn try_read_full<R: Read>(mut read: R, buf: &mut [u8]) -> Result<usize, (R::Error, usize)> {
    let mut offset = 0;
    let mut size = 0;

    loop {
        let size_read = read.read(&mut buf[offset..]).map_err(|e| (e, size))?;

        offset += size_read;
        size += size_read;

        if size_read == 0 || size == buf.len() {
            break;
        }
    }

    Ok(size)
}

#[cfg(feature = "nightly")]
pub mod asynch {
    use serde::Deserialize;

    use embedded_svc::io::asynch::Read;

    pub use super::SerdeError;

    pub async fn read_buf<'a, R, T>(read: R, buf: &'a mut [u8]) -> Result<T, SerdeError<R::Error>>
    where
        R: Read,
        T: Deserialize<'a>,
    {
        let read_len = try_read_full(read, buf)
            .await
            .map_err(|(e, _)| SerdeError::IoError(e))?;

        let (result, _) =
            serde_json_core::from_slice(&buf[..read_len]).map_err(|_| SerdeError::SerdeError)?;

        Ok(result)
    }

    async fn try_read_full<R: Read>(
        mut read: R,
        buf: &mut [u8],
    ) -> Result<usize, (R::Error, usize)> {
        let mut offset = 0;
        let mut size = 0;

        loop {
            let size_read = read.read(&mut buf[offset..]).await.map_err(|e| (e, size))?;

            offset += size_read;
            size += size_read;

            if size_read == 0 || size == buf.len() {
                break;
            }
        }

        Ok(size)
    }
}
