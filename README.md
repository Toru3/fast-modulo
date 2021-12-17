This crate is DEPRECATED.

# fast-modulo
This crate needs nightly-2021-12-15 or later. (for `asm!`)

## benchmark
OS: Ubuntu 18.04, CPU: AMD Ryzen 7 2700X, rustc rustc 1.59.0-nightly (5531927e8 2021-12-16)
```
reference::mulmod_u64   time:   [10.573 ns 10.583 ns 10.593 ns]                                   
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

mulmod_u64              time:   [10.389 ns 10.400 ns 10.413 ns]                        
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

reference::mod_u128u64  time:   [13.928 ns 13.940 ns 13.952 ns]                                    
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

mod_u128u64             time:   [14.178 ns 14.186 ns 14.194 ns]                         
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

reference::powmod_u64   time:   [955.08 ns 955.65 ns 956.21 ns]                                  
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

powmod_u64              time:   [934.02 ns 934.88 ns 935.82 ns]                       
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  1 (1.00%) high severe
```
# Licence
AGPL-3.0-or-later
