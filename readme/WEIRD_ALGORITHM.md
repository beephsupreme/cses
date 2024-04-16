# Weird Algorithm

[Weird Algorithm @ CSES Website](https://cses.fi/problemset/task/1068)  
[Source Code (lib)](https://github.com/beephsupreme/cses/blob/master/src/weird_algorithm.rs)  
[Source Code (submission)](https://github.com/beephsupreme/cses/blob/master/src/bin/submissions/weird_algorithm_sub.rs)

## Description

Consider an algorithm that takes as input a positive integer `n`. If `n` is even, the algorithm divides it by two, and
if `n` is odd, the algorithm multiplies
it by three and adds one.

The algorithm repeats until `n = 1`.

For example, the sequence
for `n = 3` is as follows:

`3 → 10 → 5 → 16 → 8 → 4 → 2 → 1`

Your task is to simulate the execution of the algorithm for a given value of `n`

## Input

The only input line contains an integer `n`.

## Output

Print a line that contains all values of `n` during the algorithm.

## Constraints

`1 ≤ n ≤ 1e6`

## Examples

Input  
`3`

Output  
`3 10 5 16 8 4 2 1`
