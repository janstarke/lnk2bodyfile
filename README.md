# lnk2bodyfile
Parse Windows LNK files and create [bodyfile](https://wiki.sleuthkit.org/index.php?title=Body_file) output, ready to be timelined with [mactime2](https://github.com/janstarke/mactime2) (or mactime, if you really want to)

# Usage

```
Parse Windows LNK files and create bodyfile output

Usage: lnk2bodyfile [OPTIONS] [LNK_FILES]...

Arguments:
  [LNK_FILES]...  Name of the LNK files to read from

Options:
  -v, --verbose...  More output per occurrence
  -q, --quiet...    Less output per occurrence
  -h, --help        Print help
  -V, --version     Print version

```