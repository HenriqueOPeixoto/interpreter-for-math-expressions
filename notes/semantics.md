# Analisador Semântico

## Definição dirigida pela sintaxe
    
    E -> TE'
    E' -> +TE' 
    E' -> -TE' 
    E' -> ε
    T -> PT'
    T' -> *PT' 
    T' -> /PT' 
    T' -> ε
    P -> FP' 
    P -> exp[F]
    P' -> ^FP' 
    P' ->  ε
    F -> (E) 
    F -> id

    E -> T{E'.inh = T.val} E'{E.val = E'.syn}
    E' -> +T{E'.inh = T.val + E'.inh} E'{E'.syn = E'.inh} 
    E' -> -T{E'.inh = T.val - E'.inh} E'{E'.syn = E'.inh} 
    E' -> ε {E'.syn = E'.inh}
    T -> P{T'inh = P.syn} T'{T.val = T'.inh}
    T' -> *P{T'.inh = P.val * T'.inh} T'{T'.syn = T'.inh} 
    T' -> /P{T'.inh = P.val / T'.inh} T'{T'.syn = T'.inh}
    T' -> ε {T'.syn = T'.inh}
    P -> F{P.val = F.val} P'{P'.inh = P.val}
    P -> exp[F{P.syn = exp(F.val)}]
    P' -> ^F{P'.inh = P.val ^ P'.inh} P'{P'.syn = P'.inh}
    P' ->  ε {P'.syn = P'.inh}
    F -> (E) {F.val = E.val}
    F -> id {F.val = id.lexval}
    