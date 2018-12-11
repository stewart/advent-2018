defmodule Day10 do
  @regex ~r/\<\s*([-\d]+),\s*([-\d]+)\>/

  def run do
    input = read_input()

    {vectors, time} = calculate(input)

    IO.puts "Part 1:"
    IO.puts display(vectors)

    IO.puts "Part 2 #{time}"
  end

  def calculate(vectors) do
    Enum.reduce_while(Stream.cycle([nil]), {vectors, minmax(vectors), 0}, fn
      _, {vectors, {{min, max}, _}, time} ->
          updated = tick(vectors)
          case minmax(updated) do
            {{newmin, newmax}, _} when newmax - newmin > max - min ->
              {:halt, {vectors, time}}
            newminmax -> {:cont, {updated, newminmax, time + 1}}
          end
    end)
  end

  def display(vectors) do
    {{xmin, xmax}, {ymin, ymax}} = minmax(vectors)

    locations = Enum.group_by(vectors, fn {{x, y}, _} -> {x, y} end)

    Enum.map(ymin..ymax, fn y ->
      Enum.reduce(xmin..xmax, "", fn x, row ->
        row <> case locations[{x, y}] do
          stars when is_list(stars) -> "#"
          _ -> "."
        end
      end)
    end) |> Enum.join("\n")
  end

  def tick(vectors) when is_list(vectors), do: Enum.map(vectors, &tick/1)
  def tick({{xpos, ypos}, {xvel, yvel}}),
    do: {{xpos + xvel, ypos + yvel}, {xvel, yvel}}

  def spread(vectors) do
    {{xmin, xmax}, {ymin, ymax}} = minmax(vectors)
    {xmax - xmin, ymax - ymin}
  end

  def minmax(vectors) do
    xminmax = vectors |> Enum.map(fn {{x, _}, _} -> x end) |> Enum.min_max()
    yminmax = vectors |> Enum.map(fn {{_, y}, _} -> y end) |> Enum.min_max()
    {xminmax, yminmax}
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/10")
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(&Regex.scan(@regex, &1))
    |> Enum.map(fn [[_, xpos, ypos], [_, xvel, yvel]] ->
      [xpos, ypos, xvel, yvel]
      |> Enum.map(&String.to_integer/1)
      |> List.to_tuple()
    end)
    |> Enum.map(fn {xpos, ypos, xvel, yvel} ->
      {{xpos, ypos}, {xvel, yvel}}
    end)
  end
end
