# kprober

kprober is a simple command line tool for kprobing a running kernel. It lists
the (filtered) content of `/proc/kallsyms` in a curses UI, allows selecting
multiple symbols, and uses [bpftrace](https://github.com/iovisor/bpftrace) to
attach simple kprobes and kretprobes to the selected symbols.

## Requirements

kprober requires:
* Linux with `/proc/kallsyms` and eBPF support
* [bpftrace](https://github.com/iovisor/bpftrace)

## Usage

You can run kprober with the `kprober` command:

```
kprober

USAGE:
    kprober [OPTIONS]

OPTIONS:
    -f, --filter <filter>     Filter symbols
    -h, --help                Print help information
    -s <symbol-source>        Symbol source [possible values: kallsyms, bpftrace]
```

Basic navigation in the UI: You can navigate the symbols list with the up and
down keys, select and deselect the current symbol with the enter key, and
switch to other control elements with the tab key.

## Examples

Running kprober without command line arguments:

```console
$ sudo kprober
```

Running kprober with a filter that only shows symbols that contain "do_":

```console
$ sudo kprober --filter do_
```
