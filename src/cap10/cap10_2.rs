use std::cell::{Cell, RefCell};

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

#[derive(Debug)]
struct UserRefCell {
    id: i32,
    year_registered: u32,
    username: String,
    active: RefCell<bool>,
}

fn main() {
    println!("\n\nCell\n\n");

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

    println!("\n\nRefCell\n\n");

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
