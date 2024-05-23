#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
pub enum SearchOption {
    #[serde(rename = "id")]
    ID(u32),
    #[serde(rename = "name")]
    Name(String),
    #[default]
    None,
}

pub trait NamedSearch: From<SearchOption> + Default {
    fn set_option(self, option: SearchOption) -> Self;

    fn set_id(self, id: u32) -> Self {
        self.set_option(SearchOption::ID(id))
    }

    fn set_name(self, name: String) -> Self {
        self.set_option(SearchOption::Name(name))
    }

    fn get_option(&self) -> Option<&SearchOption>;

    fn get_id(&self) -> Option<&u32> {
        let op = self.get_option()?;
        match op {
            SearchOption::ID(id) => Some(id),
            SearchOption::Name(_) => None,
            SearchOption::None => None,
        }
    }

    fn get_name(&self) -> Option<&String> {
        let op = self.get_option()?;
        match op {
            SearchOption::ID(_) => None,
            SearchOption::Name(name) => Some(name),
            SearchOption::None => None,
        }
    }
}
pub trait UnnamedSearch: From<SearchOption> + Default {
    fn set_option(self, option: SearchOption) -> Self;

    fn set_id(self, id: u32) -> Self {
        self.set_option(SearchOption::ID(id))
    }

    fn get_option(&self) -> Option<&SearchOption>;

    fn get_id(&self) -> Option<&u32> {
        let op = self.get_option()?;
        match op {
            SearchOption::ID(id) => Some(id),
            SearchOption::Name(_) => None,
            SearchOption::None => None,
        }
    }
}

pub async fn get<U>(url: U) -> Result<String, crate::Errors>
where
    U: reqwest::IntoUrl,
{
    use reqwest::Client;

    let client = Client::new();

    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(crate::Errors::NotFoundError);
    }

    Ok(response.text().await?)
}
