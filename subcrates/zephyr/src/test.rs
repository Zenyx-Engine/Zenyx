  pub(crate) trait Base<T> {
      fn new() -> Self;
      fn get_name(&self) -> String;
      fn get_component(&self) -> T;
      fn get_component_mut(&mut self) -> &mut T;
  }

  pub(crate) trait Update<T> {
    fn update(&mut self, component: T,delta: f32);
    fn fixed_update(&mut self, component: T,fixed_delta: f32);
  }

  // Example component implementation
  pub(crate) struct ExampleComponent {
      value: i32,
      f: usize,
      name: String,
  }

  impl ExampleComponent {
      fn thing(&self) -> usize {
          self.f

      }
  }

  impl Base<ExampleComponent> for ExampleComponent {
      fn new() -> Self {
          ExampleComponent {
              value: 0,
              name: String::from("New Name"),
              f: 9,
          }
      }

      fn get_name(&self) -> String {
          self.name.clone()
      }

      fn get_component(&self) -> ExampleComponent {
          ExampleComponent {
              value: self.value,
              name: self.name.clone(),
              f: 5,
          }
      }

      fn get_component_mut(&mut self) -> &mut ExampleComponent {
          self
      }
  }

  impl Update<ExampleComponent> for ExampleComponent {
      fn update(&mut self, component: ExampleComponent, delta: f32) {
          self.value = component.value;
          self.name = component.name;
          self.f = component.f;
      }

      fn fixed_update(&mut self, component: ExampleComponent, fixed_delta: f32) {
          self.value = component.value;
          self.name = component.name;
          self.f = component.f;
      }
  }

  #[test]
  fn test_example_component() {
      let mut example = ExampleComponent::new();
      let t = example.thing();
      
      
      // Test initial values
      assert_eq!(example.get_name(), "New Name");
      assert_eq!(example.value, 0);
      
      // Test mutable access
      example.get_component_mut().value = 42;
      assert_eq!(example.value, 42);
      
      // Test component cloning
      let cloned = example.get_component();
      assert_eq!(cloned.value, 42);
      assert_eq!(cloned.name, "New Name");
  }