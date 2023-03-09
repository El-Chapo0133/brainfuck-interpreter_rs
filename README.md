# brainfuck-interpreter_rs
This is a simple BrainFuck interpreter<br>
It uses a code entry and user_inputs and returns a 'Result<Vec<u8>, &'static str>'

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
I did not coded that because i do not know when it means, so yea lol

# Sources
you can check some BrainFuck codes here
https://sange.fi/esoteric/brainfuck/impl/interp/i.html