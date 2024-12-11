use std::str::FromStr;

pub fn parse_tuple<T>(left: &str, right: &str) -> Result<(T, T), <T as FromStr>::Err>
where
    T: FromStr,
{
    let left = left.parse()?;
    let right = right.parse()?;
    Ok((left, right))
}
