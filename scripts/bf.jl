const MAX_PROG_SIZE = 30000

function checkbounds(ptr::Int, array::Array{Int})::Nothing
    if ptr < 1 || ptr > length(array)
        throw(BoundsError(array, ptr))
    end
end

function findMatchingBrackets(code::String)::Dict{Int,Int}
    local stack = Int[]
    local matching = Dict{Int,Int}()

    for (i, char) in enumerate(code)
        if char == '['
            push!(stack, i)
        elseif char == ']'
            if isempty(stack)
                throw(ArgumentError("Unmatched closing bracket at position $i"))
            end
            opening = pop!(stack)
            matching[opening] = i
            matching[i] = opening
        end
    end

    if !isempty(stack)
        throw(ArgumentError("Unmatched opening bracket at position $(stack[end])"))
    end

    matching
end

function interpret(code::String)::Nothing
    local array = zeros(Int, MAX_PROG_SIZE)
    local brackets = findMatchingBrackets(code)

    function loop(ptr::Int, codePtr::Int)::Nothing
        if codePtr > length(code)
            return
        end

        checkbounds(ptr, array)

        if codePtr <= length(code)
            char = code[codePtr]

            if char == '+'
                array[ptr] += 1
                loop(ptr, codePtr + 1)
            elseif char == '-'
                array[ptr] -= 1
                loop(ptr, codePtr + 1)
            elseif char == '>'
                loop(ptr + 1, codePtr + 1)
            elseif char == '<'
                loop(ptr - 1, codePtr + 1)
            elseif char == '.'
                print(Char(array[ptr]))
                loop(ptr, codePtr + 1)
            elseif char == ','
                input = read(stdin, Char)
                array[ptr] = Int(input)
                loop(ptr, codePtr + 1)
            elseif char == '['
                if array[ptr] == 0
                    loop(ptr, brackets[codePtr] + 1)
                else
                    loop(ptr, codePtr + 1)
                end
            elseif char == ']'
                if array[ptr] != 0
                    loop(ptr, brackets[codePtr])
                else
                    loop(ptr, codePtr + 1)
                end
            else
                loop(ptr, codePtr + 1)
            end
        end
    end

    loop(1, 1)
end

if length(ARGS) < 1
    println(stderr, "Usage: julia bf.jl <filename>")
else
    try
        code = read(ARGS[1], String)
        interpret(code)
    catch ex
        println(stderr, "Error: $(ex)")
    end
end
