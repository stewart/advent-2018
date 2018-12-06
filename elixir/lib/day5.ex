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

  # What is the length of the shortest polymer you can produce by removing all
  # units of exactly one type and fully reacting the result?
  def part2(input) do
    chars = for ch <- ?a..?z, do: <<ch>>

    chars
    |> Enum.map(fn char ->
      input
      |> Enum.reject(fn i -> String.downcase(i) == char end)
      |> part1()
    end)
    |> Enum.min
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/05")
    |> String.trim()
    |> String.graphemes()
  end

  def react(current, [prev | rest] = acc) do
    if current != prev && String.downcase(current) == String.downcase(prev) do
      rest
    else
      [current | acc]
    end
  end
  def react(current, []), do: [current]
end
