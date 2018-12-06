defmodule DayXX do
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

  defp read_input do
    File.read!("#{__DIR__}/../data/XX")
    |> String.trim()
    |> String.split("\n")
  end
end
