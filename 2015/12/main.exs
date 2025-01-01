IO.puts("Year 2015 day 12 - JSAbacusFramework.io")

defmodule JSON do
	def count(content) do
	Regex.scan(~r/\-{0,1}[0-9]+/, content)
	|> Enum.map(fn [x] -> if {val, _ } = Integer.parse(x) do val end end)
	|> Enum.sum
	end
end

if {:ok, content} = File.read("INPUT") do
	IO.puts("Part 1: #{JSON.count(content)}")
	valid = Regex.replace(~r/\{[^\{\}]*\:"red"[^\{\}]*\}/, content, "")
	IO.puts("Part 2: #{JSON.count(valid)}")
end
