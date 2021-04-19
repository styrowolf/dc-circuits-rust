mod stuff;

use stuff::*;

fn main() {
    /*
    let mut r1 = Resistor::new(3.0);
    let mut r2 = Resistor::new(4.0);
    let mut r3 = Resistor::new(5.0);
    let mut elements: Vec<Box<dyn CircuitElement>> = Vec::new();
    elements.push(Box::new(r1));
    elements.push(Box::new(IdealAmmeter::new()));
    let parallel = ParallelConnection::new(
        vec![Circuit::new(None, vec![Box::new(IdealVoltmeter::new())]),
        Circuit::new(None, vec![Box::new(r2)]),
        Circuit::new(None, vec![Box::new(r3)]),
        ]
    );
    elements.push(Box::new(parallel));

    let mut circuit = Circuit::new(Some(9.0), elements);
    circuit.calculate();
    println!("{:?}", circuit);
    */

    let pc = ParallelConnection::new(vec![
        Box::new(Circuit::new(None, vec![
            Box::new(Resistor::new(6.0)),
            Box::new(IdealAmmeter::new()),
            Box::new(Resistor::new(2.0)),
        ])),
        Box::new(Circuit::new(None, vec![
            Box::new(Resistor::new(2.0)),
            Box::new(IdealAmmeter::new()),
        ])),
        Box::new(Circuit::new(None, vec![
            Box::new(Resistor::new(4.0)),
            Box::new(IdealAmmeter::new()),
        ])),
        Box::new(Circuit::new(None, vec![
            Box::new(IdealVoltmeter::new())
        ])),
    ]);
    let mut mc = Circuit::new(Some(22.0), vec![
        Box::new(pc),
        Box::new(Resistor::new(2.0)),
    ]);
    mc.calculate();
    println!("{:#?}", mc);
}

