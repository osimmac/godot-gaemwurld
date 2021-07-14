

use gdnative::prelude::*;

mod spacemagic;





// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) 
{
    // Register `SpaceMagic` type declared in spacemagic module
    //you can register multiple types / classes here
    //each of these is a seperate native script that can be attached to a node

   //handle.add_class::<nativetimer::NativeTimer>();
    handle.add_class::<spacemagic::SpaceMagic>();

}


// Macro that creates the entry-points of the dynamic library.
godot_init!(init);


