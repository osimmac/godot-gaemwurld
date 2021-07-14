use gdnative::{api::{RigidBody2D}, prelude::*};


/// The HelloWorld "class"
//every class needs one of these structs
#[derive(NativeClass)]
#[inherit(RigidBody2D)]

pub struct TileBody
{

    

}

#[methods]
impl TileBody {


    fn new(_owner: &RigidBody2D) -> Self {



        TileBody
        {
 
        }
    }


    // To make a mthod known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: TRef<RigidBody2D>) 
    {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello, this is tilebody");


    }


}

