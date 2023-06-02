# BunchOfGits
A rust command line application for managing polyrepo madness



## Branch Strategy
- `main` : protected branch. Only merge from `develop`
- `develop` : merge feature and bug fiz branches into develop. Delete feature and bug fix branch on merge.
- `#1-AddProjectFolderStructure` : Template for feature/bug fix branch. "#{Issue Number}-{Issue Name}".  All feature/bug fix branches should have associated issue.
