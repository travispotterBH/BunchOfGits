
pub fn add(){
    //do something with this
    /*
    Need a known place for settings. I guess always look in the root directory or there should be an os specific location
    Read in settings
    Get the bunch with the name
    modify the bunch by adding an additional config
    write settings back
    return confimration
 */
}


pub fn delete(){
    //do something with this
    /*
    read in settings
    get the bunch with the same name 
    modify the bunch by deleting the relavant part of the config
    write settings back
    return confirmaiton
    */
}

pub fn switch(){
    //do something with this
    /*
        read in settings
        given the name of the bunch, find the bunch
        for each config iem in the bunch, change the repo and branch
        handle needing to stash on change, default to yes stash
        run build script for each branch in dependency order
        return back confirmation
    */
}

pub fn previous(){
    //do something with this
    /*
        get previous state name from settings
        call switch on a saved previous state command.
        write to settings the new previous state name
     */
}

pub fn list(){
    //do something with this
    /*
        get settings
        get all bunches
        write to command line
     */
}

pub fn init(){
//initialize a new bunch current branch/repo

}
 pub fn go (){
    //(take in a bunch name, and then run needed commands to switch over all the branches)
}
 pub fn prev(){

   // (go to the previous bunch) (only got back and forth, it is less like undo, and more like a carousel)
}

 pub fn new(){

    //(create new set of branches in repos based on bunch template)
}

 pub fn config(){

    //(opens toml config file in text editor to set global settings) or flags/args inline to set
}
 pub fn pull(){
 //(switch to branches from bunch collection, and pull down latest, just fail on merge conflicts)
}
 pub fn push(){

    //(switch to brances from bunch collection, and push to remote, does not handle tracking files or committing -- user to do this)
}
 pub fn template (){

//(takes in arguments vector of repo names to be used for common development workflows) ex. three repos are commonly involved in each feature development. allow for quickly creating a branch on each of the repos of the same name
}

