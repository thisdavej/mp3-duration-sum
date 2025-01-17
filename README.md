# mp3-duration-sum ![](https://img.shields.io/crates/v/mp3-duration-sum.svg?style=flat)

Quickly calculate the total duration of all MP3 files in a directory, in either milliseconds or human-readable format. This crate uses parallel processing for faster results.

## Key Features

- Parallelizes the duration calculation using `rayon` for optimal performance.
- Supports displaying the total duration in both milliseconds (default) and human-readable format (e.g., "1h,23m").

## Installation

```bash
cargo install -f mp3-duration-sum
```

## Usage

```bash
mp3-duration-sum <directory_path> [--human-time]
```

- <directory_path>: The path to the directory containing the MP3 files.
- --human-time: (Optional) Display the total duration in a human-readable format (e.g., "3h,12m,5s" for 3 hours, 12 minutes, and 5 seconds).

## Examples

To calculate the total duration (in milliseconds) of all MP3 files in the directory `/path/to/your/music`, run:

```bash
mp3-duration-sum /path/to/your/music
```

This will output the total duration in milliseconds.

To display the total duration in a human-readable format, use the `--human-time` flag:

```bash
mp3-duration-sum /path/to/your/music --human-time
```

This will output the total duration in an easy to read format like "3h,12m,5s".

## License

This project is licensed under the MIT License.
