IO.puts("Year 2015 day 02 - I Was Told There Would Be No Math")

if {:ok, string} = File.read("INPUT") do
	result = String.split(string, "\n") |> Enum.filter(fn str -> String.length(str) > 0 end) |> Enum.map(fn line -> String.split(line, "x") |> Enum.map(fn val -> Integer.parse(val) end) end) |> Enum.map(fn [{l, _}, {w, _}, {h, _}] -> 2 * l * w + 2 * l * h + 2 * h * w + Enum.min([l*w, l*h, h*w]) end) |> Enum.sum()
	IO.puts("Part 1: #{result}")
end

if {:ok, string} = File.read("INPUT") do
	result = String.split(string, "\n") |> Enum.filter(fn str -> String.length(str) > 0 end) |> Enum.map(fn line -> String.split(line, "x") |> Enum.map(fn val -> Integer.parse(val) end) end) |> Enum.map(fn [{l, _}, {w, _}, {h, _}] -> l * w * h + 2 * Enum.min([l,w,h]) + 2 * Enum.min([Enum.max([l, w]), Enum.max([l, h]), Enum.max([h, w])]) end) |> Enum.sum()
	IO.puts("Part 2: #{result}")
end
