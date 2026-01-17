# Генератор презентации компонентов на Rust

Генерирует презентацию через json

## Инструкция по генерации

1. Перейти `target/release`
2. Создать в корне файла `input.json`
3. В `input.json` вписать почти так, как на примере

    ```json
    {
    "categories": [
        {
        "name": "{Название}",
        "devices": [
            {"model": "{Razer DeathAdder Essential}", "price": 2890, "params": {"сенсор": "10000 dpi", "скорость": "650 ips"}},
            {"model": "Logitech G Pro", "price": 9646, "params": {"сенсор": "25600 dpi", "скорость": "400 ips"}}
        ]
        },
        {
        "name": "клавиатуры", 
        "devices": [
            {"model": "Keychron K8", "price": 8500, "params": {"свитчи": "Black", "макет": "TKL"}},
            {"model": "Redragon Whisper", "price": 5750, "params": {"свитчи": "White", "макет": "TKL"}}
        ]
        }
    ]
    }
    ```

4. Запустить `pptx-prj.exe`

5. Создается презентация `comprasion.pptx`
