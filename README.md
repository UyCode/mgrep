<h1 align=center> minigrep for search contents in file </h1>

<div align=center>
<img height=50% src="assets/minigrep.png" width=50% />
</div>


a minigrep implementation in rust
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
- `-c` : print only a count of selected lines per FILE

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
