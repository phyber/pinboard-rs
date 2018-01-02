extern crate serde_json;

use error::CliError;
use super::GetArgs;

pub struct PostAddBuilder<'a> {
    url: &'a str,
    description: &'a str,
    extended: Option<&'a str>,
    tags: Option<Vec<&'a str>>,
    dt: Option<&'a str>,
    replace: Option<&'a str>,
    shared: Option<&'a str>,
    toread: Option<&'a str>,
}

impl<'a> PostAddBuilder<'a> {
    pub fn new(url: &'a str, description: &'a str) -> Self {
        Self {
            url: url,
            description: description,
            extended: None,
            tags: None,
            dt: None,
            replace: None,
            shared: None,
            toread: None,
        }
    }

    pub fn build(&self) -> Result<GetArgs, CliError> {
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
            let tags_str = self.tags.clone().unwrap().join(","); //mut_tags.join(",");
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

    pub fn dt(&mut self, dt: &'a str) -> &mut Self {
        self.dt = Some(dt);
        self
    }

    pub fn extended(&mut self, extended: &'a str) -> &mut Self {
        self.extended = Some(extended);
        self
    }

    pub fn replace(&mut self, replace: bool) -> &mut Self {
        let toggle = match replace {
            false => "no",
            true => "yes",
        };

        self.replace = Some(toggle);
        self
    }

    pub fn shared(&mut self, shared: bool) -> &mut Self {
        let toggle = match shared {
            false => "no",
            true => "yes",
        };

        self.shared = Some(toggle);
        self
    }

    pub fn tags(&mut self, tags: Vec<&'a str>) -> &mut Self {
        self.tags = Some(tags);
        self
    }

    pub fn toread(&mut self, toread: bool) -> &mut Self {
        let toggle = match toread {
            false => "no",
            true => "yes",
        };

        self.toread = Some(toggle);
        self
    }
}
