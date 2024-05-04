## Capítulo 10

Nesse capítulo será abortado, de maneira mais minuciosa, os diferentes tipos de string que existem.
Além disso, serão abordados dois tópicos que somente existe no universo do Rust, _lifetime annotations_ e _interior mutability_.

- Tipos de String
- _lifetime annotations_
- _interior mutability_

Em geral questões envolvendo _lifetime annotations_ e _interior mutability_ não iram impactar seu código,
mas enteder esse conceito ajudar no entendimento de como as coisas funcionam por debaixo dos panos em Rust

---

## Tipos de String

Entender as diferenças entre os tipos de `&str` em Rust é crucial para escrever código eficiente e seguro.
Em Rust existem dois tipo de `&str` que podem ser resumidos da seguinte forma:

1. **String literals:**

   - São criados quando você escreve uma sequência de caracteres diretamente no código, como em `let my_str = "Eu sou um &str";`.
   - Existem durante toda a execução do programa porque são incorporados diretamente no binário.
   - Possuem o tipo `&'static str`, onde `'static` é uma vida útil especial que significa que a string literal existe durante toda a vida do programa.

2. **Borrowed str:**

   - Este é o tipo comum de `&str` que você encontrará ao passar referências a strings.
   - Este tipo de `&str` não tem uma vida útil `'static`, o que significa que ele é vinculado ao tempo de vida da `String` da qual foi derivado.

Aqui está um exemplo para ilustrar esses conceitos:

```rust
fn prints_str(my_str: &str) {
    println!("{}", my_str);
}

fn main() {
    let my_string = String::from("Eu sou uma string");
    prints_str(&my_string); // Conversão implícita de &String para &str
}
```

Agora um ponto muito importante em relação ao retorno de `&str` de funções, é seguro retornar uma referência para uma string literal `'static`, pois ela existe durante toda a execução do programa. Porém, retornar uma referência para uma `String` criada dentro de uma função \**não é seguro*ß, pois ela seria desalocada assim que a função terminasse.

Aqui está um exemplo que ilustra isso:

```rust
fn works() -> &'static str {
    "Eu vivo para sempre!"
}

// Esta função não funcionará
// fn does_not_work() -> &'static str {
//     &String::from("Desculpe, eu só existo dentro da função. Não sou 'static")
// }

fn main() {
    let s1 = works();
    println!("{}", s1);
    println!("{}", s1);
}
```

Essa distinção é importante para escrever código Rust seguro e eficiente, especialmente ao lidar com strings e suas referências.

## _Lifetime Annotations_

As _Lifetime Annotations_ em Rust fornecem informações adicionais sobre quanto tempo as referências devem permanecer válidas. Embora Rust geralmente gerencie automaticamente os tempos de vida, há situações em que anotações explícitas são necessárias para garantir a segurança da memória.

Entender os tempos de vida é crucial em Rust. Eles especificam por quanto tempo as variáveis ou referências devem viver.

_Lifetime Annotations_ são mais comumente usadas com referências. Isso ocorre porque as referências não podem viver mais do que o objeto que elas referenciam. Portanto, é essencial garantir que a referência permaneça válida enquanto o objeto ainda estiver em uso.

Ao lidar com funções em Rust, os tempos de vida se tornam importantes, pois as funções têm pontos claros de início e término. Por exemplo, tentar retornar uma referência para uma variável local dentro de uma função resulta em um erro do compilador, pois o tempo de vida da variável termina quando a função retorna.

Exmplo 1 - Erro de tempo de vida

```rust
fn retorna_referencia() -> &str {
    let minha_string = String::from("Olá, mundo!"); // Criando uma String
    &minha_string // Tentando retornar uma referência para a String local
}

fn main() {
    let minha_referencia = retorna_referencia(); // Chamando a função
    println!("{}", minha_referencia); // Tentando imprimir a referência
}
```

Quando uma função retorna uma string literal, é necessário especificar isso usando uma anotação de tempo de vida `'static`. Isso indica ao Rust que a string literal vive durante toda a duração do programa, tornando seguro retorná-la de uma função.

Além disso, ao definir structs em Rust, é necessário considerar os tempos de vida dos campos que contêm referências. Usar anotações de tempo de vida apropriadas garante que as referências sejam válidas durante o tempo necessário.

O tempo de vida anônimo (`'_`) em Rust é usado quando referências estão sendo usadas, mas o tempo de vida exato não é explicitamente especificado. Isso é comum em implementações de structs, onde Rust sugere o tempo de vida anônimo quando necessário.

Em implementações de traits em Rust, também podem surgir situações envolvendo múltiplos tempos de vida. Nesses casos, diferentes tempos de vida podem ser declarados e usados independentemente para diferentes partes do programa.

