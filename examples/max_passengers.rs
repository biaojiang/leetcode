/// A taxi can take multiple passengers to the railway station at the same time. On the way back to the
/// starting point, the taxi driver may pick up additional passengers for his next trip to the airport.
///
/// The map of passenger location is represented as a square matrix, where each cell has an initial value as follows:
///
/// - A value greater than or equal to zero represents a path.
/// - A value equal to 1 represents a passenger.
/// - A value equal to -1 represents an obstruction.
///
/// The rules of motion of the taxi are as follows:
///
/// - The taxi driver starts at (0,0), and the railway station is at (n-1,m-1), where n is the number of rows
///   and m is the number of columns in the matrix.
/// - Movement towards the railway station is right or down, through valid path cells.
/// - After reaching (n-1,m-1), the taxi driver travels back to (0,0) by traveling left or up through valid
///   path cells.
/// - When passing through a path cell containing a passenger, the passenger is picked up. Once the rider is
///   picked up, the cell becomes an empty path cell.
/// - If there is no valid path between (0,0) and (n-1,m-1), then no passenger can be picked.
///
/// The goal is to collect as many passengers as possible so that the driver can maximize his earnings.
/// The Hackerrank Coding Question-1 is [Maximum Passengers].
///
/// [Maximum Passengers]: <https://prepinsta.com/hackerrank/coding-questions-and-answers/#question1-***>
///
// # Example
///
/// ```
/// let matrix = vec![
///     vec![0, 1, -1],
///     vec![1, 0, -1],
///     vec![1, 1, 1],
/// ];
///
/// assert_eq!(max_passengers(&matrix), 5);
/// ```
pub struct Solution;

impl Solution {
    /// The algorithm follows a Depth-First Search (DFS) approach with backtracking, exploring
    /// all possible paths from the starting point to the destination. It maintains separate counts
    /// of passengers picked up in the forward and backward directions to and from the destination.
    ///
    /// Key Points:
    /// 1. Depth-First Search (DFS) Approach:
    ///    - The algorithm explores the grid recursively, starting from the initial cell (0, 0)
    ///      and traversing all possible paths to reach the destination cell (n-1, m-1).
    ///
    /// 2. Backtracking:
    ///    - At each step of the DFS traversal, the algorithm explores all possible directions
    ///      (right, down, left, up) from the current cell. If the next cell is valid (not an obstruction)
    ///      and contains a passenger, the algorithm picks up the passenger and recursively explores
    ///      the next cell. If the next cell doesn't contain a passenger or is an obstruction,
    ///      the algorithm backtracks to explore other directions.
    ///
    /// 3. Counting Passengers:
    ///    - When the algorithm encounters a cell with a passenger, it increments the count of passengers
    ///      picked up in the current path. Separate counts are maintained for the forward and backward
    ///      directions to and from the destination cell.
    ///
    /// 4. Unified Method:
    ///    - The algorithm explores both forward and backward paths simultaneously by maintaining
    ///      separate counts of passengers picked up in the forward and backward directions. This ensures
    ///      that each cell's passengers are accounted for in both directions.
    ///
    /// 5. Maximum Passengers:
    ///    - After completing the DFS traversal, the algorithm calculates the maximum total number
    ///      of passengers by considering the sum of passengers picked up in both forward and backward
    ///      directions for each cell.
    ///
    /// 6. Optimization:
    ///    - To avoid double-counting passengers in overlapping paths, the algorithm considers all possible
    ///      pairs of cells with passengers picked up in both forward and backward directions and selects
    ///      the maximum sum.
    pub fn explore_path(
        matrix: &Vec<Vec<i32>>,
        x: usize,
        y: usize,
        passengers_forward: &mut Vec<Vec<i32>>,
        passengers_backward: &mut Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        let n = matrix.len();
        let m = matrix[0].len();

        // Mark the current cell as visited
        visited[x][y] = true;

        // Explore rightwards and downwards
        let directions = &[(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions {
            let next_x = x as isize + *dx;
            let next_y = y as isize + *dy;

            if next_x >= 0 && next_y >= 0 && next_x < n as isize && next_y < m as isize {
                let next_x = next_x as usize;
                let next_y = next_y as usize;
                if matrix[next_x][next_y] != -1 && !visited[next_x][next_y] {
                    let mut next_passengers_forward = passengers_forward[x][y];
                    let mut next_passengers_backward = passengers_backward[x][y];
                    if matrix[next_x][next_y] == 1 {
                        if *dx == 1 || *dy == 1 {
                            next_passengers_forward += 1; // Pick up passengers in forward direction
                        } else {
                            next_passengers_backward += 1; // Pick up passengers in backward direction
                        }
                    }
                    passengers_forward[next_x][next_y] = next_passengers_forward;
                    passengers_backward[next_x][next_y] = next_passengers_backward;
                    Self::explore_path(
                        matrix,
                        next_x,
                        next_y,
                        passengers_forward,
                        passengers_backward,
                        visited,
                    );
                }
            }
        }
    }

    /// # Arguments
    ///
    /// * `matrix` - A square matrix representing the map of passenger locations.
    ///
    /// # Returns
    ///
    /// The maximum number of passengers that the taxi driver can collect.
    pub fn max_passengers(matrix: &Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut visited = vec![vec![false; m]; n];
        let mut passengers_forward = vec![vec![0; m]; n];
        let mut passengers_backward = vec![vec![0; m]; n];

        // Explore path from (0, 0) to (n-1, m-1)
        Self::explore_path(
            matrix,
            0,
            0,
            &mut passengers_forward,
            &mut passengers_backward,
            &mut visited,
        );

        // Find the maximum total number of passengers
        let mut max_passengers = 0;
        for i in 0..n {
            for j in 0..m {
                if visited[i][j] && matrix[i][j] == 1 {
                    let mut max_sum = 0;
                    for x in 0..n {
                        for y in 0..m {
                            if visited[x][y] && matrix[x][y] == 1 {
                                let sum = passengers_forward[i][j] + passengers_backward[x][y];
                                max_sum = max_sum.max(sum);
                            }
                        }
                    }
                    max_passengers = max_passengers.max(max_sum);
                }
            }
        }

        max_passengers
    }
}

fn main() {
    // let matrix = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
    let matrix = vec![
        vec![0, 0, 0, 1],
        vec![1, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];

    let max_passengers = Solution::max_passengers(&matrix);
    println!("Maximum number of passengers: {}", max_passengers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_passengers() {
        // let matrix = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        let matrix = vec![
            vec![0, 0, 0, 1],
            vec![1, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(Solution::max_passengers(&matrix), 2);
    }

    #[test]
    fn test_max_passengers_5() {
        let matrix = vec![vec![0, 1, -1], vec![1, 0, -1], vec![1, 1, 1]];
        assert_eq!(Solution::max_passengers(&matrix), 5);
    }

    #[test]
    fn test_max_passengers_no_passenger() {
        let matrix = vec![vec![0, 0, -1], vec![0, 0, -1], vec![0, 0, 0]];
        assert_eq!(Solution::max_passengers(&matrix), 0);
    }

    #[test]
    fn test_max_passengers_no_valid_path() {
        let matrix = vec![vec![0, -1, 0], vec![-1, 0, -1], vec![0, -1, 0]];
        assert_eq!(Solution::max_passengers(&matrix), 0);
    }
}
