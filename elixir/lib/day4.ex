defmodule Day4 do
  @regex ~r/^\[(\d{4}-\d{2}-\d{2}) (\d{2}:\d{2})\] (.+)$/

  def run do
    input = read_input()

    postings =
      input
      |> Enum.map(&Regex.run(@regex, &1))
      |> Enum.map(&tl(&1))
      |> Enum.map(fn [date, time, posting] ->
        time =
          time
          |> String.split(":")
          |> Enum.map(&String.to_integer/1)
          |> List.to_tuple

        {Date.from_iso8601!(date), time, parse(posting)}
      end)
      |> Enum.sort_by(fn {date, {hour, minute}, _} -> {date.month, date.day, hour, minute} end)

    IO.puts "Part 1 #{part1(postings)}"
    IO.puts "Part 2 #{part2(postings)}"
  end

  # ID of Guard with most minutes asleep,
  # Multiplied by the minute they spent asleep the most.
  def part1(postings) do
    sleep_schedule = derive_schedule(postings)

    slept_most =
      sleep_schedule
      |> Enum.max_by(fn {_, minutes} ->
        minutes |> Map.values() |> Enum.sum()
      end)
      |> elem(0)

    most_slept_minute =
      sleep_schedule
      |> Map.get(slept_most)
      |> Enum.sort_by(fn {_, count} -> count end)
      |> Enum.reverse()
      |> hd()
      |> elem(0)

    slept_most * most_slept_minute
  end

  # Guard who spent most time asleep on a single minute, multiplied by that minute.
  def part2(postings) do
    sleep_schedule = derive_schedule(postings)

    {heavy_sleeper, schedule} =
      sleep_schedule
      |> Enum.max_by(fn {_, schedule} ->
        schedule |> Map.values() |> Enum.max()
      end)

    {minute, _} = schedule |> Enum.max_by(fn {_, times} -> times end)

    heavy_sleeper * minute
  end

  def read_input do
    File.read!("#{__DIR__}/../../data/04")
    |> String.trim()
    |> String.split("\n")
  end

  defp parse("wakes up"), do: :wake_up
  defp parse("falls asleep"), do: :sleep
  defp parse("Guard #" <> str) do
    [id] = Regex.run(~r/\d+/, str)
    {:guard, String.to_integer(id)}
  end

  defp derive_schedule(postings) do
    empty_hour = 0..59 |> Enum.into(%{}, &{&1, 0})

    postings
    |> Enum.reduce({nil, 0, %{}}, fn
      {_, {_, _}, {:guard, id}}, {_, _, acc} ->
        {id, 0, Map.update(acc, id, empty_hour, fn v -> v end)}

      {_, {_, minute}, :sleep}, {id, _, acc} ->
        {id, minute, acc}

      {_, {_, done}, :wake_up}, {id, start, acc} ->
        {id, 0, Enum.reduce(start..done - 1, acc, fn minute, acc ->
          update_in(acc, [id, minute], &(&1 + 1))
        end)}
    end)
    |> elem(2)
  end
end
