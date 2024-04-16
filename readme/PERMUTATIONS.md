# Permutations

#### [Source Code](https://github.com/beephsupreme/cses-rust/blob/master/src/solutions/permutations.rs)

## Description

A permutation of integers `1 , 2 , ... , n` is called beautiful if there are no adjacent elements whose difference is 1.
Given `n`, construct a beautiful permutation if such a permutation exists.

## Input

The only input line contains an integer `n`.

## Output

Print a beautiful permutation of integers `1 , 2 , ... , n`. If there are several solutions, you may print any of them.
If there are no solutions, print `"NO SOLUTION"`.

## Constraints

`1 ≤ n ≤ 1e6`

## Examples

Input  
`5`

Output  
`4 2 5 3 1`

---

Input  
`3`

Output  
`NO SOLUTION`
