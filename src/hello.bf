++++++++                Initialize cell 0 with 8 (ASCII for backspace)
[
    >++++               Move to cell 1 and add 4
    [
        >++>+++>+++>+   Move to cells 2 3 4 5 adding 2 3 3 1 respectively
        <<<<-           Move back to cell 1 and decrement by 1
    ]
    >+>+>->>+           Move to cells 2 3 4 5 adding 1 1 minus 1 plus 1 respectively
    [<]                 Move back to cell 0 moving left if not at cell 0
    <-                  Decrement cell 0 by 1 (loop control)
]
>>.                     Move to cell 2 and print its value (H)
>---.                   Move to cell 3 subtract 3 then print (e)
+++++++.                Move to cell 4 add 7 then print (l)
.                       Print cell 4 again (l)
+++.                    Add 3 to cell 4 then print (o)
>>.                     Move to cell 6 and print its value (space)
<-.                     Move to cell 5 subtract 1 then print ()
<.                      Move to cell 4 and print (space)
+++.                    Add 3 to cell 4 then print (W)
------.                 Subtract 6 from cell 4 then print (o)
--------.               Subtract 8 from cell 4 then print (r)
>>+.                    Move to cell 6 add 1 then print (l)
>++.                    Move to cell 7 add 2 then print (d)
                        
                        Note: The exclamation mark is printed here because after 'd' is printed 
                        the next cell (cell 8) is incremented twice from 0 to 2 which is ASCII 33 
                        the code for '!'