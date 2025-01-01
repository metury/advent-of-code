IO.puts("Year 2015 day 6 - Probably a Fire Hazard")

if {:ok, string} = File.read("INPUT") do
	array = :array.new(size: 1000 * 1000, default: false)
	result = Regex.scan(~r/((?:turn off)|(?:turn on)|(?:toggle)) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)/, String.trim(string))
	|> Enum.map(fn [_, keyword, x, y, end_x, end_y] -> [keyword, Integer.parse(x), Integer.parse(y), Integer.parse(end_x), Integer.parse(end_y)] end)
	|> Enum.map(fn [keyword, {start_x, _}, {start_y, _}, {end_x, _}, {end_y, _}] -> {keyword, for x <- start_x..end_x, y <- start_y..end_y do 1000*x + y end} end)
	|> List.foldl(array, fn {keyword, range}, acc ->
	case keyword do
		"turn on" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, true, acc) end)
		"turn off" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, false, acc) end)
		"toggle" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, not :array.get(x, acc), acc) end)
	end end)
	IO.puts("Part 1: #{Enum.count(:array.to_list(result), fn x -> x end)}")
end

if {:ok, string} = File.read("INPUT") do
	array = :array.new(size: 1000 * 1000, default: 0)
	result = Regex.scan(~r/((?:turn off)|(?:turn on)|(?:toggle)) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)/, String.trim(string))
	|> Enum.map(fn [_, keyword, x, y, end_x, end_y] -> [keyword, Integer.parse(x), Integer.parse(y), Integer.parse(end_x), Integer.parse(end_y)] end)
	|> Enum.map(fn [keyword, {start_x, _}, {start_y, _}, {end_x, _}, {end_y, _}] -> {keyword, for x <- start_x..end_x, y <- start_y..end_y do 1000*x + y end} end)
	|> List.foldl(array, fn {keyword, range}, acc ->
	case keyword do
		"turn on" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, :array.get(x, acc) + 1, acc) end)
		"turn off" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, Enum.max([0, :array.get(x, acc) - 1]), acc) end)
		"toggle" ->  Enum.reduce(range, acc, fn x, acc -> :array.set(x, :array.get(x, acc) + 2, acc) end)
	end end)
	IO.puts("Part 2: #{Enum.sum(:array.to_list(result))}")
end
