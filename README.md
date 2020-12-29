# fast-modulo
This crate needs nightly. (for `feature(asm)`)

## benchmark
OS: Ubuntu 18.04, CPU: AMD Ryzen 7 2700X, rustc 1.50.0-nightly (bb178237c 2020-12-25)
```
reference::mulmod_u64   time:   [23.264 ns 23.277 ns 23.292 ns]                                   
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild

mulmod_u64              time:   [10.253 ns 10.256 ns 10.259 ns]                        
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe

reference::mod_u128u64  time:   [37.003 ns 37.012 ns 37.021 ns]                                    
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

mod_u128u64             time:   [13.852 ns 13.856 ns 13.861 ns]                         

reference::powmod_u64   time:   [2.2661 us 2.2671 us 2.2680 us]                                  
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

powmod_u64              time:   [916.55 ns 916.74 ns 916.93 ns]                       
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
```
