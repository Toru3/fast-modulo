# fast-modulo
This crate needs nightly. (for `feature(asm)`)

## benchmark
OS: Ubuntu 18.04, CPU: AMD Ryzen 7 2700X, rustc: 1.47.0-nightly (6c8927b0c 2020-07-26)
```
reference::mulmod_u64   time:   [174.98 ns 175.08 ns 175.18 ns]                                   
                        change: [-0.4957% -0.3893% -0.2838%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 10 outliers among 100 measurements (10.00%)
  1 (1.00%) low severe
  5 (5.00%) low mild
  4 (4.00%) high mild

mulmod_u64              time:   [10.355 ns 10.359 ns 10.363 ns]                        
                        change: [+0.3039% +0.4009% +0.4971%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

reference::mod_u128u64  time:   [183.92 ns 184.04 ns 184.17 ns]                                    
                        change: [+0.2122% +0.3338% +0.4534%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

mod_u128u64             time:   [14.235 ns 14.242 ns 14.250 ns]                         
                        change: [+0.2390% +0.3663% +0.4827%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) low mild
  2 (2.00%) high
```
