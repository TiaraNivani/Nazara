mod collectors;

use collectors::dmi_collector;

fn main() {
    let output: dmi_collector::DmiInformation = dmi_collector::dmidecode();
    println!("{:?}", output);
}