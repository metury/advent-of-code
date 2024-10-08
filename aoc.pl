#!/usr/bin/perl
use strict;
use warnings;
use Path::Tiny;
use LWP::Simple;

## ======================= ##
## == Global constants. == ##
## ======================= ##

my $limit_year_low = 2015;
my $limit_year_high = 2023;
my $limit_day_low = 1;
my $limit_day_high = 25;

## ================== ##
## == Create pages == ##
## ================== ##

my $dir = path('.');
my $aoc = "Advent of Code";
my $link = "./aoc/";
my @forb = ("INPUT", "OUTPUT", "Cargo.toml", "Cargo.lock", "info.md", "main", "graph.dot", "graph.png", "graph.svg", "Makefile");
my $root = "adventofcode.md";

# Print main file of the aoc.
sub print_aoc {
	my ($name) = @_;
	open(FH, '>', $name) or die $!;
	print FH "# Advent of code\n\n";
	print FH "They are my solutions to [advent of code](https://adventofcode.com/) tasks. There are separated to each year and day. All of this can be found on [GitHub](https://github.com/metury/advent-of-code), also with the script that generates these pages.\n";
	print FH "Plus you may also play a small [Bingo](https://aoc-bingo.fly.dev/) that someone made. Also you may consider joining [Reddit](https://www.reddit.com/r/adventofcode/) where you may find useful tips, or help someone.\n\n";
	print FH "### Years\n\n";
	close(FH);
}

# Append year to the aoc file.
sub append_year {
	my ($name, $year) = @_;
	open(FH, '>>', $name) or die $!;
	print FH "- [Year $year]($link$year.md)\n";
	close(FH);
}

# Print year file.
sub print_year {
	my ($name, $year, $mdbook) = @_;
	open(MD, '>>', $mdbook) or die $!;
	print MD "\t- [Year $year]($link$year.md)\n";
	close(MD);
	open(FH, '>', $name) or die $!;
	print FH "# Advent of code - Year $year\n\n";
	print FH "This contains tasks from the [year $year](https://adventofcode.com/$year). Go back to [AOC](../$root).\n\n";
	print FH "### Days\n\n";
	close(FH);
}

# Append day to the year page.
sub append_day {
	my ($name, $year, $day) = @_;
	open(FH, '>>', $name) or die $!;
	print FH "- [Day $day]($year-$day.md)\n";
	close(FH);
}

# Print day file.
sub print_day {
	my ($name, $year, $day, $mdbook) = @_;
	open(MD, '>>', $mdbook) or die $!;
	print MD "\t\t- [Year $year day $day]($link$year-$day.md)\n";
	close(MD);
	open(FH, '>', $name) or die $!;
	print FH "# Advent of code - Year $year Day $day\n\n";
	print FH "This is a solution of the [day $day](https://adventofcode.com/$year/day/$day). Go back to year [$year]($year.md). Go back to [AOC](../$root).\n\n";
	close(FH);
}

# Print single file to the day.
sub print_file {
	my ($file, $output) = @_;
	my $file_name = $file;
	$file_name =~ s/^.*\///g;
	open(FH, '>>', $output) or die $!;
	print FH "## $file_name\n\n";
	my @lang = split /\./, $file_name;
	print FH "\`\`\`$lang[1]\n";
	open(IN, '<', $file) or die $!;
	while (<IN>) {
		$_ =~ s/\t/  /g;
		print FH $_;
	}
	print FH "\`\`\`\n\n";
	close(FH);
}

# Print info to the file.
sub print_info {
	my ($info_file, $output) = @_;
	open(FH, '>>', $output) or die $!;
	print FH "### Info\n\n";
	open(IN, '<', $info_file) or die $!;
	while (<IN>) {
		print FH $_;
	}
	print FH "\n";
	close(FH);
}

# Process signle file to the day file.
sub process_file {
	my ($aoc_dir, $year, $day, $file) = @_;
	my $file_name = $file;
	$file_name =~ s/^.*\///g;
	if (not grep {$_ eq $file_name} @forb) {
		print_file($file, "$aoc_dir/$year-$day.md");
	}
}

