IO.puts("Year 2015 day 4 - The Ideal Stocking Stuffer")

defmodule Crypto do
	def find(key, size, prefix) do
		Stream.iterate(0, fn x -> x + 1 end)
		|> Enum.take_while(fn x -> :crypto.hash(:md5, key <> "#{x}") |> Base.encode16() |> String.slice(0, size) != prefix end)
		|> length()
	end
end

if {:ok, string} = File.read("INPUT") do
	IO.puts("Part 1: #{Crypto.find(String.trim(string), 5, "00000")}")
	IO.puts("Part 2: #{Crypto.find(String.trim(string), 6, "000000")}")
end
