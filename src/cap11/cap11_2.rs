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
}
