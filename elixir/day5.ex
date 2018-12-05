defmodule Day5 do
  def run do
    input = read_input()

    IO.puts "Part 1 #{part1(input)}"
    IO.puts "Part 2 #{part2(input)}"
  end

  # How many units remain after fully reacting the polymer you scanned?
  def part1(input) do
    input
    |> List.foldr([], &react/2)
    |> Enum.count
  end

  def part2(input) do
    0
  end

  def read_input do
    File.read!("#{__DIR__}/../data/05")
    |> String.trim()
    |> String.graphemes()
  end

  defp react(current, [prev | rest] = acc) do
    if current != prev && String.downcase(current) == String.downcase(prev) do
      rest
    else
      [current | acc]
    end
  end
  defp react(current, []), do: [current]
end
