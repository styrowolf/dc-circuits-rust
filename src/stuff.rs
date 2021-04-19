mod private;
pub use private::Circuit as Circuit;
pub use private::ParallelConnection as ParallelConnection;
pub use private::Resistor as Resistor;
pub use private::IdealAmmeter as IdealAmmeter;
pub use private::IdealVoltmeter as IdealVoltmeter;

pub trait CircuitElement: private::InnerCircuitElement {
    fn calculate(&mut self) {
        self.calculate_inner();
    }
    fn resistance(&self) -> f64 {
        self.resistance_inner()
    }
}

impl CircuitElement for Circuit {}
impl CircuitElement for ParallelConnection {}
impl CircuitElement for Resistor {}
impl CircuitElement for IdealAmmeter {}
impl CircuitElement for IdealVoltmeter {}


