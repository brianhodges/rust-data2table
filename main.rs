pub use self::country::Country;
mod country;

fn main() {

    let mut countries = Vec::new();  
    countries.push(Country { name: "USA", population: "350 million" });
    countries.push(Country { name: "China", population: "1.4 billion" });
    countries.push(Country { name: "Russia", population: "145 million" });
    
    println!("DUMP:");
    println!("{:?}\n", countries);
    
    println!("TABLE:");
    println!("Countries | Population");
    println!("{}", "-".repeat(("Countries | Population").chars().count()));
    
    for country in &countries {        
        let ws = "Countries ".chars().count() - country.name.chars().count();
        println!("{}{}|{}", country.name, " ".repeat(ws), country.population);
    }
}