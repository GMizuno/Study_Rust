# Capitulo 3

Nesse capítulo aprofundaremos os estudos sobre tipo, aprederemos sobre _Array_, _Vector_ e _Tuple_. Além desses tópicos, iniciaremos as discussões sobre controle de fluxo (_Control Flow_).

- Array
- Vector
- Tuple
- Control Flow
- Match, if/else, Loop, While

---

## Colections

_Collections_ são uma estrutura de dados que geralmente são usadas quando você tem mais de um valor e deseja armazená-los em um único lugar com algum tipo de ordem, são equivalentes a _Lists_ em _Python_.
Existem diversas _collection_ em Rust, as mais conhecidas são _Array_, _Vector_ e _Tuple_ (também são as mais simples), já _Hash_ e _BinaryHeap_ são outros tipos de _collections_ mais complexos em Rust como )

---

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

---

## Vector

A diferença entre os dois é semelhante à diferença entre _&str_ e _String_: arrays são mais simples, com menos flexibilidade e funcionalidades, e podem ser mais rápidos, enquanto vetores são mais fáceis de trabalhar porque você pode alterar seu tamanho.
Vamos dar uma olhada em alguns dos principais métodos envolvendo vetores de uma forma didática:

- _push_ e _pop_: Esses são métodos que permitem adicionar (_push_) ou remover (_pop_) elementos do final do vetor, respectivamente. É como uma pilha de pratos: você pode adicionar um prato no topo da pilha (_push_) ou remover o último prato adicionado (_pop_).

- _len_ e _is_empty_: O método _len_ retorna o número de elementos no vetor, enquanto _is_empty_ retorna verdadeiro se o vetor estiver vazio e falso caso contrário. É útil para verificar o tamanho do vetor antes de realizar operações.

- _get_ e _get_mut_: Esses métodos permitem acessar elementos específicos do vetor de forma segura. _get_ retorna uma referência imutável para o elemento em um determinado índice, enquanto _get_mut_ retorna uma referência mutável. Eles evitam erros de acesso fora dos limites do vetor.

- _sort_ e _sort_by_: Esses métodos permitem ordenar os elementos do vetor. _sort_ ordena os elementos em ordem crescente usando a implementação padrão de ordenação para o tipo de dados do vetor, enquanto _sort_by_ permite especificar uma função de comparação personalizada.

- _contains_ e _contains_key_: Esses métodos verificam se um determinado elemento ou chave está presente no vetor. _contains_ verifica se um elemento está presente, enquanto _contains_key_ verifica se uma chave está presente em um vetor de tuplas chave-valor.

Outros dois métodos importante são _iter_ e _iter_mut_, eles serão abordados nas próximas seções.

Exemplo 1 - Criando Vector

```rust
let name1 = String::from("Gabriel");
let name2 = String::from("Mizuno");

let mut my_vec = Vec::new();
my_vec.push(name1);
my_vec.push(name2);
println!("My Vector {my_vec:?}");
```

Exemplo 2 - Vector é homogeneo

```rust
let mut my_vec = vec![String::from("Gabriel"), String::from("Mizuno")];

my_vec.push(1) // Erro
```

Exemplo 3 - Slicing Vector

```rust
let my_vec1 = vec![8, 10, 10];
println!("My Vector2 {my_vec1:?}");
let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let vec_three_to_five = &vec_of_ten[2..5];
let vec_start_at_two = &vec_of_ten[1..];
let vec_end_at_five = &vec_of_ten[..5];
let vec_everything = &vec_of_ten[..];

println!("Three to five: {vec_three_to_five:?}");
println!("start at two: {vec_start_at_two:?}");
println!("end_at_five: {vec_end_at_five:?}");
println!("Everything: {vec_everything:?}");
```

Exemplo 4 - Aplicando metodos

```rust
fn main() {
    let my_vec = vec!(111, 1, 2, 3, 901, 10, 20, 30, 40, 50);
    let my_vec2 = Vec::<i32>::new();
    let mut my_vec3 =vec!(111, 1, 2, 3, 901, 10, 20, 30, 40, 50);
    let mut names = vec!["Bassem", "Celeste", "Rick", "Valentine"];
    println!("Tamnaho do my_vec? {:?}", my_vec.len());
    println!("my_vec esta vazio? {:?}", my_vec.is_empty());
    println!("my_vec2 esta vazio? {:?}", my_vec2.is_empty());
    println!("my_vec index 1 eh {:?}", my_vec.get(1));
    println!("my_vec3 index 2 eh {:?}", my_vec3.get_mut(2)); // nao poderia ser my_vec pois nao eh mutavel
    my_vec3.sort();
    println!("my_vec3 ordenado {:?}", my_vec3);
    println!("my_vec contem 3? {:?}", my_vec.contains(&3));
    println!("my_vec contem 30000? {:?}", my_vec.contains(&30000));
    names.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("names ordenado {:?}", names);
}
```

---

## Tuple

Exemplo X - Criando Tuple

```rust
let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
println!("Inside the tuple is: First item: {:?}", random_tuple.0);
println!("Second item: {:?}", random_tuple.1);
println!("Third item: {:?}", random_tuple.2);
println!("Fourth item: {:?}", random_tuple.3);
println!("Fifth item: {:?}", random_tuple.4);
println!("Sixth item: {:?}", random_tuple.5);
```

Exemplo X - Destructuring

```rust
let strings = ("one".to_string(), "two".to_string(), "three".to_string());
let (a, b, c) = strings;
println!("{b}");
```

Exemplo X - Copy

```rust
fn print_number(number: i32){
    println!("My number is {}", number)
}

fn print_str(str: &str){
    println!("My string is {}", str)
}

fn main() {
    let my_tuple = ("one".to_string(), "two".to_string(), "three".to_string(), 8);
    let (a, b, c, d) = my_tuple;
    print_str(b);
    print_number(d);
    println!("{b}"); // Funciona pois &str tem Copy
    println!("{d}"); // Funciona pois i32 tem Copy
    println!("{my_tuple:?}"); // Nao funciona pois Tuple não tem Copy
}

```
