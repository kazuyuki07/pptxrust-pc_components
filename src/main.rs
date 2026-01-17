use anyhow::Result;
use ppt_rs::generator::{SlideContent, create_pptx_with_content};
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Deserialize, Debug)]
struct Device {
    model: String,
    price: f64,
    params: HashMap<String, String>,
}

#[derive(Deserialize, Debug)]
struct Category {
    name: String,
    devices: Vec<Device>,
}

#[derive(Deserialize, Debug)]
struct InputData {
    categories: Vec<Category>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_path = env::args().nth(1).unwrap_or_else(|| "input.json".to_string());
    let output_path = env::args().nth(2).unwrap_or_else(|| "comparison.pptx".to_string());

    let input = fs::read_to_string(&input_path)?;
    let data: InputData = serde_json::from_str(&input)?;

    let mut slides = Vec::new();

    for category in data.categories {
        let slide_title = format!("Сравнение {}", category.name);
        
        let mut slide_content = SlideContent::new(&slide_title)
            .add_bullet("Характеристики устройств:");

        slide_content = slide_content.add_bullet(&format!(
            "• {} | {:.0}₽ | {}",
            category.devices[0].model,
            category.devices[0].price,
            format_params(&category.devices[0].params)
        ));


        for device in &category.devices[1..] {
            slide_content = slide_content.add_bullet(&format!(
                "• {} | {:.0}₽ | {}",
                device.model,
                device.price,
                format_params(&device.params)
            ));
        }

        slide_content = slide_content
            .add_bullet("")
            .add_bullet("Рекомендация: выберите по соотношению цена/качество");

        slides.push(slide_content);
    }

    let pptx_data = create_pptx_with_content("🔥 Сравнение устройств", slides).unwrap();
    fs::write(&output_path, pptx_data)?;
    println!("PPTX создан: {}", output_path);
    println!("Файл готов для презентации сравнения устройств!");
    
    Ok(())
}

fn format_params(params: &HashMap<String, String>) -> String {
    params.iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect::<Vec<_>>()
        .join("; ")
}