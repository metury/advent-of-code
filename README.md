# [Advent of code](https://adventofcode.com/)

Some of my solutions to advent of code.

This repo is separated firstly by years, then by days. Always inputs are read from the file `INPUT` in the respected folder for that day. Also there may be `info.md` file for comenting what is done in the file and it will be parsed to the page script.

### Scripts

- `page.sh` Is for generating pages for jekyll. Also takes one optional argument where to copy the generated files.
- `template.sh` Is for creating the same rust template for other days. At least name of the problem must be set as the first argument. If there are three arguments the second is for year and the third for day. Otherwise take sys date.