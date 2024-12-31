IO.puts("Year 2015 day 01 - Not Quite Lisp")

if {:ok, string} = File.read("INPUT") do
	map = String.trim(string)
	|> String.graphemes()
	|> Enum.frequencies()
	IO.puts("Part 1: #{map["("] - map[")"]}")
end

if {:ok, string} = File.read("INPUT") do
	{basement, _} = String.trim(string)
	|> String.graphemes()
	|> Enum.map(fn c -> if c == "(" do +1 else -1 end end)
	|> List.foldl({0,0}, fn(x, {index, acc}) -> if acc >= 0 do {index + 1, acc + x} else {index, acc} end end)
	IO.puts("Part 2: #{basement}")
end
