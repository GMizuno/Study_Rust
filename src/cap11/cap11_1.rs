use std::borrow::Cow;

// Exemplo 1
#[derive(Debug)]
struct ErrorInfo {
    error: LocalError,
    message: String,
}

#[derive(Debug)]
enum LocalError {
    TooBig,
    TooSmall,
}

fn generate_massage(message: &'static str, error_info: Option<ErrorInfo>) -> Cow<'static, str> {
    match error_info {
        None => message.into(),
        Some(info) => format!("{message}: {info:?}").into(),
    }
}

// Exemplo 2

struct User {
    name: Cow<'static, str>,
}

struct User2<'a> {
    name: Cow<'a, str>,
}

// Exemplo 3 -> Concatenando &str ou String

fn concat_strings<'a, 'b>(a: Cow<'a, str>, b: Cow<'b, str>) -> String {
    let mut result = String::new();
    result.push_str(a.as_ref());
    result.push_str(b.as_ref());
    result
}

// Exemplo 3.1 -> Complicando um pouco mais

fn concat_strings2<'a, 'b>(a: &NewString<'a>, b: &NewString<'b>) -> String {
    let mut result = String::new();
    result.push_str(a.string.as_ref());
    result.push_str(b.string.as_ref());
    result
}

struct NewString<'a> {
    string: Cow<'a, str>,
}

fn main() {
    println!("EXEMPLO 1");

    let msg1 = generate_massage("Everything is fine", None);
    let msg2 = generate_massage(
        "Got an error",
        Some(ErrorInfo {
            error: LocalError::TooBig,
            message: "It was too big".to_string(),
        }),
    );

    for msg in [msg1, msg2] {
        match msg {
            Cow::Borrowed(msg) => {
                println!("Borrowed, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned, because we needed an allocation:\n {msg}")
            }
        }
    }

    println!("EXEMPLO 2");

    let user_name = "User1";
    let other_user_name = "User2".to_string();

    let user1 = User {
        name: user_name.into(),
    };
    let user2 = User {
        name: other_user_name.into(),
    };

    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(msg) => {
                println!("Borrowed name, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned name, because we needed an allocation:\n {msg}")
            }
        }
    }

    let user_name = "User1";
    let other_user_name = &"User2".to_string();

    let user1 = User2 {
        name: user_name.into(),
    };
    let user2 = User2 {
        name: other_user_name.into(),
    };

    for name in [user1.name, user2.name] {
        match name {
            Cow::Borrowed(msg) => {
                println!("Borrowed name, didn't need an allocation:\n {msg}")
            }
            Cow::Owned(msg) => {
                println!("Owned name, because we needed an allocation:\n {msg}")
            }
        }
    }

    println!("EXEMPLO 3");

    let str1 = String::from("Hello, ");
    let str2 = "world!".to_owned();

    let result1 = concat_strings(Cow::Borrowed(&str1), Cow::Borrowed(&str2)); // Passando referências
    let result2 = concat_strings(Cow::Owned(str1), Cow::Owned(str2)); // Passando propriedades próprias

    println!("Result 1: {}", result1);
    println!("Result 2: {}", result2);

    println!("EXEMPLO 3.1");

    let str1_new = NewString {
        string: Cow::Borrowed("Hello, "),
    };
    let str2_new = NewString {
        string: Cow::Borrowed("world!"),
    };

    let result3 = concat_strings2(&str1_new, &str2_new);

    println!("Result 3: {}", result3);
}
