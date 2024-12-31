IO.puts("Year 2015 day 5 - Doesn't He Have Intern-Elves For This?")

if {:ok, string} = File.read("INPUT") do
	result = String.trim(string)
	|> String.split("\n")
	|> Enum.filter(fn x -> String.length(x) > 0 end)
	|> Enum.filter(fn x -> String.match?(x, ~r/([a-z])\1/) end)
	|> Enum.filter(fn x -> String.graphemes(x) |> Enum.filter(fn x -> x == "a" or x == "e" or x == "i" or x == "o" or x == "u" end) |> Enum.count() >= 3 end)
	|> Enum.filter(fn x -> not String.match?(x, ~r/(?:ab)|(?:cd)|(?:pq)|(?:xy)/) end)
	|> Enum.count()
	IO.puts("Part 1: #{result}")
end

if {:ok, string} = File.read("INPUT") do
	result = String.trim(string)
	|> String.split("\n")
	|> Enum.filter(fn x -> String.length(x) > 0 end)
	|> Enum.filter(fn x -> String.match?(x, ~r/([a-z]{2}).*\1/) end)
	|> Enum.filter(fn x -> String.match?(x, ~r/([a-z]).\1/) end)
	|> Enum.count()
	IO.puts("Part 2: #{result}")
end
