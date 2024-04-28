## Capitulo 9

Como vimos no último capítulo, com as _closures_ você pode 'encadear' métodos entre si e fazer muitas coisas com muito pouco código. E quanto mais você conhece, mais pode encadear juntos. Este capítulo vai principalmente mostrar como usar certos métodos comuns de iteradores que funcionam convenientemente com _closures_.

- _.filter() _ e _.filter_map()_,
- _.any()_ e _.all()_
- _.cycle()_, _.fold()_ e _.zip()_

## _.filter() _ e _.filter_map()_

Além do _map_, filtragem é outra aplicação comum para iteradores. Enquanto o _map_ permite fazer algo com cada item e passá-lo adiante, a filtragem permite manter apenas os itens que correspondem a uma determinada condição. O método .filter() permite isso, mantendo os itens com base em uma expressão que retorna um booleano.

Exemplo 1 - Uso simples de filter

```rust
let months = vec![
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];
let filtered_months = months
    .into_iter()
    .filter(|month| month.len() < 5)
    .filter(|month| month.contains("u"))
    .collect::<Vec<&str>>();
println!("{:?}", filtered_months);
```

Outro método interessante é .filter_map(), que combina os métodos .filter() e .map(). Este método exige que o fechamento retorne um Option<T>, filtrando valores None e passando adiante os valores Some.

Exemplo 2 - Uso mais complexo de filtragem

```rust
struct Company {
    name: String,
    ceo: Option<String>,
}

impl Company {
    fn new(name: &str, ceo: &str) -> Self {
        let ceo = match ceo {
            "" => None,
            name => Some(name.to_string()),
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}

fn main() {
    let company_vec = vec![
        Company::new("Umbrella Corporation", "Unknown"),
        Company::new("Ovintiv", "Brendan McCracken"),
        Company::new("The Red-Headed League", ""),
        Company::new("Stark Enterprises", ""),
    ];

    let one_ceo = Company::new("Fluminense", "MB").get_ceo();
    println!("{one_ceo:?}");

    let all_the_ceos = company_vec
        .iter()
        .filter_map(|company| company.get_ceo())
        .collect::<Vec<String>>();
    println!("Todos os CEOs: {all_the_ceos:?}");
}
```

Exemplo 3 - Closure retornnado Result

```rust
let user_input = vec![
    "8.9",
    "Nine point nine five",
    "8.0",
    "7.6",
    "eleventy-twelve",
];
let successful_numbers = user_input
    .iter()
    .filter_map(|input| input.parse::<f32>().ok())
    .collect::<Vec<f32>>();
println!("{:?}", successful_numbers);
```

Nesse exemplo parse retorna Result, mas _.ok_ convert para Option (ou seja, input.parse::<f32>() retorn Ok ou Err e .ok() converte para Some ou None)

## _.any()_ e _.all()_

Nessa seção vamos falar sobre alguns métodos utéis de utilizar em Rust para trabalhar com _collections_. Dentre os métodos utéis temos

- `.any()` retorna verdadeiro se uma condição for verdadeira para qualquer um dos itens na coleção.
- `.all()` retorna verdadeiro se uma condição for verdadeira para todos os itens na coleção.
- `.find()` retorna o primeiro item que corresponde a uma condição,encapsulado em `Some(item)`, ou `None` se nenhum item corresponder.
- `.position()` retorna a posição do primeiro item que corresponde àcondição, encapsulada em `Some(position)`, ou `None` se nenhum itemcorresponder.

Exemplo 1 - Any e All

```rust
fn in_char_vec(char_vec: &Vec<char>, check: char) {
    println!(
        "Is {check} inside? {}",
        char_vec.iter().any(|&char| char == check)
    );
}
fn main() {
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');
    let smaller_vec = ('A'..'z').collect::<Vec<char>>();
    println!(
        "All alphabetic? {}",
        smaller_vec.iter().all(|&x| x.is_alphabetic())
    );
    println!(
        "All less than the character 행? {}",
        smaller_vec.iter().all(|&x| x < '행')
    );
}
```

No Exemplo 1 temos os seguintes pontos

