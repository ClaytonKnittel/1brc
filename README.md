# Clayton's 1brc

Inspired by https://github.com/gunnarmorling/1brc.

## Rules

- All programming languages allowed.
- No external libraries that help in nontrivial ways.
  - Utility libraries are allowed.
  - If unsure, ask.
- The rounding of output values must be done using the semantics of IEEE 754 rounding-direction "roundTowardPositive".

## How to structure your program

You should write a program that takes a single command line argument `--input <input_file>`, which is a path to the file you need to parse and print a summary of. For example:

```bash
./my_program --input ./measurements.txt
```

Your program should print a summary of the measurements to stdout. It should print the min/mean/max temperature value recorded per weather station in the input file, like this:

```
{Abha=-23.0/18.0/59.2, Abidjan=-16.2/26.0/67.3, Abéché=-10.0/29.4/69.0, Accra=-10.1/26.4/66.4, Addis Ababa=-23.7/16.0/67.0, Adelaide=-27.8/17.3/58.5, ...}
```

Note that the weather stations should be sorted alphabetically.

## Input file format

You'll be given a file of newline-separated rows, where each row contains the name of a weather station and a temperature reading at that weather station, separated by a semicolon. The last row of the file will be followed by a newline.

For example, the input file may look like this:

```
Abha;-23.0
Abidjan;29.4
...
```

You may assume the following about the format of the input file:

- The input file is well-formatted and adheres to the following restrictions.
- The whole input file is valid UTF-8.
- The name of weather stations is anywhere between 2 - 50 bytes long, and does not contain ';' or '\n' characters.
  - note: bytes, not characters, which may differ for non-ascii unicode.
- Temperature readings are between -99.9 and 99.9 (inclusive), always with one fractional digit.
- There is a maximum of 10,000 unique weather stations.

## Testing

I haven't set up any testing framework, but I've provided an example program in examples/ that you can verify the output of your program against. As long as the outputs match exactly, your code is correct.

You can run the example with:

```bash
cargo r --release --example simple_parser [-- --input <input_file>]
```

## Installing FlameGraph

Run the following:

```bash
git clone --depth=1 https://github.com/brendangregg/FlameGraph.git
echo "export PATH=\"\$PATH:$(pwd)/FlameGraph\"" >> ~/.bashrc
```
