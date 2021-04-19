use crate::stuff::*;

pub trait InnerCircuitElement: std::fmt::Debug {
    fn resistance_inner(&self) -> f64;
    fn voltage_inner(&self) -> Option<f64>;
    fn current_inner(&self) -> Option<f64>;
    fn set_voltage(&mut self, v: f64);
    fn set_current(&mut self, i: f64);
    fn calculate_current(&mut self);
    fn calculate_inner(&mut self);
}

#[derive(Debug)]
pub struct Circuit {
    pub elements: Vec<Box<dyn CircuitElement>>,
    pub incoming_voltage: Option<f64>,
    pub resistance: Option<f64>,
    pub current: Option<f64>,
}

impl Circuit {
    fn calculate_resistance(&self) -> f64 {
        let mut r_eq = 0.0_f64;
        for e in &self.elements {
            r_eq += e.resistance_inner();
        }
        r_eq
    }

    pub fn new(v: Option<f64>, e: Vec<Box<dyn CircuitElement>>) -> Self {
        Circuit {
            incoming_voltage: v,
            elements: e,
            resistance: None,
            current: None
        }
    }
}

#[derive(Debug)]
pub struct ParallelConnection {
    pub circuits: Vec<Box<dyn CircuitElement>>,
    pub incoming_voltage: Option<f64>,
    pub resistance: Option<f64>,
    pub current: Option<f64>,
}

impl ParallelConnection {
    pub fn new(c: Vec<Box<dyn CircuitElement>>) -> Self {
        ParallelConnection {
            circuits: c,
            incoming_voltage: None,
            current: None,
            resistance: None,
        }
    }
}

impl InnerCircuitElement for Circuit {
    fn resistance_inner(&self) -> f64 {
        if let Some(r) = self.resistance {
            r
        } else {
            self.calculate_resistance()
        }
    }

    fn voltage_inner(&self) -> Option<f64> {
        self.incoming_voltage
    }

    fn current_inner(&self) -> Option<f64> {
        self.current
    }

    fn set_voltage(&mut self, v: f64) {
        self.incoming_voltage = Some(v);
    }

    fn set_current(&mut self, i: f64) {
        self.current = Some(i);
    }

    fn calculate_current(&mut self) {
        /* self.current = Some(
            self.voltage().unwrap() / self.resistance_inner()
        ); */
    }

    fn calculate_inner(&mut self) {
        self.resistance = Some(self.calculate_resistance());
        self.current = Some(self.incoming_voltage.unwrap()
            / self.resistance.unwrap());

        let mut resistances = Vec::new();
        for e in &self.elements {
            resistances.push(e.resistance_inner());
        }
        let length = self.elements.len();

        for i in 0..length {
            let e = self.elements.get_mut(i).unwrap();
            if length == 1 {
                e.set_voltage(self.incoming_voltage.unwrap())
            } else {
                e.set_voltage(
                    self.current.unwrap() * resistances.get(i).unwrap()
                );
            }
            e.set_current(self.current.unwrap());
            e.calculate_current();
            e.calculate();
        }

    }
}

impl InnerCircuitElement for ParallelConnection {
    fn resistance_inner(&self) -> f64 {
        let mut r_eq = 0.0_f64;
        for r in &self.circuits {
            r_eq += 1.0 / r.resistance_inner();
        }
        r_eq.powi(-1)
    }

    fn voltage_inner(&self) -> Option<f64> {
        self.incoming_voltage
    }

    fn current_inner(&self) -> Option<f64> {
        self.current
    }

    fn set_voltage(&mut self, v: f64) {
        self.incoming_voltage = Some(v);
    }

    fn set_current(&mut self, i: f64) {
        self.current = Some(i);
    }

    fn calculate_current(&mut self) {
        /* self.current = Some(
            self.voltage_inner().unwrap() / self.resistance_inner()
        ); */
        let voltage = self.voltage_inner().unwrap();

        for c in &mut self.circuits {
            c.set_voltage(voltage);
            c.set_current(voltage / c.resistance_inner());
            c.calculate();
        }
    }

    fn calculate_inner(&mut self) {
        self.resistance = Some(self.resistance_inner())
    }

}

#[derive(Debug, Copy, Clone)]
pub struct IdealVoltmeter {
    pub incoming_voltage: Option<f64>,
    pub current: Option<f64>,
    pub resistance: f64,
}

impl Default for IdealVoltmeter {
    fn default() -> Self {
        IdealVoltmeter {
            incoming_voltage: None,
            current: None,
            resistance: f64::INFINITY,
        }
    }
}

impl IdealVoltmeter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl InnerCircuitElement for IdealVoltmeter {
    fn resistance_inner(&self) -> f64 {
        self.resistance
    }

    fn voltage_inner(&self) -> Option<f64> {
        self.incoming_voltage
    }

    fn current_inner(&self) -> Option<f64> {
        self.current
    }

    fn set_voltage(&mut self, v: f64) {
        self.incoming_voltage = Some(v);
    }

    fn set_current(&mut self, i: f64) {
        self.current = Some(i);
    }

    fn calculate_current(&mut self) {
        /* self.current = Some(
            self.voltage_inner().unwrap() / self.resistance_inner()
        ); */
    }

    fn calculate_inner(&mut self) {
        /* placeholder */
    }
}

#[derive(Debug, Copy, Clone)]
pub struct IdealAmmeter {
    pub incoming_voltage: Option<f64>,
    pub current: Option<f64>,
    pub resistance: f64,
}

impl Default for IdealAmmeter {
    fn default() -> Self {
        IdealAmmeter {
            incoming_voltage: None,
            current: None,
            resistance: 0.0,
        }
    }
}

impl IdealAmmeter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl InnerCircuitElement for IdealAmmeter {
    fn resistance_inner(&self) -> f64 {
        self.resistance
    }

    fn voltage_inner(&self) -> Option<f64> {
        self.incoming_voltage
    }

    fn current_inner(&self) -> Option<f64> {
        self.current
    }

    fn set_voltage(&mut self, v: f64) {
        self.incoming_voltage = Some(v);
    }

    fn set_current(&mut self, i: f64) {
        self.current = Some(i);
    }

    fn calculate_current(&mut self) {
        /* self.current = Some(
            self.voltage().unwrap() / self.resistance_inner()
        ); */
    }

    fn calculate_inner(&mut self) {
        /* placeholder */
    }

}

#[derive(Debug, Copy, Clone)]
pub struct Resistor {
    pub incoming_voltage: Option<f64>,
    pub current: Option<f64>,
    pub resistance: f64,
}

impl Resistor {
    pub fn new(r: f64) -> Self {
        Resistor {
            incoming_voltage: None,
            current: None,
            resistance: r,
        }
    }
}

impl InnerCircuitElement for Resistor {
    fn resistance_inner(&self) -> f64 {
        self.resistance
    }

    fn voltage_inner(&self) -> Option<f64> {
        self.incoming_voltage
    }

    fn current_inner(&self) -> Option<f64> {
        self.current
    }

    fn set_voltage(&mut self, v: f64) {
        self.incoming_voltage = Some(v);
    }

    fn set_current(&mut self, i: f64) {
        self.current = Some(i);
    }

    fn calculate_current(&mut self) {
        /* self.current = Some(
            self.voltage_inner().unwrap() / self.resistance_inner()
        ); */
    }

    fn calculate_inner(&mut self) {
        /* placeholder */
    }
}