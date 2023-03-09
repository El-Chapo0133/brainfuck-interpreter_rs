# brainfuck-interpreter_rs
This is a simple BrainFuck interpreter<br>
It uses a code entry and user_inputs and returns a 'Result<Vec<u8>, &'static str>'
<br><br>
No deps needed, this was made from scratch

## BrainFuck codes
| code | Commands                                                  |
|------|-----------------------------------------------------------|
| >    | increment pointer                                         |
| >    | decrement pointer                                         |
| +    | increment value at pointer                                |
| -    | decrement value at pointer                                |
| [    | begin loop (continues while value at pointer is non-zero) |
| ]    | end loop                                                  |
| ,    | read one character from input into value at pointer       |
| .    | print value at pointer to output as a character           |
|      | Any other characters are ignored                          |

## Debug code
For now, you always are in debug mode<br>
It simply print a line with :
> index in code<br>
> pointer index<br>
> value at pointer<br>

# Sources
you can check some BrainFuck codes here
https://sange.fi/esoteric/brainfuck/impl/interp/i.html