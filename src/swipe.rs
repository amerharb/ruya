//! A swipe across the picture turns the scene, as the edges do: to the left
//! for the next, to the right for the one before.
//!
//! Only a flick counts — far enough, quick enough and more sideways than up
//! or down — so a tap still names a thing, and a slow drag or a pinch is left
//! alone. And only once the picture can scroll no further that way: on a
//! phone held upright, where it fills the height and scrolls sideways, a
//! swipe first takes it to its end, as a zoomed-in photo does in Photos.
//! Wherever the whole picture shows, it is at both ends at once, and every
//! swipe turns.
//!
//! Plain arithmetic on where and when a touch began and ended, so `cargo
//! test` can check it; the stage listens for the touches.

use crate::scene::Step;

/// How far a flick goes sideways, at least, in CSS pixels.
const FAR: f64 = 50.0;

/// How long it takes, at most, in milliseconds.
const QUICK: f64 = 500.0;

/// How many times further sideways than up or down it goes, at least.
const LEVEL: f64 = 2.0;

/// A touch on the picture as it began: where, when, and whether the picture
/// was scrolled to its left end, its right end, or, all of it showing, both.
#[derive(Clone, Copy, Debug)]
pub struct Swipe {
    pub x: f64,
    pub y: f64,
    pub time: f64,
    pub at_left: bool,
    pub at_right: bool,
}

impl Swipe {
    /// The step the touch takes, ending at (`x`, `y`) at `time`: none unless
    /// it was a flick, and the picture had nothing more to show the way it
    /// went.
    pub fn end(self, x: f64, y: f64, time: f64) -> Option<Step> {
        let (dx, dy) = (x - self.x, y - self.y);
        let flick = dx.abs() >= FAR && dx.abs() >= LEVEL * dy.abs() && time - self.time <= QUICK;
        if !flick {
            return None;
        }
        // pushed to the left, what lies to the right comes in: the next
        // scene, once the picture's own right end is showing
        if dx < 0.0 {
            self.at_right.then_some(Step::Next)
        } else {
            self.at_left.then_some(Step::Previous)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A touch at (200, 300) at time 0, on a picture scrolled to the ends
    /// given.
    fn on(at_left: bool, at_right: bool) -> Swipe {
        Swipe {
            x: 200.0,
            y: 300.0,
            time: 0.0,
            at_left,
            at_right,
        }
    }

    #[test]
    fn a_flick_left_goes_on_and_right_goes_back() {
        let whole = on(true, true);
        assert_eq!(whole.end(120.0, 310.0, 200.0), Some(Step::Next));
        assert_eq!(whole.end(280.0, 290.0, 200.0), Some(Step::Previous));
        // just far enough and just quick enough; just level enough
        assert_eq!(whole.end(150.0, 300.0, 500.0), Some(Step::Next));
        assert_eq!(whole.end(300.0, 350.0, 100.0), Some(Step::Previous));
    }

    #[test]
    fn a_tap_a_short_or_slow_drag_and_a_slant_turn_nothing() {
        let whole = on(true, true);
        assert_eq!(whole.end(203.0, 302.0, 90.0), None);
        assert_eq!(whole.end(160.0, 300.0, 100.0), None);
        assert_eq!(whole.end(100.0, 300.0, 800.0), None);
        assert_eq!(whole.end(120.0, 360.0, 200.0), None);
        assert_eq!(whole.end(200.0, 120.0, 150.0), None);
    }

    #[test]
    fn a_picture_with_more_to_show_scrolls_before_it_turns() {
        let middle = on(false, false);
        assert_eq!(middle.end(120.0, 300.0, 200.0), None);
        assert_eq!(middle.end(280.0, 300.0, 200.0), None);
        // at its left end it can only go back, at its right only on
        assert_eq!(
            on(true, false).end(280.0, 300.0, 200.0),
            Some(Step::Previous)
        );
        assert_eq!(on(true, false).end(120.0, 300.0, 200.0), None);
        assert_eq!(on(false, true).end(120.0, 300.0, 200.0), Some(Step::Next));
        assert_eq!(on(false, true).end(280.0, 300.0, 200.0), None);
    }
}
