# Purpose
Develop more software primitives for doing public key cryptography:
- fast_exp
- fast_exp_mod

# Testing
Run tests on this code with this command:
```
> cargo test

running 16 tests
test tests::fast_exp_2_0 ... ok
test tests::fast_exp_2_2 ... ok
test tests::fast_exp_2_5 ... ok
test tests::fast_exp_2_3 ... ok
test tests::fast_exp_2_6 ... ok
test tests::fast_exp_2_1 ... ok
test tests::fast_exp_2_4 ... ok
test tests::fast_exp_2_7 ... ok
test tests::fast_exp_general2 ... ok
test tests::fast_exp_general3 ... ok
test tests::fast_exp_mod_8_1_10 ... ok
test tests::fast_exp_mod_8_2_10 ... ok
test tests::fast_exp_mod_8_5_10 ... ok
test tests::fast_exp_mod_8_3_10 ... ok
test tests::fast_exp_mod_8_4_10 ... ok
test tests::fast_exp_mod_8_6_10 ... ok

...

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
