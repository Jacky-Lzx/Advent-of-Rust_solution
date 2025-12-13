use std::marker::PhantomData;

// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Finish the Seligh struct definition

pub struct Sleigh<State> {
    _state: PhantomData<State>,
}

// 3. Write the Sleigh Implementations for all states
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self {
            _state: PhantomData,
        }
    }

    pub fn load(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Ready> {
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh {
            _state: PhantomData,
        }
    }

    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

impl Sleigh<Flying> {
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh {
            _state: PhantomData,
        }
    }
}

fn main() {
    // 合法流程：Empty → Ready → Flying → Ready → Empty
    let sleigh = Sleigh::new(); // Empty
    let sleigh = sleigh.load(); // Ready
    let sleigh = sleigh.take_off(); // Flying
    let sleigh = sleigh.land(); // Ready
    let _ = sleigh.unload(); // Empty

    // 非法操作：Flying 状态调用 take_off（编译报错）
    // let sleigh = Sleigh::new().load().take_off().take_off();
    // 错误提示：no method named `take_off` found for struct `Sleigh<Flying>` in the current scope
}
