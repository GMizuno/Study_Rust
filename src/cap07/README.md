## Capitulo 7

Nesse capítulo será aborado conceito de _Trait_ desde conceitos básicos até mais avançados

- _Trait_ básico
- From _Trait_
- Regra Orphan
- AsRef

---

## _Trait_ básico

Os _traits_ em Rust são semelhantes a poderes ou qualificações para um tipo (como por exemplo _Clone_ e _Debug_ em Vetores). Quando um tipo implementa um _trait_, ele ganha novas capacidades que não tinha anteriormente (podemos associar a herança em Python e Java). Além disso, ao implementar um _trait_ em um tipo, você pode garantir ao compilador que esse tipo é capaz de realizar certas operações, independentemente do tipo específico que ele é.

Em Rust, é comum utilizar uma sintaxe especial chamada atributos (attributes) para implementar automaticamente traits como Debug, pois são tão comuns. Quando você escreve #[derive(Debug)], você está automaticamente implementando o trait Debug para a estrutura ou enumeração em questão. Portanto, tudo o que você precisa fazer para implementar esse trait é adicionar `#[derive(Debug)]`.

Exemplo 1 - Implementando _Trait_ de "forma automtica"

```rust
#[derive(Debug)]
struct MyStruct {
    number: usize,
}
```

Porém, em alguns cenários podemos querer implementar nosso proprio _trait_. Para criar um _trait_ basta usar _keyword_ _trait_ e sem seguida adicionar os métodos desejado.

Exemplo 2 - Criando _Trait_ como alguns problemas conceituais

```rust
struct Dog {
    name: String,
}

struct Parrot {
    name: String,
}

trait DogLike {
    fn bark(&self) {
        println!("Woof Woof");
    }

    fn run(&self) {
        println!("The dog is running")
    }
}

impl DogLike for Dog {}
impl DogLike for Parrot {}

fn main() {
    let brain = Parrot {
        name: "Brain".to_string(),
    };

    let rover = Dog {
        name: "Rover".to_string(),
    };

    brain.bark();
    brain.run();
}
```

O exmeplo 2 possui alguns problemas conceituais como:

Claro, esse código em Rust apresenta um problema teórico porque o trait `DogLike` está sendo implementado para estruturas que podem não ser semanticamente adequadas para todas as suas funções. Aqui está um resumo do problema:

1. **Polimorfismo Implícito**: O trait `DogLike` define métodos como `bark` e `run`, que são comportamentos associados a cães. No entanto, esses métodos estão sendo implementados para estruturas como `Parrot`, que não são cães. Isso viola o princípio do polimorfismo, que sugere que o mesmo código deve funcionar com tipos diferentes contanto que eles compartilhem um contrato comum (o trait). Neste caso, implementar `DogLike` para `Parrot` pode ser confuso e levar a comportamentos inesperados, pois um papagaio não deveria "latir" ou "correr" como um cão.

2. **Quebra de Expectativas**: Quando alguém usa uma estrutura que implementa um trait, espera-se que ela se comporte de acordo com as expectativas definidas pelo trait. No entanto, ao implementar `DogLike` para `Parrot`, estamos quebrando essa expectativa, pois os métodos `bark` e `run` não refletem o comportamento natural de um papagaio.

OBS: Esse codigo também viola o Violação do Princípio da Substituição de Liskov. Pois ao implementar o trait DogLike para Parrot, estamos introduzindo comportamentos que não são semanticamente aplicáveis a um papagaio. Isso pode levar a bugs difíceis de rastrear e entender, especialmente em programas maiores. Porém, esse não é foco desse texto

Exempo 3 - Ajustando Exemplo 2

```rust
struct Animal {
    name: String,
}
trait DogLike {
    fn bark(&self);
    fn run(&self);
}
impl DogLike for Animal {
    fn bark(&self) {
        println!("{}, stop barking!!", self.name);
    }
    fn run(&self) {
        println!("{} is running!", self.name);
    }
}
fn main() {
    let rover = Animal {
        name: "Rover".to_string(),
    };
    rover.bark();
    rover.run();
}
```

Nesse caso DogLike funciona como se fosse uma Interface (traits são frequentemente comparados a interfaces de outras linguagens. Embora não sejam exatamente o mesmo conceito, eles compartilham algumas semelhanças importantes), obrigando a _Animal_ implementar os métodos de Doglike. Além disso, poderia criar um _trait_ ParrotLike com métodos necessários.

## From _Trait_

O `From` permite converter um tipo em outro de maneira flexível e intuitiva. Você já viu o `From` em ação em muitos lugares, como ao converter um `&str` em `String`, mas ele pode ser usado para fazer muitos outros tipos de conversões.

Por exemplo, o `Vec` usa o `From` para converter 18 (!) tipos diferentes. Alguns exemplos desses tipos são `&[T]`, `&mut [T]`, `&str`, `&Vec<T>`, `[T; N]`, `BinaryHeap<T>`, `String`, `Vec<T>`, `VecDeque<T>`, entre outros.

