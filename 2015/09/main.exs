IO.puts("Year 2015 day 9 - All in a Single Night")

defmodule Perms do
  def perms([]), do: [[]]

  def perms(l) do
    for h <- l, t <- perms(l -- [h]),
      do: [h|t]
  end
end

defmodule Cycle do
	def compute(_, [_]), do: 0

	def compute(distances, [t1, t2 | towns]) do
		{_, _, dist} = Enum.find(distances, fn {t3, t4, _} -> t3 == t1 and t4 == t2 end)
		dist + compute(distances, [t2 | towns])
	end
end

if {:ok, content} = File.read("INPUT") do
	distances = Regex.scan(~r/([a-zA-Z]+) to ([a-zA-Z]+) = ([0-9]+)/, String.trim(content))
	|> Enum.map(fn [_, town1, town2, dist] -> [town1, town2, Integer.parse(dist)] end)
	|> Enum.map(fn [town1, town2, {dist, _}] -> [{town1, town2, dist}, {town2, town1, dist}] end)
	|> List.flatten()
	|> Enum.sort(fn {t1, t2, _}, {t3, t4, _} -> t1 < t3 or (t1 == t3 and t2 < t4) end)
	result = Enum.map(distances, fn {town	, _, _} -> town end)
	|> Enum.dedup()
	|> Perms.perms()
	|> Enum.map(fn towns -> Cycle.compute(distances, towns) end)
	|> Enum.min()
	IO.puts("Part 1: #{result}")
end

if {:ok, content} = File.read("INPUT") do
	distances = Regex.scan(~r/([a-zA-Z]+) to ([a-zA-Z]+) = ([0-9]+)/, String.trim(content))
	|> Enum.map(fn [_, town1, town2, dist] -> [town1, town2, Integer.parse(dist)] end)
	|> Enum.map(fn [town1, town2, {dist, _}] -> [{town1, town2, dist}, {town2, town1, dist}] end)
	|> List.flatten()
	|> Enum.sort(fn {t1, t2, _}, {t3, t4, _} -> t1 < t3 or (t1 == t3 and t2 < t4) end)
	result = Enum.map(distances, fn {town	, _, _} -> town end)
	|> Enum.dedup()
	|> Perms.perms()
	|> Enum.map(fn towns -> Cycle.compute(distances, towns) end)
	|> Enum.max()
	IO.puts("Part 2: #{result}")
end
