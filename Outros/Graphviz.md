# Graphviz
## Exemplos
###  Fluxograma simples
```dot
digraph G {
    A -> B;
    B -> C;
    C -> D;
    D -> A;
}
```
### Mudando cores
```dot
digraph G {
    A [fillcolor=white, color=black];
    B [fillcolor=black, color=white];
    A -> B;
}
```
