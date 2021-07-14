



// this handles game input
//     its all the unhandled input, so if a ui captures and uses an event in _input, then this wont be able
//     to see that event
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
                    godot_print!("spacebarhomie");
                    godot_print!("{:?}", self.sand.len());
                }
                }
            }
        }

    }