# Purpose
Demonstrate edge cases in binary search.

# Current output
```
-----
bad_1: try to overflow calculation of mid
0 1073741824 2147483648
1073741825 1610612736 2147483648
1610612737 1879048192 2147483648
1879048193 2013265920 2147483648
2013265921 2080374784 2147483648
2080374785 2113929216 2147483648
2113929217 2130706432 2147483648
2130706433 2139095040 2147483648
2139095041 2143289344 2147483648
2143289345 2145386496 2147483648
2145386497 2146435072 2147483648
2146435073 2146959360 2147483648
2146959361 2147221504 2147483648
2147221505 2147352576 2147483648
2147352577 2147418112 2147483648
2147418113 2147450880 2147483648
2147450881 2147467264 2147483648
2147467265 2147475456 2147483648
2147475457 2147479552 2147483648
2147479553 2147481600 2147483648
2147481601 2147482624 2147483648
2147482625 2147483136 2147483648
2147483137 2147483392 2147483648
2147483393 2147483520 2147483648
2147483521 2147483584 2147483648
2147483585 2147483616 2147483648
2147483617 2147483632 2147483648
2147483633 2147483640 2147483648
2147483641 2147483644 2147483648
2147483645 2147483646 2147483648
2147483647 2147483647 2147483648
2147483648 2147483648 2147483648
-2147483648
-----
bad_2: overflow calculation of hi
0 0 0
thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:15:18
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
-----
bad_3: empty vector to search
thread 'main' panicked at 'attempt to subtract with overflow', src/main.rs:7:18
```

# Test cases
## bad_1
This test case attempts to cause a failure at the calculation of `mid`:
```
    while lo <= hi {
        let mid = (lo + hi) / 2;
```

The problem is that the intermediate result `lo + hi` can overflow a `usize` if the value searched for is greater than the maximum value in the data. In that case, the calculation of `mid` converges to `hi` so that the intermediate result is twice the length of the data. In the case that the data exceeds half the value represented by a `usize`, there will be an overflow and the code will panic.

Here is a simple demonstration of the panic (run in `evcxr`) for a 64-bit `usize`, i.e. a 64-bit Rust build:
```
>> let a: usize = (1usize << 63);
>> a + a
thread '<unnamed>' panicked at 'attempt to add with overflow', src/lib.rs:119:40
stack backtrace:
   0: _rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::panic
   3: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
   4: _run_user_code_6
   5: evcxr::runtime::Runtime::run_loop
   6: evcxr::runtime::runtime_hook
   7: evcxr::main
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

The traditional approach to calculating `mid` has been this:
```
let mid = lo + (hi - lo) / 2;
```
This calculation does not overflow.

## bad_2
This case tests what happens when `mid` is 0 and the value is below that -- that is, the value being searched for is less than the least value in the data.
```
        else if data[mid] > value {
            hi = mid - 1;
        }
```

Since `mid` is an unsigned, subtracting 1 from the 0 value causes an overflow and the code panics.

The best approaches to solving this are:
- Testing for `mid == 0` and returning early if necessary:
```
        else if data[mid] > value {
            if mid == 0 { break; }
            hi = mid - 1;
        }
```
- Testing for the value being out of bounds
Before the main loop even starts, the code could do bounds checks:
```
    if value < data[0] {
        return NotFound;
    }
```

A similar test could be done for the other bound:
```
    if value > items.last().unwrap() {
        return NotFound;
    }
```

The output for this test shows a different problem: the internal use of `usize` will produce correct results, but if it is downcast to `i32` on a 64-bit build, the results will be wrong for cases where the value is found in a vector whose size exceeds the range of an `i32`.

