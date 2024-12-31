IO.puts("Year 2015 day 4 - The Ideal Stocking Stuffer")

if {:ok, string} = File.read("INPUT") do
	key = String.trim(string)
	result = Stream.iterate(0, fn x -> x + 1 end)
	|> Enum.take_while(fn x -> :crypto.hash(:md5, key <> "#{x}") |> Base.encode16() |> String.slice(0, 5) != "00000" end)
	IO.puts("Part 1: #{length(result)}")
end

if {:ok, string} = File.read("INPUT") do
	key = String.trim(string)
	result = Stream.iterate(0, fn x -> x + 1 end)
	|> Enum.take_while(fn x -> :crypto.hash(:md5, key <> "#{x}") |> Base.encode16() |> String.slice(0, 6) != "000000" end)
	IO.puts("Part 2: #{length(result)}")
end
