#[derive(Debug)]
pub struct Resource {
    name: ResourceName,
    quantity: i32,
}

#[derive(Debug)]
pub enum ResourceName {
    Water,
    Milk,
    Coffee,
}

impl Resource {
    pub fn new_resource(name: ResourceName) -> Resource {
        Resource {
            name: name,
            quantity: 0,
        }
    }
    
    pub fn add_resource(&mut self, amount: i32) {
        self.quantity += amount;

    }

}
