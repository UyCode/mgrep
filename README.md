<h1 align=center> minigrep for search contents in file </h1>

<div align=center>
<img height=50% src="assets/minigrep.png" width=50% />
</div>

A simple command line tool for searching for a string in a file

> inspired by [rust book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
> and [rust course](https://course.rs/basic-practice/intro.html)

## Usage

```bash
$ cargo run -- [OPTIONS] <search> <file>
```

```bash
$ mgrep [OPTIONS] <search> <file>
```

## Options

- `-i` : ignore case distinctions
- `-c` : print word count in file (exclude dots and commas)
- `-h` : print help message

## Examples

```bash
$ mgrep "hello world" README.md
```

```bash
$ mgrep -i "hello world" README.md
```

```bash
$ mgrep -c  README.md
```
