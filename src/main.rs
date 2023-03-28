use std::process;
use std::env;

fn main() {
    if env::args().len() < 2 {
        eprintln!("Incorrect arguments: lipac <operation> <package>");
        process::exit(1);
    }
    let args: Vec<String> = env::args().skip(1).collect();

    // insert pacman handling here
    if args[1].chars().nth(1).unwrap() != 'S' {
        let err = exec::execvp("pacman", &args);
        println!("Error: {}", err);
        process::exit(2);
    }
    
    let (standard_repo_packages, other_packages) = pacman_check(args);
    println!("{:?}", standard_repo_packages);

    let (aur_packages, invalid_packages) = aur_check(other_packages);

    println!("AUR Packages: {:?}", aur_packages);
    println!("Invalid Packages: {:?}", invalid_packages);
}

fn pacman_check(pkg_list: Vec<String>) -> (Vec<String>, Vec<String>){
    // check if packages are in the standard repo
    // return a vector of packages that are in the standard repo and invalid packages

    println!("{:?}", pkg_list);

    return (Vec::new(), Vec::new());
}

fn aur_check(pkg_list: Vec<String>) -> (Vec<String>, Vec<String>){
    let mut rpc_url = "https://aur.archlinux.org/rpc/?v=5&type=info".to_owned();

	for package in pkg_list {
		rpc_url.push_str("&arg[]=");
        rpc_url.push_str(&package);
	}

    let resp = reqwest::get(&rpc_url);

    
    // get rpc_url and parse it



    // return a vector of packages that are in the AUR and invalid packages

    return (Vec::new(), Vec::new());
}  

/* /* usage:  pacman {-S --sync} [options] [package(s)]
options:
  -b, --dbpath <path>  set an alternate database location
  -c, --clean          remove old packages from cache directory (-cc for all)
  -d, --nodeps         skip dependency version checks (-dd to skip all checks)
  -g, --groups         view all members of a package group
                       (-gg to view all groups and members)
  -i, --info           view package information (-ii for extended information)
  -l, --list <repo>    view a list of packages in a repo
  -p, --print          print the targets instead of performing the operation
  -q, --quiet          show less information for query and search
  -r, --root <path>    set an alternate installation root
  -s, --search <regex> search remote repositories for matching strings
  -u, --sysupgrade     upgrade installed packages (-uu enables downgrades)
  -v, --verbose        be verbose
  -w, --downloadonly   download packages but do not install/upgrade anything
  -y, --refresh        download fresh package databases from the server
                       (-yy to force a refresh even if up to date)
      --arch <arch>    set an alternate architecture
      --asdeps         install packages as non-explicitly installed
      --asexplicit     install packages as explicitly installed
      --assume-installed <package=version>
                       add a virtual package to satisfy dependencies
      --cachedir <dir> set an alternate package cache location
      --color <when>   colorize the output
      --config <path>  set an alternate configuration file
      --confirm        always ask for confirmation
      --dbonly         only modify database entries, not package files
      --debug          display debug messages
      --disable-download-timeout
                       use relaxed timeouts for download
      --gpgdir <path>  set an alternate home directory for GnuPG
      --hookdir <dir>  set an alternate hook location
      --ignore <pkg>   ignore a package upgrade (can be used more than once)
      --ignoregroup <grp>
                       ignore a group upgrade (can be used more than once)
      --logfile <path> set an alternate log file
      --needed         do not reinstall up to date packages
      --noconfirm      do not ask for any confirmation
      --noprogressbar  do not show a progress bar when downloading files
      --noscriptlet    do not execute the install scriptlet if one exists
      --overwrite <glob>
                       overwrite conflicting files (can be used more than once)
      --print-format <string>
                       specify how the targets should be printed
      --sysroot        operate on a mounted guest system (root-only)*/ */