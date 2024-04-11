# Capitulo 1

Nesse capitulo será discutido sobre os conceitos básicos sobre Rust. Como por exemplo;

- Tipos Primitivos (inteiros, float, char e string)
- Type Inference
- Display vs Debug
- Mutabiity
- Algumas funções básicas

## Tipo Primitivos

Em Rust existe algum tipo muito básicos, chamados de Tipos Primitivos. Dentre eles temos os seguintes

- Inteiros com sinal (i8, i16, i32, i64, i128 e isize)
- Inteiros sem sinal (i8, i16, i32, i64, i128 e isize)
- Char
- Strig e str

### Inteiros

No momento em que escreve esse material isize equivale ao i32 e usize equivale eao u32.

Os intervalos para Inteiros são

- i8 => Menor valor para i8: -128 maior valor para i8: 127
- i16 => Menor valor para i16: 32768 maior valor para i16: 32767
- i32 => Menor valor para i32: -2147483648 maior valor para i32: 2147483647
- i64 => Menor valor para i64: -9223372036854775808 maior valor para i64: 9223372036854775807
- i128 => Menor valor para i128: -170141183460469231731687303715884105728 maior valor para i128: 170141183460469231731687303715884105727

- u8 => Menor valor para u8: 0 maior valor para u8: 255
- u16 => Menor valor para u16: 0 maior valor para u16: 65535
- u32 => Menor valor para u32: 0 maior valor para u32: 4294967295
- u64 => Menor valor para u64: 0 maior valor para u64: 18446744073709551615
- u128 => Menor valor para u128: 0 maior valor para u128: 340282366920938463463374607431768211455

Exemplo 1 (Criando inteiros e achando máximo/mínimo)

```rust
let my_number: u8 = 100;
let my_number2: i8 = -100;
println!(
        "The smallest i8: {} The biggest i8: {}", i8::MIN, i8::MAX
    )
```

Se não informar o tipo o compilador usará i32 !!!

Ao serem alocados na memória os inteiro (independentes de serem com ou sem sinal) ocupam os seguintes espaços

- i8 ou u8 => 1 bytes
- i16 ou u16 => 2 bytes
- i32 ou u32 => 4 bytes
- i64 ou u64 => 8 bytes
- i128 ou u128 => 16 bytes

Para obter essas informações, Rust fornece um função básica (built-in) chamada _size_of_ que esta no pacote _std_

```rust
println!("Size of a u8/i8: {} bytes", std::mem::size_of::<u8>());
println!("Size of a u16/i16: {} bytes", std::mem::size_of::<u16>());
println!("Size of a u32/i32: {} bytes", std::mem::size_of::<u32>());
println!("Size of a u64/i64: {} bytes", std::mem::size_of::<u64>());
println!("Size of a u128/i128: {} bytes", std::mem::size_of::<u128>());
```

### Char e Strings

Char são carácteres usando '' (e não "" como em outras linguagens) com 1 carácter. Todos os carácteres possuem **4 bytes**. Já uma string é uma sequência de char, diferente de um char que usa '' como sua representação uma string usa "" (obs: mais a frente será mostrado que existe alguns tipo de string, mas por agora isso é suficiente)

Exemplo 2 (Criando um char e string)

```rust
println!("Size of a char: {} bytes", std::mem::size_of::<char>())
let regular_char = 'A';
let space = ' ';
let germany_char = 'ä';
let koren_char = '国';

println!("{}", regular_char);
println!("{}", space);
println!("{}", germany_char);
println!("{}", koren_char);


let regular_string = "A";
let regular_space = ' ';
let germany_string = "ä";
let koren_string = "国";

println!("regular_string {}", regular_string);
println!("regular_space {}", regular_space);
println!("germany_string {}", germany_string);
println!("koren_string {}", koren_string);
```

Para descobrir quantos bytes uma string ocupada basta usar função built-in len.

Exemplo 3 (Função len)

```rust
let regular_string = "A";
let regular_space = " ";
let germany_string = "ä";
let koren_string = "国";

println!("regular_string {}, com tamanho {}", regular_string.len());
println!("regular_space {}, com tamanho {}", regular_space.len());
println!("germany_string {}, com tamanho {}", germany_string.len());
println!("koren_string {}, com tamanho {}", koren_string.len());
```
