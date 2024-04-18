# Capitulo 3

Nesse capítulo aprofundaremos os estudos sobre tipo, aprederemos sobre _Array_, _Vector_ e _Tuple_. Além desses tópicos, iniciaremos as discussões sobre controle de fluxo (_Control Flow_).

- Array
- Vector
- Tuple
- Control Flow
- Match, if/else, Loop, While

## Colections

_Collections_ são uma estrutura de dados que geralmente são usadas quando você tem mais de um valor e deseja armazená-los em um único lugar com algum tipo de ordem, são equivalentes a _Lists_ em _Python_.
Existem diversas _collection_ em Rust, as mais conhecidas são _Array_, _Vector_ e _Tuple_, já _Hash_ e _BinaryHeap_ são outros tipos de _collections_ mais complexos em Rust como )

## Array

Em Rust, um array é uma estrutura de dados que armazena uma _collection_ de **elementos do mesmo tipo**. Essa estrutura tem um tamanho fixo (são armazenados na _stack_) que é determinado em tempo de compilação e não pode ser alterado durante a execução do programa.

```rust
let my_array_integer = [1, 2, 3, 4, 5]; // [i32; 5]
let my_array_string = ["1", "Two", "3"]; // [&str; 3]

println!("my_array_integer {:?}, my_array_string {:?}", my_array_integer, my_array_string)

let my_array_string2 = ["a"; 5];
println!("My Array {:?}", my_array_string2);
```

Dentre as principais operações envolvendo _Array_ temos _Slice_, _len_, _iter_ (que será explicado em outra seção).

Exemplo 1 - Index e Len

```rust
let my_array_integer = [1, 2, 3, 4, 5];
println!(
    "my_array_integer index 0 {:?}, my_array_integer tem tamanho {}",
    my_array_integer[0],
    my_array_integer.len()
)
```

Exemplo 2 - Slicing

```rust

let array_of_ten = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
let two_to_five = &array_of_ten[2..5]; // Acesando do index 2 ate 4
let start_at_one = &array_of_ten[1..]; // Acessando do index 1 ate final
let end_at_five = &array_of_ten[..5]; // Acessando do index 0 ate 4
let everything = &array_of_ten[..]; // acessando tudo
let two_to_five_inclusive = &array_of_ten[2..=5]; // Acessando do index 2 ate 5

println!("Two to five: {two_to_five:?}");
println!("Start at one: {start_at_one:?}");
println!("End at five: {end_at_five:?}");
println!("Everything: {everything:?}");
println!("Two to five (inclusive): {two_to_five_inclusive:?}");
```

## Vector
