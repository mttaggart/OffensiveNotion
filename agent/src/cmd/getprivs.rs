use std::error::Error;

pub async fn handle() -> Result<String, Box<dyn Error>> {
    // TODO: Implement Linux check
    #[cfg(windows)] {
        let is_admin = is_elevated::is_elevated();  
        println!("{}", is_admin);
        Ok(String::from(format!("Admin Context: {is_admin}").to_string()))
    }
    #[cfg(not(windows))] {
        Ok(String::from(format!("Under Construction!").to_string()))
    }
}