# Capitulo 1

Nesse capitulo será discutido sobre os conceitos básicos sobre Rust. Como por exemplo;

- Tipos Primitivos (inteiros, float, char e string)
- Type Inference
- Display vs Debug
- Mutabiity
- Algumas funções básicas

## Tipo Primitivos

Em Rust existe algum tipo muito básicos, chamados de Tipos Primitivos. Dentre eles temos os seguintes

- Inteiros com sinal (_i8_, _i16_, _i32_, _i64_, _i128_ e isize)
- Inteiros sem sinal (_u8_, _i16_, _u32_, _u64_, _u128_ e usize)
- Char
- Strig e str

### Inteiros

No momento em que escreve esse material isize equivale ao _i32_ e usize equivale eao _u32_.

Os intervalos para Inteiros são

- _i8_ => Menor valor para _i8_: **-128** maior valor para _i8_: **127**
- _i16_ => Menor valor para _i16_: **32768** maior valor para _i16_: **32767**
- _i32_ => Menor valor para _i32_: **-2147483648** maior valor para _i32_: **2147483647**
- _i64_ => Menor valor para _i64_: **-9223372036854775808** maior valor para _i64_: **9223372036854775807**
- _i128_ => Menor valor para _i128_: **-170141183460469231731687303715884105728** maior valor para _i128_: **170141183460469231731687303715884105727**

- _u8_ => Menor valor para _u8_: **0** maior valor para _u8_: **255**
- _u16_ => Menor valor para _u16_: **0** maior valor para _u16_: **65535**
- _u32_ => Menor valor para _u32_: **0** maior valor para _u32_: **4294967295**
- _u64_ => Menor valor para _u64_: **0** maior valor para _u64_: **18446744073709551615**
- _u128_ => Menor valor para _u128_: **0** maior valor para _u128_: **340282366920938463463374607431768211455**

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

- i8 ou u8 => **1 bytes**
- i16 ou u16 => **2 bytes**
- i32 ou u32 => **4 bytes**
- i64 ou u64 => **8 bytes**
- i128 ou u128 => **16 bytes**

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
let string = "A ä 国";

println!("regular_string {}", regular_string);
println!("regular_space {}", regular_space);
println!("germany_string {}", germany_string);
println!("koren_string {}", koren_string)
println!("string {}", string);
```

Para descobrir quantos bytes uma string ocupada basta usar método len.

Exemplo 3 (Método len)

```rust
let regular_string = "A";
let regular_space = " ";
let germany_string = "ä";
let koren_string = "国";
let string = "A ä 国";

println!("regular_string {}, com tamanho {}", regular_string, regular_string.len());
println!("regular_space {}, com tamanho {}", regular_space, regular_space.len());
println!("germany_string {}, com tamanho {}", germany_string, germany_string.len());
println!("koren_string {}, com tamanho {}", koren_string, koren_string.len());
println!("string {}, com tamanho {}", string, string.len());
```

### Float

São números decimais, mas diferente dos inteiros somente existe dois tipos de floats _f32_ e _f64_ que ocupam **4 bytes** e **8 bytes**, respectivamente . Seus intervalos são;

- _f32_ => Menor valor para _f32_: **-3,402823 x 10+38** maior valor para _f32_: **3,402823 x 10+38**
- _f64_ => Menor valor para _f64_: **-1,797693 x 10+308** maior valor para _f64_: ** 1,797693 x 10+308**

```rust
println!("The smallest f32: {} The biggest f32: {}", f32::MIN, f32::MAX);
println!("The smallest f64: {} The biggest f64: {}", f64::MIN, f64::MAX);

println!("Size of a f32: {} bytes", std::mem::size_of::<f32>());
println!("Size of a f64: {} bytes", std::mem::size_of::<f64>());
```

### Type Inference

O Type Inference em Rust é uma característica que permite ao compilador deduzir automaticamente o tipo de uma variável com base no contexto em que é usada, sem a necessidade de especificar explicitamente o tipo.

```rust
fn soma(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let x = 5; // o compilador infere que x é do tipo i32
    let y = 5.2; // o compilador infere que y é do tipo f64
    let z = true; // o compilador infere que z é do tipo bool
    let vetor = vec![1, 2, 3, 4, 5];

    let resultado = soma(3, 5); // o compilador infere que resultado é do tipo i3
}
```

### Display vs Debug

Em Rust alguns tipos são possuem o clássico _print_ (ou seja, não podem ser printadas). Para os tipos primitivos possuem a macro _println!_ que funcionará de forma similar ao _print_ em _Python_. Porém, para tipos mais complexo essa marco pode não funcionar, mas é possível criar implementar alguns _Traits_ para contornar esse problema.

```rust
#[derive(Debug)] // Derivando Debug para habilitar a formatação de depuração automaticamente
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("Gabriel"),
        idade: 30,
    };

    let vetor = vec![1, 2, 3, 4, 5];

    println!("{:?}", pessoa); // Saída de depuração usando Debug
    println!("{}", pessoa.nome); // Saída de depuração usando Debug
    println!("{}", pessoa.idade); // Saída de depuração usando Debug
    println!("{:?}", vetor); // Vec não possue Display, mas possue Debug
    // println!("{}", vetor); => Da erro
}
```

OBS: Existe o {:#?} que pode ser entendido com pretty-print

```rust
#[derive(Debug)] // Derivando Debug para habilitar a formatação de depuração automaticamente
struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("Gabriel"),
        idade: 30,
    };

    println!("{:?}", pessoa);
    println!("{:#?}", pessoa);
}
```

De forma resumida

- {} — Display print. É usado para formatar um valor para exibição de usuário. Ele é mais voltado para apresentação amigável para humanos. Mais tipos têm Debug do que Display, então se um tipo que você deseja imprimir não puder imprimir com Display, você pode tentar Debug.
- {:?}—Debug print. É usado para formatar um valor para a saída de depuração. Isso é útil para exibir o valor de uma variável durante a depuração do código
- {:#?}—Debug print, mas bonito. Bonito significa que cada parte de um tipo é impressa em sua própria linha para facilitar a leitura.
