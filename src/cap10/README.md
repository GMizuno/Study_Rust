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

Se você remover `<'_>` dos métodos `impl`, o compilador não será capaz de inferir o tempo de vida da referência `name` e retornará um erro, exigindo que você especifique o tempo de vida de maneira explícita. Isso ocorre porque o tempo de vida dos métodos precisa corresponder ao tempo de vida da struct em que estão sendo implementados. Ao adicionar `<'_>`, você delega essa tarefa de inferência ao compilador, tornando o código mais conciso e menos propenso a erros ao alterar a estrutura no futuro.

OBS: Remover todos os parâmetros de vida não resolve o problema.

## Interior Mutability

O conceito de Interior Mutability em Rust refere-se à capacidade de modificar o conteúdo de um valor aparentemente imutável de forma segura. Isso é importante porque Rust tem regras rígidas de mutabilidade para garantir a segurança do acesso concorrente aos dados.

Imagine que você tem uma estrutura imutável, mas ainda assim deseja modificar um campo interno (atributo) dessa struct. Uma forma de fazer isso seria permitir que a estrutura seja mutável como um todo, mas isso pode introduzir riscos de segurança. Então, em vez disso, Rust fornece esses tipos que permitem a mutabilidade interna, desde que as regras de propriedade sejam respeitadas.

Então, o conceito de "Interior Mutability" em Rust nos permite alcançar mutabilidade dentro de estruturas que, de outra forma, seriam imutáveis, mantendo a segurança de acesso aos dados.

Em Rust, temos tipos como `Cell`, `RefCell`, `Mutex` e `RwLock` para lidar com a mutabilidade interna. Vou explicar cada um deles:

### 1. **Cell**:

`Cell` é uma estrutura de dados que fornece mutabilidade interna mesmo quando o dono da `Cell` é imutável. Porém, você só pode armazenar tipos que implementam `Copy` dentro de uma `Cell`. Por exemplo:

Exemplo 1 - Uso básico de Cell

```rust
use std::cell::Cell;

let x = Cell::new(5);
x.set(10);
println!("Valor de x: {:?}", x.get()); // Saída: Valor de x: 10
```

Exemplo 2 - Usando Struct com Cell

```rust
use std::cell::{Cell};

#[derive(Debug)]
struct PhoneModelCell {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

impl PhoneModelCell {
    fn make_not_on_sale(&self) {
        self.on_sale.set(false)
    }
}

fn main() {
    let super_phone_3000 = PhoneModelCell {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };

    println!("{super_phone_3000:?}");
    super_phone_3000.make_not_on_sale();
    println!("{super_phone_3000:?}");
}
```

No Exemplo 2 temos o seguinte

1. **Uso de `Cell` para Mutabilidade Interna**:

   - O campo `on_sale` é do tipo `Cell<bool>`.
   - `Cell` permite mutabilidade interna mesmo quando a estrutura como um todo é imutável.
   - Isso é útil quando precisamos modificar campos internos de uma estrutura imutável de forma segura.

2. **Modificação do Status de Venda**:

   - O método `make_not_on_sale()` é chamado para marcar o telefone como não estando mais à venda.
   - Dentro deste método, o valor do campo `on_sale` é modificado para `false` usando o método `set()` de `Cell`.

### 2. **RefCell**:

`RefCell` também permite mutabilidade interna, mas de uma maneira um pouco diferente. Ela usa o conceito de borrowing em tempo de execução para garantir a segurança em tempo de execução. Isso significa que você pode ter várias referências mutáveis (`&mut`) para os dados dentro de uma `RefCell`, mas o Rust garantirá em tempo de execução que as regras de propriedade sejam seguidas. Aqui está um exemplo:

Exemplo 3 - Uso básico de RefCell

```rust
use std::cell::RefCell;

let y = RefCell::new(5);
{
    let mut y_mut = y.borrow_mut();
    *y_mut = 10;
}
println!("Valor de y: {:?}", *y.borrow()); // Saída: Valor de y: 10
```

Exemplo 4 - Usando Struct com RefCell

```rust
use std::cell::{RefCell};

#[derive(Debug)]
struct UserRefCell {
    id: i32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

fn main() {
    let user_1 = UserRefCell {
        id: 1,
        year_registered: 2020,
        username: "User 1".to_string(),
        active: RefCell::new(true),
    };

    println!("{user_1:?}");
    let mut borrow = user_1.active.borrow_mut(); // Equivalente let borrow = &mut user_1.active OU *user_1.active.borrow_mut() = false (segunda opcao eh a recomendada)
    *borrow = false;
    println!("{user_1:?}");
}
```

