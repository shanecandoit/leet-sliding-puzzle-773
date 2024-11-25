
/* 1380. Lucky Numbers in a Matrix

Given an m x n matrix of distinct numbers, return all lucky numbers in the matrix in any order.

A lucky number is an element of the matrix such that it is the minimum element in its row and maximum in its column.

Example 1:
    Input: matrix = [[3,7,8],[9,11,13],[15,16,17]]
    Output: [15]
    Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.

Example 2:
    Input: matrix = [[1,10,4,2],[9,3,8,7],[15,16,17,12]]
    Output: [12]
    Explanation: 12 is the only lucky number since it is the minimum in its row and the maximum in its column.

Example 3:
    Input: matrix = [[7,8],[1,2]]
    Output: [7]
    Explanation: 7 is the only lucky number since it is the minimum in its row and the maximum in its column.
 */

pub struct Solution;

impl Solution {
    pub fn main() {
        println!("1380. Lucky Numbers in a Matrix");

        // ex1
        let matrix = vec![vec![3,7,8], vec![9,11,13], vec![15,16,17]];
        let result = Solution::lucky_numbers(matrix);
        println!("ex1 Result: {:?}", result);
        assert_eq!(result, vec![15]);

        // ex2
        let matrix = vec![vec![1,10,4,2], vec![9,3,8,7], vec![15,16,17,12]];
        let result = Solution::lucky_numbers(matrix);
        println!("ex2 Result: {:?}", result);
        assert_eq!(result, vec![12]);

        // ex3
        let matrix = vec![vec![7,8], vec![1,2]];
        let result = Solution::lucky_numbers(matrix);
        println!("ex3 Result: {:?}", result);
        assert_eq!(result, vec![7]);

    }

    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut row_min = vec![i32::MAX; matrix.len()];
        let mut col_max = vec![i32::MIN; matrix[0].len()];

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                row_min[i] = row_min[i].min(matrix[i][j]);
                col_max[j] = col_max[j].max(matrix[i][j]);
            }
        }

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if matrix[i][j] == row_min[i] && matrix[i][j] == col_max[j] {
                    result.push(matrix[i][j]);
                }
            }
        }

        result
    }
}

// Runtime 0 ms
// Beats 100.00%
// Memory 2.19 MB
// Beats 83.33%
// https://leetcode.com/problems/lucky-numbers-in-a-matrix/submissions/1462569339/?envType=daily-question&envId=2024-11-25
