defmodule Claim do
  @regex ~r/^\#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$/

  defstruct id: nil, left: nil, top: nil, width: nil, height: nil

  def from_string(input) do
    [id, left, top, width, height] =
      Regex.run(@regex, input)
      |> tl()
      |> Enum.map(&String.to_integer/1)

    %Claim{id: id, left: left, top: top, width: width, height: height}
  end

  def squares(%Claim{left: left, top: top, width: width, height: height}) do
    for x <- left..(left + width - 1),
        y <- top..(top + height - 1), do: {x, y}
  end

  def map(claim, acc \\ %{}) do
    claim
    |> Claim.squares()
    |> Enum.reduce(acc, fn coords, map ->
      Map.update(map, coords, [claim.id], fn ids -> ids ++ [claim.id] end)
    end)
  end

  def map_all(claims) do
    claims
    |> Enum.reduce(%{}, fn claim, map ->
      claim
      |> Claim.map(map)
    end)
  end
end

defmodule Day3 do
  def run do
    input =
      File.read!("#{__DIR__}/../data/03")
      |> String.trim()
      |> String.split("\n")
      |> Enum.map(&Claim.from_string/1)

    IO.puts "Part 1 #{part1(input)}"
    IO.puts "Part 2 #{part2(input)}"
  end

  def part1(claims) do
    claims
    |> Claim.map_all()
    |> Enum.filter(fn {_, claims} -> length(claims) >= 2 end)
    |> Enum.count()
  end

  def part2(claims) do
    claimants = claims
    |> Claim.map_all()
    |> Map.values()

    claimants
    |> List.flatten()
    |> Enum.dedup()
    |> Enum.find(fn id ->
      !Enum.find(claimants, & (id in &1 && length(&1) > 1))
    end)
  end
end
