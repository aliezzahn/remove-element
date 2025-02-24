# Remove Element

This project implements a solution to the "Remove Element" problem where we need to remove all occurrences of a given value from an array in-place and return the new length.

## Problem Description

Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` in-place. The order of the elements may be changed. Return the number of elements in `nums` which are not equal to `val`.

## Installation

1. Ensure you have Rust installed on your system
2. Clone the repository:

```bash
git clone https://github.com/yourusername/remove-element.git
cd remove-element
```

3. Build and run:

```bash
cargo run
```

## Usage

The solution is implemented in `src/main.rs`. You can run the examples:

```bash
cargo run
```

To run tests:

```bash
cargo test
```

## Solution Approach

The solution uses a two-pointer technique:

- One pointer (`k`) keeps track of the position to place valid elements
- We iterate through the array and copy non-matching elements to position `k`
- Time Complexity: O(n)
- Space Complexity: O(1)

See `docs/solution.md` for detailed explanation.

## License

This project is licensed under the MIT License.

# Solution Explanation

## Problem Requirements

We need to:

1. Remove all occurrences of a value in-place
2. Return the count of remaining elements
3. Place all non-matching elements in the first k positions
4. Work within these constraints:
   - 0 <= nums.length <= 100
   - 0 <= nums[i] <= 50
   - 0 <= val <= 100

## Algorithm

1. Handle empty array case by returning 0
2. Use a pointer `k` to track position for next valid element
3. Iterate through array:
   - If current element ≠ val, copy it to position k and increment k
   - If current element = val, skip it
4. Return k as the new length

## Example Walkthrough

For input: nums = [3,2,2,3], val = 3

```

Initial: [3,2,2,3], k = 0
i = 0: 3 = val, skip
i = 1: 2 ≠ val, nums[0] = 2, k = 1
i = 2: 2 ≠ val, nums[1] = 2, k = 2
i = 3: 3 = val, skip
Result: [2,2,2,3], return 2

```

## Complexity Analysis

- Time Complexity: O(n) - single pass through array
- Space Complexity: O(1) - in-place operation

## Implementation Details

The solution:

- Uses Rust's mutable references
- Includes test cases
- Handles edge cases (empty array)
- Preserves relative order of non-matching elements
