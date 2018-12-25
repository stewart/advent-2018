defmodule Day18 do
  def run do
    state = read_input()

    IO.puts "Part 1 #{part1(state)}"
    IO.puts "Part 2 #{part2(state)}"
  end

  def part1(state) do
    state
    |> pass_minutes(10)
    |> resource_value()
  end

  def part2(state) do
    count = 1_000_000_000

    # detect cycled state, at which minute the cycle starts, and the cycle length
    {state, start, cycle} = find_cycle(state, count)

    state
    # based on previous info, only compute remainder of last cycle
    |> pass_minutes(rem(count - start, cycle - start))
    |> resource_value()
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/18")
    |> String.split("\n")
    |> Enum.map(&String.trim/1)
    |> Enum.with_index()
    |> Enum.reduce(%{}, fn {line, y}, acc ->
      line
      |> String.graphemes()
      |> Enum.with_index()
      |> Enum.reduce(acc, fn {char, x}, acc ->
        Map.put(acc, {x, y}, case char do
          "." -> :open
          "#" -> :lumberyard
          "|" -> :trees
          _ -> raise "unexpected: #{char}"
        end)
      end)
    end)
  end

  def neighbours(map, {x, y}) do
    [
      {x - 1, y - 1}, {x, y - 1}, {x + 1, y - 1},
      {x - 1, y},                 {x + 1, y},
      {x - 1, y + 1}, {x, y + 1}, {x + 1, y + 1}
    ]
    |> Enum.map(fn xy -> {xy, Map.get(map, xy)} end)
    |> Enum.filter(fn {_, value} -> value != nil end)
  end

  def pass_minutes(state, 0), do: state
  def pass_minutes(state, n) do
    state
    |> tick()
    |> pass_minutes(n - 1)
  end

  def resource_value(state) do
    counts =
      state
      |> Map.values()
      |> Enum.reduce(%{}, fn type, acc ->
        Map.update(acc, type, 1, &(&1 + 1))
      end)

    Map.fetch!(counts, :trees) * Map.fetch!(counts, :lumberyard)
  end

  def find_cycle(state, count) do
    1..count
    |> Enum.reduce_while({state, %{state => 0}}, fn minute, {prev_state, acc} ->
      new_state = tick(prev_state)
      if Map.has_key?(acc, new_state) do
        {:halt, {new_state, Map.get(acc, new_state), minute}}
      else
        {:cont, {new_state, Map.put(acc, new_state, minute)}}
      end
    end)
  end

  def tick(state) do
    for {xy, contents} <- state, into: %{} do
      siblings =
        neighbours(state, xy)
        |> Enum.map(&elem(&1, 1))

      new_contents =
        case contents do
          :open ->
            if Enum.count(siblings, &(&1 == :trees)) >= 3 do
              :trees
            else
              :open
            end

          :trees ->
            if Enum.count(siblings, &(&1 == :lumberyard)) >= 3 do
              :lumberyard
            else
              :trees
            end

          :lumberyard ->
            if Enum.count(siblings, &(&1 == :lumberyard)) >= 1 &&
              Enum.count(siblings, &(&1 == :trees)) >= 1 do
              :lumberyard
            else
              :open
            end
        end

      {xy, new_contents}
    end
  end
end
