use typescript_definitions::TypeScriptifyTrait;
use population_simulation::pop::components::RegionPop;

fn main() {
    println!("hi");
    let x = RegionPop::new(&[1./17.;17], 200);

    dbg!(RegionPop::type_script_ify());

}

