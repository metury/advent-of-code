IO.puts("Year 2015 day 11 - Corporate Policy")

defmodule Word do
	def shift(word) do
		{word, _} = word
		|> Enum.reverse()
		|> List.foldl({[], 1}, fn x, {word, add} -> {word ++ [rem(x + add - ?a,?z - ?a + 1) + ?a], if x + add > ?z do 1 else 0 end}  end)
		Enum.reverse(word)
	end

	def valid(word) do
		word = List.to_string(word)
		not String.match?(word, ~r/o|i|l/)
		and String.match?(word, ~r/([a-z])\1.*([a-z])\2/)
		and String.match?(word, ~r/(?:abc)|(?:bcd)|(?:cde)|(?:def)|(?:efg)|(?:fgh)|(?:pqr)|(?:qrs)|(?:rst)|(?:stu)|(?:tuv)|(?:uvw)|(?:vwx)|(?:wxy)|(?:xyz)/)
	end
end

if {:ok, content} = File.read("INPUT") do
	password = String.trim(content) |> String.to_charlist()
	password = Stream.iterate(1, fn x -> x + 1 end)
	|> Enum.reduce_while(password, fn _, acc -> if Word.valid(acc) do {:halt, acc} else {:cont, Word.shift(acc)} end end)
	IO.puts("Part 1: #{password}")
	password = Stream.iterate(1, fn x -> x + 1 end)
	|> Enum.reduce_while(Word.shift(password), fn _, acc -> if Word.valid(acc) do {:halt, acc} else {:cont, Word.shift(acc)} end end)
	IO.puts("Part 2: #{password}")
end
