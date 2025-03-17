local MAX_PROG_SIZE = 30000

---@param ptr number Pointer to check
---@param array table Memory array
local function check_bounds(ptr, array)
  if ptr >= #array then
    error("Memory access out of bounds")
  end
end

---@param bf_code string The Brainfuck code to interpret
local function interpret_bf(bf_code)
  local array = {}
  for i = 0, MAX_PROG_SIZE - 1 do array[i] = 0 end
  local ptr = 0
  local code_ptr = 1
  local loop_stack = {}

  while code_ptr <= #bf_code do
    check_bounds(ptr, array)
    local char = bf_code:sub(code_ptr, code_ptr)

    if char == "+" then
      array[ptr] = array[ptr] + 1
    elseif char == "-" then
      array[ptr] = array[ptr] - 1
    elseif char == "<" then
      ptr = math.max(0, ptr - 1)
    elseif char == ">" then
      ptr = ptr + 1
    elseif char == "," then
      local input = io.read(1)
      if input then
        array[ptr] = string.byte(input)
      end
    elseif char == "." then
      io.write(string.char(array[ptr]))
      io.flush()
    elseif char == "[" then
      if array[ptr] == 0 then
        local balance = 1
        code_ptr = code_ptr + 1
        while code_ptr <= #bf_code and balance > 0 do
          local current_char = bf_code:sub(code_ptr, code_ptr)
          if current_char == "[" then
            balance = balance + 1
          elseif current_char == "]" then
            balance = balance - 1
          end
          code_ptr = code_ptr + 1
        end
        code_ptr = code_ptr - 1
      else
        table.insert(loop_stack, code_ptr)
      end
    elseif char == "]" then
      if array[ptr] ~= 0 then
        code_ptr = loop_stack[#loop_stack]
      else
        table.remove(loop_stack)
      end
    end

    code_ptr = code_ptr + 1
  end
end

---@param filename string Name of the Brainfuck code source file
local function main(filename)
  local file = io.open(filename, "r")
  if not file then
    error("Could not open file: " .. filename)
  end
  local bf_code = file:read("*all")
  file:close()
  interpret_bf(bf_code)
end

if arg and arg[1] then
  main(arg[1])
else
  print("Usage: lua bf.lua <filename>")
end
