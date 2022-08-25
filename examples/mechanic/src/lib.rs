use nvim_oxi::{
    api,
    object,
    print,
    Dictionary,
    FromObject,
    Function,
    Object,
    Result,
    ToObject,
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

fn yep() -> bool {
    true
}

impl FromObject for Car {
    fn from_obj(obj: Object) -> Result<Self> {
        Self::deserialize(object::Deserializer::new(obj))
    }
}

impl ToObject for Car {
    fn to_obj(self) -> Result<Object> {
        self.serialize(object::Serializer::new())
    }
}

fn fix(mut car: Car) -> Result<Car> {
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
        _ => (),
    }

    car.works = true;
    car.problem = None;
    Ok(car)
}

#[nvim_oxi::module]
fn mechanic() -> Result<Dictionary> {
    Ok(Dictionary::from_iter([("fix", Function::from_fn(fix))]))
}
