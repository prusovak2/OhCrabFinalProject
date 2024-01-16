# OhCrab final project
## Visualizer

* you can find the usage example in `src/oh_crab_visualizer/examples`

* in order to be usable with the visualizer, your robot must implement `Visualizable` trait. Trait requires one method `borrow_event_listener` that should just return a reference to `VisualizerEventListener`. Implementation looks as follows. 

  ```rust
  impl<'a> Visulizable<'a> for ExampleRobot {
      fn borrow_event_listener(&'a self) -> &'a VisualizerEventListener{
          &self.visualizer_event_listener
      }
  }
  ```

* Your robot instance will be created by the visualizer. Visualizer will provide `VisualizerEventListener` instance while creating your robot. To allow this you must implement a factory struct  that implements `RobotCreator` trait. Factory struct allows you to pass parameters to the creation of your robot.

  ```rust
  pub struct ExampleRobotFactory {
      some_param: i32,
  }
  
  impl ExampleRobotFactory {
      pub fn new(some_param: i32) -> ExampleRobotFactory {
          ExampleRobotFactory{some_param}
      }
  }
  
  impl RobotCreator for ExampleRobotFactory {
      fn create(&self, data_sender: VisualizerEventListener) -> Box<dyn Runnable> {
          let example_robot = ExampleRobot {properties: Robot::new(), tick_counter: 0, some_param:self.some_param, visualizer_event_listener: data_sender };
          Box::new(example_robot)
      }
  }
  ```

  * `create` method of `RobotCreator` trait will be be used by the visualizer to create your robot instance.

* Visualizer instance working with `example_robot` from above can be constructed as follows:
  ```rust
  let robot_factory = ExampleRobotFactory::new(42);
  let world_generator = crate::world_gen_utils::load_or_generate_world(15, 42);
  let config = OhCrabVisualizerConfig::new(20, false, false, 500);
  let mut visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);
  ```

  * fields of ` OhCrabVisualizerConfig` struct are likely to change a bit in the following days

* Visualizer implements

  * `run` method that simulates given number of world steps while visualizing them 
  * `simulate` method that only carries out simulation and does not do visualization

* **IMPORTANT**: 

  * in order to provide visualizer with the data it needs it is necessary to call `visualizer_event_listener.handle_event(&event);` method from `handle_event` method of your robot. Without it the visualizer cannot function.

    ```rust
    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        self.visualizer_event_listener.handle_event(&event); 
    }
    ```

  * in order to allow visualizer to use `history_cache` tool and `rizzler` tool it is necessary to invoke `robotic_lib` interfaces via `VisualizableInterfaces` wrappers e.g.

    ```rust
    VisualizableInterfaces::go(self, world, direction)
    ```

    
