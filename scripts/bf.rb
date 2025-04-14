class MemoryOutOfBounds < StandardError; end
class UnmatchedBracket < StandardError; end

def check_bounds(ptr, array)
  raise MemoryOutOfBounds, 'Memory access out of bounds' if ptr < 0 || ptr >= array.length
end

def find_matching_brackets(code)
  brackets = {}
  stack = []
  code.chars.each_with_index do |char, i|
    if char == '['
      stack.push(i)
    elsif char == ']'
      raise UnmatchedBracket, 'Unmatched closing bracket' if stack.empty?
      open_pos = stack.pop
      brackets[open_pos] = i
      brackets[i] = open_pos
    end
  end
  raise UnmatchedBracket, 'Unmatched opening bracket' unless stack.empty?
  brackets
end

def interpret_bf(code)
  max_prog_size = 30_000
  array = [0] * max_prog_size
  ptr = 0
  code_ptr = 0
  brackets = find_matching_brackets(code)

  while code_ptr < code.length
    check_bounds(ptr, array)

    case code[code_ptr]
    when '+'
      array[ptr] = (array[ptr] + 1) % 256
    when '-'
      array[ptr] = (array[ptr] - 1) % 256
    when '<'
      ptr = [0, ptr - 1].max
    when '>'
      ptr += 1
    when ','
      input = $stdin.getc || "\0"
      array[ptr] = input.ord
    when '.'
      print array[ptr].chr
      $stdout.flush
    when '['
      code_ptr = brackets[code_ptr] if array[ptr].zero?
    when ']'
      code_ptr = brackets[code_ptr] unless array[ptr].zero?
    end

    code_ptr += 1
  end
end

if __FILE__ == $PROGRAM_NAME
  if ARGV.length != 1
    warn "Usage: #{$PROGRAM_NAME} <filename>"
    exit 1
  end

  begin
    interpret_bf(File.read(ARGV[0]))
  rescue Errno::ENOENT
    warn "Error: Could not open file '#{ARGV[0]}'"
  rescue MemoryOutOfBounds, UnmatchedBracket => e
    warn "Error: #{e.message}"
  end
end
