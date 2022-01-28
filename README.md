# rfid-reader

A rust package for decoding and encoding of the Wiegand format used in 125KHz
RFID fobs.


```rust
use rfid_debug::WiegandFormat;

let standard_format = WiegandFormat {
    parity_even: 0,
    parity_odd: 25,
    card_number: (0, 16), // bit range [lower, upper)
    facility_code: (16, 24), // bit range [lower, upper)
};

let (facility, card_number) = standard_format.decode(5666862).unwrap();

println!("facility = {}, ID = {}", facility, card_number);
```
