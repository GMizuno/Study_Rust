# Capitulo 5

Nesse capítulo será abordado um dos capítulo mais abstratos e difíceis desse material de estudo.

- _Generics_
- _Options_
- _Results_

---

## _Generics_

Sabemos que em Rust é necessário tipos "concretos" para atribuir variáveis, ter input/outputs de funções. Porém, fica o seguinte questionamento, se quisermos ter um função que 'printar' uma string OU um número como podemos fazer isso?
Para resolver esse problema podemos usar _generics_ para isso. _Generics_ basicamente significam "talvez um tipo, talvez outro tipo".

Com generics, você usa colchetes angulares com o tipo dentro, assim: `<T>` Isso significa "qualquer tipo que você colocar na função". Os programadores Rust geralmente usam uma letra maiúscula para generics (T, U, V, etc.), mas o nome não importa, e você não precisa usar apenas uma letra. A única parte que importa são os colchetes angulares: _<>_.

Exemplo 1 - Criando um _Generic_

```rust
fn return_item<T>(item: T) -> T {
    println!("Here is your item.");
    // println!("Here is your item {}.", item); se fizer isso teremos um erro pois o compilador não sabe que T tem Display
    item
}
fn main() {
    let item = return_item(5);
    println!("Here is your item {}.", item);
    let item = return_item(5.5);
    println!("Here is your item {}.", item);
}
```

Exemplo 2 - Criando _Generic_ com erro

```rust
fn return_item<T>(item: T) -> T {
    println!("Here is your item {}.", item);
    item
}
fn main() {
    let item = return_item(5.5);
    println!("Here is your item {}.", item);
}
```

Para resolver o erro do Exempl 1 basta remover `println!` da linha, mas se tivermos que realizar alguma operação em _item_, como podemos dizer para compilador que T pode realizar essa operação? Para resolver isso podemos fazer o seguinte

Exemplo 3 - Resolvendo problema do Exemplo 2

```rust
use std::fmt::Debug;

fn print_item<T: Debug>(item: T) {
    println!("Here is your item: {item:?}")
}

fn main() {
    let console = return_item("Nintendo");
    let ano = return_item(10);
}
```

Agora o compilador sabe: "Ok, este tipo T vai ter Debug". Agora, o código funciona porque i32 tem Debug. Agora, podemos fornecer muitos tipos: String, &str e assim por diante, porque todos têm Debug.

## _Options_ e _Results_

De forma de direta e simples podemos dizer que _Option_ como um tipo "para quando você pode obter um valor, mas talvez não", e _Result_ como um tipo "para quando uma operação pode ter sucesso, mas talvez não". Será mais fácil explicar esses dois conceitos usando exemplos.

Exemplo 1 - Acessando index 5 de um vetor com 2 entradas

```rust
fn take_fifth_item(value: Vec<i32>) -> i32 {
    value[4]
}
fn main() {
    let new_vec = vec![1, 2];
    let index = take_fifth_item(new_vec);
}
```

Ajustando para corrigir o _Panic_ que será levantado

```rust
fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
} }

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    println!("{:?}, {:?}", try_take_fifth(small), try_take_fifth(big));
}
```

Agora não teremos _Panic_, mas se quiser realizar operções específicas com _None_ e _Som_? Nesse caso basta usar o bom e velho _match_

Exemplo 2 - Pattern Match com Option

```rust
fn try_take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}

fn handle_options(my_option: &Vec<Option<i32>>) {
    for item in my_option {
        match item {
            Some(number) => println!("Found a {number}!"),
            None => println!("Found a None!"),
        }
    }
}

fn main() {
    let small = vec![1, 2];
    let big = vec![1, 2, 3, 4, 5];
    let mut option_vec = Vec::new();
    option_vec.push(try_take_fifth(small));
    option_vec.push(try_take_fifth(big));
    handle_options(&option_vec);
}
```

Exemplo 3 - Result + Pattern Match

Nesse exemploe será mais direto, pois vamos juntar um caso de uso do _Result_ com Pattern Match.

```rust
fn see_if_number_is_even(input: i32) -> Result<(), ()> {
    if input % 2 == 0 {
        return Ok(())
    } else {
        return Err(())
    }
}

fn check_if_five(number: i32) -> Result<i32, String> {
    match number {
        5 => Ok(number),
        _ => Err(format!("Sorry, bad number. Expected: 5 Got: {number}")),
    }
}

fn main() {
    if see_if_number_is_even(5).is_ok() {
        println!("It's okay, guys")
    } else {
        println!("It's an error, guys")
    }

    for number in 4..=7 {
        println!("{:?}", check_if_five(number));
    }
}
```

## Resumindo

- Generics permitem que você use mais de um tipo em seus tipos ou funções. Sem eles, você precisaria repetir seu código toda vez que quisesse um tipo diferente.
- Você pode escrever qualquer coisa para tipos genéricos, mas na maioria das vezes as pessoas apenas escreverão T.
- Após T, você escreve quais traits o tipo terá. Ter mais traits significa que T pode fazer mais coisas. Mas também significa que a função pode aceitar menos tipos porque qualquer tipo precisa de todos os traits que você escreveu.
- Se você tem uma função que pode causar pânico, tente transformar sua saída em um Option ou um Result. Fazendo isso, você pode escrever código que nunca falha.
- Não se esqueça de que o valor Err de um Result não precisa ser um erro oficial! Se você ainda está aprendendo Rust, retornar uma String para o valor Err é mais fácil.
