pub fn get_env_var(
    env_var_name: &str
) -> String {
     match std::env::var(env_var_name).map_err(|e| format!("{}: {}", env_var_name, e)) {
        Ok(string) => string,
        Err(_) => "".to_string()
    }
}
