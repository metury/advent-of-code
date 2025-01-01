IO.puts("Year 2015 day 9 - All in a Single Night")

defmodule Cycle do
	def recfind([dist | distances], length, towns) do

	end

	def find(distances, towns) do
		recfind(distances, 0, towns)
	end
end

if {:ok, content} = File.read("INPUT") do
	distances = Regex.scan(~r/([a-zA-Z]+) to ([a-zA-Z]+) = ([0-9]+)/, String.trim(content))
	|> Enum.map(fn [_, town1, town2, dist] -> [town1, town2, Integer.parse(dist)] end)
	|> Enum.map(fn [town1, town2, {dist, _}] -> [{town1, town2, dist}, {town2, town1, dist}] end)
	|> List.flatten()
	|> Enum.sort(fn {t1, t2, _}, {t3, t4, _} -> t1 < t3 or (t1 == t3 and t2 < t4) end)
	towns = distances |> Enum.map(fn {town	, _, _} -> town end) |> Enum.dedup()
	IO.puts("Part 1: #{0}")
end

if {:ok, string} = File.read("INPUT") do
	IO.puts("Part 2: #{0}")
end
