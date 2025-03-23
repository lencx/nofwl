use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use tauri::Manager;
use tauri_plugin::PluginBuilder;

#[derive(Debug, Deserialize)]
struct Plugin {
    name: String,
    version: String,
    author: Option<String>,
    description: Option<String>,
    link: Option<String>,
    url: Option<Vec<String>>,
    dependencies: Option<Vec<String>>,
    compatibility: Option<String>,
    update_url: Option<String>,
    changelog: Option<String>,
    last_updated: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    plugins: Vec<Plugin>,
}

fn load_config() -> Config {
    let mut file = File::open("nofwl.yml").expect("Failed to open nofwl.yml");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read nofwl.yml");
    serde_yaml::from_str(&contents).expect("Failed to parse nofwl.yml")
}

fn check_dependencies(plugin: &Plugin, config: &Config) -> Result<(), String> {
    if let Some(dependencies) = &plugin.dependencies {
        for dependency in dependencies {
            if !config.plugins.iter().any(|p| &p.name == dependency) {
                return Err(format!("Missing dependency: {}", dependency));
            }
        }
    }
    Ok(())
}

fn initialize_plugins(config: &Config) {
    for plugin in &config.plugins {
        println!("Initializing plugin: {}", plugin.name);

        if let Err(err) = check_dependencies(plugin, config) {
            println!("Error initializing plugin {}: {}", plugin.name, err);
            continue;
        }

        if let Some(urls) = &plugin.url {
            for url in urls {
                println!("Connecting to Val.town API endpoint: {}", url);
                // Add code to connect to Val.town API endpoints
            }
        }
    }
}

fn main() {
    let config = load_config();
    initialize_plugins(&config);

    tauri::Builder::default()
        .plugin(PluginBuilder::default().build())
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.set_title("NoFWL").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("Hello, NoFWL!");
}
