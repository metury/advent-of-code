# Documentaion

This is a documentation for `aoc.pl` script.

## Running the script

There are several use case for this script. If you run `./aoc.pl` or `perl aoc.pl` it will be shut down because no arguments were given. There are four main types.

### Help

When you run `perl aoc.pl -h` it will prompt a short version of this documentation. Alternatively you may use `--help`. Just in any other program.

### Templates

If you choose running `perl aoc.pl -t` or `perl aoc.pl --template` you will create a templated project for some day. You will have to choose language, day and year. If none of these were given than the default is `rust` as a language and today as the date.

Then the signature is `perl aoc.pl -t [lang] [year day]` where lang can be left out. And also the tuple year and day.

Altogether this creates a simple not so empty project with the current date in their respected directories. And also it download the name of the problem from the page and puts it inside the project as well.

> [!WARNING]
> If you run template for some day that already exists it won't delete any files and exit instead. So if you want to override project you nedd to remove the respected day subfolder.

Also the main strucutre for the files is:

```txt
- [year dir]
  - [day dir]
    - INPUT
    - OUTPUT
    - info.md
    - [src files]
    - [src dir]
      - [other source files]
```

> [!IMPORTANT]
> Note that only some languages are implemented. There is **Perl**, **Rust**, **Python**, **Go** and **CPP**.

### Page generating

Also you may call `perl aoc.pl -p` or `perl aoc.pl --pages` to generate markdown pages to use with mdbook. There is an optional argument for the destination of the created files. So usualy you can call `perl aoc.pl -p /pathto/page-repo`. If the argument is not given the path `.` is used instead. Note that `SUMMARY.md` is being appended.

This page generator respects the structure of the projects which was shown earlier. Also in the script is defined forbidden files which will not be considered in the page generating.

### Gitignore generator

Also there is possibility to call `perl aoc.pl -g` or `perl aoc.pl --gitignore` to generate simple `.gitignore` file. This is to respect the copyrights on the input and not to sync useless files.

> [!CAUTION]
> Use this script on your own and please do not blame me, since this is just mine hobby project. Also I was using perl for the first time. If you see something that can be upgraded, then you may do so by either creating issue or pull request.
