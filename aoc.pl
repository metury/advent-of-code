#!/usr/bin/perl
use strict;
use warnings;
use Path::Tiny;
use LWP::Simple;

## ================== ##
## == Create pages == ##
## ================== ##

my $dir = path('.');
my $aoc = "advent of code";
my $link = "/aoc/";
my @forb = ("INPUT", "OUTPUT", "Cargo.toml", "Cargo.lock", "info.md", "main", "graph.dot", "graph.png", "graph.svg");

# Print main file of the aoc.
sub print_aoc {
	my ($name) = @_;
	open(FH, '>', $name) or die $!;
	print FH "---\n";
	print FH "layout: page\n";
	print FH "title: $aoc\n";
	print FH "permalink: $link\n";
	print FH "has_children: true\n";
	print FH "---\n\n";
	print FH "They are my solutions to [advent of code](https://adventofcode.com/) tasks. There are separated to each year and day. All of this can be found on [GitHub](https://github.com/metury/advent-of-code), also with the script that generates these pages.\n";
	print FH "Plus you may also play a small [Bingo](https://aoc-bingo.fly.dev/) that someone made. Also you may consider joining [Reddit](https://www.reddit.com/r/adventofcode/) where you may find useful tips, or help someone.\n\n";
	print FH "### Years\n\n";
	close(FH);
}

# Append year to the aoc file.
sub append_year {
	my ($name, $year) = @_;
	open(FH, '>>', $name) or die $!;
	print FH "- [Year $year]($link$year)\n";
	close(FH);
}

# Print year file.
sub print_year {
	my ($name, $year) = @_;
	open(FH, '>', $name) or die $!;
	print FH "---\n";
	print FH "layout: page\n";
	print FH "title: Year $year\n";
	print FH "parent: $aoc\n";
	print FH "permalink: $link$year\n";
	print FH "has_children: true\n";
	print FH "---\n\n";
	print FH "This contains tasks from the [year $year](https://adventofcode.com/$year). Go back to [AOC]($link).\n\n";
	print FH "### Days\n\n";
	close(FH);
}

# Append day to the year page.
sub append_day {
	my ($name, $year, $day) = @_;
	open(FH, '>>', $name) or die $!;
	print FH "- [Day $day]($link$year/$day/)\n";
	close(FH);
}