No Exemplo 4 temos o seguinte

1. **Uso de `RefCell` para Mutabilidade Interna**:

   - `RefCell` é usada para permitir a mutabilidade interna de `bool`, mesmo quando a estrutura como um todo é imutável.
   - Isso é útil quando precisamos modificar o estado interno de um campo em uma estrutura imutável.

2. **Modificação do Estado de Ativação**:
   - A função `borrow_mut()` de `RefCell` é chamada para obter uma referência mutável ao estado de ativação.
   - O estado de ativação é modificado para `false` através da referência mutável obtida.

Esses tópicos destacam como `RefCell` é usado para permitir mutabilidade interna em um campo de estrutura, **mesmo quando a estrutura como um todo é imutável.**

### 3. **Mutex**:

`Mutex` é uma abreviação de "mutual exclusion". Ele é usado para garantir que apenas um thread tenha acesso aos dados mutáveis por vez. Aqui está um exemplo:

Exemplo 5 - Uso básico de Mutex

```rust
use std::sync::Mutex;
use std::thread;

let data = Mutex::new(5);

{
    let mut data_mut = data.lock().unwrap();
    *data_mut = 10;
}

println!("Valor de data: {:?}", *data.lock().unwrap()); // Saída: Valor de data: 10
```

Em Mutex temos dois métodos muitos importantes para realizar "bloqueio" e "desbloqueio" das threads: `lock` e `try_lock`

#### **lock:**

O método `lock` é usado para realizar o bloqueio do Mutex. Quando chamado, ele bloqueia o thread atual até que o Mutex esteja disponível. Isso significa que ele garante acesso exclusivo aos dados protegidos pelo Mutex enquanto estiver bloqueado.

#### **try_lock:**

O método `try_lock` também é usado para adquirir o bloqueio do Mutex, mas de forma não bloqueante. Em vez de bloquear o thread atual, ele tenta adquirir o bloqueio e retorna imediatamente um resultado indicando se foi bem-sucedido ou não.

Exemplo - Uso básico de try_lock

```rust
use std::sync::Mutex;

let mutex = Mutex::new(5);
if let Some(mut data) = mutex.try_lock() {
    *data += 1;
    println!("Valor após incremento: {:?}", *data);
} else {
    println!("Não foi possível bloquear o Mutex neste momento.");
}

// match mutex {
//     Ok(value) => println!("The MutexGuard has value {value}"),
//     _ => println!("Didn't get the lock"),
// }
```

Neste exemplo, o thread tenta bloquear o Mutex usando `try_lock`. Se for bem-sucedido, ele incrementa o valor armazenado nos dados protegidos. Se não for bem-sucedido, imprime uma mensagem indicando que não foi possível bloquear o Mutex. O código comentado, representa uma forma alternativa para checagem

### 4. **RwLock**:

`RwLock` (Read–Write Lock) permite que múltiplos threads leiam os dados simultaneamente, mas apenas um thread pode escrever por vez. Não é apenas como um Mutex porque é seguro para threads, mas também é semelhante a um RefCell na forma como é usado: você pode obter referências mutáveis ou imutáveis para o valor dentro dele. Você usa .write().unwrap() em vez de .lock().unwrap() para modificá-lo. Você também pode usar .read().unwrap() para obter acesso de leitura. RwLock é semelhante ao RefCell porque segue as mesmas regras que o Rust usa para referências:

- Muitas variáveis com acesso .read() estão bem.
- Uma variável com acesso .write() está bem.
- Você não pode manter mais nada em cima de uma variável retornada de .write(). Você não pode ter uma variável extra feita com .write() ou mesmo com .read().

Mas RwLock também é semelhante a Mutex no sentido de que o programa travará em vez de entrar em pânico se você tentar usar .write() quando não puder obter acesso.

Exemplo - Uso basico de RwLock

```rust
use std::sync::RwLock;

let data = RwLock::new(5);

{
    let mut data_mut = data.write().unwrap();
    *data_mut = 10;
}

println!("Valor de data: {:?}", *data.read().unwrap()); // Saída: Valor de data: 10
```

Exemplo - Usando complexo de RwLock

