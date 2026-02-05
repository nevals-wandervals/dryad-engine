//! Definition of the basic mathematical /computational functionality for the operation of the engine and the client.

pub mod position;
pub mod units;

use crate::{math::position::Position, pos};

/// Converts the position to the index of the object with the specified width of grid's row.
/// Checks the non-negativity of values.
/// ``` rust
/// use dryad::math::{get_index, position::Position};
/// use dryad::pos;
///
/// // idxs: [0 (0, 0), 1 (1, 0), 2(0, 1), 3(1, 1)]
/// let pos = pos!(1, 1);
/// let idx = get_index(pos, 2);
///
/// assert_eq!(idx, 3);
/// ```
pub fn get_index(pos: Position, row_width: i32) -> usize {
    let idx = pos.y() * row_width + pos.x();
    if idx < 0 {
        return 0;
    }

    idx as usize
}

/// Converts the index to the position of the object with the specified width of grid's row.
/// ``` rust
/// use dryad::math::{get_position, position::Position};
/// use dryad::pos;
///
/// // idxs: [0 (0, 0), 1 (1, 0), 2(0, 1), 3(1, 1)]
/// let idx = 3;
/// let pos = get_position(idx, 2);
///
/// assert_eq!(pos, pos!(1, 1));
/// ```
pub fn get_position(index: usize, row_width: i32) -> Position {
    pos!(index as i32 % row_width, index as i32 / row_width)
}
