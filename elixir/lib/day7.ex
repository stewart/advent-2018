defmodule Day7 do
  @regex ~r/Step (\w+) must be finished before step (\w+) can begin./

  def run do
    input = read_input()

    IO.puts "Part 1 #{part1(input)}"
    IO.puts "Part 2 #{part2(input)}"
  end

  def part1({nodes, edges}) do
    digraph = :digraph.new()

    for n <- nodes, do: :digraph.add_vertex(digraph, n)
    for {x, y} <- edges, do: :digraph.add_edge(digraph, y, x)

    digraph
  end

  def walk(digraph, node \\ nil, acc \\ [])
  def walk(digraph, nil, []) do
    [node | _] = :digraph_utils.postorder(digraph)
    walk(digraph, node, [node])
  end
  def walk(digraph, node, acc) do
    neighbours = :digraph.in_neighbours(digraph, node)
    :digraph.del_vertex(digraph, node)

    IO.puts("Walking, step #{node}, neighbours: #{neighbours |> Enum.sort}")

    case neighbours |> Enum.sort do
      [] -> acc
      [node | _] -> walk(digraph, node, [node | acc])
    end
  end

  def part2(_input) do
    0
  end

  def read_input do
    input = File.read!("#{__DIR__}/../../data/07")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&Regex.run(@regex, &1))
    |> Enum.map(&tl/1)

    {input |> List.flatten() |> Enum.sort() |> Enum.dedup(), Enum.map(input, &List.to_tuple/1)}
  end
end
