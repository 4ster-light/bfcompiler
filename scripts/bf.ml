let max_prog_size = 30000

exception Memory_out_of_bounds
exception Unmatched_bracket

let check_bounds ptr array =
  if ptr < 0 || ptr >= Array.length array then
    raise Memory_out_of_bounds

let find_matching_brackets code =
  let brackets = Array.make (String.length code) (-1) in
  let stack = ref [] in
  for i = 0 to String.length code - 1 do
    match code.[i] with
    | '[' -> stack := i :: !stack
    | ']' -> (match !stack with
             | open_pos :: rest ->
                 brackets.(open_pos) <- i;
                 brackets.(i) <- open_pos;
                 stack := rest
             | [] -> raise Unmatched_bracket)
    | _ -> ()
  done;
  if !stack <> [] then raise Unmatched_bracket;
  brackets

let interpret_bf code =
  let array = Array.make max_prog_size 0 in
  let brackets = find_matching_brackets code in
  let rec loop ptr code_ptr =
    if code_ptr >= String.length code then ()
    else begin
      check_bounds ptr array;
      match code.[code_ptr] with
      | '+' -> array.(ptr) <- array.(ptr) + 1; loop ptr (code_ptr + 1)
      | '-' -> array.(ptr) <- array.(ptr) - 1; loop ptr (code_ptr + 1)
      | '<' -> loop (ptr - 1) (code_ptr + 1)
      | '>' -> loop (ptr + 1) (code_ptr + 1)
      | ',' -> array.(ptr) <- Char.code (input_char stdin); loop ptr (code_ptr + 1)
      | '.' -> output_char stdout (Char.chr array.(ptr)); flush stdout; loop ptr (code_ptr + 1)
      | '[' ->
          if array.(ptr) = 0 then loop ptr (brackets.(code_ptr) + 1)
          else loop ptr (code_ptr + 1)
      | ']' ->
          if array.(ptr) <> 0 then loop ptr brackets.(code_ptr)
          else loop ptr (code_ptr + 1)
      | _ -> loop ptr (code_ptr + 1)
    end
  in
  loop 0 0

let () =
  if Array.length Sys.argv < 2 then Printf.eprintf "Usage: %s <filename>\n" Sys.argv.(0)
  else try interpret_bf (
    In_channel.with_open_text Sys.argv.(1) 
    (fun ic -> In_channel.input_all ic)
  ) with
  | Sys_error msg -> Printf.eprintf "Error: %s\n" msg
  | Memory_out_of_bounds -> Printf.eprintf "Error: Memory access out of bounds\n"
  | Unmatched_bracket -> Printf.eprintf "Error: Unmatched bracket\n"
