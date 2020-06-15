use super::Error;
use crate::Sign;
use btoi::btoi;

pub(crate) fn split2_at_space(
    d: &[u8],
    is_valid: impl FnOnce(&[u8], &[u8]) -> bool,
) -> Result<(&[u8], &[u8]), Error> {
    let mut t = d.splitn(2, |&b| b == b' ');
    Ok(match (t.next(), t.next()) {
        (Some(t1), Some(t2)) => {
            if !is_valid(t1, t2) {
                return Err(Error::ParseError(
                    "Invalid space separated tokens - validation failed",
                    d.to_owned(),
                ));
            }
            (t1, t2)
        }
        _ => {
            return Err(Error::ParseError(
                "Invalid tokens - expected 2 when split at space",
                d.to_owned(),
            ))
        }
    })
}

pub(crate) fn parse_timezone_offset(d: &[u8]) -> Result<(i32, Sign), Error> {
    if d.len() < 5 || !(d[0] == b'+' || d[0] == b'-') {
        return Err(Error::ParseError("invalid timezone offset", d.to_owned()));
    }
    let sign = if d[0] == b'-' {
        Sign::Minus
    } else {
        Sign::Plus
    };
    let hours = btoi::<i32>(&d[..3])
        .map_err(|e| Error::ParseIntegerError("invalid 'hours' string", d[..3].to_owned(), e))?;
    let minutes = btoi::<i32>(&d[3..])
        .map_err(|e| Error::ParseIntegerError("invalid 'minutes' string", d[3..].to_owned(), e))?;
    Ok((hours * 3600 + minutes * 60, sign))
}
