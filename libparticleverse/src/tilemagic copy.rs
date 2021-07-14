use std:: ops::{Deref, Mul};

use gdnative::{api::{CircleShape2D, InputEventAction, Physics2DDirectBodyState, Physics2DDirectBodyStateSW, Physics2DServer, Physics2DServerSW, RectangleShape2D, TileMap, VisualServer, World2D}, prelude::*};


/// The HelloWorld "class"
//every class needs one of these structs
#[derive(NativeClass)]
#[inherit(TileMap)]
pub struct TileMagic 
{
    sand_pos:Vec<Vector3>,
    sand_shapes:VariantArray,
    sand_pos_last:Vec<Vector3>,
    next_pos:Vec<Vector3>,
    next_tick: bool,
    body_rids: Vec<Rid>,

}


const G:f32 = 1.0;
const E:f32 = 4.0;

// You may add any number of ordinary `impl` blocks as you want. However, ...



// Only __one__ `impl` block can have the `#[methods]` attribute per class / script, which
// will generate code to automatically bind any exported methods to Godot.
//this is the entry point kinda for the program?
//its the "front end" of our native module
#[methods]
impl TileMagic {

    fn new(_owner: &TileMap) -> Self {
    
        let sand_pos = vec![Vector3::zero()];
        let sand_pos_last = vec![Vector3::zero()];
        let next_pos = vec![Vector3::zero()];
        let sand_shapes = VariantArray::new_shared();


        TileMagic
        {
            sand_pos,
            sand_shapes,
            next_pos,
            sand_pos_last,
            next_tick: false,
            body_rids: vec![],



        }
    }



    // To make a method known to Godot, use the #[export] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "owner".
    //
    // In order to enable access to the owner, it is passed as the second
    // argument to every single exposed method. As a result, all exposed
    // methods MUST have `owner: &BaseClass` as their second arguments,
    // before all other arguments in the signature.
    #[export]
    fn _ready(&mut self, _owner: TRef<TileMap>) 
    {


        let x = 3;
        let y = 250;
        let mut particles = vec![];
        let space = 2;

        for i in 1..y {
            for j in 1..x{
                particles.push(Vector3::new(i as f32*space as f32, j as f32*space as f32, 4.1));
            }
    
        }
        self.sand_pos = particles.clone();
        self.sand_pos_last = particles.clone();
        self.next_pos = particles;

    //    _owner.call_deferred("method", varargs)

        self.add_particles_to_world(_owner);


            


    }


