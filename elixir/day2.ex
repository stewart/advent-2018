defmodule Day2 do
  def run do
    input =
      File.read!("#{__DIR__}/../data/02")
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(&String.graphemes/1)

    IO.puts "Part 01: #{part1(input)}"
    IO.puts "Part 02: #{part2(input)}"
  end

  def part1(input) do
    counts =
      input
      |> Enum.map(fn chars ->
        Enum.reduce(chars, %{}, fn ch, map ->
          Map.update(map, ch, 1, &(&1 + 1))
        end)
      end)

    twos =
      counts
      |> Enum.filter(fn map -> map |> Map.values() |> Enum.any?(&(&1 == 2)) end)
      |> Enum.count()

    threes =
      counts
      |> Enum.filter(fn map -> map |> Map.values() |> Enum.any?(&(&1 == 3)) end)
      |> Enum.count()

    twos * threes
  end

  def part2(input) do
    for a <- input, b <- input do
      if Enum.filter(Enum.zip(a, b), fn {x, y} -> x != y end) |> Enum.count() == 1 do
        shared =
          Enum.zip(a, b)
          |> Enum.filter(fn {x, y} -> x == y end)
          |> Enum.map(&elem(&1, 0))
          |> Enum.join("")

        throw {:break, shared}
      end
    end
  catch
    {:break, result} -> result
  end
end

Day2.run
