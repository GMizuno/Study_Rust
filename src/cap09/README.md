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

Exemplo 3

## _.any()_ e _.all()_

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