```rust
let my_rwlock = RwLock::new(5);
let read1 = my_rwlock.read().unwrap();
let read2 = my_rwlock.read().unwrap();
println!("{read1:?}, {read2:?}");

// let write1 = my_rwlock.write().unwrap(); // deadlock

drop(read1);
drop(read2);

let mut write1 = my_rwlock.write().unwrap();
*write1 = 6;
println!("{:?}", my_rwlock);

println!("\n\nEXEMPLO COM TRY_LOCK\n\n");

let my_rwlock = RwLock::new(5);
let read1 = my_rwlock.read().unwrap();
let read2 = my_rwlock.read().unwrap();
println!("{read1:?}, {read2:?}");

let mut write1 = my_rwlock.try_write();

match write1 {
    Ok(mut value) => {
        *value += 10;
        println!("Value {value}")
    }
    Err(_) => println!("Couldn't get write access, sorry"),
};

println!("\n\nEXEMPLO 2 COM TRY_LOCK\n\n");

let my_rwlock2 = RwLock::new(5);
let mut write2 = my_rwlock2.try_write();

println!("{write2:?}");

match write2 {
    Ok(mut value) => {
        println!("Value was {value}");
        *value += 10;
        println!("Now Value is {value}")
    }
    Err(_) => println!("Couldn't get write access, sorry"),
};
```

Nesse exemplo temos o seguinte

#### 1. **Aquisição de Acesso de Leitura:**

```rust
let read1 = my_rwlock.read().unwrap();
let read2 = my_rwlock.read().unwrap();
```

- Dois acessos de leitura são adquiridos ao RwLock usando o método `read()`.
- Visto que o RwLock permite múltiplos acessos de leitura simultaneamente, isso é permitido.

#### 2. **Liberação dos Acessos de Leitura:**

```rust
drop(read1);
drop(read2);
```

- Os acessos de leitura são liberados.

#### 3. **Aquisição de Acesso de Escrita:**

```rust
let mut write1 = my_rwlock.write().unwrap();
*write1 = 6;
```

- Um acesso de escrita é adquirido ao RwLock usando o método `write()`.
- Como já não há mais acessos de leitura ativos, o acesso de escrita é permitido.
- O valor dentro do RwLock é modificado para 6.

#### 4. **Exemplo com `try_write`:**

- O exemplo seguinte mostra o uso do método `try_write()`, que tenta adquirir um acesso de escrita sem bloquear o thread.
- Se não for possível adquirir o acesso imediatamente, ele retorna um erro.

#### 5. **Tratamento de Erro ao Usar `try_write`:**

- O método `try_write()` é usado para tentar adquirir um acesso de escrita.
- Se for bem-sucedido, o valor é incrementado em 10 e impresso.
- Se não for bem-sucedido, uma mensagem de erro é impressa.

## Casos de Uso

### 1. **Compartilhamento de Dados em Ambientes Concorrentes:**

- As estruturas de dados mencionadas são úteis quando você precisa compartilhar dados entre threads em um ambiente concorrente.
- Por exemplo, você pode usar `Mutex` ou `RwLock` para proteger um recurso compartilhado e garantir que apenas um thread o acesse por vez.

### 2. **Cache com Mutex:**

- Você pode implementar um cache em um ambiente concorrente usando um `Mutex` para proteger o acesso aos dados armazenados em cache.
- Cada thread que deseja acessar ou atualizar o cache adquire o bloqueio do Mutex antes de realizar operações de leitura ou escrita.

### 3. **Contagem de Acessos com RefCell:**

- `RefCell` é útil quando você precisa de mutabilidade interna em uma estrutura imutável, como em contadores de acessos.
- Por exemplo, você pode usar `RefCell` para contar quantas vezes uma função é chamada, permitindo acesso mutável ao contador de acessos dentro da função.

### 4. **Atomicidade de Operações com Cell:**

- `Cell` pode ser usado para garantir atomicidade em operações em variáveis compartilhadas entre threads.
- Por exemplo, você pode usar `Cell` para implementar uma variável global que é atualizada por várias threads de forma segura.

### 5. **Leitura-escrita de Dados com RwLock:**

- `RwLock` é útil quando você tem um conjunto de dados que é frequentemente lido, mas raramente escrito.
- Por exemplo, em um aplicativo da web, você pode usar `RwLock` para proteger o acesso a um banco de dados onde a leitura de dados é comum, mas a gravação de dados é menos frequente.

Esses são apenas alguns exemplos de casos de uso práticos para as estruturas `Cell`, `RefCell`, `Mutex` e `RwLock` em Rust. Elas são poderosas ferramentas para lidar com a mutabilidade interna e garantir a segurança do acesso concorrente aos dados em programas Rust.
