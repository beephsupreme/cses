# Increasing Array

[Increasing Array @ CSES Website](https://cses.fi/problemset/task/1094)  
[Source Code (lib)](https://github.com/beephsupreme/cses/blob/master/src/increasing_array.rs)  
[Source Code (submission)](https://github.com/beephsupreme/cses/blob/master/src/bin/submissions/increasing_array_sub.rs)

## Description

You are given an array of `n` integers. You want to modify the array so that it is increasing, i.e., every element is at
least as large as the previous element.
On each move, you may increase the value of any element by one. What is the minimum number of moves required?

## Input

The first input line contains an integer `n`: the size of the array.

Then, the second line contains `n` integers `x_1 ,x_2 , ... , x_n`: the contents of the array.

## Output

Print the minimum number of moves.

## Constraints

`1 ≤ n ≤ 2e5`

`1 ≤ x_i ≤ 1e9`

## Examples

Input

```
5  
3 2 5 1 7
```

Output  
`5`