Você pode ver essas implementações na documentação do `Vec` à esquerda (https://doc.rust-lang.org/std/vec/struct.Vec.html). Com tantas implementações de `Vec::from()`, há muitas oportunidades para explorar e experimentar.

No código de exemplo fornecido, você viu como usar o `Vec::from()` para criar um `Vec` a partir de um array `[T; N]`, de uma `String` e de um `&str`. Experimentar e observar os resultados dessas conversões pode ajudar a entender melhor como o `From` funciona e como pode ser útil em situações do mundo real.

Exemplo 1 - Usando From

```rust
let array_vec = Vec::from([8, 9, 10]);
println!("Vec from array: {array_vec:?}");
let str_vec = Vec::from("What kind of Vec am I?");
println!("Vec from str: {str_vec:?}");
let string_vec = Vec::from("What will a String be?".to_string());
println!("Vec from String: {string_vec:?}");
```

Exemplo 2 - Implementando proprio From

```rust
#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}
impl City {
    fn new(name: &str, population: u32) -> Self {
        Self {
            name: name.to_string(),
            population,
        }
    }
}
#[derive(Debug)]
struct Country {
    cities: Vec<City>,
}

impl From<Vec<City>> for Country {
    fn from(cities: Vec<City>) -> Self {
        Self { cities }
    }
}
impl Country {
    fn print_cities(&self) {
        for city in &self.cities {
            println!("city {city:?}")
        }
    }
}

fn main() {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);
    let finland_cities = vec![helsinki, turku];
    println!("Vetores de city {finland_cities:?}");
    let finland = Country::from(finland_cities);
    println!("Country {finland:?}");
    finland.print_cities();
}
```

Nesse exemplos tem os seguintes pontos:

- Implementamos From para Country que converte um vetor de City para Country.
  Nessa implementação recebemos um vetor e serializamos na struct City.
- Outra forma de resolver esse problem seria: primeiro implentar Clone para City `#[derive(Debug, Clone)] struct City ...` e em seguida fazer `let finland2 = Country{ cities: vec![helsinki, turku] };`

## Regra Orphan

Nesta seção vamos falar sobre Orphan Rule em Rust e como ela afeta a implementação de traits para tipos que você não criou.

1. **Regra do Órfão**:

   - A regra do órfão estabelece que você pode implementar um trait em um tipo que não criou, e também pode implementar um trait que pertence a outra pessoa em seu próprio tipo.
   - No entanto, você não pode implementar um trait que pertence a outra pessoa em um tipo que também pertence a outra pessoa.
   - Essa restrição é crucial para manter a consistência e prevenir problemas de ambiguidade e controle de código.

2. **Motivação para a Regra do Órfão**:

   - A regra do órfão evita situações em que múltiplas implementações de um trait podem levar à ambiguidade e inconsistência no comportamento de um tipo.
   - Sem essa regra, seria difícil garantir a coerência e a integridade dos tipos, especialmente quando diferentes implementações de um trait estão dispersas em diferentes locais do código.

3. **O Que Fazer para Contornar a Regra do Órfão**:
   - Uma maneira comum de contornar a regra do órfão é envolver o tipo de outra pessoa em uma `tuple struct`, criando assim um novo tipo.
   - Esse padrão é conhecido como o "newtype idiom" (idioma do novo tipo), e é uma técnica eficaz para criar novos tipos que se comportam de maneira semelhante a tipos existentes, mas possuem implementações de trait personalizadas.

Em resumo, a regra do órfão é uma salvaguarda importante em Rust para garantir a coerência e a integridade dos tipos e suas implementações de trait. Ao entender essa regra, você pode tomar decisões mais conscientes ao projetar e implementar código em Rust.

1. **Você pode implementar seu trait em tipos de outra pessoa**:

Suponha que você tenha um trait `Displayable` que define um método `display` para imprimir um objeto de forma legível. Você pode implementar esse trait para o tipo `Vec<T>` da seguinte forma:

```rust
trait Displayable {
    fn display(&self);
}

impl<T: std::fmt::Debug> Displayable for Vec<T> { // T tem q ter Debug
    fn display(&self) {
        for item in self {
            println!("{:?}", item);
        }
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.display(); // imprime os elementos do vetor
}
```

2. **Você pode implementar um trait de outra pessoa em seu próprio tipo**:

Suponha que você tenha um trait `Drawable` que define um método `draw` para desenhar um objeto em uma tela. Você pode implementar esse trait para o seu próprio tipo `Circle` da seguinte forma:

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        // Implementação para desenhar um círculo
        println!("Desenhando um círculo com raio {}", self.radius);
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    circle.draw(); // desenha o círculo
}
```

3. **Você não pode implementar um trait de outra pessoa em um tipo de outra pessoa**:

Suponha que você queira implementar o trait `Display` para o tipo `String` para imprimir a string de uma maneira específica. No entanto, você não pode fazer isso diretamente, pois não pode alterar a definição do tipo `String` que pertence à biblioteca padrão. Isso violaria a regra do órfão. Em vez disso, você pode usar uma nova estrutura que envolva `String` para criar um novo tipo e, em seguida, implementar o trait `Display` para esse novo tipo. Isso contornaria a regra do órfão. Uma forma de burlar essa regra fazendo um novo tipo, `struct File(String);`, nesse caso estamos usando File e não String

## AsRef

Às vezes, você deseja uma função que possa aceitar tanto uma String quanto um &str. Você pode fazer isso com o trait AsRef, que é usado para fornecer uma referência de um tipo para outro tipo. Você pode pensar nele como uma espécie de versão barata de From: em vez de converter de um tipo para outro, você faz uma conversão barata de uma referência para outra.

```rust
fn print_it<T: AsRef<str>>(input: T) {
    println!("{}", input.as_ref())
}
fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
```

## Resumo

Se você tem muitos tipos e deseja que todos tenham os mesmos métodos, escreva um trait.
Tipos que implementam um trait serão todos diferentes. Mas todos eles têm garantia de ter os métodos do trait.
Da mesma forma, cada pessoa que fala um idioma será diferente. Mas todos eles têm garantia de conhecer o idioma.
Você pode implementar seus traits em tipos de outras pessoas. Você pode implementar traits de outras pessoas em seus tipos. Mas você não pode implementar traits de outras pessoas em tipos de outras pessoas.
O trait From é bastante simples e você o vê em toda parte. Verifique a origem do código se estiver curioso sobre como é feito para um tipo específico.
Receber um AsRef
