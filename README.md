# rusty_python
I've always liked the idea of using python as a front end and a more performant language as a backend. 
In this case I used Rust. For benchmarking I wrote non regex (brute force) solutions to a duplicate character
problem. I used the cpython rust crate. Didn't add that much overhead so I can see it being useful in proper python 
projects where performance is a concern.

## Benchmarks

| Language     | Fastest (ms) | Slowest (ms) |Mean (ms) | 
| ----------- | ----------- | ------------| -------------|
| Python      | 35.7       |    37.6         |   1.96     |
| Rust backend   | 1.89       |     2.11        |  36.3  |