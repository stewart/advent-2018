defmodule Day2 do
  def run do
    input =
      File.read!("#{__DIR__}/../data/02")
      |> String.trim()
      |> String.split("\n");

    IO.puts "Part 1 #{inspect(part1(input))}"
    IO.puts "Part 2 #{inspect(part2(input))}"
  end

  def part1(input) do
    0
  end

  def part2(input) do
    0
  end
end

Day2.run
