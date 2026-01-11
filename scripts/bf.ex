defmodule Brainfuck do
  @max_memory_size 30_000

  def interpret(filename) do
    with {:ok, content} <- File.read(filename) do
      run(content, %{0 => 0}, 0, 0, find_matching_brackets(content))
    else
      {:error, reason} -> {:error, "Failed to read file: #{reason}"}
    end
  end

  defp run(code, memory, ptr, code_ptr, brackets) do
    if code_ptr >= String.length(code) do
      {:ok, "Program terminated successfully"}
    else
      char = code |> String.at(code_ptr)

      case char do
        "+" ->
          new_memory = Map.update(memory, ptr, 1, fn v -> rem(v + 1, 256) end)
          run(code, new_memory, ptr, code_ptr + 1, brackets)

        "-" ->
          new_memory = Map.update(memory, ptr, -1, fn v -> rem(v - 1 + 256, 256) end)
          run(code, new_memory, ptr, code_ptr + 1, brackets)

        "<" ->
          new_ptr = max(ptr - 1, 0)
          run(code, memory, new_ptr, code_ptr + 1, brackets)

        ">" ->
          if ptr + 1 >= @max_memory_size do
            {:error, "Memory access out of bounds"}
          else
            run(code, memory, ptr + 1, code_ptr + 1, brackets)
          end

        "," ->
          case IO.read(:stdio, 1) do
            :eof ->
              run(code, Map.put(memory, ptr, 0), ptr, code_ptr + 1, brackets)

            char when is_binary(char) ->
              byte = String.to_charlist(char) |> List.first() || 0
              run(code, Map.put(memory, ptr, byte), ptr, code_ptr + 1, brackets)

            _ ->
              run(code, Map.put(memory, ptr, 0), ptr, code_ptr + 1, brackets)
          end

        "." ->
          value = Map.get(memory, ptr, 0)
          IO.write([value])
          run(code, memory, ptr, code_ptr + 1, brackets)

        "[" ->
          if Map.get(memory, ptr, 0) == 0 do
            case Map.get(brackets, code_ptr) do
              nil -> {:error, "Unmatched '['"}
              matching_pos -> run(code, memory, ptr, matching_pos + 1, brackets)
            end
          else
            run(code, memory, ptr, code_ptr + 1, brackets)
          end

        "]" ->
          if Map.get(memory, ptr, 0) != 0 do
            case Map.get(brackets, code_ptr) do
              nil -> {:error, "Unmatched ']'"}
              matching_pos -> run(code, memory, ptr, matching_pos, brackets)
            end
          else
            run(code, memory, ptr, code_ptr + 1, brackets)
          end

        _ ->
          run(code, memory, ptr, code_ptr + 1, brackets)
      end
    end
  end

  defp find_matching_brackets(code) do
    code
    |> String.graphemes()
    |> Enum.with_index()
    |> Enum.reduce({%{}, []}, fn
      {"[", idx}, {brackets, stack} ->
        {brackets, [idx | stack]}

      {"]", idx}, {brackets, [open_idx | rest]} ->
        new_brackets = brackets |> Map.put(open_idx, idx) |> Map.put(idx, open_idx)
        {new_brackets, rest}

      {"]", _idx}, {_brackets, []} ->
        raise "Unmatched closing bracket"

      _other, acc ->
        acc
    end)
    |> then(fn {brackets, stack} ->
      if stack != [] do
        raise "Unmatched opening bracket"
      else
        brackets
      end
    end)
  end
end

args = System.argv()

if length(args) != 1 do
  IO.puts(:stderr, "Usage: elixir bf.ex <filename>")
  System.halt(1)
end

filename = args |> List.first()

case Brainfuck.interpret(filename) do
  {:ok, _} ->
    :ok

  {:error, reason} ->
    IO.puts(:stderr, "Error: #{reason}")
    System.halt(1)
end
