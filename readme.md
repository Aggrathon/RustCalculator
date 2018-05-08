# Rust Calculator

A commandline calculator written in rust.  
Usage:
```
	calc 2 + 5 / 4! - cos pi 
```

## Features
- Can handle complex expressions
- Operators: +, -, *, /, (, ), ^, ! ...
- Functions: sin, cos, tan, abs, log ...
- Constants: pi, rnd, e ...

## Help
**I cannot use * or ()**  
Put the query in "" or '' to avoid shell interference.  
E.g. `calc "(2+3)*5"` instead of `calc (2+3)*5`

## Binaries
Either build from source (`cargo build --release`) or download the latest [release](https://github.com/Aggrathon/RustCalculator/releases).