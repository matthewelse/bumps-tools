# The Bumps Tools

A set of tools for extracting data from the (now dated) bumps CD-ROM. To use
these tools, you need to start with a copy of the CD-ROM.

## Extracting data files from the install CD

1. Install [unshield](https://github.com/twogood/unshield). On MacOS, `brew
   install unshield` should work.
2. Extract the data files from the installer CD.

```bash
$ cd $CD_PATH
$ mkdir -p data
$ unshield x -d data INSTALL/data1.cab
$ ls data
Data_Files  Help_Files  Program_Executable_Files  Text_Files
```

The `Data_Files` directory contains a number of .dat files, which are encrypted
using a simple algorithm, described in `src/decode.rs`.

## Decoding data files

Use the `decode` tool.

```bash
$ cd $THIS_REPO_PATH
$ cargo run -q -- decode --file $CD_PATH/data/Data_Files/Data/Colleges.dat
!
! NAME				FILE
!

[1st Trinity]
29	1	158
1st Trinity Black Prince	blackp
1st Trinity			1trin
1st Trinity 2			1trin2
1st Trinity 3			1trin3
...
```

