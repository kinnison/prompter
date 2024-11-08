//! Implementation of data sources

use std::path::{Path, PathBuf};

use git2::{Oid, Repository, RepositoryOpenFlags, StatusOptions};

use crate::config::DataSource;

impl DataSource {
    fn max_needed(&self, psvars: &mut Vec<String>) {
        use DataSource::*;
        let needed = match self {
            Git(base) => *base + 4,
            Sudo(n) => *n,
            Rust(n) => *n,
            Key(n) => *n,
            Flake(n) => *n,
        } + 1;
        while psvars.len() < needed {
            psvars.push(String::new());
        }
    }
    pub fn fill_vars(&self, psvars: &mut Vec<String>) {
        self.max_needed(psvars);
        use DataSource::*;
        match self {
            Git(base) => fill_git_vars(*base, psvars),
            Sudo(var) => psvars[*var] = calc_sudo(),
            Rust(var) => psvars[*var] = calc_rust(),
            Key(var) => psvars[*var] = calc_key(),
            Flake(var) => psvars[*var] = calc_flake(),
        }
    }
}

// label, prepath, info, postpath
fn fill_git_vars(base: usize, psvars: &mut [String]) {
    let Ok(repo) =
        Repository::open_ext(".", RepositoryOpenFlags::CROSS_FS, std::env::var_os("HOME"))
    else {
        return;
    };
    // Okay we definitely found a git repo
    psvars[base] = "git".into();
    // Split PWD into before repo and after repo
    let mut root = repo.path().to_owned();
    if root.ends_with(".git/") {
        root = root.parent().unwrap().to_owned();
    }
    let cwd = std::env::current_dir().unwrap();
    let rest = cwd.strip_prefix(&root).unwrap_or(Path::new(""));
    let home = std::env::var_os("HOME").unwrap_or("!!".into());
    if let Ok(stuff) = root.strip_prefix(home) {
        root = Path::new("~").join(stuff);
    }
    psvars[base + 1] = format!("{}", root.display());
    psvars[base + 3] = format!("/{}", rest.display());
    // Do we have a head
    let (head, code): (String, Oid) = if let Ok(head) = repo.head() {
        (
            if head.is_tag() {
                format!("tag:{}", head.shorthand().unwrap_or("!"))
            } else {
                head.shorthand().unwrap_or("!").into()
            },
            head.target().unwrap_or(Oid::zero()),
        )
    } else {
        // No head
        (String::from("UNKNOWN"), Oid::zero())
    };
    let mut opts = StatusOptions::new();
    opts.show(git2::StatusShow::IndexAndWorkdir)
        .include_untracked(true)
        .renames_head_to_index(true)
        .renames_index_to_workdir(true)
        .renames_from_rewrites(true)
        .update_index(true);
    let stats = if let Ok(stats) = repo.statuses(Some(&mut opts)) {
        // We have some statuses to check
        let mut conflicted = 0;
        let mut unknown = false;
        let mut added = false;
        let mut renamed = false;
        let mut removed = false;
        let mut modified = false;
        for entry in stats.iter() {
            let status = entry.status();
            if status.is_conflicted() {
                conflicted += 1;
            }
            unknown |= status.is_wt_new(); // Untracked files
            added |= status.is_index_new();
            renamed |= status.is_index_renamed() | status.is_wt_renamed();
            removed |= status.is_index_deleted() | status.is_wt_deleted();
            modified |= status.is_index_modified() | status.is_wt_modified();
        }
        let mut ret = String::new();
        if unknown {
            ret.push('?')
        }
        if added {
            ret.push('A')
        }
        if removed {
            ret.push('D')
        }
        if renamed {
            ret.push('R')
        }
        if modified {
            ret.push('M')
        }
        if conflicted > 0 {
            ret.push_str(&format!("C[{conflicted}]"));
        }
        if ret.is_empty() {
            ret
        } else {
            ret.insert(0, ',');
            ret
        }
    } else {
        String::new()
    };
    // Info time
    // branch:sha(,stats)
    psvars[base + 2] = format!("{head}:{code:.7}{stats}")
}

fn calc_sudo() -> String {
    String::new()
}
fn calc_rust() -> String {
    String::new()
}
fn calc_key() -> String {
    String::new()
}
fn calc_flake() -> String {
    if find_upwards("flake.nix").is_some() {
        String::from("❄ ")
    } else {
        String::new()
    }
}

fn find_upwards(stem: impl AsRef<Path>) -> Option<PathBuf> {
    let stem = stem.as_ref();
    let here = std::env::current_dir().unwrap();
    let mut here = here.as_path();
    while std::fs::metadata(here.join(stem)).is_err() {
        here = here.parent()?;
    }
    Some(here.join(stem))
}
