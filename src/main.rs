use std::collections::HashMap;

fn main() 
{
    let mut recipes: HashMap<String,Recipe>= HashMap::new();

     let iron_ore = Recipe
    {
        recipe_product: String::from("Iron Ore"),
        production_time: 2.0,
        output_quantity: 1,
        input_requirements: Vec::new(),
    };

    let copper_ore = Recipe
    {
        recipe_product: String::from("Copper Ore"),
        production_time: 2.0,
        output_quantity: 1,
        input_requirements: Vec::new(),
    };

    let iron_plate: Recipe = Recipe
    {
        recipe_product:String::from("Iron Plate"),
        production_time:3.2,
        output_quantity:1,
        input_requirements: vec![
            IngredientRequirement
            {
                ingredient_name: String::from("Iron Ore"),
                input_quantity:1,
            }
        ],
    };

    let copper_plate = Recipe
    {
        recipe_product: String::from("Copper Plate"),
        production_time: 3.2,
        output_quantity: 1,
        input_requirements: vec![
            IngredientRequirement
            {
                ingredient_name: String::from("Copper Ore"),
                input_quantity: 1,
            }
        ],
    };

    let gear = Recipe
    {
        recipe_product: String::from("Iron Gear Wheel"),
        production_time: 1.0,
        output_quantity: 1,
        input_requirements: vec![
            IngredientRequirement
            {
                ingredient_name: String::from("Iron Plate"),
                input_quantity: 2,
            }
        ],
    };

    let automation_science = Recipe
    {
        recipe_product: String::from("Automation Science Pack"),
        production_time: 10.0,
        output_quantity: 1,
        input_requirements: vec![
            IngredientRequirement
            {
                ingredient_name: String::from("Iron Gear Wheel"),
                input_quantity: 1,
            },
            IngredientRequirement
            {
                ingredient_name: String::from("Copper Plate"),
                input_quantity: 1,
            }
        ],
    };

    let firearm_magazine: Recipe = Recipe
    {
        recipe_product: String::from("Firearm Magazine"),
        production_time: 2.0,
        output_quantity:1,
        input_requirements: vec![
            IngredientRequirement
            {
                ingredient_name: String::from("Iron Plate"),
                input_quantity:4,
            }
        ],
    };

    recipes.insert(String::from("Iron Ore"), iron_ore);
    recipes.insert(String::from("Copper Ore"), copper_ore);
    recipes.insert(String::from("Iron Plate"), iron_plate);
    recipes.insert(String::from("Copper Plate"), copper_plate);
    recipes.insert(String::from("Iron Gear Wheel"), gear);
    recipes.insert(String::from("Automation Science Pack"), automation_science);
    recipes.insert(String::from("Firearm Magazine"),firearm_magazine);

}





struct Recipe
{
    recipe_product: String,
    production_time: f32,
    output_quantity: i32,
    input_requirements: Vec<IngredientRequirement>,
}
struct IngredientRequirement
{
    ingredient_name: String,
    input_quantity: i32,
}