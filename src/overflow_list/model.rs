//! Finite packing of the current registered inline items, never a data source.
use glory::ui::Result;

pub const MAX_OVERFLOW_ITEMS: usize = 128;
pub const MAX_OVERFLOW_QUERIES: usize = MAX_OVERFLOW_ITEMS + 2;
pub const MAX_OVERFLOW_EXTENT: f64 = 1_000_000.;

pub fn extent(value: f64) -> Result {
    if value.is_finite() && (0.0..=MAX_OVERFLOW_EXTENT).contains(&value) {
        Ok(())
    } else {
        Err(glory::render::Error::InvalidProps(
            "overflow dimensions must be finite and within 0..=1000000 pixels",
        )
        .into())
    }
}

/// The first prefix that fits, reserving the overflow control only when needed.
pub fn capacity(
    widths: &[f64],
    available: f64,
    trigger: f64,
    gap: f64,
    keep_trigger: bool,
) -> Result<usize> {
    if widths.len() > MAX_OVERFLOW_ITEMS {
        return Err(glory::render::Error::InvalidProps(
            "overflow list exceeds 128 registered items",
        )
        .into());
    }
    extent(available)?;
    extent(trigger)?;
    extent(gap)?;
    for width in widths {
        extent(*width)?;
    }
    Ok(pack(
        widths,
        available,
        trigger,
        gap,
        keep_trigger,
        1,
        false,
    ))
}

/// Pack in display order, placing the overflow trigger at the edge it replaces.
pub(super) fn pack(
    widths: &[f64],
    available: f64,
    trigger: f64,
    gap: f64,
    keep_trigger: bool,
    max_rows: usize,
    from_start: bool,
) -> usize {
    let fits = |items: &[f64], with_trigger: bool| {
        let mut row = 1;
        let mut used = None;
        let first = (with_trigger && from_start).then_some(trigger);
        let last = (with_trigger && !from_start).then_some(trigger);
        for width in first.into_iter().chain(items.iter().copied()).chain(last) {
            if width > available + 0.25 {
                return false;
            }
            let next = used.map_or(width, |used: f64| used + gap + width);
            if next > available + 0.25 {
                row += 1;
                if row > max_rows {
                    return false;
                }
                used = Some(width);
            } else {
                used = Some(next);
            }
        }
        true
    };
    if !keep_trigger && fits(widths, false) {
        return widths.len();
    }
    let mut low = 0;
    let mut high = widths.len();
    while low < high {
        let count = low + (high - low).div_ceil(2);
        let items = if from_start {
            &widths[widths.len() - count..]
        } else {
            &widths[..count]
        };
        if fits(items, true) {
            low = count;
        } else {
            high = count - 1;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn prefix_reserves_a_real_control_only_for_actual_overflow() {
        assert_eq!(capacity(&[60., 70., 80.], 226., 40., 8., false).unwrap(), 3);
        assert_eq!(capacity(&[60., 70., 80.], 225., 40., 8., false).unwrap(), 2);
        assert_eq!(capacity(&[60., 70., 80.], 120., 40., 8., false).unwrap(), 1);
        assert_eq!(capacity(&[60., 70., 80.], 30., 40., 8., false).unwrap(), 0);
        assert_eq!(capacity(&[], 0., 40., 8., false).unwrap(), 0);
        assert_eq!(capacity(&[60., 70., 80.], 226., 40., 8., true).unwrap(), 2);
    }
    #[test]
    fn wrapping_reserves_the_trigger_on_the_last_row_and_preserves_the_requested_edge() {
        assert_eq!(
            pack(&[40., 40., 40., 40., 40.], 100., 20., 4., false, 2, false),
            3
        );
        assert_eq!(
            pack(&[40., 40., 40., 40.], 100., 20., 4., false, 2, false),
            4
        );
        assert_eq!(pack(&[80., 15., 15.], 75., 20., 4., false, 1, true), 2);
        assert_eq!(pack(&[80., 15., 15.], 75., 20., 4., false, 1, false), 0);
    }

    #[test]
    fn every_dimension_and_current_membership_is_bounded() {
        for bad in [f64::NAN, f64::INFINITY, -1., MAX_OVERFLOW_EXTENT + 1.] {
            assert!(capacity(&[bad], 100., 20., 8., false).is_err());
            assert!(capacity(&[10.], bad, 20., 8., false).is_err());
            assert!(capacity(&[10.], 100., bad, 8., false).is_err());
            assert!(capacity(&[10.], 100., 20., bad, false).is_err());
        }
        assert!(capacity(&[0.; MAX_OVERFLOW_ITEMS], 0., 0., 0., false).is_ok());
        assert!(capacity(&[0.; MAX_OVERFLOW_ITEMS + 1], 0., 0., 0., false).is_err());
        assert_eq!(MAX_OVERFLOW_QUERIES, 130);
    }
}
