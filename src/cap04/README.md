# Capitulo 4

Nesse capítulo será explicado como criar tipos (_Struct_ ou _Enum_) customizadas e como implementar _métodos_ (parecido com o que temos em _Python_). Por fim, juntaremos alguns conceitos que foram tratados na [seção 03](../cap03/README.md).

- Enums e Struct
- Implementando Struct e Enums
- Destructuring e Operador _Dot_

---

## Enums e Structs

_Structs_ e _enums_ têm uma sintaxe semelhante, elas também funcionam juntas porque _structs_ podem conter _enums_, e _enums_ podem conter _structs_. Como elas se parecem, às vezes os usuários novos em Rust as confundem. Entretanto, aqui está uma regra prática para seguir: **se você tem muitas coisas para agrupar, isso é uma _struct_, mas se você tem muitas escolhas e precisa selecionar uma, isso é uma _enum_.**

Exemplo 1 - Diferença entre _Enum_ e _Struct_

```rust
struct FileDirectroy; // unit Struct

enum Climate {
    Tropical,
    Dry,
    Temperate,
    Continental,
    Polar,
}
```

No exemplo FileDirectroy possi várias características (como por exemplo, listar arquivos e listar informações dos arquivos ), já Climate não possui características e sim opções. Em particular, FileDirectory é uma _unit Struct_ pois para cria-la somente é necessário escreve seu nome, ou seja, sem incluir atributos. Em Rust, existe outros dois tipos de _Structs_, _Tuple Structs_ e _Named Structs_.

Exemplo 2 - _Tuple Structs_ e _Named Structs_

```rust

struct ColorRgb(u8, u8, u8); // tuple struct

struct SizeAndColor { // name struct
    size: u32,
    color: ColorRgb,
}

```

Exemplo 3 - Operando com Enums

```rust

enum ThingsInTheSky {
    Sun,
    Stars,
}
fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun,
        _ => ThingsInTheSky::Stars,
    }
}
fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun => println!("I can see the sun!"),
        ThingsInTheSky::Stars => println!("I can see the stars!"),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    let skystate2 = create_skystate(22);
    check_skystate(&skystate);
    check_skystate(&skystate2);
}

```

Outro aspecto importante e prático em Rust sobre _enums_ é que se um _Enum_ não contém nenhum dado, então suas "escolhas" podem ser convertidas em um inteiro. Isso acontece porque o Rust atribui a cada variante dessas enums simples um número que começa com 0 para seu próprio uso.

```rust
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}
fn main() {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter, Summer];
    for season in four_seasons {
        println!("{}", season as u32);
    }
}
```

OBS: É ncessário colocar use Season::\* pois estamos usando as "escolhas" de _Season_. Caso não queria fazer isso será necessário trocar `let four_seasons = vec![Spring, Summer, Autumn, Winter];` por `let four_seasons = vec![Season::Spring, Season::Summer, Season::Autumn, Season::Winter];`

## Implementando Struct e Enums

Para escrever funções para uma struct ou uma enum, use a palavra-chave _impl_ e, em seguida, um escopo com `{}` para escrever as funções. Essas funções são chamadas de métodos. Existem dois tipos de métodos em um bloco _impl_:

- Métodos: Estes recebem self de alguma forma (_&self_ ou _&mut self_ ou _self_). Métodos regulares usam um ponto .. .clone() é um exemplo de um método regular.
- _Associated functions_ (conhecidas como _static methods_ em algumas linguagens): Estas não recebem _self_. _static methods_ são chamadas de forma diferente, digitando :: entre o nome do tipo e o nome da função. String::from() é uma função associada, e o mesmo acontece com Vec::new(). **Você vê funções associadas mais frequentemente usadas para criar novas variáveis.**
