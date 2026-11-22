//! Finite pager model. Validate source pairs before publishing a Page.
//! Window construction visits at most seven logical indices.
use glory::render::{Direction, Error};
pub const MAX_CAROUSEL_COUNT: usize = 1_000_000;
pub const MAX_CAROUSEL_OVERSCAN: usize = 3;
pub const MAX_CAROUSEL_INDICATORS: usize = 9;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    pub count: usize,
    pub current: usize,
    pub looping: bool,
    pub overscan: usize,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    Previous,
    Next,
    First,
    Last,
}
impl Page {
    pub fn validate(self) -> Result<Self, Error> {
        if self.count > MAX_CAROUSEL_COUNT
            || self.overscan > MAX_CAROUSEL_OVERSCAN
            || self.count == 0 && self.current != 0
            || self.count > 0 && self.current >= self.count
        {
            return Err(Error::InvalidProps(
                "carousel count/current/overscan budget",
            ));
        }
        Ok(self)
    }
    pub fn propose(self, step: Step) -> Option<usize> {
        if self.validate().is_err() || self.count == 0 {
            return None;
        }
        let next = match step {
            Step::First => 0,
            Step::Last => self.count - 1,
            Step::Next if self.current + 1 < self.count => self.current + 1,
            Step::Previous if self.current > 0 => self.current - 1,
            Step::Next if self.looping => 0,
            Step::Previous if self.looping => self.count - 1,
            _ => self.current,
        };
        (next != self.current).then_some(next)
    }
    pub fn window(self) -> Vec<usize> {
        if self.validate().is_err() {
            return Vec::new();
        }
        let mut window = Vec::with_capacity(2 * self.overscan + 1);
        if self.count == 0 {
            return window;
        }
        if self.looping {
            window.push(self.current);
            for distance in 1..=self.overscan {
                window.push((self.current + distance) % self.count);
                window.push((self.current + self.count - distance % self.count) % self.count);
            }
            window.sort_unstable();
            window.dedup();
        } else {
            let first = self.current.saturating_sub(self.overscan);
            let end = self
                .current
                .saturating_add(self.overscan)
                .saturating_add(1)
                .min(self.count);
            window.extend(first..end);
        }
        window
    }
    pub fn indicators(self) -> Vec<usize> {
        if self.validate().is_err() || self.count == 0 {
            return Vec::new();
        }
        let length = self.count.min(MAX_CAROUSEL_INDICATORS);
        let start = self
            .current
            .saturating_sub(length / 2)
            .min(self.count - length);
        (start..start + length).collect()
    }
    pub(super) fn index_at(self, delta: isize) -> usize {
        let count = self.count.max(1) as isize;
        let target = self.current as isize + delta;
        if self.looping {
            target.rem_euclid(count) as usize
        } else {
            target.clamp(0, count - 1) as usize
        }
    }
    pub(super) fn distance_to(self, index: usize) -> isize {
        let d = index as isize - self.current as isize;
        if self.looping && d.unsigned_abs() * 2 > self.count {
            d - d.signum() * self.count as isize
        } else {
            d
        }
    }
    pub(super) fn scroll_offset(
        self,
        delta: f64,
        extent: f64,
        direction: Direction,
        origin: f64,
    ) -> Result<f64, Error> {
        self.validate()?;
        if !delta.is_finite() || !extent.is_finite() || extent <= 0. || !origin.is_finite() {
            return Err(Error::InvalidProps("carousel free scroll geometry"));
        }
        if self.count < 2 {
            return Ok(0.);
        }
        let offset = (origin
            + if direction == Direction::Rtl {
                delta / extent
            } else {
                -delta / extent
            })
        .clamp(-(self.overscan as f64), self.overscan as f64);
        Ok(if self.looping {
            offset
        } else {
            offset.clamp(
                -(self.current as f64),
                (self.count - 1 - self.current) as f64,
            )
        })
    }
    pub fn swipe(
        self,
        delta: f64,
        extent: f64,
        direction: Direction,
    ) -> Result<Option<usize>, Error> {
        self.validate()?;
        if !delta.is_finite() || !extent.is_finite() || extent <= 0. {
            return Err(Error::InvalidProps("carousel swipe geometry"));
        }
        let threshold = (extent * 0.15).clamp(24., 80.);
        if delta.abs() < threshold {
            return Ok(None);
        }
        let forward = if direction == Direction::Rtl {
            delta > 0.
        } else {
            delta < 0.
        };
        Ok(self.propose(if forward { Step::Next } else { Step::Previous }))
    }
}
