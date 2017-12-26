extern crate serde_json;

use error::CliError;
use super::GetArgs;

pub struct PostAddBuilder {
    url: String,
    description: String,
    extended: Option<String>,
    tags: Option<Vec<String>>,
    dt: Option<String>,
    replace: Option<String>,
    shared: Option<String>,
    toread: Option<String>,
}

impl PostAddBuilder {
    pub fn new(url: &str, description: &str) -> Self {
        Self {
            url: url.to_owned(),
            description: description.to_owned(),
            extended: None,
            tags: None,
            dt: None,
            replace: None,
            shared: None,
            toread: None,
        }
    }

    pub fn build(self) -> Result<GetArgs, CliError> {
        // Process above struct here.
        let mut args = GetArgs::new();
        args.insert("url".to_string(), self.url.to_owned());
        args.insert("description".to_string(), self.description.to_owned());

        // This is horrible. Work out how match works in this case.
        if self.extended != None {
            args.insert("extended".to_string(), self.extended.unwrap().to_owned());
        }

        if self.tags != None {
            //let mut_tags = self.tags.unwrap();
            let tags_str = self.tags.unwrap().join(","); //mut_tags.join(",");
            args.insert("tags".to_string(), tags_str.to_owned());
        }

        if self.dt != None {
            args.insert("dt".to_string(), self.dt.unwrap().to_owned());
        }

        if self.replace != None {
            args.insert("replace".to_string(), self.replace.unwrap().to_owned());
        }

        if self.shared != None {
            args.insert("shared".to_string(), self.shared.unwrap().to_owned());
        }

        if self.toread != None {
            args.insert("toread".to_string(), self.toread.unwrap().to_owned());
        }

        Ok(args)
    }

    pub fn dt<'a>(&'a mut self, dt: &str) -> &'a mut Self {
        self.dt = Some(dt.to_owned());
        self
    }

    pub fn extended<'a>(&'a mut self, extended: &str) -> &'a mut Self {
        self.extended = Some(extended.to_owned());
        self
    }

    pub fn replace<'a>(&'a mut self, replace: bool) -> &'a mut Self {
        let toggle = match replace {
            false => "no".to_owned(),
            true => "yes".to_owned(),
        };

        self.replace = Some(toggle);
        self
    }

    pub fn shared<'a>(&'a mut self, shared: bool) -> &'a mut Self {
        let toggle = match shared {
            false => "no".to_owned(),
            true => "yes".to_owned(),
        };

        self.shared = Some(toggle);
        self
    }

    pub fn tags<'a>(&'a mut self, tags: Vec<String>) -> &'a mut Self {
        self.tags = Some(tags.to_owned());
        self
    }

    pub fn toread<'a>(&'a mut self, toread: bool) -> &'a mut Self {
        let toggle = match toread {
            false => "no".to_owned(),
            true => "yes".to_owned(),
        };

        self.toread = Some(toggle);
        self
    }
}
