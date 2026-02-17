use std::fmt;
use std::io::stdin;
use std::io;
struct RobotsTxt {
    agent_block: Vec<Agent>,
    sitemap: Option<String>,
}

struct Agent {
    name: String,
    directives: Vec<Directive>,
}

enum Directive {
    Allow(String),
    Disallow(String),
}

#[derive(Debug)]
enum RobotsTxtError {
    DuplicateAgentName(String),
    InvalidPath(String),
}

impl RobotsTxt {
    fn new() -> Self {
        Self {
            agent_block: Vec::new(),
            sitemap: None,
        }
    }
}

impl RobotsTxt {
    fn add_agent(&mut self, agent:Agent) -> Result<(), RobotsTxtError> {
       if self.agent_block.iter().any(|a| a.name == agent.name) {
           return Err(RobotsTxtError::DuplicateAgentName(agent.name))
       }
        self.agent_block.push(agent);
        Ok(())
    }
}

impl RobotsTxt {
    fn add_sitemap (&mut self, sitemap_path:Option<String>) {
        self.sitemap = sitemap_path;
    }
}

impl Agent {
    fn new (name:String) -> Self {
        Self {
            name,
            directives: Vec::new(),
        }
    }
}

impl Agent {
    fn add_directive (&mut self, directive:Directive) {
        self.directives.push(directive);
    }
}

impl Directive  {
    fn new (path: String, allow: bool) -> Result<Self, RobotsTxtError> {
        if path.starts_with("/") {
            if allow {
                Ok(Directive::Allow(path))
            } else {
                Ok(Directive::Disallow(path))
            }
        } else {
            Err(RobotsTxtError::InvalidPath(path))
        }
    }
}

impl fmt::Display for Directive {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        match self {
            Directive::Allow(path) => {
                writeln!(f, "Allow: {}", path)
            }
            Directive::Disallow(path) => {
                writeln!(f, "Disallow: {}", path)
            }
        }
    }
}

impl fmt::Display for Agent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User-agent: {}\n", self.name)?;
        for directive in &self.directives {
            write!(f, "{}", directive)?;
        }
        Ok(())
    }
}

impl fmt::Display for RobotsTxt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for agent in &self.agent_block {
            write!(f, "{}", agent)?;
        }
        if let Some(url) = &self.sitemap {
            writeln!(f, "\nSitemap: {}", url)?;
        }
        Ok(())
    }
}

impl fmt::Display for RobotsTxtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RobotsTxtError::DuplicateAgentName(name) => {
            write!(f, "Hey, dingus... there's already a {} block", name)
            },
            RobotsTxtError::InvalidPath(name) =>
            write!(f, "a valid path must start with a forward slash (/), {} is not valid", name)
        }
    }
}

fn main() {
    let mut file = RobotsTxt::new();
    let mut is_first:bool = true;

    loop {
        if is_first == true {
            println!("Enter the first agent name to add (or type END to finish and create your robots.txt file.)");
        } else {
            println!("Enter the name of the next agent you want to add or END to finish");
        }
        is_first = false;
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read agent name");
        let name = input.trim();
        if name == "END" {
        break;
        }
        let mut a = Agent::new(name.to_string());
        loop {
            println!("Enter a path you want to control or type DONE to go back to agent selection.");
            let mut path = String::new();
            io::stdin().read_line(&mut path).expect("Failed to read path.");
            let path = path.trim();
            if path == "DONE" {
                break;
            }
            let mut rule:String = String::new();
            println!("The path is disallowed by default; type ALLOW to override the default or press Enter to continue.");
            io::stdin().read_line(&mut rule).expect("Failed to read rule");
            let rule = rule.trim();
            let choice = if rule == "ALLOW" {true} else {false};
            match Directive::new(String::from(path), choice) {
                Ok(d) => a.add_directive(d),
                Err(e) => eprintln!("Error: {}", e)
            }
            }
            file.add_agent(a);

        }
    println!("Enter the path to your sitemap or press Enter to finish and build the robots.txt file.");
    let mut sitemap = String::new();
    io::stdin().read_line(&mut sitemap).expect("Failed to reach sitemap path");
    let s = sitemap.trim();
    let sitemap_option = if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    };
    file.add_sitemap(sitemap_option);
    print!("{}", file);

    }