Embora lidar com tempos de vida em Rust possa ser desafiador, é importante lembrar que muitas vezes é possível evitar problemas usando tipos de propriedade ou clones.

Exemplo 2 - Lifetime com Struct

```rust
#[derive(Debug)]
struct City {
    name: &'static str, // Se fosse &str seria um problema, pois se o valor da referencia for limpando o q seria feito com esse referencia?
    date_founded: u32,
}

#[derive(Debug)]
struct City2<'a> {
    name: &'a str, // Se fosse &str seria um problema, pois se o valor da referencia for limpando o q seria feito com esse referencia?
    date_founded: u32,
}

fn main() {
    let city = City {
        name: "Niteroi",
        date_founded: 1995,
    };
    let my_city = city;
    println!("{my_city:?}");

    // Retorna um erro, pois city_name não "vivera" tempo suficiente. Note que estamos usando City e nao City2
    // let city_name = vec!["Niteroi".to_string(), "Rio de Janeiro".to_string()];
    // let my_city2 = City {
    //     name: &city_name[0],
    //     date_founded: 1995,
    // };
    // println!("{my_city2:?}");

    let city_name = vec!["Niteroi".to_string(), "Rio de Janeiro".to_string()];
    let my_city2 = City2 {
        name: &city_name[0],
        date_founded: 1995,
    };
    println!("{my_city2:?}");
}
```

No exemplo acima temos dois pontos muito importante no tocante a criação de City e City2. Em Rust, uma referência não pode sobreviver mais do que a vida útil do valor ao qual ela se refere. No caso do `city_name`, uma vez que o vetor é criado dentro da função `main()`, ele é destruído quando a função termina. Então, quando você tenta criar `my_city2` usando uma referência ao primeiro elemento do vetor, você recebe um erro porque a referência não pode sobreviver além da vida do vetor.

Entretanto, ao usar `City2`, que possui uma vida útil genérica `'a`, você está especificando que a vida útil da referência contida no campo `name` é vinculada à vida útil do `City2` em si, permitindo que a referência exista enquanto `City2` existir. Isso resolve o problema do tempo de vida da referência e permite que o código compile sem erros.

Exemplo 3 - Outro exemplo envolvendo lifetime

```rust
struct Adventurer<'a> {
    name: &'a str,
    hit_point: u32,
}

impl Adventurer<'_> {
    // Se nao incluir <'_> esse anonymous lifetime o compilador retornar um erro
    fn take_damage(&mut self) {
        self.hit_point -= 20;
        println!("{} has {} hit point left!!!", self.name, self.hit_point);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit point left!!!", self.name, self.hit_point)
    }
}
fn main() {
    let mut billy = Adventurer {
        name: "Biily",
        hit_point: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
```

Já nesse último exemplo sobre _lifetime_ temos o seguinte. Em Rust, ao definir uma struct com referências, é necessário especificar a vida útil das referências. Isso é feito através do uso de parâmetros de vida (`'a`, `'b`, etc.) na definição da estrutura. Quando você usa `<'_>` como no exemplo, está indicando um tempo de vida anônimo ou implícito, que será inferido pelo compilador. Isso é útil quando você está implementando métodos em uma struct que já possui parâmetros de vida, mas você não precisa especificar novamente esses parâmetros nos métodos.

No exemplo, a estrutura `Adventurer` é definida com um parâmetro de vida `'a`, indicando que o tempo de vida da referência `name` está vinculado ao tempo de vida da instância de `Adventurer`. Quando você implementa os métodos `take_damage` e `Display` para `Adventurer`, você precisa usar o mesmo tempo de vida `'a`, caso contrário, o compilador não será capaz de inferir corretamente o tempo de vida da referência.

Então, ao adicionar `<'_>` nos métodos `impl` para `Adventurer`, você está dizendo ao compilador para inferir o tempo de vida da referência `name` com base no tempo de vida da instância de `Adventurer` que está sendo usada. Isso permite que o código compile sem erros, pois o compilador é capaz de determinar corretamente a vida útil das referências.

Se você remover `<'_>` dos métodos `impl`, o compilador não será capaz de inferir o tempo de vida da referência `name` e retornará um erro, exigindo que você especifique o tempo de vida de maneira explícita. Isso ocorre porque o tempo de vida dos métodos precisa corresponder ao tempo de vida da estrutura em que estão sendo implementados. Ao adicionar `<'_>`, você delega essa tarefa de inferência ao compilador, tornando o código mais conciso e menos propenso a erros ao alterar a estrutura no futuro.

OBS: Remover todos os parâmetros de vida não resolve o problema.
