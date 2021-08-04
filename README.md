# fast-modulo
This crate needs nightly. (for `feature(asm)`)

## benchmark
OS: Ubuntu 18.04, CPU: AMD Ryzen 7 2700X, rustc 1.51.0-nightly (257becbfe 2020-12-27)
```
reference::mulmod_u64   time:   [23.077 ns 23.102 ns 23.134 ns]                                   
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

mulmod_u64              time:   [10.223 ns 10.230 ns 10.236 ns]                        

reference::mod_u128u64  time:   [36.684 ns 36.705 ns 36.727 ns]                                    
Found 18 outliers among 100 measurements (18.00%)
  16 (16.00%) high mild
  2 (2.00%) high severe

mod_u128u64             time:   [13.862 ns 13.869 ns 13.878 ns]                         

reference::powmod_u64   time:   [2.2811 us 2.2840 us 2.2872 us]                                  
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

powmod_u64              time:   [916.39 ns 917.03 ns 917.77 ns]                       
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
```

# Licence
AGPL-3.0-or-later
