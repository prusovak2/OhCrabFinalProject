# OhCrab presentation notes
* our crates published in university registry, with all documentation necessary

## Collection tool
* allows user to collect specific amount of content, if the collection meets given energy criteria
* uses A\* path finding algorithm, Manhattan distance
* if the target is on given Tile, it moves the robot on the cheapest tile around
* returns error if not meeting energy criterie, backpack is fule, given content is not destroyable or it is not found in the world

## Weather tool
* predicts weather
    * you can ask it what the weather will be like in N ticks
    * or you can ask in how many ticks there will be some specific weather
* basically an exploit in robotic-lib 
  * `tick` method on `EnviromentalConditions` is `pub` (it should probably be `pub(crate)`) and  `EnviromentalConditions` is clonable.
  * original idea was to get a vector of `WeatherType`s and count the weather based on number of ticks, but weather type vector cannot be obtained and without knowing its length we cannot really compute it?

## Visualizer

* graphical application for displaying the state of the world and the robot

### Showcase
* I'll use mock robot that only tries to pick up and put down water and coins, looks at the sky, and walks straight until it reaches an obstacle and then it turns. 
* `visualizer-interactive`
    * visulizer has two modes - interactive simulates one world tick when a user presses `do tick` button. Simulate mode simulates given number of ticks not requiring user input. Lets start with interactive mode. 
    * robot, contents on tiles - can be switched to labels or hidden. 
    * backpack
    * energy bar 
        * energy balance per tick
    * time
    * used tools
        * `oxag_audio_tool` by `Oxidizing Agents` to make sounds
        * `history-cache` by `Rusty Krab` to display history of robot actions
        * `rizzler` by `Rust and furious` to display messages from robot
    * world can be scrolled
    * and zoomed
    * and zoomed out -zoom out to see the whole world - wolrd size 256
* `visualizer-simulate`
    * same world, same robot in non-interactive mode
* `visualizer-smaller`
    * world size 128
    * simulation with the sounds turned off

### Implementation

* libraries
    * `ggez`
        * a framework for 2D games development
        * is used for rendering the tile grid and handling the main update/draw loop
        * I chose it because it has reasonably straightforward API and is in active development
    * `egui`
        * the most popular Rust UI library
        * provides set of readymade UI components (windows, buttons, sliders, etc.) I used to provide control over the application 
        * (it uses the so-called immediate mode UI, i.e.) it updates and redraws the UI every frame
            * this simplifies UI event handling and means that it can be integrated with `ggez` renderer

* architecture
    * I used ready made gui component s from egui for cotrol elements, but I draw the tile grid "manualy" (and compute scrolling and zooming...)
    * the main design problem was how to collect all the data the visualizer needs
        * initilize world map at the beggining
        * intercept every robotic-lib event (robot moved, enrgy consumed, time changed)
        * intercept every interface invocation - get data from `rizzler` tool and for `history-cache` tool
    * I use `std::sync::mpsc::Channel` to send data to the visualizer
        * to avoid necessity to have a cyclic references between robot and visualuizer and being stuck in ownership hell (visulizer needs reference to robot to run the simulation, robot needs reference to visualizer to pass it data) 
        *   each non-empty `update` method invocation processes one item in the channel
    * I wanted to make the visualizer as easy to use for robot developers as possible
    * visualizer runs the world simulation, this is how is it used
        ```rust
        let robot_factory = ExampleRobotFactory::new(42);
        let world_generator = crate::world_gen_utils::load_or_generate_world(128, 420);
        
        let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(400), false);
        let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);
        
        match visualizer.run() {
            Ok(_) => {}
            Err(err) => println!("Visualizer run returned error {:?}", err),
        }
        ```
        * user must implement struct that implements `RobotCreator` trait that requires method, that creates robot instance, given instance of `VisualizerEventListener`. visualizer uses this method to create robot instance it self
        ```rust
        pub trait RobotCreator {
            fn create(&self, event_listener: VisualizerEventListener) -> Box<dyn Runnable>;
        }
        ```
        * `VisualizerEventListener` uses `Channel` to send world data to visualizer
        * to ensure that the world map gets send to the visualizer before the first robots action, visualizer wraps robot in its own `VisualizableRobot` before passing it to `robotic-lib` `Runner`.
            * the original idea was to use this trick to intercept world `events` as well, but events get issued from interfaces invocations on the robot instance that invoked the interface (not on the robot instance that was passed to the `Runner` when the `Runner`) was created. Due to that `VisualizableRobot` wrapper can only be used to intercept invoations of `robot.update` and cannot be used to intercept `robot.handle_event`. User is requiered to invoke `visualizer_event_listener.handle_event` method from `handle_event` method of their robot.
        * `VisualizableInterfaces` - to intercept interaface invocations for `rizzler` and `history_cache`. 

## Distribution Robot
### Overall idea and motivation
* Aims to collect Trees, Rocks and Fishes
* Distribute them into all markets in the world the most equal way
* Evolutionary algorithm to solve partitioning problem
* 3 different phases:
    * exploration phase
    * algorithm solving phase
    * distribution phase