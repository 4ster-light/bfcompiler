open System
open System.IO
open System.Collections.Generic

let maxProgSize = 30000

let checkBounds ptr (array: int array) =
    if ptr < 0 || ptr >= array.Length then
        failwith "Memory access out of bounds"

let findMatchingBrackets (code: string) =
    let stack = Stack<int>()
    let brackets = Dictionary<int, int>()

    for i = 0 to code.Length - 1 do
        match code.[i] with
        | '[' -> stack.Push(i)
        | ']' ->
            if stack.Count = 0 then
                failwith "Unmatched bracket"

            let openIdx = stack.Pop()
            brackets.[openIdx] <- i
            brackets.[i] <- openIdx
        | _ -> ()

    if stack.Count > 0 then
        failwith "Unmatched bracket"

    brackets

let interpret (code: string) =
    let array = Array.zeroCreate<int> maxProgSize
    let brackets = findMatchingBrackets code

    let rec loop ptr codePtr =
        if codePtr >= code.Length then
            ()
        else
            checkBounds ptr array

            match code.[codePtr] with
            | '+' ->
                array.[ptr] <- array.[ptr] + 1
                loop ptr (codePtr + 1)
            | '-' ->
                array.[ptr] <- array.[ptr] - 1
                loop ptr (codePtr + 1)
            | '>' -> loop (ptr + 1) (codePtr + 1)
            | '<' -> loop (ptr - 1) (codePtr + 1)
            | '.' ->
                Console.Write(char array.[ptr])
                loop ptr (codePtr + 1)
            | ',' ->
                array.[ptr] <- Console.Read()
                loop ptr (codePtr + 1)
            | '[' ->
                if array.[ptr] = 0 then
                    loop ptr (brackets.[codePtr] + 1)
                else
                    loop ptr (codePtr + 1)
            | ']' ->
                if array.[ptr] <> 0 then
                    loop ptr (brackets.[codePtr])
                else
                    loop ptr (codePtr + 1)
            | _ -> loop ptr (codePtr + 1)

    loop 0 0

[<EntryPoint>]
let main argv =
    if argv.Length < 1 then
        Console.Error.WriteLine("Usage: dotnet run <filename>")
        1
    else
        try
            let code = File.ReadAllText(argv.[0])
            interpret code
            0
        with ex ->
            Console.Error.WriteLine($"Error: {ex.Message}")
            1
