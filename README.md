# fast-modulo
This crate needs nightly. (for `feature(asm)`)

## benchmark
OS: Ubuntu 18.04, CPU: AMD Ryzen 7 2700X, rustc 1.50.0-nightly (bb178237c 2020-12-25)
```
reference::mulmod_u64   time:   [23.242 ns 23.255 ns 23.267 ns]                                   
                        change: [+0.6678% +0.7357% +0.8097%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

mulmod_u64              time:   [10.262 ns 10.270 ns 10.279 ns]                        
                        change: [-0.0325% +0.0948% +0.2124%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  1 (1.00%) high severe

reference::mod_u128u64  time:   [36.692 ns 36.708 ns 36.724 ns]                                    
                        change: [-0.9907% -0.9083% -0.8310%] (p = 0.00 < 0.05)
                        Change within noise threshold.

mod_u128u64             time:   [13.935 ns 13.943 ns 13.951 ns]                         
                        change: [-1.1197% -1.0431% -0.9694%] (p = 0.00 < 0.05)
                        Change within noise threshold.
```
