pub struct GameState {
    highroller: bool,
    chips: i32,
    luck: i32,
    inventory: [bool; 3],
}

impl GameState {
    pub fn new(highroller: bool, chips: i32, luck: i32) -> Self {
        GameState {
            highroller,
            chips,
            luck,
            inventory: [false, false, false],
        }
    }

    pub fn get_highroller(&self) -> bool {
        self.highroller
    }

    pub fn set_highroller(&mut self, highroller: bool) {
        self.highroller = highroller;
    }

    pub fn get_chips(&self) -> i32 {
        self.chips
    }

    pub fn set_chips(&mut self, chips: i32) {
        self.chips = chips;
    }

    pub fn get_luck(&self) -> i32 {
        self.luck
    }

    pub fn set_luck(&mut self, luck: i32) {
        self.luck = luck;
    }

    pub fn get_inventory(&self, index: usize) -> bool {
        self.inventory[index]
    }

    pub fn set_inventory(&mut self, index: usize, owned: bool) {
        self.inventory[index] = owned;
    }
}