# Print day file.
sub print_day {
	my ($name, $year, $day) = @_;
	open(FH, '>', $name) or die $!;
	print FH "---\n";
	print FH "layout: page\n";
	print FH "title: Day $day\n";
	print FH "parent: Year $year\n";
	print FH "grand_parent: $aoc\n";
	print FH "permalink: $link$year/$day/\n";
	print FH "---\n\n";
	print FH "This is a solution of the [day $day](https://adventofcode.com/$year/day/$day). Go back to year [$year]($link$year). Go back to [AOC]($link).\n\n";
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
	my ($aoc_dir, $year, $day_dir) = @_;
	if ($day_dir->is_dir() and $day_dir =~ /.*\/[0-9][1-9]/ ){
	my $day = (split /\//, $day_dir)[1];
		$day =~ s/^0//g;
		print_day("$aoc_dir/$year-$day.md", $year, $day);
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
	my ($aoc_dir, $year_dir, $aoc_file) = @_;
	if ($year_dir->is_dir() and $year_dir =~ /20[0-9][0-9]/) {
		my @year_path = (split /\//, $year_dir);
		my $year = $year_path[@year_path - 1];
		print_year("$aoc_dir/$year.md", $year);
		append_year("$aoc_file", $year);
		opendir(DIR, $year_dir) or die $!;
		my @directories = sort readdir(DIR);
		closedir(DIR);
		foreach my $day_dir (@directories) {
			$day_dir = path("$year_dir/$day_dir");
			process_day($aoc_dir, $year, $day_dir);
		}
	}
}

# Create all pages.
sub create_pages {
	my ($path) = @_;
	my $aoc_dir = "$path/aoc";
	my $aoc_file = "$path/adventofcode.md";
	mkdir "$aoc_dir";
	print_aoc($aoc_file);
	opendir(DIR, $dir) or die $!;
	my @directories = sort readdir(DIR);
	closedir(DIR);
	foreach my $year_dir (@directories) {
		process_year($aoc_dir, path("$dir/$year_dir"), $aoc_file);
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
	my $written_day = "0$day" if ($day =~ /[1-9]/);
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
	my $written_day = "0$day" if ($day =~ /[1-9]/);
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	system("cargo", "new", "$year/aoc-$year-$day");
	rename("$year/aoc-$year-$day", "$year/$written_day");
	open(FH, '>', "$year/$written_day/src/main.rs");
	print FH "use std::fs\n\n";
	print FH "fn read_file(filepath: &str) -> Vec<&str> {\n";
	print FH "\tlet contents = fs::read_to_string(filepath);\n";
	print FH "\tlet binding = contents.expect(\"REASON\");\n";
	print FH "\tlet lines: Vec<&str> = binding.split('\\n').collect();\n";
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
	my $written_day = "0$day" if ($day =~ /[1-9]/);
	mkdir $year;
	if (path("$year/$written_day")->is_dir()) {
		die "This project already exists.\n";
	}
	mkdir "$year/$written_day";
	open(FH, '>', "$year/$written_day/main.py");
	print FH "#!/usr/bin/env python3\n\n";
	print FH "def read_file(file):\n\twith open(file, 'r') as f:\n\t\tfor line in f:\n\t\t\tprint(line)\n\n";
	print FH "def part1():\n\tprint(f\"Part 1: {0}\")\n\n";
	print FH "def part2():\n\tprint(f\"Part 2: {0}\")\n\n";
	print FH "if __name__ == \"__main__\":\n\tprint(\"Year $year day $day - $name\")\n\tpart1()\n\tpart2()\n\n";
	close(FH);
	system("chmod", "+x", "$year/$written_day/main.py");
}

## ================== ##
## == Main Program == ##
## ================== ##

if (1 > @ARGV) {
	print "There must be at least one argument. Run -h or --help for more information.\n";
	exit;
}

if ($ARGV[0] eq "-h" or $ARGV[0] eq "--help") {
	print "🎄🎄 aoc.pl is simple tool for organizing advent of code folders throughout hte years.\n";
	print "It is used for two purposes:\n";
	print "\t1) Creating pages for jekyll. - This is done by calling it wiht -p or --pages and optional path where to create it.\n";
	print "\t2) Creating projects for a given day. - You have more options.\n";
	print "\t\ta) Default language is rust. Then you call it by adding -t or --template.\n";
	print "\t\t\t\ So call ./aoc.pl -t to create rust project for today.\n";
	print "\t\tb) Next you may specify your language: by also adding the languege.\n";
	print "\t\t\t\ So call ./aoc.pl -t py to create python project for today.\n";
	print "\t\tc) Next you may also specify which year and day you want to solve.\n";
	print "\t\t\t\ So call ./aoc.pl -t rust 2023 5 to create rust project for 2023/12/5.\n";
	print "\t\td) You may leave out the language and only add year and day you want to solve.\n";
	print "\t\t\t\ So call ./aoc.pl -t 2023 5 to create rust (default) project for 2023/12/5.\n";
	print "\nCurrently supported languages: Rust (rust, r), Python (python, py), ... (some may be added later on).\n";
	exit;
}

if ($ARGV[0] eq "-p" or $ARGV[0] eq "--pages") {
	my $path = '.';
	if (@ARGV > 1) {
		$path = $ARGV[1];
	}
	print "🎄 Creating pages to $path. 🎄\n";
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
	my $name = get_name($day, $year);
	if ($lang eq "rust" or $lang eq "RUST" or $lang eq "Rust" or $lang eq "r") {
		print "🎄 Creating rust 🦀 project for $name (day: $day, year: $year). 🎄\n";
		rust_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang eq "py" or $lang eq "python" or $lang eq "Python") {
		print "🎄 Creating python 🐍 project for $name (day: $day, year: $year). 🎄\n";
		python_template($day, $year, $name);
		general_template($day, $year);
	}
	else {
		print "The given language $lang is not supported.\n";
	}
}
