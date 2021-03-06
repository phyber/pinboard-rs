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

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function since we want the same PostAddBuilder for each test.
    fn setup() -> PostAddBuilder<'static> {
        PostAddBuilder::new(
            "http://example.org",
            "Just an example"
            )
    }

    #[test]
    fn url() {
        let pb = setup();

        assert_eq!(pb.url, "http://example.org");
    }

    #[test]
    fn description() {
        let pb = setup();

        assert_eq!(pb.description, "Just an example");
    }

    #[test]
    fn dt() {
        let mut pb = setup();
        pb.dt("test string");

        assert_eq!(pb.dt, Some("test string"));
    }

    #[test]
    fn extended() {
        let mut pb = setup();
        pb.extended("Some kind of description");

        assert_eq!(pb.extended, Some("Some kind of description"));
    }

    #[test]
    fn replace_false() {
        let mut pb = setup();
        pb.replace(false);

        assert_eq!(pb.replace, Some("no"));
    }

    #[test]
    fn replace_true() {
        let mut pb = setup();
        pb.replace(true);

        assert_eq!(pb.replace, Some("yes"));
    }

    #[test]
    fn shared_false() {
        let mut pb = setup();
        pb.shared(false);

        assert_eq!(pb.shared, Some("no"));
    }

    #[test]
    fn shared_true() {
        let mut pb = setup();
        pb.shared(true);

        assert_eq!(pb.shared, Some("yes"));
    }

    #[test]
    fn toread_false() {
        let mut pb = setup();
        pb.toread(false);

        assert_eq!(pb.toread, Some("no"));
    }

    #[test]
    fn toread_true() {
        let mut pb = setup();
        pb.toread(true);

        assert_eq!(pb.toread, Some("yes"));
    }

    #[test]
    fn build() {
        let mut pb = setup();

        let res = pb.extended("a description")
            .dt("a time")
            .replace(false)
            .shared(false)
            .toread(true)
            .build();

        assert!(res.is_ok(), "Post Add Build failed");

        let args = res.unwrap();

        assert_eq!(args.get("url"), Some(&String::from("http://example.org")));
        assert_eq!(args.get("description"), Some(&String::from("Just an example")));
        assert_eq!(args.get("extended"), Some(&String::from("a description")));
        assert_eq!(args.get("dt"), Some(&String::from("a time")));
        assert_eq!(args.get("replace"), Some(&String::from("no")));
        assert_eq!(args.get("shared"), Some(&String::from("no")));
        assert_eq!(args.get("toread"), Some(&String::from("yes")));
    }
}