# Process whole day directory.
sub process_day {
	my ($aoc_dir, $year, $day_dir, $mdbook) = @_;
	if ($day_dir->is_dir() and $day_dir =~ /.*\/[0-9][1-9]/ ){
	my $day = (split /\//, $day_dir)[1];
		$day =~ s/^0//g;
		print_day("$aoc_dir/$year-$day.md", $year, $day, $mdbook);
		append_day("$aoc_dir/$year.md", $year, $day);
		if (-e "$day_dir/info.md" ) {
			print_info("$day_dir/info.md", "$aoc_dir/$year-$day.md");
		}
		my $day_iter = $day_dir->iterator;
		while (my $file = $day_iter->()) {
			if ($file->is_dir() and $file =~ /^.*\/src/) {
				my $src_iter = $file->iterator;
				while (my $src_file = $src_iter->()) {
					process_file($aoc_dir, $year, $day, $src_file);
				}
			}
			elsif (not $file->is_dir()) {
				process_file($aoc_dir, $year, $day, $file);
			}
		}
	}
}

# Process whole year directory.
sub process_year {
	my ($aoc_dir, $year_dir, $aoc_file, $mdbook) = @_;
	if ($year_dir->is_dir() and $year_dir =~ /20[0-9][0-9]/) {
		my @year_path = (split /\//, $year_dir);
		my $year = $year_path[@year_path - 1];
		print_year("$aoc_dir/$year.md", $year, $mdbook);
		append_year("$aoc_file", $year);
		opendir(DIR, $year_dir) or die $!;
		my @directories = sort readdir(DIR);
		closedir(DIR);
		foreach my $day_dir (@directories) {
			$day_dir = path("$year_dir/$day_dir");
			process_day($aoc_dir, $year, $day_dir, $mdbook);
		}
	}
}

# Create all pages.
sub create_pages {
	my ($path) = @_;
	my $aoc_dir = "$path/aoc";
	my $aoc_file = "$path/$root";
	my $mdbook = "$path/mdbook.md";
	open(MD, '>', $mdbook) or die $!;
	print MD "# Advent of code\n\n";
	print MD "- [Advent of code](./$root)\n";
	close(MD);
	mkdir "$aoc_dir";
	print_aoc($aoc_file);
	opendir(DIR, $dir) or die $!;
	my @directories = sort readdir(DIR);
	closedir(DIR);
	foreach my $year_dir (@directories) {
		process_year($aoc_dir, path("$dir/$year_dir"), $aoc_file, $mdbook);
	}
}

## =============== ##
## == Templates == ##
## =============== ##

sub get_name {
	my ($day, $year) = @_;
	my $url = "https://adventofcode.com/$year/day/$day";
	my $html = get($url);
	if (defined $html) {
		if ($html =~ /--- Day [0-9]+: ([^-]*) ---/){
			return $1;
		}
	} else {
		die "Failed to retrieve HTML from $url";
	}
}

sub general_template {
	my ($day, $year) = @_;
	my $written_day = $day;
	if ($day =~ /^[1-9]$/) {
		$written_day = "0$day";
	}
	mkdir $year;
	my $path = "$year/$written_day";
	mkdir $path;
	if (not -e "$path/INPUT") {
		open(FH, '>', "$path/INPUT");
		print FH "";
		close(FH);
	}
	if (not -e "$path/info.md"){
		open(FH, '>', "$path/info.md");
		print FH "#### Part 1\n\n#### Part 2\n\n";
		close(FH);
	}
}

sub rust_template {
	my ($day, $year, $name) = @_;
	my $written_day = $day;
	if ($day =~ /^[1-9]$/) {
		$written_day = "0$day";
	}
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	system("cargo", "new", "$year/aoc-$year-$day");
	rename("$year/aoc-$year-$day", "$year/$written_day");
	open(FH, '>', "$year/$written_day/src/main.rs") or die $!;
	print FH "use std::fs;\n\n";
	print FH "fn read_file(filepath: &str) -> Vec<String> {\n";
	print FH "\tlet contents = fs::read_to_string(filepath);\n";
	print FH "\tlet binding = contents.expect(\"REASON\");\n";
	print FH "\tlet lines = binding.split('\\n')\n\t\t.filter(|c| c.len() > 0)\n\t\t.map(|c| c.to_string()).collect();\n";
	print FH "\tlines\n}\n\n";
	print FH "fn part1() {\n";
	print FH "\tprintln!(\"Part 1: {}\", 0);\n}\n\n";
	print FH "fn part2() {\n";
	print FH "\tprintln!(\"Part 2: {}\", 0);\n}\n\n";
	print FH "fn main() {\n";
	print FH "\tprintln!(\"Year $year day $day - $name\");\n";
	print FH "\tpart1();\n\tpart2();\n}\n";
	close(FH);
}

sub python_template {
	my ($day, $year, $name) = @_;
	my $written_day = $day;
	if ($day =~ /^[1-9]$/) {
		$written_day = "0$day";
	}
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	mkdir "$year/$written_day";
	open(FH, '>', "$year/$written_day/main.py") or die $!;
	print FH "#!/usr/bin/env python3\n\n";
	print FH "def read_file(file):\n\twith open(file, 'r') as f:\n\t\tfor line in f:\n\t\t\tprint(line)\n\n";
	print FH "def part1():\n\tprint(f\"Part 1: {0}\")\n\n";
	print FH "def part2():\n\tprint(f\"Part 2: {0}\")\n\n";
	print FH "if __name__ == \"__main__\":\n\tprint(\"Year $year day $day - $name\")\n\tpart1()\n\tpart2()\n\n";
	close(FH);
	system("chmod", "+x", "$year/$written_day/main.py");
}

sub perl_template {
	my ($day, $year, $name) = @_;
	my $written_day = $day;
	if ($day =~ /^[1-9]$/) {
		$written_day = "0$day";
	}
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	mkdir "$year/$written_day";
	open(FH, '>', "$year/$written_day/main.pl") or die $!;
	print FH "#!/usr/bin/perl\n\n";
	print FH "sub read_file {\n\tmy (\$path) = \@_;\n\topen(IN, '<', \$path) or die \$!;\n\twhile (<IN>) {\n\t\t\$_;\n\t}\n\tclose(IN);\n}\n\n";
	print FH "sub part1 {\n\tprint \"Part 1: 0\";\n}\n\n";
	print FH "sub part2 {\n\tprint \"Part 2: 0\";\n}\n\n";
	print FH "print \"Year $year day $day 0 $name\";\npart1()\npart2()\n";
	close(FH);
	system("chmod", "+x", "$year/$written_day/main.pl");
}

sub cpp_template {
	my ($day, $year, $name) = @_;
	my $written_day = $day;
	if ($day =~ /^[1-9]$/) {
		$written_day = "0$day";
	}
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	mkdir "$year/$written_day";
	open(FH, '>', "$year/$written_day/main.cpp") or die $!;
	print FH "#include <iostream>\n#include <fstream>\n#include <vector>\n#include <string>\n\n";
	print FH "using namespace std;\n\n";
	print FH "vector<string> read_file(const string& filepath){\n\tifstream ifs;\n\tifs.open(filepath);\n\tstring line;\n\tvector<string> ret;\n\t";
	print FH "while(getline(ifs, line)){\n\t\tret.push_back(line);\n\t}\n\treturn ret;\n}\n\n";
	print FH "void part1(){\n\tcout << \"Part 1:\" << 0 << endl;\n}\n\n";
	print FH "void part2(){\n\tcout << \"Part 2:\" << 0 << endl;\n}\n\n";
	print FH "int main(int argc, char** argv) {\n\tcout << \"Year $year day $day - $name\" << endl;\n\tpart1();\n\tpart2();\n}\n\n";
	close(FH);
	open(FH, '>', "$year/$written_day/Makefile") or die $!;
	print FH "FILES = main\nFILES_O := \$(addsuffix .o, \$(FILES))\n\nmain: \$(FILES_O)\n\tg++ -o main \$(FILES_O)\n\n";
	print FH "%.o: %.cpp\n\tg++ -c \$<\n\nclean:\n\trm -f main *.o *.gch\n\n";
	close(FH);
}

## ================== ##
## == Main Program == ##
## ================== ##

if (1 > @ARGV) {
	print "🎄 There must be at least one argument. Run -h or --help for more information. 🎄\n";
	exit;
}

if ($ARGV[0] eq "-h" or $ARGV[0] eq "--help") {
	print "🎄 aoc.pl is simple tool for organizing advent of code folders throughout the years. 🎄\n";
	print "It is used for two purposes:\n";
	print "\t1) Creating pages for mdbook. - This is done by calling it wiht -p or --pages and optional path where to create it.\n";
	print "\t2) Creating projects for a given day. - You have more options.\n";
	print "\t\ta) Default language is rust. Then you call it by adding -t or --template.\n";
	print "\t\t\t\ So call ./aoc.pl -t to create rust project for today.\n";
	print "\t\tb) Next you may specify your language: by also adding the languege.\n";
	print "\t\t\t\ So call ./aoc.pl -t py to create python project for today.\n";
	print "\t\tc) Next you may also specify which year and day you want to solve.\n";
	print "\t\t\t\ So call ./aoc.pl -t rust 2023 5 to create rust project for 2023/12/5.\n";
	print "\t\td) You may leave out the language and only add year and day you want to solve.\n";
	print "\t\t\t\ So call ./aoc.pl -t 2023 5 to create rust (default) project for 2023/12/5.\n";
	print "\t3) Also you may call `./aoc.pl -g` or `--gitignore` to create basic gitignore file.\n";
	print "\nCurrently supported languages: Rust (rust, r), Python (python, py), Perl (perl, pl), Cpp (cpp, cc, c++).\n";
	exit;
}

if ($ARGV[0] eq "-p" or $ARGV[0] eq "--pages") {
	my $path = '.';
	if (@ARGV > 1) {
		$path = $ARGV[1];
	}
	print "🎄 Creating pages to `$path`. 🎄\n";
	create_pages($path);
} elsif ($ARGV[0] eq "-t" or $ARGV[0] eq "--template") {
	my ($sec, $min, $hour, $day, $month, $year) = localtime(time);
	$year += 1900;
	my $lang = "rust";
	if (@ARGV == 2) {
		$lang = $ARGV[1];
	} elsif (@ARGV == 3) {
		$year = $ARGV[1];
		$day = $ARGV[2];
	} elsif (@ARGV > 3) {
		$lang = $ARGV[1];
		$year = $ARGV[2];
		$day = $ARGV[3];
	}
	if (not $year =~ /^2[0-9][0-9][0-9]$/ or $year < $limit_year_low or $year > $limit_year_high) {
		print "🎄 Given year $year is not in a good format. Or is outside the limits [$limit_year_low - $limit_year_high]. 🎄\n";
		exit;
	}
	if (not $day =~ /^[1-9][0-9]*$/ or $day < $limit_day_low or $day > $limit_day_high) {
		print "🎄 Given day $day is not in a good foramt. Or is beyond the limit [$limit_day_low - $limit_day_high]. 🎄\n";
		exit;
	}
	my $name = get_name($day, $year);
	if ($lang =~ /rust/i or $lang eq "r") {
		print "🎄 Creating rust 🦀 project for $name (day: $day, year: $year). 🎄\n";
		rust_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang eq "py" or $lang =~ /python/i) {
		print "🎄 Creating python 🐍 project for $name (day: $day, year: $year). 🎄\n";
		python_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang eq "pl" or $lang =~ /perl/i) {
		print "🎄 Creating perl 🐪 project for $name (day: $day, year: $year). 🎄\n";
		perl_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang =~ /c[+p][+p]/i or $lang =~ /cc/i) {
		print "🎄 Creating cpp ➕➕ project for $name (day: $day, year: $year). 🎄\n";
		cpp_template($day, $year, $name);
		general_template($day, $year);
	} else {
		print "The given language $lang is not supported.\n";
	}
} elsif ($ARGV[0] eq "-g" or $ARGV[0] eq "--gitignore") {
	print "🎄 Creating basic `.gitignore` file. 🎄\n";
	open(FH, '>', ".gitignore") or die $!;
	print FH "main\nINPUT\nOUTPUT\naoc/\nadventofcode.md\ntarget/\n*.lock\n*.dot\n*.png\n*.o\n*.gch\n";
	close(FH);
} else {
	print "🎄 Nothing was called try using -h or --help. 🎄\n";
}
