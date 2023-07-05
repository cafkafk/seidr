#[test]
fn main() {
    assert!(true);
}

/*
#[cfg(test)]
mod config {
    use gg::git::RepoFlags::{Clone, Push};
    use gg::git::{Category, Config, GitRepo, Link};
    use relative_path::RelativePath;
    use std::collections::HashMap;
    use std::env::current_dir;
    use std::fs::File;
    use std::io::prelude::*;
    #[test]
    fn init_config() {
        let _config = Config {
            categories: HashMap::new(),
        };
    }
    #[test]
    fn init_config_populate() {
        let default_category = Category {
            flags: Some(vec![]),
            repos: Some(HashMap::new()),
            links: Some(HashMap::new()),
        };
        let mut config = Config {
            categories: HashMap::new(),
        };
        config
            .categories
            .insert(format!("{}", 0).to_string(), default_category);
        for i in 0..=5 {
            config
                .categories
                .get_mut(&format!("{}", 0).to_string())
                .expect("category not found")
                .repos
                .as_mut()
                .expect("failed to get repo")
                .insert(
                    format!("{}", i).to_string(),
                    GitRepo {
                        name: "test repo".to_string(),
                        path: "/tmp".to_string(),
                        url: "https://github.com/cafkafk/gg".to_string(),
                        flags: Some(vec![Clone, Push]),
                    },
                );
        }
    }
    #[test]
    fn read_config_populate() {
        let _config = Config::new(&RelativePath::new("./src/test/config.yaml").to_string());
    }
    #[test]
    fn write_config() {
        let root = current_dir().expect("failed to get current dir");
        let config = Config::new(
            &RelativePath::new("./src/test/config.yaml")
                .to_logical_path(&root)
                .into_os_string()
                .into_string()
                .expect("failed to turn config into string"),
        );

        let mut test_file = File::create(
            RelativePath::new("./src/test/test.yaml")
                .to_logical_path(&root)
                .into_os_string()
                .into_string()
                .expect("failed to turn config into string"),
        )
        .expect("failed to create test file");
        let contents = serde_yaml::to_string(&config).expect("failed to turn config into string");
        test_file
            .write_all(contents.as_bytes())
            .expect("failed to write contents of config into file");

        let test_config = Config::new(&RelativePath::new("./src/test/test.yaml").to_string());
        assert_eq!(config, test_config);
    }
    #[allow(dead_code)]
    fn get_category<'cat>(config: &'cat Config, name: &'cat str) -> &'cat Category {
        config.categories.get(name).expect("failed to get category")
    }
    fn get_repo<F>(config: &Config, cat_name: &str, repo_name: &str, f: F)
    where
        F: FnOnce(&GitRepo),
    {
        f(config
            .categories
            .get(cat_name)
            .expect("failed to get category")
            .repos
            .as_ref()
            .expect("failed to get repo")
            .get(repo_name)
            .expect("failed to get category"))
    }
    fn get_link<F>(config: &Config, cat_name: &str, link_name: &str, f: F)
    where
        F: FnOnce(&Link),
    {
        f(config
            .categories
            .get(cat_name)
            .expect("failed to get category")
            .links
            .as_ref()
            .expect("failed to get repo")
            .get(link_name)
            .expect("failed to get category"))
    }
    #[test]
    fn is_config_readable() {
        let root = current_dir().expect("failed to get current dir");
        let config = Config::new(
            &RelativePath::new("./src/test/config.yaml")
                .to_logical_path(root)
                .into_os_string()
                .into_string()
                .expect("failed to turnn config into string"),
        );

        let _flags = vec![Clone, Push];
        // FIXME not very extensive
        #[allow(clippy::bool_assert_comparison)]
        {
            get_repo(&config, "config", "qmk_firmware", |repo| {
                assert_eq!(repo.name, "qmk_firmware");
                assert_eq!(repo.path, "/home/ces/org/src/git/");
                assert_eq!(repo.url, "git@github.com:cafkafk/qmk_firmware.git");
            });
            get_link(&config, "stuff", "gg", |link| {
                assert_eq!(link.name, "gg");
                assert_eq!(link.tx, "/home/ces/.dots/gg");
                assert_eq!(link.rx, "/home/ces/.config/gg");
            });
        }
        /*
        {
            assert_eq!(config.links[0].name, "gg");
            assert_eq!(config.links[0].rx, "/home/ces/.config/gg");
            assert_eq!(config.links[0].tx, "/home/ces/.dots/gg");
            assert_eq!(config.links[1].name, "starship");
            assert_eq!(config.links[1].rx, "/home/ces/.config/starship.toml");
            assert_eq!(config.links[1].tx, "/home/ces/.dots/starship.toml");
            // FIXME doesn't check repoflags
        }*/
    }
}*/

/*
#[cfg(test)]
mod repo_actions {
    use gg::git::GitRepo;
    use gg::relative_path::RelativePath;
    use gg::std::env::current_dir;
    use gg::std::process::Command;
    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_repo_actions() {
        let test_repo_name: String = "test".to_string();
        let root = current_dir().unwrap();
        let test_repo_dir: String = RelativePath::new("./src/test")
            .to_logical_path(&root)
            .into_os_string()
            .into_string()
            .unwrap();
        let test_repo_url: String = "git@github.com:cafkafk/test.git".to_string();
        println!("{}", test_repo_dir);
        let mut config = Config {
            repos: vec![],
            links: vec![],
        };
        let repo = GitRepo {
            name: test_repo_name.to_owned(),
            path: test_repo_dir.to_owned(),
            url: test_repo_url.to_owned(),
            clone: true,
        };
        config.repos.push(repo);
        // BUG FIXME can't do this in flake
        // should have a good alternative
        // config.clone_all();
        // config.pull_all();
        for r in config.repos.iter() {
            Command::new("touch")
                .current_dir(&(r.path.to_owned() + &r.name))
                .arg("test")
                .status()
                .expect("failed to create test file");
        }
        config.add_all();
        config.commit_all_msg(&"test".to_string());
    }
}
*/
