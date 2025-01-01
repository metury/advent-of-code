IO.puts("Year 2015 day 12 - JSAbacusFramework.io")

if {:ok, content} = File.read("INPUT") do
	result = Regex.scan(~r/\-{0,1}[0-9]+/, content)
	|> Enum.map(fn [x] -> if {val, _ } = Integer.parse(x) do val end end)
	|> Enum.sum
	IO.puts("Part 1: #{result}")
	{result, _}= Regex.scan(~r/(?:\-{0,1}[0-9]+)|(?:\:\s*"red")|(?:\})|(?:\{)/, content)
	|> List.foldl({0, 0}, fn [x], {sum, red} ->
	cond do
		x == ":\"red\"" ->
			{sum, if red == 0 do 1 else red end}
		x == "}" ->
			{sum, Enum.max([0, red-1])}
		x == "{" ->
			{sum, if red > 0 do red + 1 else 0 end}
		red == 0 ->
			{val, _ } = Integer.parse(x)
			{sum + val, red}
		true ->
			{sum, red}
		end end)
		IO.puts("Part 2: #{result}")
end
