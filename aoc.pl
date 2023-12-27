#!/usr/bin/perl
use strict;
use warnings;
use Path::Tiny;

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

sub process_file {
	my ($aoc_dir, $year, $day, $file) = @_;
	my $file_name = $file;
	$file_name =~ s/^.*\///g;
	if (not grep {$_ eq $file_name} @forb) {
		print_file($file, "$aoc_dir/$year-$day.md");
	}
}

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

my ($path) = @ARGV;

create_pages($path);

