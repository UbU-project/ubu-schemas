//! Compare format-validated RFC 3339 timestamps without adding a dependency.
//! Schema `format` and `pattern` validate the calendar fields; this keyword
//! supplies only the cross-field ordering JSON Schema cannot express.
fn instant(value: &str) -> Option<(i64, &str)> {
    let number = |start, end| value.get(start..end)?.parse::<i64>().ok();
    let year = number(0, 4)?;
    let month = number(5, 7)?;
    let day = number(8, 10)?;
    let previous = year - 1;
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let month_days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
    let days = 365 * previous + previous.div_euclid(4) - previous.div_euclid(100)
        + previous.div_euclid(400)
        + *month_days.get(usize::try_from(month - 1).ok()?)?
        + i64::from(leap && month > 2)
        + day
        - 1;
    let tail = value.get(19..)?;
    let zone_index = tail.find(['Z', '+', '-'])?;
    let fraction = tail
        .get(..zone_index)?
        .strip_prefix('.')
        .unwrap_or("")
        .trim_end_matches('0');
    let zone = tail.get(zone_index..)?;
    let offset = if zone == "Z" {
        0
    } else {
        let hours = zone.get(1..3)?.parse::<i64>().ok()?;
        let minutes = zone.get(4..6)?.parse::<i64>().ok()?;
        (hours * 3600 + minutes * 60) * if zone.starts_with('-') { -1 } else { 1 }
    };
    Some((
        days * 86400 + number(11, 13)? * 3600 + number(14, 16)? * 60 + number(17, 19)? - offset,
        fraction,
    ))
}

pub fn strictly_before(start: &str, end: &str) -> bool {
    instant(start).zip(instant(end)).is_some_and(|(a, b)| a < b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timestamps_compare_instants_across_offsets_days_and_fractional_seconds() {
        for (start, end) in [
            ("2026-09-27T15:00:00Z", "2026-09-27T15:30:00Z"),
            ("2026-09-27T17:00:00+02:00", "2026-09-27T15:30:00Z"),
            ("2026-12-31T23:30:00-01:00", "2027-01-01T00:31:00Z"),
            ("2024-02-29T23:59:59.999Z", "2024-03-01T00:00:00Z"),
            ("2026-09-27T15:00:00.1Z", "2026-09-27T15:00:00.11Z"),
        ] {
            assert!(strictly_before(start, end));
            assert!(!strictly_before(end, start));
        }
        for (start, end) in [
            ("2026-09-27T15:00:00Z", "2026-09-27T17:00:00+02:00"),
            ("2026-09-27T15:00:00.1Z", "2026-09-27T15:00:00.100Z"),
        ] {
            assert!(!strictly_before(start, end));
            assert!(!strictly_before(end, start));
        }
    }
}
