defmodule Day1 do
  @moduledoc """
  Documentation for Day1.
  """

  def part1 do
    input()
    |> Enum.sum
  end

  def part2 do
    input()
    |> Stream.cycle()
    |> Enum.reduce_while({0, MapSet.new()}, fn modulation, {frequency, set} ->
      new_frequency = frequency + modulation

      if MapSet.member?(set, new_frequency) do
        {:halt, new_frequency}
      else
        {:cont, {new_frequency, MapSet.put(set, new_frequency)}}
      end
    end)
  end

  def input do
    File.read!("#{__DIR__}/../../data/01")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
  end
end
