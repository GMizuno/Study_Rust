struct Company {
    name: String,
    ceo: Option<String>,
}

fn get_current_datetime() -> String {
    "2024-01-27T23:11:23".to_string()
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

    println!("\n\nEXEMPLO COM filter_map RETORNO SENDO RESULT\n\n");

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

    println!("\n\nEXEMPLO MAIS COMPLEXO\n\n");

    let results: Vec<Result<String, String>> = company_vec // poderia remover Vec<Result<String, String>>, compilador entenderia o tipo
        .iter()
        .map(|company| {
            company.get_ceo().ok_or_else(|| {
                let err_message = format!("No CEO found for {}", company.name);
                println!("{err_message} at {}", get_current_datetime()); // logging the message
                err_message
            })
        })
        .collect();

    results
        .iter()
        .filter(|res| res.is_ok())
        .for_each(|res| println!("{res:?}"))
}
