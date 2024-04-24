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

Exemplo 1 - Implementando e "herdando" habilidades

```rust
#[derive(Debug)] // Herdando {:?}
enum AnimalType {
    Cat,
    Dog,
}

#[derive(Debug)] // Herdando {:?}
struct Animal {
    age: u8,
    animal_type: AnimalType,
}

impl Animal {
    // Self == Animal (Type)
    fn new_cat() -> Self {
        Self {
            age: 10,
            animal_type: AnimalType::Cat,
        }
    }

    fn check_type(&self) {
        // Acessando objeto (self) atraves de referencia
        match self.animal_type {
            AnimalType::Dog => println!("The Animal is a dog!!!!"),
            AnimalType::Cat => println!("The Animal is a cat!!!!"),
        }
    }
    fn change_to_cat(&mut self) {
        // Acessando objeto de forma mutavel
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}")
    }

    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }

    fn change_age(&mut self, age: u8) {
        self.age = age;
        println!("Changed animal age! Now it's {}", self.age.to_string());
    }

    fn check_age_and_type(&self) {
        // let t = self.animal_type; Da erro de Borrow, t "toma" o valor de self.animal_type
        println!(
            "The Animal is a {:?} with age  {}!!!!",
            self.animal_type, self.age
        );
    }
}
fn main() {
    let mut new_animal = Animal::new_cat();
    println!("Animal Type {new_animal:?}");
    new_animal.check_type();
    new_animal.check_age_and_type();
    new_animal.change_to_cat();
    new_animal.check_type();
    new_animal.change_to_dog();
    new_animal.check_type();
    new_animal.change_to_cat();
    new_animal.change_age(100);
    new_animal.check_age_and_type();
    new_animal.change_age(50);
    new_animal.check_age_and_type();
}
```

Um ponto importante de resaltar é que Self significa o tipo _Self_, e _self_ significa a variável chamada _self_ que se refere ao próprio objeto. Portanto, em nosso código, _Self_ significa o tipo Animal. Além disso, `fn change_to_dog(&mut self)` significa `fn change_to_dog(&mut Animal)`.
Por fim, fica o questinamento que ao declara que _new_animal_ é mutável podemos quer um método que modifica todos o objeto, seja a idade ou tipo. Seria esse um comportamento desejado? Existe atributos privado (parecido com Java)? Essa discussão será feita mais para frente na seção 10 (_lifetime adn interior mutabiility_)

## Destructuring e Operador _Dot_

Assim com Tuplas é possível realizar Destructuring em _Structs_

Exemplo 1 - Destructuting de Struct

```rust
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool,
}

fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false,
    };
    let Person {
        name: fake_name,
        real_name,
        height: cm,
        happiness,
    } = papa_doc;

    println!("They call him {fake_name} but his real name is {real_name}. He is {cm} cm tall and is he happy? {happiness}");
}
```

Exemplo 2 - Renomeando e Destructuring

```rust
struct Person {
    name: String,
    real_name: String,
    height: u8,
    happiness: bool
}
fn main() {
    let papa_doc = Person {
        name: "Papa Doc".to_string(),
        real_name: "Clarence".to_string(),
        height: 170,
        happiness: false
};
let Person {
    name: fake_name, // será criado uma variável fake_name
    real_name,
    height: cm,
    happiness
} = papa_doc;
    println!("They call him {fake_name} but his real name is {real_name}. He is {cm} cm tall and is he happy? {happiness}"); }
```

Já vimos que quando você tem uma referência, você precisa usar \* para chegar ao valor. No Exemplo 1 temos um erro ao compilar o código, pois _my_number_ é um número (_i32_) e _reference_ é um ponteiro (enderço de memória), ou seja, não podemos comparar um referência com um não referência. Já no segundo exemplo temos _name_ que é um _string_ e _double_ref_ que é uma referência, entretando não vamos ter nenhum erro de compilção. O fato de não termos erro de compilação decorre de o método _.is_empty()_ é para o tipo _String_, mas o chamamos em _&&String_. Isso ocorre porque quando você usa um método, Rust fará o dereferenciamento para você até atingir o tipo original. O `.` (ponto) em um método é chamado de operador _Dot_, e ele faz o dereferenciamento de graça
Exemplo 1 - Erro de compilação

```rust
let my_number = 9;
let reference = &my_number;
println!("{}", my_number == reference);
```

Exemplo 1 - Operador _Dot_

```rust
let my_name = "Billy".to_string();
let double_ref = &&my_name;
println!("{}", double_ref.is_empty());
```

## Resumindo

- Structs são um pouco como tuplas com nomes. Elas podem conter todos os tipos de dados diferentes e podem Destructuring
- Structs podem conter enums, e enums podem conter structs.
- Geralmente, após criar uma struct ou enum, você começará um bloco impl e dará a ele alguns métodos. Na maioria das vezes, eles vão usar &self ou &mut self se você precisar alterá-lo.
- Nem todos os métodos dentro de um bloco impl precisam de self: se você quiser iniciar uma nova struct ou enum, ela criará um Self e o retornará. Você até mesmo pode querer um sem self que retorne outra coisa. O compilador não se importa se você tem self dentro de um bloco impl.
- Para obter dados de dentro de uma enum, você geralmente usará match ou algo semelhante. Uma enum é sobre ter apenas uma escolha, então você tem que verificar qual foi escolhida!
