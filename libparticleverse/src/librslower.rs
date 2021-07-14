
use std::{convert::TryInto, ops::Index};

use gdnative::{api::{InputEventAction, TileMap}, prelude::*};

/// The HelloWorld "class"
//every class needs one of these structs
#[derive(NativeClass)]
#[inherit(TileMap)]
pub struct HelloWorld
{
    sand:VariantArray,
    lava:VariantArray,
    sand_mass:i32,
    lava_mass:i32,
    mass: i32,
    freezemap:bool,

}

enum TileType
{
    Obsidian,
    Sand,
    Lava,

}

#[derive(Clone, Copy)]
enum Direction
{
    Top = 0,
    TopRight = 1,
    Right = 2,
    DownRight = 3,
    Down = 4,
    DownLeft = 5,
    Left = 6,
    TopLeft = 7
}



// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<HelloWorld>();
}

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_owner: &TileMap) -> Self {
        let sand = VariantArray::new_shared();       
        let lava = VariantArray::new_shared();
        let sand_mass = 0;
        let lava_mass = 0;
        let mass = 0;
        let freezemap = false;

        HelloWorld
        {
            sand,
            lava,
            sand_mass,
            lava_mass,
            mass,
            freezemap, 
        }
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
//this is the entry point kinda for the program?
//its the "front end" of our native module
#[methods]
impl HelloWorld {

    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: &TileMap) 
    {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello, lool world!");
        godot_print!("rip");
        godot_print!("{}",_owner.to_string());
        godot_print!("{}", Direction::Top as i32);
        //_owner.set_process_input(true);

    }


    #[export]
    fn _enter_tree(&self, _owner: &TileMap)
    {
        godot_print!("printfirstplzzzzzz");
        
    }

    #[export]
    fn _physics_process(&mut self, _owner: &TileMap, delta: f32)
    {
      //  self.lava = track_lava();
        self.track_sand(_owner);
      //  lava_gravity
        self.sand_physics(_owner);


        
    }

    //this handles game input
    //its all the unhandled input, so if a ui captures and uses an event in _input, then this wont be able
    //to see that event
    #[export]
    fn _unhandled_input(&self, _owner: &TileMap, event: Variant)
    {
        //convert event variant into event refInputEvent
        let event:Option<Ref<InputEvent>> = event.try_to_object();
        //now we can handle the events
        match event
        {
            None => {},
            Some(input) => 
            {
                //define an action thats in the action map in godot
                let action = "ui_select";

                //this is slow, but checks if space was pressed.
                unsafe{if input.assume_safe().is_action_pressed(GodotString::from(action), false)
                {
                    godot_print!("spacebarhomier");
                    godot_print!("{:?}", self.sand.len());
                }
                }
            }
        }

    }

    //this tracks sand and updates mass
    fn track_sand(&mut self,_owner: &TileMap)
    {
    
        //here we already track all the sand,
        //based off all the tracking we should be able to know what the neighbors are
        //and skip using getcellv()
        let sand_cells = _owner.get_used_cells_by_id(4);
        self.sand_mass = sand_cells.len()*32;
        self.sand = sand_cells;

    }

    fn sand_physics(&self,_owner: &TileMap)
    {   

        for sand in self.sand.iter()
        {
            let sand = sand.to_vector2();
            let rotationfactor = self.find_rotation(&sand);
            let mut rawrf = rotationfactor as i32;
            let mut by = self.move_options(&sand);
            let mut moveoptions = by.clone();
            let by = self.neighbors(&mut by, &mut rawrf);
            let cell = _owner.get_cellv(sand);
            rawrf = rotationfactor as i32;
            moveoptions = self.relative_move(&mut moveoptions, &mut rawrf);
            if by[4] == -1
            {
                _owner.set_cellv(moveoptions[4], cell, false, false, false);
                _owner.set_cellv(sand, -1, false, false, false);

            }
            else if by[5] == -1
            {
                _owner.set_cellv(moveoptions[5], cell, false, false, false);
                _owner.set_cellv(sand, -1, false, false, false);
 
            }
            else if by[3] == -1
            {
                _owner.set_cellv(moveoptions[3], cell, false, false, false);
                _owner.set_cellv(sand, -1, false, false, false); 

            }
            else
            {
                continue;
            }

        }
       

    }

    fn find_rotation(&self,cell: &Vector2) -> Direction
    {
        let cellangle = cell.angle_from_x_axis().radians;



        if -5.0*3.1416/8.0 <= cellangle && cellangle < -3.0*3.1416/8.0
        {
            return Direction::Top
        }
        else if -3.0*3.1416/8.0 <= cellangle && cellangle < -3.1416/8.0
        {
            return Direction::TopRight
        }
        else if -3.1416/8.0 <= cellangle && cellangle < 3.1416/8.0
        {
            return Direction::Right
        }
        else if 3.1416/8.0 <= cellangle && cellangle < 3.0*3.1416/8.0
        {
            return Direction::DownRight
        }
        else if 3.0*3.1416/8.0 <= cellangle && cellangle < 5.0*3.1416/8.0
        {
            return Direction::Down
        }
        else if 5.0*3.1416/8.0 <= cellangle && cellangle < 7.0*3.1416/8.0
        {
            return Direction::DownLeft;
        }
        else if 7.0*3.1416/8.0 <= cellangle && cellangle < 3.1416
        {
            return Direction::Left;
        }
        else
        {
            return Direction::TopLeft
        }

        
    }

    //provides a vec of new coordinates to move, is also true neighbors
    fn move_options(&self, cell: &Vector2) -> Vec<Vector2>
    {
        vec![
            Vector2::new(cell.x, cell.y-1.0),
            Vector2::new(cell.x+1.0,cell.y-1.0),
            Vector2::new(cell.x+1.0,cell.y),
            Vector2::new(cell.x+1.0,cell.y+1.0),
            Vector2::new(cell.x,cell.y+1.0),
            Vector2::new(cell.x-1.0,cell.y+1.0),
            Vector2::new(cell.x-1.0,cell.y),
            Vector2::new(cell.x-1.0,cell.y-1.0)
        ]
    }
    //half of my time is spent in this call
    //provides neighbors after rotation transform
    fn neighbors(&self, by: &mut Vec<Vector2>, rotationfactor: &mut i32) -> Vec<i64>
    {
        if *rotationfactor == 0
        {
            return self.get_neighbors(by);
        }
        else
        {
            let val = by.remove(0);
            by.push(val);
            *rotationfactor += -1;
            return self.neighbors(by, rotationfactor);
        }

    }

    //this is expensive
    //this needs to be smart
    fn get_neighbors(&self, by: &Vec<Vector2>) -> Vec<i64>
    { 
        //array with positon of all sand particles (currently only type on map)
        let sands: Vec<Vector2>= self.sand.iter().map(|p|p.to_vector2()).collect();

        let mut neighborz: Vec<i64> = vec![];

        for pos in by
        {
            match sands.binary_search_by(
            {
                Ok(_) => neighborz.push(4),
                Err(_) => (),

            }   
            else
            {
                neighborz.push(0);
            }
        }


        return neighborz;

        
    }



    fn relative_move(&self, moveoptions: &mut Vec<Vector2>, rotationfactor: &mut i32) -> Vec<Vector2>
    {
        if *rotationfactor == 0
        { 
            return moveoptions.clone();
        }
        else
        {
            let val = moveoptions.remove(0);
            moveoptions.push(val);
            *rotationfactor += -1;
            return self.relative_move(moveoptions, rotationfactor);
        }


    }


}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);


