use std::time::Instant;

use gdnative::{api::{InputEventAction, TileMap}, prelude::*};


/// The HelloWorld "class"
//every class needs one of these structs
#[derive(NativeClass)]
#[inherit(TileMap)]
pub struct TileFG
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

// You may add any number of ordinary `impl` blocks as you want. However, ...
impl TileFG {
    /// The "constructor" of the class.
    fn new(_owner: &TileMap) -> Self {
        let sand = VariantArray::new_shared();       
        let lava = VariantArray::new_shared();
        let sand_mass = 0;
        let lava_mass = 0;
        let mass = 0;
        let freezemap = false;


        TileFG
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

// Only __one__ `impl` block can have the `#[methods]` attribute per class / script, which
// will generate code to automatically bind any exported methods to Godot.
//this is the entry point kinda for the program?
//its the "front end" of our native module
#[methods]
impl TileFG {

    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&self, _owner: TRef<TileMap>) 
    {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello, lool this is updated!");
        godot_print!("rip");
        godot_print!("{}",_owner.to_string());
        godot_print!("{}", Direction::Top as i32);
        _owner.set_process_input(true);
    }


    #[export]
    fn _enter_tree(&self, _owner: &TileMap)
    {
        godot_print!("printfirstplzzzzzz");
        
    }

    #[export]
    fn _physics_process(&mut self, _owner: &TileMap, delta: f32)
    {


    }

    #[export]
    fn _on_timer_timeout(&mut self, _owner: &TileMap)
    {
        //in theory i should have at least 250ms of processing time

             //  self.lava = track_lava();
        self.track_sand(_owner);
        //  lava_gravity
        //this takes 16ms with a bunch of 1000 sands
        self.sand_physics(_owner);


    }


    //this handles game input
    //its all the unhandled input, so if a ui captures and uses an event in _input, then this wont be able
    //to see that event
    // #[export]
    // fn _unhandled_input(&self, _owner: &TileMap, event: Variant)
    // {
    //     //convert event variant into event refInputEvent
    //     let event:Option<Ref<InputEvent>> = event.try_to_object();
    //     //now we can handle the events
    //     match event
    //     {
    //         None => {},
    //         Some(input) => 
    //         {
    //             //define an action thats in the action map in godot
    //             let action = "ui_select";

    //             //this is slow, but checks if space was pressed.
    //             unsafe{if input.assume_safe().is_action_pressed(GodotString::from(action), false)
    //             {
    //                 godot_print!("spacebarhomie");
    //                 godot_print!("{:?}", self.sand.len());
    //             }
    //             }
    //         }
    //     }

    // }

    //this tracks sand and updates mass
    fn track_sand(&mut self,_owner: &TileMap)
    {
    
        let sand_cells = _owner.get_used_cells_by_id(4);
       // self.sand_mass = sand_cells.len()*32;
        self.sand = sand_cells;


    }

    fn sand_physics(&self,_owner: &TileMap)
    {
        //multithread? does each cell have independent state?
        for sand in self.sand.iter()
        {

            let sand = sand.to_vector2();

            //all this logic will be replaced
            let rotationfactor = self.find_rotation(&sand);
            let mut rawrf = rotationfactor as i32;
            let mut by = self.move_options(&sand);
            let mut moveoptions = by.clone();
            let by = self.neighbors(&mut by, &mut rawrf, _owner);
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

    //this defines a cell's downward motion
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
    //provides neighbors after rotation transform
     //tail recursion!!! poggies champlord
    fn neighbors(&self, by: &mut Vec<Vector2>, rotationfactor: &mut i32, _owner: &TileMap) -> Vec<i64>
    {
        if *rotationfactor == 0
        {
            return self.get_neighbors(by,_owner);
        }
        else
        {
            let val = by.remove(0);
            by.push(val);
            *rotationfactor += -1;
            return self.neighbors(by, rotationfactor, _owner);
        }


    }

    //this can be much smarter
    //reduce useage by tracking duplicate neighbors (if cell 0,0 neighbors 1,0, then 1,0 obviously has a neighbor
        // at 1,0 so no need to check there)

    //ignore neigbors that we arent moving towards
    
    fn get_neighbors(&self, by: &Vec<Vector2>, _owner: &TileMap) -> Vec<i64>
    {   
        vec![
            _owner.get_cellv(by[0]),
            _owner.get_cellv(by[1]),
            _owner.get_cellv(by[2]),
            _owner.get_cellv(by[3]),
            _owner.get_cellv(by[4]),
            _owner.get_cellv(by[5]),
            _owner.get_cellv(by[6]),
            _owner.get_cellv(by[7]),
        ]
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

    // //this will define a cells downward motion 
    // //it does this by tracking a "true angle, "
    // fn center_tracker(
    //     &self,
    //     sand: &Vector2,
    //     look_ahead: &i8,
    //     last_ang: &f32,
    //     last_direction: &Direction,
    //     true_ang: &f32,
    // )-> Direction
    // {
    //     //for simple cases just check downward neighbor
    //     //if downword neighbor is blocked (so the cell is pressed against it)
    //     //choose left or right neighbor ONLY ONCE as sand
    //     //water could chose left or right continuously 
       

        

    //     //simple case 1
    //     //if aligned with y axis
    //     if   1.45 < true_ang < 1.65
    //     {
    //         return  Direction::Top;   
    //     }
    //     else if -1.65 < true_ang < -1.45
    //     {
    //         return Direction::Down;
    //     }
    //     //simple case 2
    //     //if aligned with x axis
    //     else if -0.12 < true_ang < 0.12
    //     {
    //         return  Direction::Right;
    //     }
    //     else if 2.98 < true_ang < -2.98
    //     {
    //         return  Direction::Left;
    //     }
    //     //complex case
    //     else
    //     {

    //     }



    // }


}
