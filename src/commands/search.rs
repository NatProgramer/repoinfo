use reqwest::Client;
use std::process;

pub async fn search_repos(repository: &str, page: u16, per_page: u16) {
    let base_url = "https://api.github.com";
    let client = Client::new();

    let request = client
        .get(format!(
            "{base_url}/search/repositories?q={repository}&per_page={per_page}&page={page}"
        ))
        .header("User-Agent", repository)
        .send()
        .await;

    match request {
        Ok(response) => {
            let repo_list = response.json::<RepoSearchResult>().await;

            match repo_list {
                Ok(content) => {
                    println!("Results: {:#?}", content.total_count);
                    println!("Showed: {}", per_page);

                    for repo in content.items {
                        println!("{}", format!("{:->135}", ""));
                        println!("Owner: {}", repo.owner.login);
                        println!(
                            "Name: {name} {main_branch} {desc} {stars} {issues}  {forks}",
                            name = if repo.name.len() > 10 {
                                format!("{:<20}", format!("{}...", &repo.name[0..10]))
                            } else {
                                format!("{:<20}", repo.name)
                            },
                            main_branch = format!("{:<30}", format!("󰘬 {}", repo.default_branch)),
                            desc = match repo.description {
                                Some(description) => {
                                    if description.len() > 25 {
                                        format!(
                                            "{:<50}",
                                            format!("{}...", &description[0..25].trim())
                                        )
                                    } else {
                                        format!("{:<50}", description)
                                    }
                                }
                                None => {
                                    String::from(format!("{:<50}", "No description"))
                                }
                            },
                            stars = format!("{:<10}", format!(" {}", repo.stargazers_count)),
                            forks = format!("{:<10}", format!(" {}", repo.forks)),
                            issues = format!("{:<10}", format!(" {}", repo.open_issues))
                        );

                        match repo.homepage {
                            Some(homepage) => {
                                if homepage.len() > 0 {
                                    println!("Homepage 󰋜 : {}", homepage)
                                }
                            }
                            None => (),
                        }
                    }
                }
                Err(err) => {
                    eprintln!("The program has exited with error: {}", err);
                    panic!()
                }
            };
        }
        Err(err) => {
            eprintln!("Program has exited with error: {}", err);
            process::exit(1)
        }
    }
}

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct OwnerUser {
    login: String,
}

#[derive(Debug, Deserialize)]
struct RepoSearchResultItem {
    name: String,
    description: Option<String>,
    owner: OwnerUser,
    forks: u64,
    default_branch: String,
    homepage: Option<String>,
    open_issues: u64,
    stargazers_count: u64,
}

#[derive(Debug, Deserialize)]
struct RepoSearchResult {
    total_count: i64,
    items: Vec<RepoSearchResultItem>,
}