use nvim_oxi::{
    self as oxi,
    api,
    print,
    Deserializer,
    Dictionary,
    FromObject,
    FromObjectResult,
    Function,
    Object,
    Serializer,
    ToObject,
    ToObjectResult,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Car {
    manufacturer: CarManufacturer,

    miles: u32,

    #[serde(default)]
    problem: Option<CarProblem>,

    #[serde(default = "yep")]
    works: bool,
}

fn yep() -> bool {
    true
}

#[derive(Copy, Clone, Serialize, Deserialize)]
enum CarManufacturer {
    Nikola,
    Tesla,
    Volkswagen,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CarProblem {
    DoesntMove,
    KillsPedestrians,
    Pollutes,
}

impl FromObject for Car {
    fn from_obj(obj: Object) -> FromObjectResult<Self> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for Car {
    fn to_obj(self) -> ToObjectResult {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl oxi::lua::Poppable for Car {
    unsafe fn pop(
        lstate: *mut oxi::lua::ffi::lua_State,
    ) -> Result<Self, oxi::lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_obj(obj)
            .map_err(oxi::lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl oxi::lua::Pushable for Car {
    unsafe fn push(
        self,
        lstate: *mut oxi::lua::ffi::lua_State,
    ) -> Result<std::ffi::c_int, oxi::lua::Error> {
        self.to_obj()
            .map_err(oxi::lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

fn fix(mut car: Car) -> oxi::Result<Car> {
    if car.works {
        return Ok(car);
    }

    if car.problem.is_none() {
        api::err_writeln("Well, what's the issue?");
        return Ok(car);
    }

    use CarManufacturer::*;
    use CarProblem::*;

    match (car.manufacturer, car.problem.unwrap()) {
        (Nikola, DoesntMove) => print!("Try going downhill"),
        (Tesla, KillsPedestrians) => print!("Hands on the wheel!!"),
        (Volkswagen, Pollutes) => print!("Software update?"),
        _ => {},
    }

    car.works = true;
    car.problem = None;

    Ok(car)
}

#[nvim_oxi::module]
fn mechanic() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([("fix", Function::from_fn(fix))]))
}
