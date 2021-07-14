use core::f32;
use std::{ops::{Deref, Mul}};

//import legion ECS crate
use legion::*;
//import godot api stuffs
use gdnative::{api::{Physics2DDirectBodyState, Physics2DServer}, prelude::*};


//Trait thats will be implemented for legion::internals::Entity
//used to attach entity as userdata for motion callback of bodies, so the callback can access associated entity in ECS world 
trait BitsExt
{
    fn to_bits(self) -> u64;


    fn from_bits(bits:u64) -> Self;

}


impl BitsExt for Entity
{
    fn to_bits(self) -> u64
    {
        let n:u64;
        unsafe{
            n = std::mem::transmute::<Entity, u64>(self);
        };
        return n;
    }

    fn from_bits(bits:u64) -> Self
    {
        unsafe{std::mem::transmute::<u64,Entity>(bits)}
        
    }

}

//GAME COMPONENTS
//these are structs that are operated on as data.

//CurrentPosition, the most upto date position
#[derive(Clone, Copy, Debug, PartialEq)]
struct CurrentPosition
{
    position: Vector2
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct LastPosition
{
    positon: Vector2
}
#[derive(Clone, Copy, Debug, PartialEq)]

//Godots resource handle
struct BodyRID
{
    rid: Rid
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct ShapeRID
{
    rid: Rid
}




/// The SpaceMagic "class"
//every class to be used in godot needs one of these structs
#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct SpaceMagic 
{
    ecs_world: World,
    valid_entities: Vec<Entity>,
    ecs_resources: Resources,
    ecs_schedule: Schedule,

}


const G:f32 = 100.0;
const E:f32 = 4.0;
const EULERNUM:f64 = 1.2;


// You may add any number of ordinary `impl` blocks as you want. However, ...

// Only __one__ `impl` block can have the `#[methods]` attribute per class / script, which
// will generate code to automatically bind any exported methods to Godot.
//this is the entry point kinda for the program?
//its the "front end" of our native module
#[methods]
impl SpaceMagic 
{

