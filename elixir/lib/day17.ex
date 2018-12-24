defmodule Day17 do
  defmodule Reservoir do
    defstruct meters: %{}, depth: nil, flow: [{:fall, {500, 0}}]

    def fill(%__MODULE__{flow: [ ]} = state), do: state
    def fill(%__MODULE__{flow: [{:fall, {x, y}} | flow]} = state) do
      (y + 1)..state.depth
      |> Enum.find(fn lower_y ->
        get(state, {x, lower_y}) in ~w[clay water]a
      end)
      |> case do
        nil ->
          new_meters =
            Enum.reduce((y + 1)..state.depth, state.meters, fn wet_y, meters ->
              Map.put(meters, {x, wet_y}, :wet)
            end)
          %__MODULE__{state | meters: new_meters, flow: flow}
        clay_y ->
          new_meters =
            Enum.reduce((y + 1)..(clay_y - 1), state.meters, fn wet_y, meters ->
              Map.put(meters, {x, wet_y}, :wet)
            end)
          %__MODULE__{
            state |
            meters: new_meters,
            flow: Enum.uniq([fill: {x, clay_y - 1}] ++ flow)
          }
      end
      |> fill
    end
    def fill(%__MODULE__{flow: [{:fill, {x, y}} | flow]} = state) do
      left_boundary =
        x
        |> Stream.iterate(fn new_x -> new_x - 1 end)
        |> Enum.find(fn new_x ->
          get(state, {new_x - 1, y}) == :clay or
          get(state, {new_x, y + 1}) in [nil, :wet]
        end)
      right_boundary =
        x
        |> Stream.iterate(fn new_x -> new_x + 1 end)
        |> Enum.find(fn new_x ->
          get(state, {new_x + 1, y}) == :clay or
          get(state, {new_x, y + 1}) in [nil, :wet]
        end)
      {spread, new_flow} =
        if get(state, {left_boundary, y + 1}) in [nil, :wet] or
          get(state, {right_boundary, y + 1}) in [nil, :wet] do
          drops =
            [left_boundary, right_boundary]
            |> Enum.filter(fn boundary_x ->
              get(state, {boundary_x, y + 1}) == nil
            end)
            |> Enum.map(fn boundary_x -> {:fall, {boundary_x, y}} end)
          {:wet, drops}
        else
          {:water, [fill: {x, y - 1}]}
        end
      new_meters =
        Enum.reduce(
          left_boundary..right_boundary,
          state.meters,
          fn new_x, meters ->
            Map.put(meters, {new_x, y}, spread)
          end
        )
      %__MODULE__{state | meters: new_meters, flow: Enum.uniq(new_flow ++ flow)}
      |> fill
    end

    def count(reservoir, types) do
      reservoir.meters
      |> Map.values
      |> Enum.count(fn meter -> meter in types end)
    end

    defp get(%__MODULE__{meters: meters}, coords), do: Map.get(meters, coords)
  end

  def run do
    meters = read_input()

    {min_y, max_y} =
      meters |> Map.keys |> Enum.map(&elem(&1, 1)) |> Enum.min_max

    reservoir =
      %Reservoir{
        meters: meters,
        depth: max_y,
        flow: [{:fall, {500, min_y - 1}}]
      }

    filled_reservoir = Reservoir.fill(reservoir)

    IO.puts "Part 1 #{Reservoir.count(filled_reservoir, [:water, :wet])}"
    IO.puts "Part 2 #{Reservoir.count(filled_reservoir, [:water])}"
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/17")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&String.split(&1, ", "))
    |> Enum.map(&Enum.sort/1)
    |> parse()
  end

  def parse(input) when is_list(input) do
    Enum.reduce(input, %{}, fn [x, y], acc ->
      for x <- parse(x), y <- parse(y), into: acc, do: {{x, y}, :clay}
    end)
  end

  def parse(input) when is_binary(input) do
    Regex.scan(~r[\d+], input)
    |> List.flatten
    |> Enum.map(&String.to_integer/1)
    |> case do
      [x, y] -> for x <- x..y, do: x
      [x] -> [x]
      [] -> raise "Huh?"
    end
  end
end
