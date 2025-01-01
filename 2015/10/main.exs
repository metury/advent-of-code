IO.puts("Year 2015 day 10 - Elves Look, Elves Say")

defmodule Digits do
	def next(current, counter, []), do: [counter, current]

	def next(current, counter, [h | l]) do
		cond do
			current == h -> next(current, counter + 1, l)
			current != h -> [counter, current] ++ next(h, 1, l)
		end
	end

	def next([h | l]), do: next(h, 1, l)
end

if {:ok, content} = File.read("INPUT") do
	input = String.trim(content) |> String.graphemes() |> Enum.map(fn x -> if {val, _} = Integer.parse(x) do val end end)
	result = Enum.reduce(1..40, input, fn _, acc -> Digits.next(acc) end)
	IO.puts("Part 1: #{length(result)}")
	result = Enum.reduce(1..10, result, fn _, acc -> Digits.next(acc) end)
	IO.puts("Part 2: #{length(result)}")
end
