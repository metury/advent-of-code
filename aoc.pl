#!/usr/bin/perl
use strict;
use warnings;
use Path::Tiny;
use LWP::Simple;
use Term::ANSIColor;

## ======================= ##
## == Global constants. == ##
## ======================= ##

our $limit_year_low = 2015;
our $limit_year_high = 2024;
our $limit_day_low = 1;
our $limit_day_high = 25;
our $default_lang = "go";

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
	print FH "Plus you may also play a small [Bingo](https://aoc-bingo.fly.dev/) that someone made. Also you may consider joining [Reddit](https://www.reddit.com/r/adventofcode/) where you may find useful tips, or help someone. Lastly there is an [unofficial survey](https://jeroenheijmans.github.io/advent-of-code-surveys/).\n\n";
	close(FH);
	if (-e "info.md" ) {
		print_info("info.md", "$name");
	}
	open(FH, '>>', $name) or die $!;
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
	my ($name, $year, $mdbook, $year_dir) = @_;
	open(MD, '>>', $mdbook) or die $!;
	print MD "\t- [Year $year]($link$year.md)\n";
	close(MD);
	open(FH, '>', $name) or die $!;
	print FH "# Advent of code - Year $year\n\n";
	print FH "This contains tasks from the [year $year](https://adventofcode.com/$year). Go back to [AOC](../$root).\n\n";
	close(FH);
	if (-e "$year_dir/info.md" ) {
			print_info("$year_dir/info.md", "$name");
	}
	open(FH, '>>', $name) or die $!;
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
	print FH "### Information\n\n";
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
	if ($day_dir->is_dir() and $day_dir =~ /.*\/[0-9][0-9]/ ){
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
		print_year("$aoc_dir/$year.md", $year, $mdbook, $year_dir);
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

sub update_summary {
	my ($file) = @_;

	if (not -e $file) {
		open(my $in, '>', $file) or die $!;
	}

	open(my $in, '<', $file) or die $!;
	my $content = do { local $/; <$in> };
	close($in);

	$content =~ s/<!--AOC-->.*<!--AOC-->//sg;

	open(my $out, '>', $file) or die $!;
	print $out $content;
	close($out);
}

# Create all pages.
sub create_pages {
	my ($path) = @_;
	my $aoc_dir = "$path/aoc";
	my $aoc_file = "$path/$root";
	my $mdbook = "$path/SUMMARY.md";
	update_summary($mdbook);
	open(MD, '>>', $mdbook) or die $!;
	print MD "<!--AOC-->\n";
	print MD "# Advent of code\n\n";
	print MD "- [Advent of code](./$root)\n";
	close(MD);
	mkdir "$aoc_dir";
	print_aoc($aoc_file);
	opendir(DIR, $dir) or die $!;
	my @directories = reverse sort readdir(DIR);
	closedir(DIR);
	foreach my $year_dir (@directories) {
		process_year($aoc_dir, path("$dir/$year_dir"), $aoc_file, $mdbook);
	}
	open(MD, '>>', $mdbook) or die $!;
	print MD "<!--AOC-->";
	close(MD);
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

sub go_template {
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
	open(FH, '>', "$year/$written_day/main.go") or die $!;
	print FH "package main\n\n";
	print FH "import (\n\t\"fmt\"\n\t\"log\"\n\t\"os\"\n\t\"regexp\"\n\t\"strconv\"\n\t\"time\"\n)\n\n";
	print FH "const (\n\tBlue   = \"\033[1;34m\"\n\tYellow = \"\033[1;33m\"\n\tGreen  = \"\033[1;32m\"\n\tReset  = \"\033[0m\"\n)\n\n";
	print FH"func print_result(dur time.Duration, part, result int) {\n\tfmt.Println(\"Part \" + fmt.Sprint(part) + \" [\" + Blue + fmt.Sprint(dur) + Reset + \"]: \" + Yellow + fmt.Sprint(result) + Reset)\n}\n\n";
	print FH "func read_file(file_path string) string {\n\tcontent, err := os.ReadFile(file_path)\n\tif err != nil {\n\t\tlog.Fatal(err)\n\t}\n\treturn string(content)\n}\n\n";
	print FH "func part_one() {\n\tvar result int\n\tstart := time.Now()\n\tend := time.Now()\n\tprint_result(end.Sub(start), 1, result)\n}\n\n";
	print FH "func part_two() {\n\tvar result int\n\tstart := time.Now()\n\tend := time.Now()\n\tprint_result(end.Sub(start), 2, result)\n}\n\n";
	print FH "func main() {\n\tfmt.Println(\"Year \" + Green + \"$year\" + Reset + \" day \" + Green + \"$day - Claw Contraption\" + Reset)\n\tpart_one()\n\tpart_two()\n}";
	close(FH);
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

sub notify_creation {
	my ($lang, $name, $day, $year) = @_;
	print "Creating ";
	print colored("$lang", 'cyan bold');
	print " project for ";
	print colored("$name", 'cyan bold');
	print " (day: ";
	print colored("$day", 'cyan');
	print ", year: ";
	print colored("$year", 'cyan');
	print ").\n";
}

if (1 > @ARGV) {
	print colored("There must be at least one argument. Run -h or --help for more information.\n", 'cyan bold');
	exit;
}

if ($ARGV[0] eq "-h" or $ARGV[0] eq "--help") {
	print colored(" ______     ______     ______     ______   __        \n", 'cyan');
	print colored("/\\  __ \\   /\\  __ \\   /\\  ___\\   /\\  == \\ /\\ \\       \n", 'cyan');
	print colored("\\ \\  __ \\  \\ \\ \\/\\ \\  \\ \\ \\____  \\ \\  _-/ \\ \\ \\____  \n", 'cyan');
	print colored(" \\ \\_\\ \\_\\  \\ \\_____\\  \\ \\_____\\  \\ \\_\\    \\ \\_____\\ \n", 'cyan');
	print colored("  \\/_/\\/_/   \\/_____/   \\/_____/   \\/_/     \\/_____/ \n", 'cyan');
	print colored("                                                     \n\n", 'cyan');

	print "This is a simple yet useful tool for organizing advent of code folders throughout the years. You may call it with multiple arguments.\n\n";

	print colored("Template generating - create an almost empty solution for a given day.\n\n", 'green bold');

	print colored("\tperl aoc.pl [-t | --template] [programming language] [year day]\t", 'cyan');
	print "Define every property.\n";
	print colored("\tperl aoc.pl [-t | --template] [programming language]           \t", 'cyan');
	print "Use today as the date.\n";
	print colored("\tperl aoc.pl [-t | --template] [year day]                       \t", 'cyan');
	print "Use default language: rust.\n";
	print colored("\tperl aoc.pl [-t | --template]                                  \t", 'cyan');
	print "Use today date and default language.\n\n";

	print colored("Extracting and generating pages - create multiple markdown files for mdbook.\n\n", 'green bold');

	print colored("\tperl aoc.pl [-p | --pages] [files destination]\t", 'cyan');
	print "Define your prefered destination.\n";
	print colored("\tperl aoc.pl [-p | --pages]                    \t", 'cyan');
	print "Use current directory for the destination.\n\n";

	print colored("Create .gitignore - create simple gitignore to respect not sharing input and not to sync useless files.\n\n", 'green bold');

	print colored("\tperl aoc.pl [-g | --gitignore]\n\n", 'cyan');

	print colored("See help.\n\n", 'green bold');

	print colored("\tperl aoc.pl [-h | --help]\n\n", 'cyan');
	exit;
}

if ($ARGV[0] eq "-p" or $ARGV[0] eq "--pages") {
	my $path = '.';
	if (@ARGV > 1) {
		$path = $ARGV[1];
	}
	print "Creating pages to ";
	print colored("$path", 'cyan bold');
	print ".\n";
	create_pages($path);
} elsif ($ARGV[0] eq "-t" or $ARGV[0] eq "--template") {
	my ($sec, $min, $hour, $day, $month, $year) = localtime(time);
	$year += 1900;
	my $lang = $default_lang;
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
		print "Given year ";
		print colored("$year", 'cyan bold');
		print " is not in a good format. Or is outside the limits ";
		print colored("[$limit_year_low - $limit_year_high]", 'cyan');
		print ".\n";
		exit;
	}
	if (not $day =~ /^[1-9][0-9]*$/ or $day < $limit_day_low or $day > $limit_day_high) {
		print "Given day ";
		print colored("$day", 'cyan bold');
		print " is not in a good foramt. Or is beyond the limit ";
		print colored("[$limit_day_low - $limit_day_high]", 'cyan');
		print ".\n";
		exit;
	}
	my $name = get_name($day, $year);
	if ($lang =~ /rust/i or $lang eq "r") {
		notify_creation("rust", $name, $day, $year);
		rust_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang eq "py" or $lang =~ /python/i) {
		notify_creation("python", $name, $day, $year);
		python_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang eq "pl" or $lang =~ /perl/i) {
		notify_creation("perl", $name, $day, $year);
		perl_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang =~ /c[+p][+p]/i or $lang =~ /cc/i) {
		notify_creation("cpp", $name, $day, $year);
		cpp_template($day, $year, $name);
		general_template($day, $year);
	} elsif ($lang =~ /golang/i or $lang =~ /go/i) {
		notify_creation("go", $name, $day, $year);
		go_template($day, $year, $name);
		general_template($day, $year);
	} else {
		print "The given language ";
		print colored("$lang", 'cyan bold');
		print " is not supported.\n";
	}
} elsif ($ARGV[0] eq "-g" or $ARGV[0] eq "--gitignore") {
	print "Creating basic `.gitignore` file.\n";
	open(FH, '>', ".gitignore") or die $!;
	print FH "main\nINPUT\nOUTPUT\naoc/\nadventofcode.md\ntarget/\n*.lock\n*.dot\n*.png\n*.o\n*.gch\n*.svg\n";
	close(FH);
} else {
	print colored("Wrong argument. Nothing was called try using -h or --help.\n", 'cyan bold');
}
