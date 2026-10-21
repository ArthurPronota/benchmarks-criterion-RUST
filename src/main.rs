fn main() {
    println!("Hello, world!");
} /* Out:
C:\Users\user\Job\Rust\CookBookProjects\o_089>cargo bench
   Compiling o_089 v0.1.0 (C:\Users\user\Job\Rust\CookBookProjects\o_089)
    Finished `bench` profile [optimized] target(s) in 1.36s
     Running unittests src\main.rs (target\release\deps\o_089-d57a3931b00af4ce.exe)

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches\sum_bench.rs (target\release\deps\sum_bench-16be6e11b6ea8df8.exe)
sum_to_1000             time:   [751.94 ps 763.77 ps 783.66 ps]
                        change: [+0.5421% +2.2656% +4.3934%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
*/