    //funcion used to add particles to world
    fn add_particles_to_world(&mut self, _owner: TRef<TileMap>) 
    {
      let particles = &self.sand_pos.clone();
   //   let last_particles = &self.sand_pos_last.clone();

      //get current world 2d to get space_rid to add particles to the game physics world 
      let world2d_ref = _owner.get_world_2d().unwrap();
      let world2d = unsafe{world2d_ref.assume_safe()};
      let world2d = world2d.deref();
      let space_rid = world2d.space();
      //create reference to accsess physics server
      let physics_server = unsafe{Physics2DServer::godot_singleton()};
      //create body 


      let mut index = 0usize;
      for particle in particles
      {
        let body = physics_server.body_create();
        self.body_rids.push(body);
        physics_server.body_set_mode(body, Physics2DServer::BODY_MODE_RIGID);
        //create shape
        let shape = CircleShape2D::new();
        shape.set_radius(1.0);
        //add shape to body for collision
        physics_server.body_add_shape(body, shape.get_rid(), Transform2D::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)   , false);
        //add body to space for collision
       physics_server.body_set_space(body, space_rid);
        //move initial position
        physics_server.body_set_state(body, Physics2DServer::BODY_STATE_TRANSFORM, Transform2D::new(0.0,0.0,0.0,0.0,particle.x,particle.y));
        //add callback to self that the body moved, last parameter is optional and can be used as index
        //if many bodies are there and a single callback?`
        physics_server.body_set_param(body, Physics2DServer::BODY_PARAM_LINEAR_DAMP, 0.0);
        physics_server.body_set_state(body, Physics2DServer::BODY_STATE_CAN_SLEEP, false);
    //    physics_server.collisi
        physics_server.body_set_force_integration_callback(body, _owner, "_body_moved", index);
        index+=1;
        physics_server.body_apply_central_impulse(body, Vector2::new(0.0, 0.0));

      }
    // self.display_particles(particles, _owner, last_particles);
      

      

      
    }

    //in progress, currently takes a list of particles and their color, then will display them on the
    //tile map they are associated with 
    fn display_particles(&self,particles:&Vec<Vector3>, _owner: TRef<TileMap>,_last_particles:&Vec<Vector3>)  
    {
        

        let mut index = 0usize;
        for particle in particles
        {
            let last = _last_particles[index];
 



            let last_tile_cords = _owner.world_to_map(Vector2::new(last.x, last.y));

         
            _owner.set_cellv(last_tile_cords, -1i64, false, false, false);
            index+=1;
        }

        for particle in particles{
            let position = Vector2::new(particle.x,particle.y);
            let particle_color = particle.z as i64;

            let tile_coords = _owner.world_to_map(position);
            _owner.set_cellv(tile_coords, particle_color,false, false, false);

        }

        
    
        
    }

    //when called, it will write the updates to the positions, and type of a particle
    fn update_particles(&mut self)
    {
        self.sand_pos_last = self.sand_pos.clone();
        self.sand_pos = self.next_pos.clone();
                    //odot_print!("{}",self.next_pos.len());
       // self.next_pos.clear();


    }

    fn sweet_gravity(&mut self, delta:f32)
    {
        let physics_server = unsafe{Physics2DServer::godot_singleton()};
        let mut index = 0usize;
        for particle in self.next_pos.clone()
        {
            let mut force = Vector2::zero();
            let mut otherforce = Vector2::zero();
            'papi:for other_particle in self.next_pos.clone()
            {
                let pos = particle.xy();
                let other_pos = other_particle.xy();
                if pos == other_pos
                {
                    continue 'papi;
                }
                else 
                {
                    //calculate force of gravity on every other particle and add it to a vector.
                    let dvec = pos - other_pos;

                    if  dvec.length() < 5.0 && dvec.length() > 0.1 
                    {   
                        let f = (E)/ dvec.length().powf(2.0);
                        otherforce+= dvec.normalize().mul(f);
                        continue 'papi;

                    }
                    else if dvec.length() > 0.2
                    {
                        let f = (G)/ dvec.length().powf(2.0);
                        force-= dvec.normalize().mul(f);
                        continue 'papi;


                        
                    }


                }
            }
            physics_server.body_apply_central_impulse(self.body_rids[index], (otherforce+force).mul(delta));
            index+=1;

        }
    }

    #[export]
    fn _body_moved(&mut self, _owner: &TileMap, state:Variant, index: usize)
    {

           //  godot_print!("{}",index);
            //unpack variant which actually holds Physics2DDirectBodyState, which inherits object class.
            //must specify the type 
            let state:Option<Ref<Physics2DDirectBodyState>> = state.try_to_object();
            let state = unsafe{state.unwrap().assume_safe()};
        //  godot_print!("{:#?}",state.transform());
            let state_transform = state.transform();

            let particle = Vector3::new(state_transform.m31, state_transform.m32, 4.1);
            //this should only get called when needed
    //   godot_print!("{}",particle.x);

            self.next_pos[index] = particle;
        //as soon as last tick ends, get positions.
        if self.next_tick == true //&& //index != self.last_index
        {

            if index == self.next_pos.len()-1
                {
                    self.next_tick = false;
                    self.update_particles()
                }
       
        }

    }


    #[export]
    fn _enter_tree(&self, _owner: &TileMap)
    {
        godot_print!("printfirstplzzzzzz");
        
    }

    #[export]
    fn _physics_process(&mut self, _owner: &TileMap, delta: f32)
    {
        self.sweet_gravity(delta);


        
    }

    #[export]
    fn _on_timer_timeout(&mut self, _owner: TRef<TileMap>)
    {
        self.next_tick = true;

        godot_print!("swag plz dont crash");

        let particles = &self.sand_pos.clone();
        let last_p = &self.sand_pos_last.clone();
       // _owner.get_us
        self.display_particles(particles, _owner, last_p);

       
    }

}

    // //this tracks sand and updates mass
    // fn track_sand(&mut self,_owner: &TileMap)
    // {
    
    //     let sand_cells = _owner.get_used_cells_by_id(4);
    //     godot_print!("got used cells");
    //    // self.sand_mass = sand_cells.len()*32;
    //     self.sand = sand_cells;
    //     godot_print!("set used cells");


    // }

    // fn sand_physics(&self,_owner: &TileMap)
    // {
    //     //multithread? does each cell have independent state?
    //     for sand in self.sand.iter()
    //     {
    //         let sand = sand.to_vector2();

    //         //all this logic will be replaced
    //         let rotationfactor = self.find_rotation(&sand);
    //         let mut rawrf = rotationfactor as i32;
    //         let mut by = self.move_options(&sand);
    //         let mut moveoptions = by.clone();
    //         let by = self.neighbors(&mut by, &mut rawrf, _owner);
    //         let cell = _owner.get_cellv(sand);
    //         rawrf = rotationfactor as i32;
    //         moveoptions = self.relative_move(&mut moveoptions, &mut rawrf);

    //         if by[4] == -1
    //         {
    //             _owner.set_cellv(moveoptions[4], cell, false, false, false);
    //             _owner.set_cellv(sand, -1, false, false, false);

    //         }
    //         else if by[5] == -1
    //         {
    //             _owner.set_cellv(moveoptions[5], cell, false, false, false);
    //             _owner.set_cellv(sand, -1, false, false, false);
 
    //         }
    //         else if by[3] == -1
    //         {
    //             _owner.set_cellv(moveoptions[3], cell, false, false, false);
    //             _owner.set_cellv(sand, -1, false, false, false); 
                
    //         }
    //         else
    //         {
    //             continue;
    //         }

            
    //     }
    //     godot_print!("physicsd");

    // }

    // //this defines a cell's downward motion
    // fn find_rotation(&self,cell: &Vector2) -> Direction
    // {
    //     let cellangle = cell.angle_from_x_axis().radians;

    //     if -5.0*3.1416/8.0 <= cellangle && cellangle < -3.0*3.1416/8.0
    //     {
    //         return Direction::Top
    //     }
    //     else if -3.0*3.1416/8.0 <= cellangle && cellangle < -3.1416/8.0
    //     {
    //         return Direction::TopRight
    //     }
    //     else if -3.1416/8.0 <= cellangle && cellangle < 3.1416/8.0
    //     {
    //         return Direction::Right
    //     }
    //     else if 3.1416/8.0 <= cellangle && cellangle < 3.0*3.1416/8.0
    //     {
    //         return Direction::DownRight
    //     }
    //     else if 3.0*3.1416/8.0 <= cellangle && cellangle < 5.0*3.1416/8.0
    //     {
    //         return Direction::Down
    //     }
    //     else if 5.0*3.1416/8.0 <= cellangle && cellangle < 7.0*3.1416/8.0
    //     {
    //         return Direction::DownLeft;
    //     }
    //     else if 7.0*3.1416/8.0 <= cellangle && cellangle < 3.1416
    //     {
    //         return Direction::Left;
    //     }
    //     else
    //     {
    //         return Direction::TopLeft
    //     }

        
    // }

    // //provides a vec of new coordinates to move, is also true neighbors
    // fn move_options(&self, cell: &Vector2) -> Vec<Vector2>
    // {
    //     vec![
    //         Vector2::new(cell.x, cell.y-1.0),
    //         Vector2::new(cell.x+1.0,cell.y-1.0),
    //         Vector2::new(cell.x+1.0,cell.y),
    //         Vector2::new(cell.x+1.0,cell.y+1.0),
    //         Vector2::new(cell.x,cell.y+1.0),
    //         Vector2::new(cell.x-1.0,cell.y+1.0),
    //         Vector2::new(cell.x-1.0,cell.y),
    //         Vector2::new(cell.x-1.0,cell.y-1.0)
    //     ]
    // }
    // //provides neighbors after rotation transform
    //  //tail recursion!!! poggies champlord
    // fn neighbors(&self, by: &mut Vec<Vector2>, rotationfactor: &mut i32, _owner: &TileMap) -> Vec<i64>
    // {
    //     if *rotationfactor == 0
    //     {
    //         return self.get_neighbors(by,_owner);
    //     }
    //     else
    //     {
    //         let val = by.remove(0);
    //         by.push(val);
    //         *rotationfactor += -1;
    //         return self.neighbors(by, rotationfactor, _owner);
    //     }


    // }

    // //this can be much smarter
    // //reduce useage by tracking duplicate neighbors (if cell 0,0 neighbors 1,0, then 1,0 obviously has a neighbor
    //     // at 1,0 so no need to check there)

    // //ignore neigbors that we arent moving towards
    
    // fn get_neighbors(&self, by: &Vec<Vector2>, _owner: &TileMap) -> Vec<i64>
    // {   
    //     vec![
    //         _owner.get_cellv(by[0]),
    //         _owner.get_cellv(by[1]),
    //         _owner.get_cellv(by[2]),
    //         _owner.get_cellv(by[3]),
    //         _owner.get_cellv(by[4]),
    //         _owner.get_cellv(by[5]),
    //         _owner.get_cellv(by[6]),
    //         _owner.get_cellv(by[7]),
    //     ]
    // }

    // fn relative_move(&self, moveoptions: &mut Vec<Vector2>, rotationfactor: &mut i32) -> Vec<Vector2>
    // {
    //     if *rotationfactor == 0
    //     { 
    //         return moveoptions.clone();
    //     }
    //     else
    //     {
    //         let val = moveoptions.remove(0);
    //         moveoptions.push(val);
    //         *rotationfactor += -1;
    //         return self.relative_move(moveoptions, rotationfactor);
    //     }


    // }

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


