# BunchOfGits
A rust command line application for managing polyrepo madness


## Project Goals:
- Learn Rust
- Learn how to build a CLI tool
- Develop a deeper understanding of git and version control
- Make some friends along the way

## Problem Statement
It can be very cumbersome to jungle dependent branches across different repositories when working in a highly distributed repository.
Call them poly-repos or multi-repos, it doesn't matter, the cost of constantly switching branches across multiple repos to get your computer to a specific devlopment state can be a enormous. 

I'm imagining:
- a templating system for common workflows
- a way to switch to a set of branches quickly (hopefully in one command no matter how many branches)
- not having to worry about stashing/poping


We will leverage git worktrees and a custom cli tool(operating under the working name BunchOfGits, and called with
``` bash 
buncha [Options] 
```



## Teaser For What is To Come: 

Quickly switch between git repositories and specified branches to get your machine to the
right state for the feature or bug changes you care about right now. Bookmarks for git workflows.
Just a bunch of gits.

Usage: bunch_of_gits <COMMAND>

Commands:
  add       Adds the current repository to a specific bunch
  switch    Switch to the bunch configuration specified
  list      List all available bunch configurations and their settings
  init      Initialize bunch of gits with a new config file
  new       Create a new bunch
  template  Set up a template
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
