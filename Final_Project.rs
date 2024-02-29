struct Product 
{
    name:String,
    description:String,
    price:f64,
    quantity:u32,
}

struct Inventory 
{
    products:Vec<Product>,
}

enum Action 
{
    Add,
    Edit,
    Report,
    Exit,
}

fn get_input(prompt: &str) -> String 
{
    println!("{}", prompt);
    let mut input=String::new();
    std::io::stdin().read_line(&mut input).expect("Kindly write correctly");
    input.trim().to_string()
}

fn get_number(prompt: &str) -> f64 
{
    loop {
        let input=get_input(prompt);
        match input.parse::<f64>() 
        {
            Ok(num) => return num,
            Err(_) => println!("Invalid input Please enter a number"),
        }
    }
}

fn create_product() -> Product 
{
    Product {
        name: get_input("Enter product name: "),
        description: get_input("Enter product description: "),
        price: get_number("Enter product price: "),
        quantity: get_number("Enter product quantity: ") as u32,
    }
}

fn update_product(inventory: &mut Inventory) 
{
    let name=get_input("press Enter button(then you can add) ");
    let product=match name.is_empty() 
    {
        true => create_product(),
        false =>
        {
            let index=inventory.products.iter()
                .position(|p| p.name==name)
                .expect("Product not found.");
            inventory.products.remove(index);
            create_product()
        }
    };
    inventory.products.push(product);
    println!("Product saved successfully.");
}

fn generate_report(inventory: &Inventory) 
{
    println!("Inventory Report:");
    for product in &inventory.products 
    {
        println!("{} - {} ({} x ${:.2})", product.name,product.description, product.quantity, product.price);
    }
}

fn get_action() -> Action 
{
    loop 
    {
    
        let input=get_input("======Final Rust Project======               Choose the Action and write it             (add, edit, report): ");
        match input.to_lowercase().as_str() 
        {
            "add" => return Action::Add,
            "edit" => return Action::Edit,
            "report" => return Action::Report,
            "exit" => return Action::Exit,
            _ => println!("Invalid"),
        }
    }
}

fn main() 
{
    let mut inventory=Inventory { products: Vec::new() };

    loop {
        match get_action() 
        {
            Action::Add | Action::Edit => update_product(&mut inventory),
            Action::Report => generate_report(&inventory),
            Action::Exit => break,
        }
    }
}
