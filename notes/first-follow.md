# Linguagem Original

    <E> -> <E> + <T> | <E> - <T> | <T>

    <T> -> <T>*<P> | <T>/<P> | <P>

    <P> -> <P>^<F> | exp[<F>] | <F>

    <F> -> (<E>) | id

# Remoção da recursão à esquerda

     <E> -> <E> + <T> | <E> - <T> | <T>

     E -> TE'
     E' -> + TE' | - TE' | ε

     <T> -> <T>*<P> | <T>/<P> | <P>

     T -> PT'
     T' -> *PT' | /PT' | ε
    
     <P> -> <P>^<F> | exp[<F>] | <F>

     P -> FP' | exp[F]
     P' -> ^P' | ε


# First e Follow

    First (E) = {} U First(T) = { exp, (, id }
    First (T) = {} U First(P) = { exp, (, id }
    First (P) = { exp } U First(F) = { exp, (, id }
    First (F) = { (, id }
    First (E') = { +, -, ε }
    First (T') = { *, /, ε }
    First (P') = { ^, ε }

    Follow (E) = { ), $ }
    Follow (T) = { } U First(E') = { +, -, $ }
    Follow (P) = { } U First(T') = { *, /, $ }
    Follow (F) = { ] } U First(P') = { ], ^, $ }
    Follow (E') = { } U Follow(E) = { ), $ }
    Follow (T') = { } U Follow(T) = { +, -, $ }
    Follow (P') = { } U Follow(P) = { *, /, $ }
