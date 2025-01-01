IO.puts("Year 2015 day 8 - Matchsticks")

if {:ok, string} = File.read("INPUT") do
	result = String.trim(string)
	|> String.split("\n")
	|> Enum.map(fn x -> {String.length(x), x} end)
	|> Enum.map(fn {size, x} -> {size, String.replace(x, ~r/(?:\\\\)|(?:\\x[a-f0-9]{2})|(?:\\")/, "A")} end)
	|> Enum.map(fn {size, x} -> size - String.length(x) + 2 end)
	|> Enum.sum()
	IO.puts("Part 1: #{result}")
end

if {:ok, string} = File.read("INPUT") do
	result = String.trim(string)
	|> String.split("\n")
	|> Enum.map(fn x -> {String.length(x), x} end)
	|> Enum.map(fn {size, x} -> {size, String.replace(x, ~r/\"|\\/, "AA")} end)
	|> Enum.map(fn {size, x} -> String.length(x) + 2 - size end)
	|> Enum.sum()
	IO.puts("Part 2: #{result}")
end
