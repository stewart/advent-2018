defmodule Day6 do
  def run do
    input = read_input()

    IO.puts "Part 1 #{part1(input)}"
    IO.puts "Part 2 #{part2(input)}"
  end

  def part1(_input) do
    0
  end

  def part2(_input) do
    0
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/06")
    |> String.trim()
  end
end
