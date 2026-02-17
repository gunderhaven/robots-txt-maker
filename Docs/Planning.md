# Robots.txt maker

## What is a robots.txt file.

Each agent block is a struct with a vec of directives (one directive per path -- allow/disallow).



struct RobotsTxt {
    agent_block: vec of structs each with agent name and directives,
    sitemap: Option<String>,
}
