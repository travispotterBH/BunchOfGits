# BunchOfGits
A rust command line application for managing polyrepo madness


## Project Goals:
- Learn Rust
- Learn how to build a CLI tool
- Develop a deeper understanding of git and version control
- Develop a deeper understanding of streams, buffers, writing to and from files with serialized data
- Make some friends along the way


## Branch Strategy
- `main` : protected branch. Only merge from `develop`
- `develop` : merge feature and bug fix branches into develop. Delete feature and bug fix branch on merge.
- `#1-AddProjectFolderStructure` : Template for feature/bug fix branch. "#{Issue Number}-{Issue Name}".  All feature/bug fix branches should have associated issue.

