IO.puts("Year 2015 day 3 - Perfectly Spherical Houses in a Vacuum")

if {:ok, string} = File.read("INPUT") do
	{map, _, _} = string |> String.graphemes()
	|> List.foldl({%{}, 0,0}, fn(x, {map, acc_x, acc_y}) ->
	cond do
		x == "<" -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x - 1, acc_y}
		x == ">" -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x + 1, acc_y}
		x == "^" -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y - 1}
		x == "v" -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y + 1}
		true     -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y}
	end end)
	IO.puts("Part 1: #{map_size(map)}")
end

if {:ok, string} = File.read("INPUT") do
	{map, _, _, _, _, _} = string |> String.graphemes()
	|> List.foldl({%{},0,0,false, 0, 0}, fn(x, {map, acc_x, acc_y, robo, robo_x, robo_y}) ->
	cond do
		x == "<" and not robo -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x - 1, acc_y, true, robo_x, robo_y}
		x == ">" and not robo -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x + 1, acc_y, true, robo_x, robo_y}
		x == "^" and not robo -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y - 1, true, robo_x, robo_y}
		x == "v" and not robo -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y + 1, true, robo_x, robo_y}
		x == "<" and robo     -> {Map.put_new(map, "#{robo_x}X#{robo_y}", true), acc_x, acc_y, false, robo_x - 1, robo_y}
		x == ">" and robo     -> {Map.put_new(map, "#{robo_x}X#{robo_y}", true), acc_x, acc_y, false, robo_x + 1, robo_y}
		x == "^" and robo     -> {Map.put_new(map, "#{robo_x}X#{robo_y}", true), acc_x, acc_y, false, robo_x, robo_y - 1}
		x == "v" and robo     -> {Map.put_new(map, "#{robo_x}X#{robo_y}", true), acc_x, acc_y, false, robo_x, robo_y + 1}
		not robo              -> {Map.put_new(map, "#{acc_x}X#{acc_y}", true), acc_x, acc_y, true, robo_x, robo_y}
		robo                  -> {Map.put_new(map, "#{robo_x}X#{robo_y}", true), acc_x, acc_y, false, robo_x, robo_y}
	end end)
	IO.puts("Part 2: #{map_size(map)}")
end
