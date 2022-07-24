# Compiler for Math Expressions

A compiler I developed for Compilers class in UFMT, considering the following language:

    <E> -> <E> + <T> | <E> - <T> | <T>

    <T> -> <T>*<P> | <T>/<P> | <P>

    <P> -> <P>^<F> | exp[<F>] | <F>

    <F> -> (<E>) | id

It is currently capable of:

* Lexical Scanning
* Syntax Parsing
* Evaluation of expressions

## How to use:
In order to run the program, run the binary and give a filename as parameter. The file is the input containg the code.

    ./my_compiler input.txt

If you have cargo installed, you can use:

    cargo r --release input.txt

The compiler will perform a syntax parsing and then attempt to convert infix notation from input file to postfix. Once the conversion is done it will evaluate the expressions and return the answer for each one.

## Input file example:

    1+2*3^4
    (1+2)*3^4
    1+(2*3)^4
    1+(2*3)+exp[4]

## How to build:
With cargo installed, run:
    
    cargo build

If you want the optimized binary, run:

    cargo build --release

## Other information:
The `notes/` directory contains the following files:

* `first-follow.md`: Contains information about first-follow sets of the language. It also shows how left recursion has been removed;
* `Tabela M do Projeto Prático.pdf`: Parse table. It can also be found in the code;
* `semantics.md`: Information used to build the syntax directed definition (SDD). This approach was not implemented in code, since postfix notation already allowed me to compute the result values.
