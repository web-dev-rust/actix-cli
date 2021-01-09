use toml::Value;
use crate::error::ActixCliError;
use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct CrudConfig {
    pub routes: String,
    pub model: String,
    pub project_use: String,
    pub model_name: String,
}

pub fn create_crud_info(toml: Value, project_name: String) -> Result<Option<CrudConfig>, ActixCliError> {
    let toml_table= toml.as_table().ok_or_else(|| ActixCliError::ConfigTableFormat)?;
    if toml_table.contains_key("crud") {
        let crud = toml_table.get("crud").unwrap().as_table().ok_or_else(|| ActixCliError::ConfigTableFormat)?;
        if crud.contains_key("model") && crud.contains_key("routes") && crud.contains_key("name") {
            let name = crud["name"].as_str().ok_or_else(|| ActixCliError::CrudNameIsRequired)?;
            let model = create_crud_model(crud["model"].as_table().unwrap(), name)?;
            let routes = create_crud_routes(crud["routes"].as_table().unwrap(), name)?;
            let project_use = format!("use crate::{}_web::controllers::crud::{{create_{}, get_{}, update_{}, delete_{}, show_{}}};", 
                project_name.replace("-","_"), name, name, name, name, name);
            return Ok(Some(CrudConfig {routes, model, project_use, model_name: name.to_string()}));
        }
    }

   Ok(None)
}

fn create_crud_model(model: &std::collections::BTreeMap<String, Value>, name: &str) -> Result<String, ActixCliError> {
    let mut struct_def = "
pub struct ".to_string() + &name.to_case(Case::Pascal) + " {";
    let _ = model.iter().map(|(key, value)| {
        let _type = value.as_str().ok_or_else(|| ActixCliError::CrudStructType)?;
        let field = key.to_owned() + ": " + _type + ", ";
        struct_def.push_str(&field);
        Ok(())
    }).try_for_each(|field: Result<(), ActixCliError>| {
        field
    })?;
    struct_def.push_str("}");

    return Ok(struct_def);
}

fn create_crud_routes(routes: &std::collections::BTreeMap<String, Value>, name: &str) -> Result<String, ActixCliError> {
    let mut routes_def = "
.service(
    web::scope(\"api/\")
    ".to_string();
    let _ = routes.iter().map(|(key, value)| {
        let mut route = value.as_str().ok_or_else(|| ActixCliError::CrudRouteShouldBeString)?.to_string();
        if !route.starts_with(name) {
            route = name.to_string() + "/" + &route;
        }
        let route_config = match &key[..] {
            "create" => Ok(format!("
            .route(\"{}\", web::post().to(create_{}))", route ,name)),
            "read" => Ok(format!("
            .route(\"{}\", web::get().to(get_{}))", route ,name)), 
            "update" => Ok(format!("
            .route(\"{}\", web::put().to(update_{}))", route ,name)), 
            "delete" => Ok(format!("
            .route(\"{}\", web::delete().to(delete_{}))", route ,name)), 
            "list" => Ok(format!("
            .route(\"{}\", web::post().to(show_{}))", route ,name)),
            _ => Err(ActixCliError::UnknwonCrudItem)
        }?;
        routes_def.push_str(&route_config);

        Ok(())
    }).try_for_each(|field: Result<(), ActixCliError>| {
        field
    })?;
    routes_def.push_str("
)");

    return Ok(routes_def);
}