pub(crate) fn encode(value: u64) -> Result<i64, crate::KeyError> {
    i64::try_from(value).map_err(|_| crate::KeyError::LedgerUnavailable)
}

pub(crate) fn read(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<u64> {
    u64::try_from(row.get::<_, i64>(index)?).map_err(|error| {
        rusqlite::Error::FromSqlConversionFailure(
            index,
            rusqlite::types::Type::Integer,
            Box::new(error),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::{encode, read};

    #[test]
    fn signed_storage_limits_and_corrupt_negative_values_are_checked() {
        let maximum = u64::try_from(i64::MAX).unwrap();
        assert_eq!(encode(maximum).unwrap(), i64::MAX);
        assert!(encode(maximum + 1).is_err());
        let connection = rusqlite::Connection::open_in_memory().unwrap();
        assert_eq!(
            connection
                .query_row("SELECT ?1", [i64::MAX], |row| read(row, 0))
                .unwrap(),
            maximum
        );
        assert!(
            connection
                .query_row("SELECT -1", [], |row| read(row, 0))
                .is_err()
        );
    }
}
