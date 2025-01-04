use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tera::{Context, Tera};

pub struct TemplateEngine {
    tera: Arc<Tera>,
}

impl TemplateEngine {
    pub fn new(template_path: &str) -> Result<Self, tera::Error> {
        let tera = Tera::new(template_path)?;
        Ok(Self {
            tera: Arc::new(tera),
        })
    }

    pub fn load(&self, template_name: &str) -> Result<Template, String> {
        if self.tera.get_template(template_name).is_err() {
            return Err(format!("Template '{}' not found", template_name));
        }

        Ok(Template {
            name: template_name.to_string(),
            tera: self.tera.clone(),
        })
    }
}

#[derive(Debug)]
pub struct Template {
    name: String,
    tera: Arc<Tera>,
}

impl Template {
    pub fn render(&self, data: Value) -> Result<String, String> {
        // Convert data from request to hashmap
        let data_map: HashMap<String, Value> =
            serde_json::from_value(data).map_err(|e| format!("Failed to convert data: {}", e))?;

        let mut context = Context::new();
        for (key, value) in data_map {
            context.insert(key.as_str(), &value);
        }

        self.tera
            .render(&self.name, &context)
            .map_err(|e| format!("Failed to render template: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::tempdir;

    fn create_dummy_template() -> TemplateEngine {
        let temp_dir = tempdir().unwrap();
        let template_path = temp_dir.path().join("template.txt");
        fs::write(&template_path, "Hello, {{ name }}!").unwrap();

        let template_path_str = temp_dir.path().to_str().unwrap();
        TemplateEngine::new(&format!("{}/*.txt", template_path_str)).unwrap()
    }

    #[test]
    fn test_template_engine_new() {
        let temp_dir = tempdir().unwrap();
        let template_path = temp_dir.path().join("template.txt");
        fs::write(&template_path, "Hello, {{ name }}!").unwrap();

        let template_path_str = temp_dir.path().to_str().unwrap();
        let engine = TemplateEngine::new(&format!("{}/*.txt", template_path_str));

        assert!(engine.is_ok());
    }

    #[test]
    fn test_load_existing_template() {
        let engine = create_dummy_template();

        let template = engine.load("template.txt");
        assert!(template.is_ok());
    }

    #[test]
    fn test_load_nonexistent_template() {
        let engine = create_dummy_template();

        let template = engine.load("nonexistent.txt");
        assert!(template.is_err());
    }

    #[test]
    fn test_render_template_success() {
        let engine = create_dummy_template();
        let template = engine.load("template.txt").unwrap();

        let data = json!({ "name": "World" });
        let rendered = template.render(data);

        assert!(rendered.is_ok());
        assert_eq!(rendered.unwrap(), "Hello, World!");
    }

    #[test]
    fn test_render_template_missing_data() {
        let engine = create_dummy_template();
        let template = engine.load("template.txt").unwrap();

        let data = json!({}); // Missing "name"
        let rendered = template.render(data);

        assert!(rendered.is_err());
    }

    #[test]
    fn test_render_template_invalid_data() {
        let engine = create_dummy_template();
        let template = engine.load("template.txt").unwrap();

        let data = json!("invalid_data"); // Not a JSON object
        let rendered = template.render(data);

        assert!(rendered.is_err());
    }

    #[test]
    fn test_render_template_missing_value_in_data() {
        let engine = create_dummy_template();
        let template = engine.load("template.txt").unwrap();

        let data = json!({"age": 30 }); // "age" not defined in template
        let rendered = template.render(data);

        assert!(rendered.is_err());
    }
}
