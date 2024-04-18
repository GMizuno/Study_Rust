# Capitulo 3

Nesse capítulo aprofundaremos os estudos sobre tipo, aprederemos sobre _Array_, _Vector e \_Tuple_. Além desses tópicos, iniciaremos as discussões sobre controle de fluxo (_Control Flow_).

- Array
- Vector
- Tuple
- Control Flow
- Match, if/else, Loop, While

## Colections

_Collections_ são uma estrutura de dados que geralmente são usadas quando você tem mais de um valor e deseja armazená-los em um único lugar com algum tipo de ordem, são equivalentes a _Lists_ em _Python_.
Existem diversas _collection_ em Rust, as mais conhecidas são _Array_, _Vector_ e _Tuple_, já _Hash_ e _BinaryHeap_ são outros tipos de _colletions_ mais complexos em Rust como )

## Array

Em Rust, um array é uma estrutura de dados que armazena uma _collection_ de **elementos do mesmo tipo**. Essa estrutura tem um tamanho fixo (são armazenados na _stack_) que é determinado em tempo de compilação e não pode ser alterado durante a execução do programa.

```rust
let my_array_integer = [1, 2, 3, 4, 5]; // [i32; 5]
let my_array_string = ["1", "Two", "3"]; // [&str; 3]

println!("my_array_integer {:?}, my_array_string {:?}", my_array_integer, my_array_string)
```

Dentre as principais operações envolvendo _Array_ temos _Slice_, _len_, _iter_ (que será explicado em outra seção).

```rust
let my_array_integer = [1, 2, 3, 4, 5];
println!(
    "my_array_integer index 0 {:?}, my_array_integer tem tamanho {}",
    my_array_integer[0],
    my_array_integer.len()
)
```

## Vector