    fn new(_owner: &Node2D) -> Self {

        //init ecs world, resouces, and schedule
        let ecs_world = World::default();

        let ecs_resources = Resources::default();

        let ecs_schedule = Schedule::builder()
        .add_system(sweet_gravity_system())
        .build();




        SpaceMagic
        {
            ecs_world,
            valid_entities: vec![],
            ecs_resources,
            ecs_schedule,
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
    fn _ready(&mut self, _owner: TRef<Node2D>) 
    {

        //get easy access to ecs world
        let ecs_world = &mut self.ecs_world;
        let x = 3;
        let y = 150;
       // let mut particles:Vec<Entity> = vec![];
        let space = 8;


             
           //get current world 2d to get space_rid to add particles to the game physics world 
           let world2d_ref = _owner.get_world_2d().unwrap();
           let world2d = unsafe{world2d_ref.assume_safe()};
           let world2d = world2d.deref();
           let space_rid = world2d.space();
           //create reference to accsess physics server
           let physics_server = unsafe{Physics2DServer::godot_singleton()};
           //create body 
           let mut index = 0usize;


        for i in 0..y 
        {
            for j in 0..x
            {
                let position = Vector2::new(i as f32*space as f32, j as f32*space as f32);
                let (body,shape) = add_particle_to_godot_world(physics_server, _owner, space_rid, position);
                let entity = ecs_world.push((
                    CurrentPosition{position:position},
                    LastPosition{positon:Vector2::zero()},
                    BodyRID{rid:body},
                    ShapeRID{rid:shape}
      
                 ));
                self.valid_entities.push(entity);

             //set call back and use entity as userdata
             physics_server.body_set_force_integration_callback(body, _owner, "_body_moved", entity.to_bits());
            }
    
        }

    }


    #[export]
    fn _body_moved(&mut self, _owner: &Node2D, state:Variant, data: u64)
    {       

            let ecs_world = &mut self.ecs_world;
            let entity = Entity::from_bits(data);


            //unpack variant which actually holds Physics2DDirectBodyState, which inherits object class.
            //must specify the type 
            let state:Option<gdnative::Ref<Physics2DDirectBodyState>> = state.try_to_object();
            let state = unsafe{state.unwrap().assume_safe()};


            let space_state = state.get_space_state().unwrap();
            let space_state = unsafe{space_state.assume_safe()};





           let contacts_count = state.get_contact_count();

        //    // Check if we've made contact
        //     if contacts_count > 0
        //     {

        //         //WE'VE BEEN HIT!
        //         for contact in 0..contacts_count
        //         {
        //             //for now, for each contact just remove it from the simulation and 
        //             let physics_server = unsafe{Physics2DServer::godot_singleton()};

        //             //this is rid of body colliding with object
        //             let body_rid = state.get_contact_collider(contact);
        //             let shape_rid = physics_server.body_get_shape(body_rid, 0);

        //             if let Some(body_entry) = ecs_world.entry(entity)
        //             {

                    

        //             //this is RID of this body
        //             let self_body = body_entry.get_component::<BodyRID>().unwrap().rid;
        //             let self_shape = body_entry.get_component::<ShapeRID>().unwrap().rid;


        //             let collider_speed = physics_server.body_get_state(body_rid, Physics2DServer::BODY_STATE_LINEAR_VELOCITY).to_vector2();
        //             let body_speed = state.linear_velocity();
        //             if body_speed.length() > collider_speed.length() && body_speed.length() > 40.0
        //             {
        //                 godot_print!("shits_happening, rid of deleted object: {:?}", body_rid);
        //                 //this object is moving faster
        //                 //so delete slower one and add its mass and size to this one
        //                 let add_mass = physics_server.body_get_param(body_rid, Physics2DServer::BODY_PARAM_MASS);
        //                 let add_radius = physics_server.shape_get_data(shape_rid).to_f64();

        //             //    physics_server.shape_set_data(*self.shape_rids.get(index).unwrap(), (add_radius+physics_server.shape_get_data(*self.shape_rids.get(index).unwrap()).to_f64()).to_variant());
        //                 physics_server.body_set_param(self_body, Physics2DServer::BODY_PARAM_MASS, physics_server.body_get_param(self_body, Physics2DServer::BODY_PARAM_MASS)+ add_mass);

        //                 //set collided body mass to zero to make it get deleted.
        //                 physics_server.body_set_param(body_rid, Physics2DServer::BODY_PARAM_MASS,0.001);
                        

                        

        //             }
                    
        //             else if physics_server.body_get_param(self_body,Physics2DServer::BODY_PARAM_MASS) < 0.1
        //             {
                        
        //                 //that means im the object thats moving slower... in a collision... i dont feel so good.
        //                 godot_print!("DELETING, rid of deleted object: {:?}", self_body);

        //                 physics_server.free_rid(self_body);
        //                 physics_server.free_rid(self_shape);

        //                 ecs_world.remove(entity);
  
        //                 let shit_index =  self.valid_entities.iter().position(|x| x == &entity).unwrap();
        //                 self.valid_entities.remove(shit_index);
                   

        //             }
                    
        //         }
        //         }
        //     }
          

            //get transform which contains position
            let state_transform = state.transform();

            //get new positon
            let particle = Vector2::new(state_transform.m31, state_transform.m32);

        
            //set new position
            if let Some(body_entry) = ecs_world.entry(entity)
            {

            let stuff = body_entry.into_component_mut::<CurrentPosition>().unwrap();

            stuff.position = particle;
            }
    }


    #[export]
    fn _enter_tree(&self, _owner: &Node2D)
    {
        godot_print!("printfirstplzzzzzz");
        
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Node2D, delta: f32)
    {
        
        let newlist:Vec<CurrentPosition> = update_position_list(&self.ecs_world);

        self.ecs_resources.insert::<f32>(delta);
        self.ecs_resources.insert::<Vec<CurrentPosition>>(newlist);

        
        self.ecs_schedule.execute(&mut self.ecs_world, &mut self.ecs_resources);

        _owner.update();

        

    


        
    }


    //this is called everytime _owner.update() is called in physics processing method
    //redraws positions every time physics happpens
    #[export]
    fn _draw(&self, _owner: &Node2D)
    {
        //an octogon
        let mut colors = ColorArray::new();
        colors.push(Color::rgb(0.1, 0.1, 0.9));
        colors.push(Color::rgb(0.1, 0.1, 0.9));
        colors.push(Color::rgb(0.1, 0.1, 0.9));
        colors.push(Color::rgb(0.1, 0.1, 0.9));
        colors.push(Color::rgb(0.9, 0.1, 0.0));
        colors.push(Color::rgb(0.9, 0.1, 0.0));
        colors.push(Color::rgb(0.9, 0.1, 0.0));
        colors.push(Color::rgb(0.9, 0.1, 0.0));

        //octogon maths
        //s can be set to custom length and all other values will be derived from it
        let s = 1.8f32;
        let a = s/2.414214;
        let b = a/2.0f64.sqrt() as f32;
        let ab = a+b as f32;

        let mut query = Read::<CurrentPosition>::query();

        for particle in query.iter(&self.ecs_world)
        {
            let particle = particle.position;

    
            let mut points = Vector2Array::new();
            points.push(Vector2::new(b-0.5*s, 0.0-0.5*s)+particle);
            points.push(Vector2::new(ab-0.5*s, 0.0-0.5*s)+particle);
            points.push(Vector2::new(s-0.5*s, b-0.5*s)+particle);
            points.push(Vector2::new(s-0.5*s, ab-0.5*s)+particle);
            points.push(Vector2::new(ab-0.5*s, s-0.5*s)+particle);
            points.push(Vector2::new(b-0.5*s, s-0.5*s)+particle);
            points.push(Vector2::new(0.0-0.5*s, ab-0.5*s)+particle);
            points.push(Vector2::new(0.0-0.5*s, b-0.5*s)+particle);


            _owner.draw_polygon(points, colors.clone(),Vector2Array::new(),Null::null(),Null::null(),false);

        }
        
    }

}

//funcion used to add particle to godot world
//it returns RID to body and shape every time it adds one
fn add_particle_to_godot_world(physics_server:&Physics2DServer, _owner: TRef<Node2D>,space_rid:Rid, positon: Vector2 ) -> (Rid,Rid)
    {
    //create body
    let body = physics_server.body_create();

    physics_server.body_set_mode(body, Physics2DServer::BODY_MODE_RIGID);

    //create shape
    let shape = physics_server.circle_shape_create();
    //set radius to one
    physics_server.shape_set_data(shape, 1.0.to_variant());

    //add shape to body for collision
    physics_server.body_add_shape(body, shape, Transform2D::new(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)   , false);
    //add body to space for collision
    physics_server.body_set_space(body, space_rid);
    //move initial position
    physics_server.body_set_state(body, Physics2DServer::BODY_STATE_TRANSFORM, Transform2D::new(1.0,0.0,0.0,1.0,positon.x ,positon.y));
    //add callback to self that the body moved, last parameter is optional and can be used as index
    //if many bodies are there and a single callback?`
    physics_server.body_set_param(body, Physics2DServer::BODY_PARAM_LINEAR_DAMP, 0.0);
    physics_server.body_set_state(body, Physics2DServer::BODY_STATE_CAN_SLEEP, true);
    physics_server.body_set_param(body, Physics2DServer::BODY_PARAM_BOUNCE, 1.0);
    physics_server.body_set_param(body, Physics2DServer::BODY_PARAM_FRICTION, 0.0);
    physics_server.body_set_max_contacts_reported(body, 6);

    physics_server.body_set_continuous_collision_detection_mode(body, Physics2DServer::CCD_MODE_CAST_SHAPE);
    // let rb = body.get
        
    // let other:TRef<RigidBody2D> = unsafe{rb.try_to_object().unwrap().assume_safe()};
    // other.cotac
    //godot_print!("{:?}",rb);


    physics_server.body_apply_central_impulse(body, Vector2::new(0.0, 0.0));

    // self.display_particles(particles, _owner, last_particles);
    return (body,shape);
      
}

    

//this function gets all the positions of bodies currently in the world and adds them to a list.
//used in sweet_gravity system, which runs once per entity. each entity calculates force of gravity to all other particle posistions.
fn update_position_list(world:&World) -> Vec<CurrentPosition>
{
    let mut query = Read::<CurrentPosition>::query();
    let mut list:Vec<CurrentPosition> = vec![];

    for position in query.iter(world)
    {
        list.push(*position);

    }
    list
}

#[system(par_for_each)]
fn sweet_gravity(positon: &CurrentPosition,body: &BodyRID, #[resource] delta: &f32, #[resource] other_positons: &Vec<CurrentPosition>)
{

    let position = positon.position;
    let physics_server = unsafe{Physics2DServer::godot_singleton()};
    let mut force = Vector2::zero();
    let mut otherforce = Vector2::zero();
    'papi:for other_particle in other_positons
    {
        let pos = position;
        let other_pos = other_particle.position;
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
    //apply sum of forces to body
    physics_server.body_apply_central_impulse(body.rid, (otherforce+force).mul(*delta));

}    
