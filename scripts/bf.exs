defmodule Brainfuck do
  @max_prog_size 30_000

  defmodule MemoryOutOfBounds, do: defexception(message: "Memory access out of bounds")
  defmodule UnmatchedBracket, do: defexception(message: "Unmatched bracket")

  defp check_bounds(ptr, array) do
    if ptr < 0 or ptr >= length(array), do: raise(MemoryOutOfBounds)
  end

  defp find_matching_brackets(code) do
    code
    |> String.to_charlist()
    |> Enum.with_index()
    |> Enum.reduce_while({%{}, []}, fn {char, pos}, {brackets, stack} ->
      case char do
        ?[ ->
          {:cont, {brackets, [pos | stack]}}

        ?] ->
          case stack do
            [open_pos | rest] ->
              {:cont, {Map.put(brackets, open_pos, pos) |> Map.put(pos, open_pos), rest}}

            [] ->
              {:halt, {:error, UnmatchedBracket}}
          end

        _ ->
          {:cont, {brackets, stack}}
      end
    end)
    |> case do
      {brackets, []} -> {:ok, brackets}
      {_, [_ | _]} -> {:error, UnmatchedBracket}
      {:error, ex} -> {:error, ex}
    end
  end

  def interpret_bf(code) do
    with {:ok, brackets} <- find_matching_brackets(code) do
      array = List.duplicate(0, @max_prog_size)
      loop(code, 0, 0, array, brackets)
    else
      {:error, ex} -> raise(ex)
    end
  end

  defp loop(code, ptr, code_ptr, array, brackets) do
    if code_ptr >= String.length(code) do
      :ok
    else
      check_bounds(ptr, array)

      case String.at(code, code_ptr) do
        "+" ->
          array = List.replace_at(array, ptr, Enum.at(array, ptr) + 1)
          loop(code, ptr, code_ptr + 1, array, brackets)

        "-" ->
          array = List.replace_at(array, ptr, Enum.at(array, ptr) - 1)
          loop(code, ptr, code_ptr + 1, array, brackets)

        "<" ->
          loop(code, max(ptr - 1, 0), code_ptr + 1, array, brackets)

        ">" ->
          loop(code, ptr + 1, code_ptr + 1, array, brackets)

        "," ->
          array =
            List.replace_at(array, ptr, :stdio |> IO.getn(1) |> String.to_charlist() |> hd())

          loop(code, ptr, code_ptr + 1, array, brackets)

        "." ->
          array |> Enum.at(ptr) |> List.wrap() |> List.to_string() |> IO.write()
          loop(code, ptr, code_ptr + 1, array, brackets)

        "[" ->
          if Enum.at(array, ptr) == 0 do
            loop(code, ptr, Map.get(brackets, code_ptr) + 1, array, brackets)
          else
            loop(code, ptr, code_ptr + 1, array, brackets)
          end

        "]" ->
          if Enum.at(array, ptr) != 0 do
            loop(code, ptr, Map.get(brackets, code_ptr), array, brackets)
          else
            loop(code, ptr, code_ptr + 1, array, brackets)
          end

        _ ->
          loop(code, ptr, code_ptr + 1, array, brackets)
      end
    end
  end

  def main(args) do
    case args do
      [filename] ->
        case File.read(filename) do
          {:ok, code} ->
            interpret_bf(code)

          {:error, reason} ->
            IO.puts(:stderr, "Error reading file: #{:file.format_error(reason)}")
        end

      _ ->
        IO.puts(:stderr, "Usage: elixir #{__ENV__.file} <filename>")
    end
  end
end

Brainfuck.main(System.argv())
