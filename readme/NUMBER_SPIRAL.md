# Number Spiral

## [Source Code](https://github.com/beephsupreme/cses-rust/blob/master/src/solutions/number_spiral.rs)

## Description

A number spiral is an infinite grid whose upper-left square has number 1. Here are the first five layers of the spiral:

|    |    |    |    |    |
|----|----|----|----|----|
| 1  | 2  | 9  | 10 | 25 |
| 4  | 3  | 8  | 11 | 24 |
| 5  | 6  | 7  | 12 | 23 |
| 16 | 15 | 14 | 13 | 22 |
| 17 | 18 | 19 | 20 | 21 |

Your task is to find out the number in row `y` and column `x`.

## Input

The first input line contains an integer `t`: the number of tests.
After this, there are `t` lines, each containing integers `y` and `x`.

## Output

For each test, print the number in row `y` and column `x`.

## Constraints

`1 ≤ t ≤ 1e5`

`1 ≤ y, x ≤ 1e9`

## Examples

Input

```text
3  
2 3  
1 1  
4 2  
```

Output

```test
8  
1  
15
```

---

Input
`3`

Output  
`NO SOLUTION`
