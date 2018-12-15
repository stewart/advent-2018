defmodule Day11 do
  def run do
    IO.puts("Part 1: #{inspect(part1())}")
    IO.puts("Part 2: #{inspect(part2())}")
  end

  def part1 do
    summed_area_table(4172)
    |> most_powerful_area(3)
    |> elem(0)
    |> Tuple.to_list()
    |> Enum.take(2)
    |> List.to_tuple()
  end

  def part2 do
    table = summed_area_table(4172)

    Enum.reduce(1..300, nil, fn size, best ->
      area = most_powerful_area(table, size)
      if is_nil(best) or elem(area, 1) > elem(best, 1),
        do: area,
        else: best
    end)
    |> elem(0)
  end

  def summed_area_table(serial_number, size \\ 300) do
    Enum.reduce(1..size, %{}, fn y, table ->
      Enum.reduce(1..size, table, fn x, table ->
        Map.put(table, {x, y},
          power({x, y}, serial_number) +
            Map.get(table, {x, y - 1}, 0) +
            Map.get(table, {x - 1, y}, 0) -
            Map.get(table, {x - 1, y - 1}, 0)
        )
      end)
    end)
  end

  def most_powerful_area(table, size, table_size \\ 300) do
    offset = size - 1

    coords = for x <- 1..(table_size - offset), y <- 1..(table_size - offset),
      do: {x, y, size}

    coords
    |> Enum.map(fn {x, y, size} ->
      {
        {x, y, size},
        Map.fetch!(table, {x + offset, y + offset}) +
          Map.get(table, {x - 1, y - 1}, 0) -
          Map.get(table, {x + offset, y - 1}, 0) -
          Map.get(table, {x - 1, y + offset}, 0)
      }
    end)
    |> Enum.max_by(fn {_, value} -> value end)
  end

  def power({x, y}, serial_number) do
    rack_id = x + 10

    rack_id
    |> Kernel.*(y)
    |> Kernel.+(serial_number)
    |> Kernel.*(rack_id)
    |> div(100)
    |> rem(10)
    |> Kernel.-(5)
  end
end