1. **Declaração da função `in_char_vec`:** Define uma função chamada `in_char_vec` que recebe dois parâmetros: `char_vec`, uma referência a um vetor de caracteres (`&Vec<char>`), e `check`, um caractere (`char`) que será verificado se está presente no vetor.

2. **`println!` macro:** Esta macro é usada para imprimir uma mensagem na saída padrão. Dentro dela, há uma string de formatação com um marcador de posição `{}` onde os valores serão inseridos. `{check}` é um marcador de posição que será substituído pelo valor da variável `check`.

3. **`char_vec.iter().any(...)`:** `iter()` é um método que cria um iterador sobre o vetor de caracteres. `any(...)` é um método que verifica se pelo menos um elemento do iterador satisfaz uma condição. A condição é especificada por uma função de fechamento (`|&char| char == check`), que compara cada caractere do vetor com `check`. Se algum deles for igual a `check`, `any` retorna `true`, indicando que `check` está presente no vetor.

4. Já `.all(|&x| x.is_alphabetic()): all()` é um método que verifica se todos os elementos do iterador satisfazem uma condição. A condição é especificada por uma função de fechamento (`|&x| x.is_alphabetic()`), que é aplicada a cada elemento (x) do iterador. Dentro da função de fechamento, `x.is_alphabetic()` retorna true se o caractere x for uma letra do alfabeto (maiúscula ou minúscula).

5. Por fim, `.all(|&x| x < '행'): all()` é usado aqui para verificar se todos os elementos do iterador são menores que o caractere '행'. A função de fechamento (`|&x| x < '행'`) é aplicada a cada elemento (x) do iterador. Dentro da função, comparamos cada caractere x com '행'. Se todos os caracteres forem estritamente menores que '행' na ordem lexicográfica Unicode, o método all() retorna true.

Exemplo 2 - Métodos find e postion

```rust
fn main() {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
    println!("{:?}", num_vec.iter().find(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 3 == 0));
    println!("{:?}", num_vec.iter().find(|number| *number % 11 == 0));
    println!("{:?}", num_vec.iter().position(|number| *number % 11 == 0));
}
```

No exemplo 2, temos que;

3. `.find(|number| *number % 3 == 0)` o método `find()` é chamado no iterador. Ele busca o primeiro elemento que satisfaz uma condição especificada por uma função de fechamento (`|number| *number % 3 == 0`). Neste caso, está procurando o primeiro número no vetor que seja divisível por 3.

4. `.position(|number| *number % 3 == 0)` o método `position()` é similar ao `find()`, mas retorna a posição do primeiro elemento que satisfaz a condição especificada pela função de fechamento. Neste caso, está buscando a posição do primeiro número no vetor que é divisível por 3.

5. `.find(|number| *number % 11 == 0)` similar ao primeiro exemplo, este `find()` busca o primeiro número no vetor que é divisível por 11.

```rust
println!("{:?}", num_vec.iter().position(|number| *number % 11 == 0));
```

7. `.position(|number| *number % 11 == 0)` da mesma forma, este `position()` busca a posição do primeiro número no vetor que é divisível por 11.

Esses métodos são úteis para operações de busca e filtragem em coleções em Rust.

## _.cycle()_, _.fold()_ e _.zip()_

ZIP ESTA NO CAPITULO ANTERIOR OLHAR EM NA PAGINA 166
ZIP ESTA NO CAPITULO ANTERIOR OLHAR EM NA PAGINA 166
ZIP ESTA NO CAPITULO ANTERIOR OLHAR EM NA PAGINA 166
ZIP ESTA NO CAPITULO ANTERIOR OLHAR EM NA PAGINA 166
ZIP ESTA NO CAPITULO ANTERIOR OLHAR EM NA PAGINA 166

```rust
use std::collections::HashMap;
fn main() {
    let some_keys = vec![0, 1, 2, 3, 4, 5];
    let some_values = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap = some_keys
        .into_iter()
        .zip(some_values.into_iter())
        .collect::<HashMap<_, _>>();

    println!(
        "The value at key 2 is: {}",
        number_word_hashmap.get(&2).unwrap()
    );
}
```
