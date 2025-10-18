# algotrade
Algorithmic trading in Rust language

## build

```sh
./build.sh
```


### decimal numbers
The `decimal_numbers` module provides the `DecimalNumber` type, to be used in  alternative to floating point numbers. Any decimal number is represented using an integer `coefficient` value and an integer `scale` factor, so that the actual decimal number is <code> (coefficient &times; 10<sup>-scale</sup>)</code>. 

This implementation provides base-10 mathematical operations able to avoid issues related to rounding precisions of the IEEE 754 floating point representations, making the decimal numbers more suitable for algorithmic trading in finance. 
