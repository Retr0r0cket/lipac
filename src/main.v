module main

import os
import net.http
import json

fn main() {
	if os.args.len < 2 {
		println('Usage: lipac <operation> <package>')
		exit(1)
	}
	if os.args[1][1] != 'S' {
		os.execvp("pacman", if os.args[1] == '-Yc' {['-Rns", "$(pacman -Qqtd)']} else {os.args[1..]})
	}
	

}

fn get_aur_packages(pkglist []string) {
	rpc_url := "https://aur.archlinux.org/rpc/?v=5&type=info"
	for package in pkglist {
		rpc_url += "&arg[]=" + package
	}
	// rpc_json :=  

	// impliment if result count = pkglist.len just return pkglist
	valid_packages := 
	invalid_packages :=
	return valid_packages, invalid_packages
}

// TO-DO

// Use pacman -D to install packages as dependencies when needed

// Pacman -S --help:
/* usage:  pacman {-S --sync} [options] [package(s)]
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
      --sysroot        operate on a mounted guest system (root-only)*/