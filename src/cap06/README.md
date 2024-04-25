## Capitulo 6

Nessa seção será apresentado algumas _collections_ mais complexas e o operador `?`. Por fim, seremos apresentados ao _panic!_ e _unwarp_ que são funções muito utéis na legibilidade.

- HashMap e BTreeMap
- HashSet e BTreeSet
- BinaryHeap
- VecDeque
- Operador ?
- _panic!_ e _unwarp_

Nessa seção não será discutido muita operações e conceito teoricos das _collection_.

---

## HashMap e BTreeMap

- HashMap: É uma coleção de pares chave-valor onde todas as chaves devem ser únicas. Ela armazena os elementos em uma estrutura de tabela de dispersão, o que permite acesso rápido aos valores com base em suas chaves.

- BTreeMap: Similar ao HashMap, mas armazena os elementos em uma árvore de busca binária balanceada (B-tree), que mantém os elementos ordenados por chave.

```rust
use std::collections::{HashMap, BTreeMap};

struct City {
    name: String,
    population: HashMap<i32, i32>,
}

struct City2 {
    name: String,
    population: BTreeMap<i32, i32>,
}


fn main() {
    let mut tallinn = City {
        name: "Tallinn".to_string(),
        population: HashMap::new(),
    };
    tallinn.population.insert(2020, 437_619);
    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);

    // Esse for pode mudar todas vez q eh rodado pois HashMap nao possui ordem
    for (year, population) in tallinn.population {
        println!("In {year}, Tallin had a population of {population}")
    }

    let mut tallinn2 = City2 {
        name: "Tallinn".to_string(),
        population: BTreeMap::new(),
    };
    tallinn2.population.insert(2020, 437_619);
    tallinn2.population.insert(1372, 3_250);
    tallinn2.population.insert(1851, 24_000);

    // BTreeMap possui ordem
    println!("Usando BTreeMap");
    for (year, population) in tallinn2.population {
        println!("In {year}, Tallin had a population of {population}")
    }

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");
    println!("{:?}", book_hashmap.get(&1));

    let mut book_hashmap = HashMap::new();
    book_hashmap.insert(1, "L'Allemagne Moderne");
    let key = 1;
    match book_hashmap.get(&key) {
        Some(val) => println!("Key {key} has a value already: {val}"),
        None => {
            book_hashmap.insert(key, "Le Petit Prince");
        }
    }
    println!("{:?}", book_hashmap.get(&1));
}
```

## HashSet e BTreeSet

- HashSet: É uma coleção de elementos únicos, sem uma ordem específica. Ela usa uma tabela de dispersão para garantir a unicidade dos elementos.
- BTreeSet: Similar ao HashSet, mas armazena os elementos em uma árvore de busca binária balanceada, mantendo-os ordenados.

```rust
use std::collections::{BTreeSet, HashSet};

fn main() {
    let many_numbers = vec![
        37, 3, 25, 11, 27, 3, 37, 21, 36, 19, 37, 30, 48, 28, 16, 33, 2, 10, 1, 12, 38, 35, 30, 21,
        20, 38, 16, 48, 39, 31, 41, 32, 50, 7, 15, 1, 20, 3, 33, 12, 1, 11, 34, 38, 49, 1, 27, 9,
        46, 33,
    ];
    println!("How many numbers in the Vec? {}", many_numbers.len());
    let mut number_hashset = HashSet::new();
    for number in many_numbers {
        number_hashset.insert(number);
    }
    let hashset_length = number_hashset.len();
    println!(
        "There are {hashset_length} unique numbers, so we are missing {}.",
        50 - hashset_length
    );
    println!("It does not contain: ");
    for number in 0..=50 {
        if number_hashset.get(&number).is_none() {
            print!("{number} ");
        }
    }

    let mut tree_set = BTreeSet::new();
    tree_set.insert(3);
    tree_set.insert(6);
    tree_set.insert(1);

    println!("Conjunto ordenado: {:?}", tree_set);
}
```

## BinaryHeap e VecDeque

- BinaryHeap: Uma fila de prioridade implementada como uma árvore binária (normalmente uma heap máxima). Os elementos são removidos sempre na ordem de maior prioridade.

- VecDeque: Uma deque (double-ended queue) implementada como um vetor redimensionável. Ela suporta inserção e remoção eficientes em ambas as extremidades.

```rust
use std::collections::{BinaryHeap, VecDeque};

fn main() {
    println!("\n\nBINARYHEAP\n\n");

    let many_numbers = vec![0, 5, 10, 15, 20, 25, 30];
    let mut heap = BinaryHeap::new();
    for num in many_numbers {
        heap.push(num);
    }
    println!("First item is largest, others are out of order: {heap:?}");
    println!("First item is largest, others are out of order: {heap:?}");
    while let Some(num) = heap.pop() {
        println!("Popped off {num}. Remaining numbers are: {heap:?}");
    }

    println!("\n\nVECDEQUE\n\n");

    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for i in 0..600000 {
        my_vec.pop_front(); // existe o pop_back
    }
}
```
