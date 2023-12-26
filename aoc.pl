#!/usr/bin/perl
use strict;
use warnings;
use Path::Tiny;

my $dir = path('.');
my $aoc = "advent of code";
my $link = "/aoc/";
my $aoc_dir = "aoc";
my $aoc_file = "adventofcode.md";
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

# Print day file.
sub print_day {
	my ($name, $year, $day) = @_;
	open(FH, '>', $name) or die $!;
	print FH "---\n";
	print FH "layout: page\n";
	print FH "title: Day $day\n";
	print FH "parent: Year $year\n";
	print FH "grand_parent: $aoc\n";
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
	print FH "\`\`\`\n";
	close(FH);
}

mkdir "$aoc_dir";
print_aoc($aoc_file);

# Appending previous files is not inside.

my $iter = $dir->iterator;
while (my $year_dir = $iter->()) {
	if ($year_dir->is_dir() and not $year_dir eq "aoc" and not $year_dir eq ".git") {
		print_year("$aoc_dir/$year_dir.md", $year_dir);
		my $inner_iter = $year_dir->iterator;
		while (my $day_dir = $inner_iter->()) {
			my $day = (split /\//, $day_dir)[1];
			$day =~ s/^0//g;
			print_day("$aoc_dir/$year_dir-$day.md", $year_dir, $day);
			my $day_iter = $day_dir->iterator;
			while (my $file = $day_iter->()) {
				if ($file->is_dir() and $file =~ /^.*\/src/) {
					my $src_iter = $file->iterator;
					while (my $src_file = $src_iter->()) {
						my $src_file_name = $file;
						$src_file_name =~ s/^.*\///g;
						if (not grep {$_ eq $src_file_name} @forb) {
							print_file($src_file, "$aoc_dir/$year_dir-$day.md");
						}
					}
				}
				elsif (not $file->is_dir()) {
					my $file_name = $file;
					$file_name =~ s/^.*\///g;
					if (not grep {$_ eq $file_name} @forb) {
						print_file($file, "$aoc_dir/$year_dir-$day.md");
					}
				}
			}
		}
	}
}
